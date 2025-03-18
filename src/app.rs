use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

pub struct AppState {
    pub counter: u32,
}

pub struct Values {
    pub temp: f64,
    pub ppm: u32,
    pub flama: bool,
}

pub static VALUES: Mutex<CriticalSectionRawMutex, Values> = Mutex::new(Values {
    temp: 9.0,
    ppm: 0,
    flama: false,
});

pub static STATE: Mutex<CriticalSectionRawMutex, AppState> = Mutex::new(AppState { counter: 0 });
