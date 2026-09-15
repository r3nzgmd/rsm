use sysinfo::System;

pub struct Memory {
    pub used_ram: f64,
    pub used_swap: f64,
}

impl Memory {
    pub fn update(sys: &System) -> Self {
        Self {
            used_ram: sys.used_memory() as f64,
            used_swap: sys.used_swap() as f64,
        }
    }
}
