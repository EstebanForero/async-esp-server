use embassy_time::Delay;
use esp_hal::{
    gpio::AnyPin,
    i2c::master::{AnyI2c, Config, I2c},
    Async,
};
use hd44780_driver::{
    bus::I2CBus,
    charset::{CharsetUniversal, Fallback},
    memory_map::{MemoryMap1602, StandardMemoryMap},
    setup::DisplayOptionsI2C,
    HD44780,
};
use heapless::String;

use crate::utils::FloatRepresentation;

pub struct Display<'a> {
    display:
        HD44780<I2CBus<I2c<'a, Async>>, StandardMemoryMap<16, 2>, Fallback<CharsetUniversal, 32>>,
}

impl<'a> Display<'_> {
    /// Creates a new display instance using the provided I2C peripheral, SCL and SDA pins, and I2C address.
    ///
    /// This function initializes an I2C bus using the provided I2C peripheral and the specified SCL (clock)
    /// and SDA (data) pins. It then sets up an HD44780 display with the given I2C address, resets and clears
    /// the display, and returns a `Display` instance ready for use. The display is specifically configured
    /// for a 16x2 layout with a universal character set and a fallback of 32 characters.
    ///
    /// # Arguments
    /// * `i2c` - An `AnyI2c` instance representing the I2C peripheral used for communication with the display.
    /// * `scl` - An `AnyPin` instance configured as the SCL (clock) line for the I2C bus.
    /// * `sda` - An `AnyPin` instance configured as the SDA (data) line for the I2C bus.
    /// * `i2c_address` - The I2C address of the display module (e.g., `0x27` for common I2C LCD backpacks).
    ///
    /// # Returns
    /// A new instance of `Self` containing the initialized HD44780 display, ready for displaying text or other data.
    ///
    /// # Examples
    /// ```rust
    /// use esp_hal::gpio::AnyPin;
    /// use esp_hal::i2c::master::AnyI2c;
    ///
    /// // Assume these are obtained from the microcontroller's peripherals
    /// let i2c = peripherals.I2C0;
    /// let scl = peripherals.GPIO18;
    /// let sda = peripherals.GPIO23;
    /// let i2c_address = 0x27; // Common address for I2C LCD backpacks, adjust as needed
    ///
    /// // Create a new display instance
    /// let mut display = Display::new(i2c, scl, sda, i2c_address);
    ///
    /// // Now you can use the display, e.g., to show a message
    /// display.display_temperature(23.5); // Example usage
    /// ```
    pub fn new(i2c: AnyI2c, scl: AnyPin, sda: AnyPin, i2c_address: u8) -> Self {
        let i2c_bus = I2c::new(i2c, Config::default())
            .unwrap()
            .with_scl(scl)
            .with_sda(sda)
            .into_async();

        let Ok(mut lcd) = HD44780::new(
            DisplayOptionsI2C::new(MemoryMap1602::new()).with_i2c_bus(i2c_bus, i2c_address),
            &mut Delay,
        ) else {
            panic!("Failed to initialize display");
        };

        lcd.reset(&mut Delay).unwrap();
        lcd.clear(&mut Delay).unwrap();

        lcd.set_cursor_xy((0, 0), &mut Delay).unwrap();
        lcd.write_str("Temperature", &mut Delay).unwrap();

        Self { display: lcd }
    }

    pub fn display_temperature(&mut self, temp: f64) {
        let mut temperature_string: String<16> = String::new();

        let (int_part, dec_part) = temp.float_to_parts(1);

        ufmt::uwrite!(&mut temperature_string, "Temp: {}.{}", int_part, dec_part).unwrap();

        self.display.set_cursor_xy((0, 0), &mut Delay).unwrap();
        self.display
            .write_str(&temperature_string, &mut Delay)
            .unwrap();
    }

    pub fn display_gas(&mut self, gas: u16) {
        let mut gas_string: String<16> = String::new();

        ufmt::uwrite!(&mut gas_string, "Gas: {}", gas).unwrap();

        self.display.set_cursor_xy((0, 1), &mut Delay).unwrap();
        self.display.write_str(&gas_string, &mut Delay).unwrap();
    }
}
