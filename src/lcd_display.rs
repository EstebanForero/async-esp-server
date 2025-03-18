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

        Self { display: lcd }
    }

    pub fn display_temperature(&mut self, temp: f64) {
        let mut temperature_string: String<16> = String::new();

        let (int_part, dec_part) = temp.float_to_parts(1);

        ufmt::uwrite!(
            &mut temperature_string,
            "Temperature: {}.{}",
            int_part,
            dec_part
        )
        .unwrap();

        self.display.set_cursor_xy((0, 0), &mut Delay).unwrap();
        self.display
            .write_str(&temperature_string, &mut Delay)
            .unwrap();
    }

    pub fn display_gas(&mut self, gas: u32) {
        let mut gas_string: String<16> = String::new();

        ufmt::uwrite!(&mut gas_string, "Gas: {}", gas).unwrap();

        self.display.set_cursor_xy((0, 1), &mut Delay).unwrap();
        self.display.write_str(&gas_string, &mut Delay).unwrap();
    }
}
