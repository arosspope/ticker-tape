use anyhow::{Result};
use esp_idf_hal::gpio::{
    PinDriver,
    Gpio0, Gpio1, Gpio2,
    Output
};
use max7219::{connectors::*, *};

#[derive(Debug)]
pub struct Error;

impl From<core::convert::Infallible> for Error {
    fn from(_: core::convert::Infallible) -> Self {
        Error {}
    }
}

// impl From<nb::Error<()>> for Error {
//     fn from(_: nb::Error<()>) -> Self {
//         Error {}
//     }
// }

impl From<max7219::DataError> for Error {
    fn from(_: max7219::DataError) -> Self {
        Error {}
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error {}
    }
}

pub type DisplayPins<'a> =
    PinConnector<PinDriver<'a, Gpio0, Output>, PinDriver<'a, Gpio1, Output>, PinDriver<'a, Gpio2, Output>>;

pub struct DotDisplay<'a> {
    display: MAX7219<DisplayPins<'a>>,
    display_is_on: bool,
}

impl DotDisplay<'_> {
    pub fn from(display: MAX7219<DisplayPins>) -> DotDisplay<'_> {
        let mut controller = DotDisplay {
            display,
            display_is_on: true,
        };

        // Start the DotDisplay in a known state
        controller.reset_display().unwrap();
        controller.turn_off_display().unwrap();
        controller
    }

    pub fn write_display(&mut self, input: &[u8; 8]) -> Result<()> {
        if !self.display_is_on {
            self.turn_on_display()?;
        }

        self.display.write_raw(0, &input).unwrap();

        Ok(())
    }

    pub fn turn_off_display(&mut self) -> Result<()> {
        if !self.display_is_on {
            // return Err(Error{});
        }

        self.display.power_off().unwrap();
        self.display_is_on = false;
        Ok(())
    }

    pub fn turn_on_display(&mut self) -> Result<()> {
        if self.display_is_on {
            // return Err(Error);
        }

        self.display.power_on().unwrap();
        self.display_is_on = true;
        Ok(())
    }

    pub fn toggle_display(&mut self) -> Result<()> {
        if self.display_is_on {
            self.turn_off_display().unwrap();
        } else {
            self.turn_on_display().unwrap();
        }

        Ok(())
    }

    pub fn reset_display(&mut self) -> Result<()> {
        Ok(self.display.clear_display(0).unwrap())
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<()> {
        if brightness > 100 {
            // return Err(Error);
        }

        let brightness = (brightness as f32 * 2.55) as u8;
        self.display.set_intensity(0, brightness).unwrap();
        Ok(())
    }
}
