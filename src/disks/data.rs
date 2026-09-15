use crate::ui::size;
use sysinfo::Disks;
use ratatui::widgets::ListItem;

pub struct DisksList {
    pub name_on_list: String,
    pub kind: String,
    pub file_system: String,
    pub mount_point: String,
    pub write_speed: f64,
    pub read_speed: f64,
    pub used_space: f64,
    pub available_space: f64,
    pub total_space: f64,
}

impl DisksList {
    pub fn update(disks: &Disks) -> Vec<Self> {
        let mut list: Vec<Self> = Vec::new();

        for disk in disks {
            let name = disk.name().to_string_lossy().to_string();
            let total_space = disk.total_space() as f64;
            let available_space = disk.available_space() as f64;

            let disk_info = Self {
                name_on_list: disk_name_on_list(&name, total_space),
                kind: disk.kind().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                write_speed: disk.usage().written_bytes as f64,
                read_speed: disk.usage().read_bytes as f64,
                used_space: total_space - available_space,
                available_space,
                total_space,
            };

            list.push(disk_info);
        }
        
        list
    }
}

pub fn list_items<'a>(disks: &Vec<DisksList>) -> Vec<ListItem<'a>> {
    let mut list: Vec<ListItem> = Vec::new();

    for disk in disks {
        list.push(ListItem::new(disk.name_on_list.clone()));
    }

    list
}

// small function for parsing disk information into its name on the list
pub fn disk_name_on_list(name: &String, total_space: f64) -> String {
    format!("{} ({})", name, size::parse_full(total_space))
}
