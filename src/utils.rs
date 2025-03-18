pub trait FloatRepresentation {
    fn float_to_parts(self, decimals: u8) -> (u16, u16);
}

impl FloatRepresentation for f64 {
    fn float_to_parts(self, decimals: u8) -> (u16, u16) {
        let num_part = self as u16;
        let multiplier = 10_u16.pow(decimals.into());

        let dec_part = ((self - num_part as f64) * multiplier as f64) as u16;

        (num_part, dec_part)
    }
}
