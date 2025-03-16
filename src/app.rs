use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

pub struct AppState {
    pub counter: u32,
}

pub static STATE: Mutex<CriticalSectionRawMutex, AppState> = Mutex::new(AppState { counter: 0 });
