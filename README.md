# Morse Encoder - Proof of Concept

This is a proof of concept for a Morse code encoder that generates audio signals.

## Description

The program encodes text into Morse code and generates a sine wave representation of the code. The output is written into a raw PCM file and played using the `rodio` library.

## Features
- Converts text to Morse code.
- Generates sine waves to represent the Morse code.
- Outputs audio via the `rodio` library.

## Requirements
- Rust (1.81).
- `rodio` crate for audio playback.
