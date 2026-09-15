use crate::system::{data, data::StaticOsInfo};
use crate::SystemDetails;
use ratatui::widgets::Paragraph;
use ratatui::text::Line;

pub fn display<'a>(system: &SystemDetails, context: &StaticOsInfo) -> Paragraph<'a> {
    Paragraph::new(vec![
        Line::from(format!("Kernel: {}", context.kernel)),
        Line::from(format!("Name: {}", context.name)),
        Line::from(format!("Distribution: {}", context.distribution)),
        Line::from(format!("OS Version: {}", context.os_version)),
        Line::from(format!("Hostname: {}", context.hostname)),
        Line::from(format!("Init system: {}", context.init_system)),
        Line::from(format!("Boot time: {}", context.boot_time)),
        Line::from(format!("Uptime: {}", data::uptime(system.uptime))),
    ])
}
