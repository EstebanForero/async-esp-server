use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Flex, GpioPin, Level, Output, OutputConfig};
use esp_hal::i2c::master::AnyI2c;
use esp_hal::peripherals::ADC1;
use esp_println::println;
use esp_hal::analog::adc::{Adc, AdcConfig, AdcPin, Attenuation};
use crate::gas_sensor::GasSensor;
use crate::{gas_sensor, lcd_display};
use crate::temp_sensor::{self, TemperatureSensor};

struct SensorValues {
    temp: f64,
    gas: u32,
    flame: bool,
}

static SENSOR_VALS_SIGNAL: Signal<CriticalSectionRawMutex, SensorValues> = Signal::new();

#[embassy_executor::task]
pub async fn sensor_reader_task(temperature_pin: GpioPin<15>, adc: ADC1, pin: GpioPin<34>,flame_pin: GpioPin<2>) {
    let mut wire_pin = Flex::new(temperature_pin);
    wire_pin.set_as_open_drain(esp_hal::gpio::Pull::Up);
    wire_pin.set_as_output();

    let mut gas_sensor = GasSensor::new(adc, pin);
    let mut temperature_sensor = TemperatureSensor::new(&mut wire_pin).await;
    let mut flame_sensor = Output::new(flame_pin, Level::High, OutputConfig::default());

    loop {
        let Ok(temp) = temperature_sensor.read_temperature() else {
            continue;
        };

        let gas_value = gas_sensor.get_value();

        let mut flame_value = false;

        if flame_sensor.is_set_low(){
            flame_value = true;

        } else {
            flame_value = false;
        }
        SENSOR_VALS_SIGNAL.signal(SensorValues {
            temp: temp,
            gas: gas_value as u32 ,
            flame: flame_value,
        });
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
pub async fn display_task(i2c: AnyI2c, scl: GpioPin<18>, sda: GpioPin<23>) {
    let i2c_address = 0x27;

    let mut display = lcd_display::Display::new(i2c, scl.into(), sda.into(), i2c_address);

    loop {
        let values = SENSOR_VALS_SIGNAL.wait().await;
        println!("Temp: {}", values.temp);
        println!("Gas: {}", values.gas);
        println!("Flame {}",values.flame);
        display.display_temperature(values.temp);
        display.display_gas(values.gas);
    }
}

#[embassy_executor::task]
pub async fn alarms_task() {}

