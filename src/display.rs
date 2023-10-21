use log::*;
use anyhow::{Result, Error};
use esp_idf_hal::gpio::{
    PinDriver,
    Gpio0, Gpio1, Gpio2,
    Output
};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use max7219::{connectors::*, *};

pub type DisplayPins<'a> =
    PinConnector<PinDriver<'a, Gpio0, Output>, PinDriver<'a, Gpio1, Output>, PinDriver<'a, Gpio2, Output>>;

pub struct DotDisplay<'a> {
    display: MAX7219<DisplayPins<'a>>,
    display_is_on: bool,
}

impl DotDisplay<'_> {
    pub fn from(display: MAX7219<DisplayPins>) -> Result<DotDisplay<'_>, DataError> {
        let mut controller = DotDisplay {
            display,
            display_is_on: true,
        };

        // Start the DotDisplay in a known state
        controller.reset_display()?;
        controller.turn_off_display()?;
        Ok(controller)
    }

    pub fn write_display(&mut self, input: &[u8; 8]) -> Result<(), DataError> {
        if !self.display_is_on {
            self.turn_on_display()?;
        }

        self.display.write_raw(0, &input)?;

        Ok(())
    }

    pub fn turn_off_display(&mut self) -> Result<(), DataError> {
        if !self.display_is_on {
            return Err(DataError::Pin);
        }

        self.display.power_off()?;
        self.display_is_on = false;
        Ok(())
    }

    pub fn turn_on_display(&mut self) -> Result<(), DataError> {
        if self.display_is_on {
            return Err(DataError::Pin);
        }

        self.display.power_on()?;
        self.display_is_on = true;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn toggle_display(&mut self) -> Result<(), DataError> {
        if self.display_is_on {
            self.turn_off_display()?;
        } else {
            self.turn_on_display()?;
        }

        Ok(())
    }

    pub fn reset_display(&mut self) -> Result<(), DataError> {
        self.display.clear_display(0)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), DataError> {
        if brightness > 100 {
            return Err(DataError::Pin);
        }

        let brightness = (brightness as f32 * 2.55) as u8;
        self.display.set_intensity(0, brightness)?;
        Ok(())
    }
}

pub struct Ticker<'a> {
    shift: usize,
    index: usize,
    len: usize,
    message: [u8; 100],
    pub display: DotDisplay<'a>,
}

impl Ticker<'_> {
    pub fn new<'a>(display: DotDisplay<'a>) -> Ticker<'a> {
        Ticker {
            shift: 0,
            index: 0,
            len: 0,
            message: [0; 100], 
            display: display, 
        }
    }

    pub fn set_message(&mut self, message: &str) {
        debug!("Setting ticker-tape message: {:?}", message);
        self.len = message.len();
        self.message[..self.len].copy_from_slice(message.as_bytes().try_into().unwrap());
    }

    pub fn tick(&mut self) {
        if self.shift >= 8 {
            self.shift = 0;
            self.index = (self.index + 1) % self.len;
        }

        let c = self.message[self.index] as char;
        
        if let Some(mut glyph) = BASIC_FONTS.get(c) {
            glyph.iter_mut().for_each(|x| *x = x.reverse_bits() << self.shift);

            self.display.write_display(&glyph).expect("Failed to write dot-matrix");
        }

        self.shift += 1;
    }
}