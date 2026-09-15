use crate::{
    SortProcessesBy,
    Instances,
    disks::data::DisksList,
    memory::data::Memory,
    cpu::data::CpuList,
    processes::data::Process,
    networks::data::NetworksList
};
use sysinfo::System;

pub struct SystemDetails {
    pub uptime: u64,
    pub cpus: CpuList,
    pub memory: Memory,
    pub disks: Vec<DisksList>,
    pub processes_list: Vec<Process>,
    pub networks: Vec<NetworksList>,
}

impl SystemDetails {
    pub fn update(instances: &Instances, sort_processes: &SortProcessesBy) -> Self {
        Self {
            uptime: System::uptime(),
            cpus: CpuList::update(&instances.system, &instances.components),
            memory: Memory::update(&instances.system),
            disks: DisksList::update(&instances.disks),
            processes_list: Process::update(&instances.system, sort_processes),
            networks: NetworksList::update(&instances.networks),
        }
    }    
}
