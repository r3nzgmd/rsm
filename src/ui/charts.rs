use crate::cpu::data::CoreInfo;
use ratatui::style::{Style, Color};
use ratatui::widgets::{Gauge, LineGauge, ListItem};
use ratatui::text::{Line, Span};
use ratatui::symbols;

// function to determine dynamic color, based on usage/ratio
pub fn dynamic_color(ratio: &f64) -> Color {
    match ratio * 100.0 {
        u if u >= 90.0 => Color::Rgb(150, 0, 0),
        u if u >= 75.0 => Color::Rgb(255, 0, 0),
        u if u >= 60.0 => Color::Rgb(255, 130, 0),
        u if u >= 40.0 => Color::Rgb(255, 255, 0),
        u if u >= 15.0 => Color::Rgb(150, 255, 0),
        _ => Color::Rgb(0, 255, 0)
    }
}

// memory usage chart
pub fn memory_gauge<'a>(current: &f64, max: &f64) -> Gauge<'a> {
    let ratio = current / max;
    let ratio = if ratio.is_nan() || ratio < 0.0 || ratio > 1.0 { 0.0 } else { ratio };
    
    let color = dynamic_color(&ratio);

    Gauge::default()
        .gauge_style(Style::default().fg(color).bg(Color::Rgb(50, 50, 50)))
        .use_unicode(true)
        .label("")
        .ratio(ratio)
}

// cpu usage chart
pub fn cpu_gauge<'a>(current: f64) -> LineGauge<'a> {
    let ratio = current / 100.0;
    let ratio = if ratio.is_nan() || ratio < 0.0 || ratio > 1.0 { 0.0 } else { ratio };

    let color = dynamic_color(&ratio);
    let label = Span::styled(
        format!(" {:<4}", format!("{:.0}%", ratio * 100.0)), Style::default().fg(color)
    );

    LineGauge::default()
        .filled_symbol(symbols::line::HEAVY_DOUBLE_DASH_HORIZONTAL)
        .unfilled_symbol(symbols::line::HEAVY_DOUBLE_DASH_HORIZONTAL)
        .filled_style(Style::default().fg(color))
        .unfilled_style(Style::default().fg(Color::DarkGray))
        .label(label)
        .ratio(ratio)
}

// disk usage chart
pub fn disk_gauge<'a>(ratio: f64) -> LineGauge<'a> {
    let ratio_validated = if ratio < 0.0 || ratio > 1.0 || ratio.is_nan() { 0.0 } else { ratio };

    let color = dynamic_color(&ratio);

    LineGauge::default()
        .filled_symbol(symbols::line::THICK.horizontal)
        .unfilled_symbol(symbols::line::THICK.horizontal)
        .filled_style(Style::default().fg(color))
        .unfilled_style(Style::default().fg(Color::DarkGray))
        .label("")
        .ratio(ratio_validated)
}

pub fn disk_gauge_label<'a>(ratio: f64) -> Span<'a> {
    let color = dynamic_color(&ratio);
    Span::styled(
        format!("{:.2}%", ratio * 100.0), Style::default().fg(color)
    )
}

// also acts as a Vec<ListItem> for cores list
pub fn cpu_threads_usage(coreinfo: &Vec<CoreInfo>) -> Vec<ListItem<'_>> {
    let list_items: Vec<ListItem> = coreinfo.iter()
        .map(|core| {
            let color = dynamic_color(&(core.usage as f64 / 100.0));

            let bar_total_size = 38.0;
            let bar_filled_size = ((core.usage / 100.0) * bar_total_size).round() as usize;
            let bar_unfilled_size = bar_total_size as usize - bar_filled_size;
            let bar_filled = Span::styled(
                symbols::line::HEAVY_DOUBLE_DASHED.horizontal.repeat(bar_filled_size), Style::default().fg(color)
            );
            let bar_unfilled = Span::styled(
                symbols::line::HEAVY_DOUBLE_DASHED.horizontal.repeat(bar_unfilled_size), Style::default().fg(Color::DarkGray)
            );

            let name_span = Span::raw(format!("{:<6} ", core.name));
            let usage_span = Span::styled(
                format!("{:<4}  ", format!("{:.0}%", core.usage)), Style::default().fg(color)
            );
            let max_usage = Span::raw(" 100%");

            ListItem::new(Line::from(vec![name_span, usage_span, bar_filled, bar_unfilled, max_usage]))
        })
        .collect();
 
    list_items
}
