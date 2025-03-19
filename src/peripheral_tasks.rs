use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Flex, GpioPin};
use esp_hal::i2c::master::AnyI2c;
use esp_println::println;

use crate::lcd_display;
use crate::temp_sensor::{self, TemperatureSensor};

struct SensorValues {
    temp: f64,
    gas: u32,
    flame: bool,
}

static SENSOR_VALS_SIGNAL: Signal<CriticalSectionRawMutex, SensorValues> = Signal::new();

#[embassy_executor::task]
pub async fn sensor_reader_task(temperature_pin: GpioPin<15>) {
    let mut wire_pin = Flex::new(temperature_pin);
    wire_pin.set_as_open_drain(esp_hal::gpio::Pull::Up);
    wire_pin.set_as_output();

    let mut temperature_sensor = TemperatureSensor::new(&mut wire_pin).await;

    loop {
        let temp = temperature_sensor.read_temperature().unwrap();
        SENSOR_VALS_SIGNAL.signal(SensorValues {
            temp: temp,
            gas: 0,
            flame: false,
        });
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
pub async fn display_task(i2c: AnyI2c, scl: GpioPin<18>, sda: GpioPin<23>) {
    let i2c_address = 0x27;

    let display = lcd_display::Display::new(i2c, scl.into(), sda.into(), i2c_address);

    loop {
        let values = SENSOR_VALS_SIGNAL.wait().await;
        println!("{}", values.temp);
    }
}

#[embassy_executor::task]
pub async fn alarms_task() {}
