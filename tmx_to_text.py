# Run: python tmx_to_text.py -s en -t fr -o en-fr.csv file.tmx
# Description: Parse TMX file and extract source and target language text
# Author: Matey Krastev
# Requires: lxml

import argparse
import csv
from lxml import etree as ET

argparser = argparse.ArgumentParser(description="Parse TMX file")
argparser.add_argument("FILE", help="TMX file to parse")
argparser.add_argument("-s", "--src", help="Source language", required=True)
argparser.add_argument("-t", "--tgt", help="Target language", required=True)
argparser.add_argument("-o", "--output", help="Output file")
argparser.add_argument(
    "-d", "--debug", action="store_true", help="Print debug information"
)
argparser.add_argument("--encoding", help="Encoding of the input file", default="utf16")

args = argparser.parse_args()
print(args)

src = args.src.upper()
tgt = args.tgt.upper()

with open(args.FILE, encoding=args.encoding) as f:
    parser = ET.XMLParser(recover=True, encoding=args.encoding)
    tree = ET.parse(f, parser=parser)

root = tree.getroot()
header = root.find("header")
body = root.find("body")
info = {"version": root.attrib["version"], "header": header.attrib}

if args.debug:
    print(info)

srcpile = []
tgtpile = []

for tu in body:
    # For each translation unit
    for tag in tu:
        # For each translation unit variant
        if tag.tag != "tuv":
            continue

        langcode = tag.attrib["{http://www.w3.org/XML/1998/namespace}lang"]

        if langcode.startswith(src):
            srcpile.append(tag.find("seg").text)
        elif langcode.startswith(tgt):
            tgtpile.append(tag.find("seg").text)
        else:
            continue

with open(args.output, "w") as f:
    writer = csv.writer(f)
    for row in zip(srcpile, tgtpile):
        writer.writerow(row)
