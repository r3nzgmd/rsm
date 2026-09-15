use crate::ui::{size, charts};
use crate::{SystemDetails, SystemContext};
use ratatui::widgets::Gauge;
use ratatui::text::{Line, Span};
use ratatui::style::Style;

pub struct MemoryLines<'a> {
    pub ram_usage: Line<'a>,
    pub ram_usage_gauge: Gauge<'a>,
    pub available_ram: Line<'a>,
    pub swap_usage: Line<'a>,
    pub swap_usage_gauge: Gauge<'a>,
    pub available_swap: Line<'a>,
}

pub fn display<'a>(system: &SystemDetails, context: &SystemContext) -> MemoryLines<'a> {
    let total_ram = context.total_ram as f64;
    let used_ram = system.memory.used_ram;
    let available_ram = total_ram - used_ram;

    let total_swap = context.total_swap as f64;
    let used_swap = system.memory.used_swap;
    let available_swap = total_swap - used_swap;

    
    MemoryLines {
        ram_usage: ram_usage_line(used_ram, total_ram).centered(),
        ram_usage_gauge: charts::memory_gauge(&used_ram, &total_ram),
        available_ram: Line::from(format!("Free RAM: {}", size::parse_full(available_ram))).right_aligned(),
        swap_usage: swap_usage_line(used_swap, total_swap).centered(),
        swap_usage_gauge: charts::memory_gauge(&used_swap, &total_swap),
        available_swap: Line::from(format!("Free Swap: {}", size::parse_full(available_swap))).right_aligned(),
    }
}

// RAM: {used} / {total}  (%)
fn ram_usage_line<'a>(used: f64, total: f64) -> Line<'a> {
    let ratio = used / total;
    let color = charts::dynamic_color(&ratio);
    Line::from(vec![
        Span::from(format!("RAM: {} / {}  (", size::parse_full(used), size::parse_full(total))),
        Span::styled(format!("{:.1}%", ratio * 100.0), Style::default().fg(color)),
        Span::raw(")"),
    ])
}

// Swap: {used} / {total}  (%)
fn swap_usage_line<'a>(used: f64, total: f64) -> Line<'a> {
    let ratio = used / total;
    let color = charts::dynamic_color(&ratio);
    Line::from(vec![
        Span::from(format!("Swap: {} / {}  (", size::parse_full(used), size::parse_full(total))),
        Span::styled(format!("{:.1}%", ratio * 100.0), Style::default().fg(color)),
        Span::raw(")"),
    ])
}
