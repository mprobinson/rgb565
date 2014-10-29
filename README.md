rgb565
======

Simple image to rgb565 raw converter. MIT license.

Usage
=====

Convert image file loadable by rust-image to raw headerless rgb565.
Origin is top left pixel.
Usage: ./target/rgb565 -i infile -o outfile [OPTIONS]
-h --help	Display this help

Notes
=====

This program converts without dithering. If you would like dithering, you can can
use ffmpeg, eg:

ffmpeg -vcodec png -i input.png -vcodec rawvideo -f rawvideo -pix_fmt rgb565 output.raw
