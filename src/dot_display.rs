use esp_idf_hal::gpio::{
    PinDriver,
    Gpio0, Gpio1, Gpio2,
    Output
};
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
