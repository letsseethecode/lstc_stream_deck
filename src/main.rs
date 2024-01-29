#![no_std]
#![no_main]

use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};
use panic_halt as _;

trait Morse {
    fn dot(&mut self);
    fn dash(&mut self);
    fn space(&mut self);
    fn separate(&self);
    fn end(&self);
}

const DOT: u16 = 150;
const DASH: u16 = DOT * 3;
const GAP: u16 = DOT;
const SEP: u16 = DOT * 3;
const SPACE: u16 = DOT * 3;
const END: u16 = 1000;

impl Morse for Pin<Output, PB5> {
    fn dot(&mut self) {
        self.set_high();
        arduino_hal::delay_ms(DOT);
        self.set_low();
        arduino_hal::delay_ms(GAP);
    }

    fn dash(&mut self) {
        self.set_high();
        arduino_hal::delay_ms(DASH);
        self.set_low();
        arduino_hal::delay_ms(GAP);
    }

    fn space(&mut self) {
        arduino_hal::delay_ms(SPACE);
    }

    fn separate(&self) {
        arduino_hal::delay_ms(SEP);
    }

    fn end(&self) {
        arduino_hal::delay_ms(END);
    }
}

fn convert_char(c: char) -> &'static str {
    match c {
        'A' => ".-",
        'B' => "-...",
        'C' => "-.-.",
        'D' => "-..",
        'E' => ".",
        'F' => "..-.",
        'G' => "--.",
        'H' => "....",
        'I' => "..",
        'J' => ".---",
        'K' => "-.-",
        'L' => ".-..",
        'M' => "--",
        'N' => "-.",
        'O' => "---",
        'P' => ".--.",
        'Q' => "--.-",
        'R' => ".-.",
        'S' => "...",
        'T' => "-",
        'U' => "..-",
        'V' => "...-",
        'W' => ".--",
        'X' => "-..-",
        'Y' => "-.--",
        'Z' => "--..",
        _ => " ",
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();

    let message = "SOS";
    // ... --- ...

    // let message = "THIS IS A LONGER MESSAGE";
    // - .... .. ... / .. ... / .- / .-.. --- -. --. . .-. / -- . ... ... .- --. .

    loop {
        for c in message.chars() {
            let morse = convert_char(c);
            for d in morse.chars() {
                match d {
                    '.' => led.dot(),
                    '-' => led.dash(),
                    _ => led.space(),
                }
            }
            led.separate();
        }
        led.end();
    }
}
