use sysinfo::{
    System,
    CpuRefreshKind,
    Components,
    MemoryRefreshKind,
    ProcessRefreshKind,
    Disks,
    DiskRefreshKind,
    Networks,
    UpdateKind,
    RefreshKind,
};

pub struct Instances {
    pub system: System,
    pub disks: Disks,
    pub networks: Networks,
    pub components: Components,
}

pub struct Refreshes {
    pub system_refresh: RefreshKind,
    pub disks_refresh: DiskRefreshKind,

}

impl Instances {
    pub fn new(system_refresh: RefreshKind, disks_refresh: DiskRefreshKind) -> Self {
        Self {
            system: System::new_with_specifics(system_refresh),
            disks: Disks::new_with_refreshed_list_specifics(disks_refresh),
            networks: Networks::new_with_refreshed_list(),
            components: Components::new_with_refreshed_list(),
        }
    }
}

// defining what system data is refreshed every second
impl Refreshes {
    pub fn new() -> Self {
        let processes_refresh = ProcessRefreshKind::nothing()
            .with_cpu()
            .with_memory()
            .with_exe(UpdateKind::OnlyIfNotSet);

        let system_refresh = RefreshKind::nothing()
            .with_cpu(
                CpuRefreshKind::nothing()
                .with_cpu_usage()
                .with_frequency()
            )
            .with_memory(
                MemoryRefreshKind::nothing()
                .with_ram()
                .with_swap()
            )
            .with_processes(processes_refresh);

        let disks_refresh = DiskRefreshKind::nothing()
            .with_kind()
            .with_storage()
            .with_io_usage();

        Self {
            system_refresh,
            disks_refresh,
        }
        
    }
}
