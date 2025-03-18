use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::signal::Signal;

static TEMP_SIGNAL: Signal<CriticalSectionRawMutex, f64> = Signal::new();
static GAS_SIGNAL: Signal<CriticalSectionRawMutex, f64> = Signal::new();
static FLAME_SIGNAL: Signal<CriticalSectionRawMutex, f64> = Signal::new();

#[embassy_executor::task]
async fn temp_reader_task() {}

#[embassy_executor::task]
async fn gas_reader_task() {}

#[embassy_executor::task]
async fn flame_reader_task() {}

#[embassy_executor::task]
async fn display_task() {}

#[embassy_executor::task]
async fn alarms_task() {}
