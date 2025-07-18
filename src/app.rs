use core::array;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use heapless::String;
use serde::{Deserialize, Serialize};
use ufmt::uwrite;

use super::utils::FloatRepresentation;

pub struct AppState {
    pub counter: u32,
}

#[derive(Debug, Default, Clone)]
pub struct SensorValues {
    pub temp: f64,
    pub gas: u16,
    pub flame: bool,
}

impl SensorValues {
    pub fn to_string(self) -> String<12> {
        let mut string = String::new();

        let (int_part, dec_part) = self.temp.float_to_parts(2);

        uwrite!(
            &mut string,
            "{}.{},{},{}",
            int_part,
            dec_part,
            self.gas,
            self.flame as u8
        )
        .unwrap();

        string
    }

    pub fn to_bytes(&self) -> [u8; 5] {
        let temp_scaled = (self.temp * 100.0) as u16;
        let temp_bytes = temp_scaled.to_le_bytes();
        let gas_bytes = self.gas.to_le_bytes();
        let flame_byte = self.flame as u8;
        [
            temp_bytes[0],
            temp_bytes[1],
            gas_bytes[0],
            gas_bytes[1],
            flame_byte,
        ]
    }
}

#[derive(Clone)]
pub enum Risk {
    Low,
    Moderate,
    High,
}

impl Risk {
    pub fn to_byte(&self) -> u8 {
        match self {
            Risk::Low => 0,
            Risk::Moderate => 1,
            Risk::High => 2,
        }
    }
}

pub const HISTORY_LENGTH: usize = 10;

pub struct ValueHistory<const N: usize> {
    temp: History<f64, N>,
    ppm: History<u16, N>,
    flame: History<bool, N>,
    new_change: bool,
}

pub struct ValueHistoryArray([SensorValues; 10]);

impl ValueHistoryArray {
    pub fn to_string(self) -> String<130> {
        let mut string = String::new();

        for value in self.0 {
            string.push_str(&value.to_string()).unwrap();
            string.push('|').unwrap();
        }

        string.pop().unwrap();

        string
    }
}

impl<const N: usize> ValueHistory<N> {
    pub fn push_values(&mut self, sensor_values: SensorValues) {
        self.new_change = true;
        self.flame.push_value(sensor_values.flame);
        self.ppm.push_value(sensor_values.gas);
        self.temp.push_value(sensor_values.temp);
    }

    pub fn current_values(&self) -> SensorValues {
        SensorValues {
            flame: *self.flame.get_current_value(),
            gas: *self.ppm.get_current_value(),
            temp: *self.temp.get_current_value(),
        }
    }

    pub fn new_change(&mut self) -> bool {
        if self.new_change {
            self.new_change = false;
            true
        } else {
            false
        }
    }
}

impl ValueHistory<10> {
    /// Very expesive function to run
    pub fn get_current_values_history(&self) -> ValueHistoryArray {
        let temp_values = self.temp.get_values_ordered();
        let ppm_values = self.ppm.get_values_ordered();
        let flame_values = self.flame.get_values_ordered();
        let arr = array::from_fn(|i| SensorValues {
            temp: *temp_values[i],
            gas: *ppm_values[i],
            flame: *flame_values[i],
        });

        ValueHistoryArray(arr)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub temp_threshold: f64,
    pub gas_threshold: u16,
    pub alarms_enabled: bool,
    pub data_point_interval: u8,
}

impl Config {
    pub fn to_bytes(&self) -> [u8; 6] {
        let temp_threshold_scaled = (self.temp_threshold * 100.0) as u16;
        let temp_threshold_bytes = temp_threshold_scaled.to_le_bytes();
        let gas_threshold_bytes = self.gas_threshold.to_le_bytes();
        let alarms_enabled_byte = self.alarms_enabled as u8;
        let data_point_interval_byte = self.data_point_interval;
        [
            temp_threshold_bytes[0],
            temp_threshold_bytes[1],
            gas_threshold_bytes[0],
            gas_threshold_bytes[1],
            alarms_enabled_byte,
            data_point_interval_byte,
        ]
    }

    pub fn from_bytes(bytes: [u8; 6]) -> Self {
        let temp_threshold_scaled = u16::from_le_bytes([bytes[0], bytes[1]]);
        let gas_threshold = u16::from_le_bytes([bytes[2], bytes[3]]);
        let alarms_enabled = bytes[4] != 0;
        let data_point_interval = bytes[5];
        Self {
            temp_threshold: (temp_threshold_scaled as f64) / 100.0,
            gas_threshold,
            alarms_enabled,
            data_point_interval,
        }
    }
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

pub static CONFIG: Mutex<CriticalSectionRawMutex, Config> = Mutex::new(Config {
    temp_threshold: 5.,
    gas_threshold: 1500,
    alarms_enabled: true,
    data_point_interval: 3,
});

pub static VALUE_HISTORY: Mutex<CriticalSectionRawMutex, ValueHistory<10>> =
    Mutex::new(ValueHistory {
        temp: History::default_value(0.0),
        ppm: History::default_value(0),
        flame: History::default_value(false),
        new_change: true,
    });

pub static CURRENT_VALUE: Mutex<CriticalSectionRawMutex, SensorValues> = Mutex::new(SensorValues {
    temp: 0.,
    gas: 0,
    flame: false,
});
