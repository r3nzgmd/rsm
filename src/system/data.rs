use sysinfo::{System, Pid};
use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct StaticOsInfo {
    pub kernel: String,
    pub name: String,
    pub distribution: String,
    pub os_version: String,
    pub hostname: String,
    pub boot_time: String,
    pub init_system: String,
}

impl StaticOsInfo {
    pub fn get(system: &System) -> Self {
        // helper variables to handle potential errors
        let name = System::name().unwrap_or_else(|| String::from("-"));
        let os_version = System::long_os_version().unwrap_or_else(|| String::from("-"));
        let hostname = System::host_name().unwrap_or_else(|| String::from("-"));

        Self {
            kernel: System::kernel_long_version(),
            name,
            distribution: System::distribution_id(),
            os_version,
            hostname,
            boot_time: boot_time(),
            init_system: init_system(system),
        }
    }
}

fn boot_time() -> String {
    let boot_timestamp = System::boot_time();

    if let Some(utc_datetime) = DateTime::from_timestamp(boot_timestamp as i64, 0) {
        utc_datetime.with_timezone(&Local).format("%d-%m-%Y, %H:%M:%S").to_string()
    } else {
        String::from("Error fetching boot time")
    }
}

fn init_system(system: &System) -> String {
    if cfg!(unix) {
        if let Some(proc) = system.process(Pid::from(1)) {
            return proc.name().to_string_lossy().to_string();
        }
    }

    if cfg!(windows) {
        return String::from("Service Control Manager");
    }

    String::from("unknown")
}

pub fn uptime(uptime: u64) -> String {
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    match uptime {
        s if s < 60 => {
            format!("{}s", seconds)
        }

        m if m < 3600 => {
            format!("{}m, {}s", minutes, seconds)
        }

        h if h < 86400 => {
            format!("{}h, {}m, {}s", hours, minutes, seconds)
        }

        _ => {
            format!("{}d, {}h, {}m, {}s", days, hours, minutes, seconds)
        }
    }
}

