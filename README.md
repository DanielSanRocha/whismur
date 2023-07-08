# Whismur

<img src="whismur.png" width="180" height="180"/>

## Description

Whismur is a small application written Rust to create a bridge between a serial port and a Jack MIDI Port. Very useful for controlling MIDI devices/creating sounds using your arduino.

## How It Works

Just program your arduino to print to the serial port and configure whismur to listen to this serial port for certain characters and emit MIDI events, I personally use it for connecting a p10 pedal to arduino and controlling a kick drum with the pedal.

<img src="screenshot.png" width="550" height="200"/>

## Installation

For linux, download the prebuilt binaries available in the [releases](https://github.com/DanielSanRocha/whismur/releases). For other OSs, run it from source.

## Running from Source

Just clone the project and run
```bash
cargo run
```
on the root directory, you need to have Rust configured in your machine.

## Arduino Example Code

Below an example of arduino code for printing to the serial using two buttons. You can use this snippet for programming your arduino and use it for controlling a drums for example (using whismur!).

```C
const int pedalPin = 2;
const int otherPedalPin = 3;

int pedalState = LOW;
int otherPedalState = LOW;

void setup() {
  Serial.begin(9600);
  pinMode(pedalPin, INPUT);

  Serial.print("Arduino is Ready!");
}

void loop() {
  int newPedalState = digitalRead(pedalPin);
  int newOtherPedalState = digitalRead(otherPedalPin);

  if(newPedalState == HIGH && pedalState == LOW) {
    Serial.print("p");
    pedalState = newPedalState;
  } else if (newPedalState == LOW && pedalState == HIGH) {
    Serial.print("r");
    pedalState = newPedalState;
  }

  if(newOtherPedalState == HIGH && otherPedalState == LOW) {
    Serial.print("q");
    otherPedalState = newOtherPedalState;
  } else if (newOtherPedalState == LOW && otherPedalState == HIGH) {
    Serial.print("w");
    otherPedalState = newOtherPedalState;
  }
}
```

## Acknowledgments

Made with ❤️Love❤️ by Daniel Santana.
