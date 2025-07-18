#![no_std]
#![feature(impl_trait_in_assoc_type)]

pub mod app;
pub mod cors_layer;
pub mod gas_sensor;
pub mod lcd_display;
pub mod mqtt;
pub mod peripheral_tasks;
pub mod temp_sensor;
pub mod utils;
pub mod wifi;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
