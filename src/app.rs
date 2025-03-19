use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

pub struct AppState {
    pub counter: u32,
}

#[derive(Debug)]
pub struct SensorValues {
    pub temp: f64,
    pub gas: u16,
    pub flame: bool,
}

pub enum Risk {
    Low,
    Moderate,
    High,
}

pub struct ValueHistory<const N: usize> {
    temp: History<f64, N>,
    ppm: History<u16, N>,
    flame: History<bool, N>,
}

impl<const N: usize> ValueHistory<N> {
    pub fn update_values(&mut self, sensor_values: SensorValues) {
        self.flame.push_value(sensor_values.flame);
        self.ppm.push_value(sensor_values.gas);
        self.temp.push_value(sensor_values.temp);
    }
}

pub struct Config {
    pub temp_threshold: f64,
    pub gas_threshold: u16,
    pub alarms_enabled: bool,
    pub data_point_interval: u8,
}

pub struct History<T: Default + Copy, const N: usize> {
    inner_values: [T; N],
    pointer: usize,
}

impl<T: Default + Copy, const N: usize> History<T, N> {
    pub fn push_value(&mut self, val: T) {
        self.inner_values[self.pointer] = val;
        self.pointer += 1;

        if self.pointer >= N {
            self.pointer = 0;
        }
    }

    const fn default_value(val: T) -> Self {
        Self {
            inner_values: [val; N],
            pointer: 0,
        }
    }

    pub fn get_current_value(&self) -> &T {
        &self.inner_values[self.pointer]
    }

    pub fn get_values_ordered(&self) -> [&T; N] {
        let initial_pointer = self.pointer;
        let mut current_pointer = if initial_pointer + 1 == N {
            0
        } else {
            initial_pointer + 1
        };
        let mut return_values: [&T; N] = [&self.inner_values[0]; N]; // Temporary valid initialization
        return_values[0] = &self.inner_values[initial_pointer];
        let mut return_values_pointer = 1;
        while current_pointer != initial_pointer {
            return_values[return_values_pointer] = &self.inner_values[current_pointer];
            current_pointer = if current_pointer + 1 == N {
                0
            } else {
                current_pointer + 1
            };
            return_values_pointer += 1;
        }
        return_values
    }
}

pub struct App {
    pub config: Config,
    pub value_history: ValueHistory<10>,
}

pub static APP_STATE: Mutex<CriticalSectionRawMutex, App> = Mutex::new(App {
    config: Config {
        temp_threshold: 17.,
        gas_threshold: 300,
        alarms_enabled: true,
        data_point_interval: 3,
    },
    value_history: ValueHistory {
        temp: History::default_value(0.0),
        ppm: History::default_value(0),
        flame: History::default_value(true),
    },
});

pub static STATE: Mutex<CriticalSectionRawMutex, AppState> = Mutex::new(AppState { counter: 0 });
