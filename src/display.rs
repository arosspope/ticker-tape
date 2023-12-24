use anyhow::{bail, Error, Result};
use esp_idf_hal::gpio::{Gpio0, Gpio1, Gpio2, Output, PinDriver};
use font8x8::{UnicodeFonts, BASIC_FONTS};
use log::*;
use max7219::{connectors::*, *};

pub type DisplayPins<'a> = PinConnector<
    PinDriver<'a, Gpio0, Output>,
    PinDriver<'a, Gpio1, Output>,
    PinDriver<'a, Gpio2, Output>,
>;

pub struct DotDisplay<'a> {
    display: MAX7219<DisplayPins<'a>>,
    display_is_on: bool,
    brightness: usize,
}

impl DotDisplay<'_> {
    pub fn from(display: MAX7219<DisplayPins>) -> Result<DotDisplay<'_>, Error> {
        let mut controller = DotDisplay {
            display,
            display_is_on: true,
            brightness: 0,
        };

        // Start the DotDisplay in a known state
        controller.reset_display()?;
        controller.turn_off_display()?;

        Ok(controller)
    }

    pub fn write_display(&mut self, input: &[u8; 8]) -> Result<(), Error> {
        if !self.display_is_on {
            self.turn_on_display()?;
        }

        if let Err(_) = self.display.write_raw(0, input) {
            error!("Failed to write to display");
        }

        Ok(())
    }

    pub fn turn_off_display(&mut self) -> Result<(), Error> {
        if !self.display_is_on {
            error!("Display already off");
        }

        if let Err(_) = self.display.power_off() {
            error!("Failed to power off display");
        }

        self.display_is_on = false;
        Ok(())
    }

    pub fn turn_on_display(&mut self) -> Result<(), Error> {
        if self.display_is_on {
            error!("Display already on");
        }

        if let Err(_) = self.display.power_on() {
            error!("Failed to power on display");
        }
        self.display_is_on = true;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn toggle_display(&mut self) -> Result<(), Error> {
        if self.display_is_on {
            self.turn_off_display()?;
        } else {
            self.turn_on_display()?;
        }

        Ok(())
    }

    pub fn reset_display(&mut self) -> Result<(), Error> {
        if let Err(_) = self.display.clear_display(0) {
            error!("Failed to clear display");
        }
        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Error> {
        if brightness > 100 {
            error!("Brightness greater than 100%");
        }

        let brightness = brightness as f32 * 2.55;
        if let Err(_) = self.display.set_intensity(0, brightness as u8) {
            error!("Failed to set intensity of display")
        }
        self.brightness = brightness as usize;
        Ok(())
    }

    pub fn brightness(&self) -> usize {
        self.brightness
    }
}

pub struct Ticker<'a> {
    shift: isize,
    index: usize,
    pub speed_ms: usize,
    pub message_len: usize,
    pub message: [u8; 100],
    pub display: DotDisplay<'a>,
}

impl Ticker<'_> {
    pub fn new(display: DotDisplay<'_>) -> Ticker<'_> {
        Ticker {
            shift: 0,
            index: 0,
            speed_ms: 70,
            message_len: 0,
            message: [0; 100],
            display,
        }
    }

    pub fn set_message(&mut self, message: &str) -> Result<(), Error> {
        debug!("Setting ticker-tape message: {:?}", message);
        if message.is_empty() {
            bail!("Empty message");
        }

        if message.len() + 1 > self.message.len() {
            bail!("Message too long");
        }

        self.message_len = message.len();
        self.message[..self.message_len].copy_from_slice(message.as_bytes());
        self.message[self.message_len] = b' ';
        self.message_len += 1;
        Ok(())
    }

    fn glyph(c: char, shift: isize) -> [u8; 8] {
        let unknown = BASIC_FONTS.get('?').unwrap();
        let mut glyph = BASIC_FONTS.get(c).unwrap_or(unknown);

        glyph.iter_mut().for_each(|x| {
            *x = x.reverse_bits();
            let mut y = *x as isize;

            if shift < 0 {
                y >>= shift.abs();
            } else {
                y <<= shift;
            }

            *x = y as u8;
        });

        glyph
    }

    pub fn tick(&mut self) {
        if self.shift >= 8 {
            self.shift = 0;
            self.index = (self.index + 1) % self.message_len;
        }

        let previous = if self.index > 0 {
            self.message[self.index - 1] as char
        } else {
            0 as char
        };
        let next = self.message[self.index] as char;

        let mut previous_glyph = Ticker::glyph(previous, self.shift);
        let next_glyph = Ticker::glyph(next, self.shift - 8);

        // OR two glyphs together
        previous_glyph
            .iter_mut()
            .enumerate()
            .for_each(|(index, el)| *el |= next_glyph[index]);

        // Write to the display
        self.display
            .write_display(&previous_glyph)
            .expect("Failed to write dot-matrix");
        self.shift += 1;
    }
}
