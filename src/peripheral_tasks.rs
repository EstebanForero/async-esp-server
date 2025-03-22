use super::app::{Risk, SensorValues};
use crate::app::{CONFIG, VALUE_HISTORY};
use crate::gas_sensor::GasSensor;
use crate::lcd_display;
use crate::temp_sensor::TemperatureSensor;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use esp_hal::gpio::{Flex, GpioPin, Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::i2c::master::AnyI2c;
use esp_hal::peripherals::ADC1;
use esp_println::println;

pub static SENSOR_VALS_SIGNAL: Signal<CriticalSectionRawMutex, SensorValues> = Signal::new();
static RISK_SIGNAL: Signal<CriticalSectionRawMutex, Risk> = Signal::new();

#[embassy_executor::task]
pub async fn test_load() {
    let mut last_values = SensorValues {
        temp: 0.,
        gas: 0,
        flame: false,
    };

    let mut save_counter = 0;

    let mut state = State::Increase;

    enum State {
        Decrease,
        Increase,
    }

    loop {
        let config = CONFIG.lock().await.clone();

        let mut sensor_values = SensorValues {
            temp: last_values.temp,
            gas: last_values.gas,
            flame: !last_values.flame,
        };

        match state {
            State::Decrease => {
                sensor_values.temp -= 0.01;
                sensor_values.gas -= 1
            }
            State::Increase => {
                sensor_values.temp += 0.01;
                sensor_values.gas += 1
            }
        }

        if sensor_values.temp > 90. || sensor_values.gas > 9000 {
            state = State::Decrease;
        }

        if sensor_values.temp < 5. || sensor_values.gas < 50 {
            state = State::Increase;
        }

        last_values = sensor_values.clone();

        save_counter += 1;

        SENSOR_VALS_SIGNAL.signal(sensor_values.clone());

        if save_counter > config.data_point_interval {
            let mut value_history = VALUE_HISTORY.lock().await;
            println!("pushing values: {:?}", sensor_values);
            value_history.push_values(sensor_values);
            save_counter = 0;
        }

        Timer::after_millis(50).await;
    }
}

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

        let gas_value = gas_sensor.get_value().await;

        let flame_value = flame_sensor.is_low();

        SENSOR_VALS_SIGNAL.signal(SensorValues {
            temp,
            gas: gas_value,
            flame: flame_value,
        });
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::task]
pub async fn display_task(i2c: AnyI2c, scl: GpioPin<18>, sda: GpioPin<23>) {
    let i2c_address = 0x27;

    let mut display = lcd_display::Display::new(i2c, scl.into(), sda.into(), i2c_address);

    let mut save_counter = 0;

    loop {
        let values = SENSOR_VALS_SIGNAL.wait().await;
        let config = CONFIG.lock().await.clone();

        display.display_temperature(values.temp);
        display.display_gas(values.gas);

        let risk = get_risk(&values, config.gas_threshold, config.temp_threshold);

        if save_counter > config.data_point_interval {
            let mut value_history = VALUE_HISTORY.lock().await;
            value_history.push_values(values);
            save_counter = 0;
        }

        save_counter += 1;

        if config.alarms_enabled {
            RISK_SIGNAL.signal(risk);
        }
    }
}

fn get_risk(sensor_values: &SensorValues, gas_threshold: u16, temp_threshold: f64) -> Risk {
    println!("{:#?}",sensor_values);
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
pub async fn alarms_task(red : GpioPin<12>, green: GpioPin<13>, blue : GpioPin<14>, buzzer : GpioPin<27>) {
    let mut r = Output::new(red, Level::Low, OutputConfig::default()); 
    let mut g = Output::new(green, Level::Low, OutputConfig::default()); 
    let mut b = Output::new(blue, Level::Low, OutputConfig::default()); 
    let mut piezzo_buzzer = Output::new(buzzer, Level::Low, OutputConfig::default()); 
    loop {
        let risk = RISK_SIGNAL.wait().await;
        

        match risk {
            Risk::Low => {
                println!("Low Risk");
                r.set_level(Level::High);
                g.set_level(Level::Low);
                b.set_level(Level::Low);   // Cian (Verde + Azul)
                piezzo_buzzer.set_level(Level::Low);

            }
            Risk::Moderate => {
                println!("Moderate Risk");
                r.set_level(Level::Low);
                g.set_level(Level::Low);
                b.set_level(Level::High);
                piezzo_buzzer.set_level(Level::Low);
            }
            Risk::High => {
                println!("High Risk");
                r.set_level(Level::Low);   // Rojo
                g.set_level(Level::High);
                b.set_level(Level::High);
                piezzo_buzzer.set_level(Level::High);
            }
        }
    }
}
