#![no_std]
#![no_main]

use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};
use panic_halt as _;

trait Morse {
    fn dit(&mut self);
    fn dah(&mut self);
    fn space(&mut self);
    fn separate(&self);
    fn end(&self);
}

const DIT: u16 = 150;
const DAH: u16 = DIT * 3;
const GAP: u16 = DIT;
const SEP: u16 = DIT * 3;
const SPC: u16 = DIT * 3;
const END: u16 = 1000;

impl Morse for Pin<Output, PB5> {
    fn dit(&mut self) {
        self.set_high();
        arduino_hal::delay_ms(DIT);
        self.set_low();
        arduino_hal::delay_ms(GAP);
    }

    fn dah(&mut self) {
        self.set_high();
        arduino_hal::delay_ms(DAH);
        self.set_low();
        arduino_hal::delay_ms(GAP);
    }

    fn space(&mut self) {
        arduino_hal::delay_ms(SPC);
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
                    '.' => led.dit(),
                    '-' => led.dah(),
                    _ => led.space(),
                }
            }
            led.separate();
        }
        led.end();
    }
}
