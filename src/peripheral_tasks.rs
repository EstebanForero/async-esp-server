use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::GPIO;
use esp_hal::{delay, peripheral};
use esp_hal::gpio::{AnyPin, Flex, GpioPin};
use esp_println::println;

use crate::temp_sensor::{self, TemperatureSensor};

struct SensorValues {
    temp: f64,
    gas: u32,
    flame: bool,
}

static SENSOR_VALS_SIGNAL: Signal<CriticalSectionRawMutex, SensorValues> = Signal::new();

#[embassy_executor::task]
pub async fn sensor_reader_task(temperature_pin : GpioPin<15>){

    let mut wire_pin = Flex::new(temperature_pin);
    wire_pin.set_as_open_drain(esp_hal::gpio::Pull::Up);
    wire_pin.set_as_output();

    let mut temperature_sensor = TemperatureSensor::new(&mut wire_pin).await;

    loop {
        let temp = temperature_sensor.read_temperature().unwrap();
        SENSOR_VALS_SIGNAL.signal(SensorValues {
            temp: temp,
            gas:0,
            flame:false,
        });
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
pub async fn display_task() {
    loop {
        let values= SENSOR_VALS_SIGNAL.wait().await;
        println!("{}",values.temp);
    }
}

#[embassy_executor::task]
pub async fn alarms_task() {}
