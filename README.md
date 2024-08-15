# TMX to TEXT

This script converts a TMX (Translation Memory eXchange) file to a text file. The text file will contain the source and target text of the TMX file in a CSV-parsable format.

We provide two implementations of the script:

- `tmx_to_text.py`: A simple script that uses the `lxml` module to parse the TMX file.
- `tmx_to_text` (binary): A compiled version of the script written in Rust. Can be faster than the Python version for UTF-8 encoded files.

## Usage

### Python

```bash
python tmx_to_text.py <tmx_file> -s en -t de [-e <utf8,utf16>] -o <output_file>
```

### Rust

```bash
tmx_to_text <tmx_file> -s en -t de [-e <utf8,utf16>] -o <output_file>
```

## Installation

For Python, just make sure you have the `lxml` module installed:

```bash
pip install lxml
```

For Rust, you can download the binary from the releases page or compile it yourself with `cargo`:

```bash
cargo build --release
```
