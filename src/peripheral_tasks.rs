use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::signal::Signal;

struct SensorValues {
    temp: f64,
    gas: u32,
    flame: bool,
}

static SENSOR_VALS_SIGNAL: Signal<CriticalSectionRawMutex, SensorValues> = Signal::new();

#[embassy_executor::task]
pub async fn sensor_reader_task() {}

#[embassy_executor::task]
pub async fn display_task() {}

#[embassy_executor::task]
pub async fn alarms_task() {}
