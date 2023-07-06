# Whismur

<img src="whismur.png" width="180" height="180"/>

## Disclaimer

This is a work in progress (WIP).

## Description

Whismur is a small application written Rust to create a bridge between a serial port and a Jack MIDI Port. Very useful for controlling MIDI devices/creating sounds using your arduino.

## How It Works

Just program your arduino to print to the serial port simple characters and configure whismur to listen to this characters and emit MIDI events, I personally use it for connecting a p10 pedal to my computer, through arduino and whismur, and controlling a kick drum with the pedal.

## Installation

For linux, download the prebuilt binaries available in the [releases](https://github.com/DanielSanRocha/whismur/releases) (WIP!). For other OSs, run it from source.

## Running from Source

Just clone the project and run
```bash
cargo run
```
on the root directory.

## Arduino Example Code

WIP

## Acknowledgments

Made with ❤️Love❤️ by Daniel Santana.
