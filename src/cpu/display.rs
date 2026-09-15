use crate::{ui::charts, update::SystemDetails, SystemContext};
use ratatui::widgets::LineGauge;
use ratatui::text::{Text, Line};
use ratatui::widgets::{List, Block};
use ratatui::style::Style;

pub struct CpuLines<'a> {
    pub model: Line<'a>,
    pub cores: Line<'a>,
    pub global_usage_label: Text<'a>,
    pub global_usage: LineGauge<'a>,
    pub temperature: Line<'a>,
}

pub fn display_cpu_info<'a>(system: &SystemDetails, context: &SystemContext) -> CpuLines<'a> {
    CpuLines {
        model: Line::from(format!(
            "CPU: {} {}, {:.2}Ghz",
            context.cpu_model.vendor_id,
            context.cpu_model.brand,
            system.cpus.dynamic_info.global_frequency as f64 / 1000.0
        )),

        cores: Line::from(format!(
            "Cores: {} physical / {} logical ({} cores / {} threads)",
            context.cpu_model.total_cores,
            context.cpu_model.total_threads,
            context.cpu_model.total_cores,
            context.cpu_model.total_threads,
        )),

        global_usage_label: Text::from("Global usage:"),
        global_usage: charts::cpu_gauge(system.cpus.dynamic_info.global_usage as f64),

        temperature: Line::from(format!(
            "Temperature: {:.1}°C  ({:.1}°F)",
            system.cpus.dynamic_info.temp.0,
            system.cpus.dynamic_info.temp.1,
        )),
    }
}

pub fn display_core_list<'a>(system: &'a SystemDetails) -> List<'a> {
    let items = charts::cpu_threads_usage(&system.cpus.cores_list);

    List::new(items)
        .block(Block::default())
        .highlight_symbol("> ")
        .highlight_style(Style::new().bold())
}
