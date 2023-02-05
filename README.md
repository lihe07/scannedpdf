# Scanned PDF - rs

A fast, low memory footprint, simple images to PDF crate.

## Features

- Fast
	Combine 1000 A4-sized images into a compressed PDF within a minute.
- Low memory footprint
	`scannedpdf` does not store the whole file in RAM. Typically, the memory footprint depends on the size of the individual images and does not exceed 100Mb.
- Simple
	Only focusing on converting images to PDF, `scannedpdf` provides deadly simple API.
- High compression rate
	Combine a total of 800Mb of images into a single 70Mb PDF. (Note: The compression rate depends on images. 768 A4-sized comic images were used in the test, split 50/50 between black and white and colour)
- Unicode outlines(bookmarks) support
	Outline titles are encoded in UTF-16BE. Every valid Unicode characters including emoji are displayed correctly.

## Usage

1. Adding `scannedpdf` to your project dependencies by `cargo add scannedpdf`
	Note: by default, flate compression is enabled. Disable it by `cargo add scannedpdf --no-default-features`
2. View examples or documentation and get started.

## Resources

- [Docs.rs - scannedpdf](https://docs.rs/scannedpdf)
- [Examples](./tests)

## Why another PDF lib?

I had been using [printpdf](https://github.com/fschutt/printpdf) before I started this project. I was working on a comic downloader that involved merging large numbers of images. `printpdf` didn't do very well in terms of memory, performance and compression. Also, `printpdf` cannot handle unicode text in bookmarks, which appears as garbled code on many devices.
