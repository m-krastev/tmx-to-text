use clap::{Parser, ValueEnum};

use csv::Writer;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

use quick_xml::{events::Event, reader::Reader};

/// Blazing fast TMX parser utility
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true, help = "TMX file to parse")]
    file: String,

    #[arg(short, long)]
    src: String,

    #[arg(short, long)]
    tgt: String,

    /// Output file
    #[arg(short, long)]
    output: String,

    // /// Print debug information
    // #[arg(short, long, default_value_t = false)]
    // debug: bool,
    /// Encoding of the input file, either utf8 or utf16. If utf8, the parser can be faster.
    #[arg(short, long)]
    encoding: Encoding,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
enum Encoding {
    Utf8,
    Utf16,
}

fn main() {
    let matches: Args = Args::parse();
    dbg!(&matches);

    let file_path = matches.file;
    let src: String = matches.src.to_uppercase();
    let tgt: String = matches.tgt.to_uppercase();
    let output_path: String = matches.output;
    // let debug: bool = matches.debug;
    let encoding = matches.encoding;

    let file = File::open(file_path).expect("Unable to open file");
    let file = BufReader::new(file);
    let mut srcpile: Vec<String> = Vec::new();
    let mut tgtpile: Vec<String> = Vec::new();
    let mut current_lang = String::new();
    let mut current_text = String::new();

    match encoding {
        Encoding::Utf16 => {
            let parser = EventReader::new(file);

            for e in parser {
                match e {
                    Ok(XmlEvent::StartElement {
                        name, attributes, ..
                    }) => {
                        if name.local_name == "tuv" {
                            // First attribute is the language
                            current_lang = attributes[0].value.to_owned();
                        } else if name.local_name == "seg" {
                            current_text.clear();
                        }
                    }
                    Ok(XmlEvent::Characters(data)) => {
                        current_text.push_str(&data);
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if name.local_name == "tuv" {
                            if current_lang.starts_with(&src) {
                                srcpile.push(current_text.to_owned());
                            } else if current_lang.starts_with(&tgt) {
                                tgtpile.push(current_text.to_owned());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        }
        Encoding::Utf8 => {
            let mut reader = Reader::from_reader(file);
            reader.config_mut().trim_text(true);
            let mut buf = Vec::new();
            // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
            loop {
                // NOTE: this is the generic case when we don't know about the input BufRead.
                // when the input is a &str or a &[u8], we don't actually need to use another
                // buffer, we could directly call `reader.read_event()
                match reader.read_event_into(&mut buf) {
                    Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                    // exits the loop when reaching end of file
                    Ok(Event::Eof) => break,

                    Ok(Event::Start(e)) => match e.name().as_ref() {
                        b"tuv" => {
                            current_lang = e
                                .try_get_attribute("xml:lang")
                                .unwrap()
                                .unwrap()
                                .unescape_value()
                                .unwrap()
                                .into_owned();
                        }
                        b"seg" => current_text.clear(),
                        _ => (),
                    },
                    Ok(Event::Text(e)) => {
                        current_text.push_str(e.unescape().unwrap().into_owned().as_str());
                    }

                    Ok(Event::End(e)) => {
                        if e.name().as_ref() == b"tuv" {
                            if current_lang.starts_with(&src) {
                                srcpile.push(current_text.clone());
                            } else if current_lang.starts_with(&tgt) {
                                tgtpile.push(current_text.clone());
                            }
                        }
                    }

                    // There are several other `Event`s we do not consider here
                    _ => (),
                }
                // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
                buf.clear();
            }
        }
    }

    let mut wtr = Writer::from_path(output_path).expect("Unable to create output file");

    for (src, tgt) in srcpile.iter().zip(tgtpile.iter()) {
        wtr.write_record(&[src.as_bytes(), tgt.as_bytes()])
            .expect("Unable to write record");
    }
    wtr.flush().expect("Unable to flush writer");
}
