# Steganography Encoder/Decoder
> A tool that allows you to encode text inside an image.

[![Build Passing](https://img.shields.io/badge/Build-Passing-brightgreen?style=flat-square)](https://stegencoder.almuqbel.workers.dev/)

# Usage

The easiest and intended way to use this tool is by visiting:

https://stegencoder.almuqbel.workers.dev/

## Example

A short demonstration of encoding and decoding text using the web application:

https://youtu.be/SKmU84WPJTY

# Explanation

## The Rust encoding engine

This steganography engine uses LSB (Least Significant Bit) encoding.

For example, the first pixel in a 24-bit image may look like this:

`#4D8260`

meaning that the red, green, and blue values are `4D`, `82`, and `60`, respectively.

When we convert these to binary, we get:

`01001101`, `10000010`, and `01100000`.

The human eye cannot differentiate between a colour that is 1/255 shades more red, green, or blue. Therefore, if we change the last bit on the right (the least significant bit, meaning that it changes the colour by only 1/255 of the RGB value), a human would not notice.

Knowing that we can change three values from each pixel without making any noticeable visual change, and knowing that each ASCII character takes one byte, we can use the least significant bits to represent text. We also know that, because pixels in a 24-bit image contain three colour channels, it takes **8 / 3** pixels to store one character. In other words, storing three characters requires eight pixels.

If we wanted to store the lowercase character `'a'` in the image, we would first need to know its ASCII value. This is trivial for a computer, and `'a'` has the decimal value **97**, or in binary:

`01100001`

`stegEncoder` takes each bit from `01100001` and stores it in the least significant bit of each colour channel. So our previous pixel, `#4D8260`, becomes `#4C8361`, which is virtually impossible to distinguish by eye.

`stegEncoder` also adds a header before the encoded text. This consists of an 11-byte signature and a 4-byte value indicating the length of the hidden text. Therefore, every encoded image reserves 15 bytes (40 pixels) before the actual message begins.

To calculate how much text an image can hold when using only one LSB per colour channel, use:

`3 * (height * width) / 8`

where `height` and `width` are measured in pixels.

## The Next.js frontend

To use the `stegEncoder` engine, a frontend is required. Rust cannot be executed directly in a browser, so the Rust code is compiled to WebAssembly, allowing it to run on any modern browser.

This results in approximately **1.3–1.6×** slower execution than native Rust, but it is still significantly faster than JavaScript. Benchmarks of similar low-level operations show that WebAssembly can be **2–10×** faster than JavaScript for comparable cryptographic and systems-level workloads.

The frontend uses Bun as its runtime, which provides significantly faster package management. It also uses [react-dropzone](https://react-dropzone.js.org/) for file uploads, along with [Tailwind CSS](https://tailwindcss.com/) for its utility-based styling.

# Design Decisions

> The reasons why specific languages and packages were chosen for this project, along with explanations of certain implementation decisions.

## Rust

Rust was chosen for the main encoding engine primarily because of its speed.

It is also considerably safer than C and C++ with regard to memory management.

## [Image crate](https://docs.rs/image/latest/image/)

This is the most commonly used image-processing crate in Rust and provides a wide range of functionality. Also easy to do mathematical operations on.

## Image Signature

```rust
const SIGNATURE: &[u8] = b"stegEncoder"; // a signature to ensure the image was encoded by this encoder
```

The signature allows the decoder to determine whether an image actually contains hidden data. Since `"stegEncoder"` is 11 bytes long, there is an extremely small chance that an unmodified image would coincidentally contain the same sequence of least significant bits.

## Next.js

Next.js was chosen because of its straightforward integration with Cloudflare and because it comes with excellent support for [Tailwind CSS](https://tailwindcss.com/). It also provides good browser performance.

# AI Usage

This repository uses minimal AI assistance.

### Rust encoder
For the Rust `stegEncoder`, AI was used strictly for debugging and for discussing implementation decisions, such as comparing the use of one versus two LSBs.

### Frontend
For the frontend, AI generated some Tailwind CSS and recommended using a file upload library such as Dropzone, although I found `react-dropzone` myself through a Google search.

### README
For this README, AI was used only to check spelling, punctuation, and grammar.
