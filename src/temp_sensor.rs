use anyhow::Result;
use embassy_time::{Delay, Duration, Timer};
use esp_hal::gpio::Flex;
use esp_println::println;
use onecable::{ds18b20::DS18B20, OneWire};

pub struct TemperatureSensor<'a> {
    temp_sensor: DS18B20,
    wire: OneWire<'a, Flex<'a>>,
}

impl<'a> TemperatureSensor<'a> {
    /// Creates a new temperature sensor instance using the provided one-wire bus pin.
    ///
    /// This function initializes the one-wire bus, reads the ROM code of the connected DS18B20 sensor,
    /// and creates a `DS18B20` instance from it. It retries reading the ROM code until successful.
    ///
    /// # Arguments
    /// * `wire_pin` - A mutable reference to a `Flex` pin configured for one-wire communication. The pin
    ///   must be set as open-drain with a pull-up resistor and configured as an output before passing
    ///   it to this function. See the example below for the proper setup.
    ///
    /// # Returns
    /// A new instance of `Self` containing the initialized `DS18B20` sensor and the one-wire bus.
    ///
    /// # Examples
    /// ```rust
    /// use esp_hal::gpio::Flex;
    ///
    /// let pin = peripherals.GPIO15; // Example pin
    ///
    /// // Assuming `pin` is a GPIO pin (e.g., GpioPin from esp_hal)
    /// let mut wire_pin = Flex::new(pin);
    /// wire_pin.set_as_open_drain(esp_hal::gpio::Pull::Up);
    /// wire_pin.set_as_output();
    ///
    /// let sensor = TemperatureSensor::new(&mut wire_pin).await;
    /// ```
    ///
    /// # Notes
    /// - This function is asynchronous and must be called within an async context (e.g., an `async fn`).
    /// - It loops indefinitely until it successfully reads the ROM code from the sensor. In production
    ///   code, consider adding a timeout mechanism to prevent infinite loops.
    /// - Internally, a `Delay` object (typically from `esp_hal::delay::Delay`) is used for timing
    ///   operations and is initialized as `let mut delay = Delay;`. Ensure the environment supports this.
    pub async fn new(wire_pin: &'a mut Flex<'a>) -> Self {
        let mut wire = OneWire::new(wire_pin);

        let mut delay = Delay;

        let output = wire.initialize_bus(&mut delay).unwrap();

        println!("Initialize bus status is: {output}");

        Timer::after(Duration::from_secs(1)).await;

        //let rom_code = match wire.read_rom(&mut delay) {
        //    Ok(rom) => rom,
        //    Err(e) => {
        //        println!("Failed to read ROM: {:?}", e);
        //        panic!("ROM read error");
        //    }
        //};

        let rom_code;

        loop {
            let rom_val = wire.read_rom(&mut delay);

            if let Err(err) = rom_val {
                println!("Error reading rom code: {err}, retriying");
                Timer::after(Duration::from_secs(1)).await;
                continue;
            } else if let Ok(rom) = rom_val {
                rom_code = rom;
                break;
            }
        }
        println!("The rom code is: {rom_code}");

        let temp_sensor = DS18B20::try_from(rom_code).unwrap();

        Self { temp_sensor, wire }
    }

    pub fn read_temperature(&mut self) -> Result<f64> {
        let val = self
            .temp_sensor
            .read_temperature(&mut self.wire, &mut Delay, &mut Delay)?
            .into();

        Ok(val)
    }
}
