use esp_hal::{
    analog::adc::{Adc, AdcConfig, AdcPin},
    gpio::{AnyPin, GpioPin},
    peripherals::ADC1,
    Blocking,
};
use esp_println::println;

pub struct GasSensor<'a> {
    //pin: GpioPin<34>,
    adc: Adc<'a, ADC1, Blocking>,
    analog_pin: AdcPin<GpioPin<34>, ADC1>,
}

impl<'a> GasSensor<'_> {
    pub fn new(adc: ADC1, pin: GpioPin<34>) -> Self {
        let mut adc_config = AdcConfig::default();

        let analog_pin = adc_config.enable_pin(pin, esp_hal::analog::adc::Attenuation::_11dB);

        let adc = Adc::new(adc, adc_config);

        Self { adc, analog_pin }
    }

    pub fn get_value(&mut self) -> u16 {
        let value;
        loop {
            let val_err = self.adc.read_oneshot(&mut self.analog_pin);

            if let Err(err) = val_err {
                println!("Error in gas sensor get_value {err:?}");
                continue;
            }

            value = val_err.unwrap();
            break;
        }

        value
    }
}
