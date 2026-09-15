use sysinfo::{System, Components};

// main cpu structs
pub struct CpuList {
    pub dynamic_info: CpuDynamicInfo,
    pub cores_list: Vec<CoreInfo>,
}

#[derive(Clone)]
pub struct CpuModel {
    pub vendor_id: String,
    pub brand: String,
    pub total_cores: usize,
    pub total_threads: usize,
}

// helper structs
pub struct CoreInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

pub struct CpuDynamicInfo {
    pub global_frequency: u64,
    pub global_usage: f32,
    pub temp: (f32, f32),
}

impl CpuList {
    pub fn update(system: &System, components: &Components) -> Self {
        let mut cores_list: Vec<CoreInfo> = Vec::new();
        for core in system.cpus() {
            let core_info = CoreInfo {
                name: core.name().to_string(),
                usage: core.cpu_usage(),
                frequency: core.frequency(),
            };

            cores_list.push(core_info);
        }

        let dynamic_info = CpuDynamicInfo {
            global_frequency: global_frequency(&cores_list),
            global_usage: system.global_cpu_usage(),
            temp: temp(components),
        };

        Self {
            dynamic_info,
            cores_list,
        }
    }
}

fn global_frequency(cores: &Vec<CoreInfo>) -> u64 {
    let mut summed_frequency: u64 = 0;

    for core in cores {
        summed_frequency += core.frequency;
    }

    summed_frequency / cores.len() as u64
}

pub fn temp(components: &Components) -> (f32, f32) {
    let temperatures: Vec<f32> = components
        .iter()
        .filter(
            |c| c.label().to_lowercase().contains("cpu") || c.label().to_lowercase().contains("core")
        )
        .map(|c| c.temperature().unwrap())
        .collect();

    if !temperatures.is_empty() {
        let temp_c = temperatures.iter().sum::<f32>() / temperatures.len() as f32;
        return (
            temp_c,
            temp_c * 1.8 + 32.0
        );
    }

    (0.0, 0.0)
}
