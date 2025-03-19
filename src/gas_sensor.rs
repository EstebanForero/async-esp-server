use esp_hal::{
    analog::adc::{Adc, AdcConfig},
    gpio::GpioPin,
    peripherals::ADC1,
};

struct GasSensor {}

impl GasSensor {
    fn new(adc: ADC1, pin: GpioPin<34>) -> Self {
        let mut adc_config = AdcConfig::default();

        let mut pin = adc_config.enable_pin(pin, esp_hal::analog::adc::Attenuation::_11dB);

        let mut adc = Adc::new(adc, adc_config);

        todo!()
    }
}
