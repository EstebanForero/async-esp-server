use super::app::{Risk, SensorValues};
use crate::app::APP_STATE;
use crate::gas_sensor::GasSensor;
use crate::lcd_display;
use crate::temp_sensor::TemperatureSensor;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Flex, GpioPin, Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::i2c::master::AnyI2c;
use esp_hal::peripherals::ADC1;
use esp_println::println;

pub static SENSOR_VALS_SIGNAL: Signal<CriticalSectionRawMutex, SensorValues> = Signal::new();
static RISK_SIGNAL: Signal<CriticalSectionRawMutex, Risk> = Signal::new();

#[embassy_executor::task]
pub async fn sensor_reader_task(
    temperature_pin: GpioPin<15>,
    adc: ADC1,
    pin: GpioPin<34>,
    flame_pin: GpioPin<19>,
) {
    let mut wire_pin = Flex::new(temperature_pin);
    wire_pin.set_as_open_drain(esp_hal::gpio::Pull::Up);
    wire_pin.set_as_output();

    let mut gas_sensor = GasSensor::new(adc, pin);
    let mut temperature_sensor = TemperatureSensor::new(&mut wire_pin).await;

    let input_config = InputConfig::default();
    let flame_sensor = Input::new(flame_pin, input_config);

    loop {
        let Ok(temp) = temperature_sensor.read_temperature() else {
            continue;
        };

        let gas_value = gas_sensor.get_value();

        let flame_value = flame_sensor.is_low();

        SENSOR_VALS_SIGNAL.signal(SensorValues {
            temp,
            gas: gas_value,
            flame: flame_value,
        });
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
pub async fn display_task(i2c: AnyI2c, scl: GpioPin<18>, sda: GpioPin<23>) {
    let i2c_address = 0x27;

    let mut display = lcd_display::Display::new(i2c, scl.into(), sda.into(), i2c_address);

    let mut save_counter = 0;

    loop {
        let values = SENSOR_VALS_SIGNAL.wait().await;
        let mut app_state = APP_STATE.lock().await;

        println!("{:#?}", values);
        display.display_temperature(values.temp);
        display.display_gas(values.gas);

        let risk = get_risk(
            &values,
            app_state.config.gas_threshold,
            app_state.config.temp_threshold,
        );

        if save_counter > app_state.config.data_point_interval {
            app_state.value_history.push_values(values);
            save_counter = 0;
        }

        save_counter += 1;

        if app_state.config.alarms_enabled {
            RISK_SIGNAL.signal(risk);
        }
    }
}

fn get_risk(sensor_values: &SensorValues, gas_threshold: u16, temp_threshold: f64) -> Risk {
    if sensor_values.flame {
        return Risk::High;
    }

    if sensor_values.gas > gas_threshold && sensor_values.temp > temp_threshold {
        return Risk::High;
    }

    if sensor_values.gas > gas_threshold || sensor_values.temp > temp_threshold {
        return Risk::Moderate;
    }

    Risk::Low
}

#[embassy_executor::task]
pub async fn alarms_task() {
    loop {
        let risk = RISK_SIGNAL.wait().await;

        match risk {
            Risk::Low => {
                todo!(); // Here goes the code that should be executed when the risk is low
            }
            Risk::Moderate => {
                todo!(); // Here goes the code that should be executed when the risk is moderate
            }
            Risk::High => {
                todo!(); // Here goes the code that should be executed when the risk is high
            }
        }
    }
}
