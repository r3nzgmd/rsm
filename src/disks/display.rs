use crate::disks::data::{DisksList, list_items};
use crate::ui::{layout::AppLayout, charts, size};
use ratatui::Frame;
use ratatui::widgets::{Block, List, Paragraph};
use ratatui::style::Style;

pub fn display_list(disks: &Vec<DisksList>) -> List<'_> {
    let items = list_items(disks);

    List::new(items)
        .block(Block::default())
        .highlight_symbol("> ")
        .highlight_style(Style::new().bold())
}

pub fn display_details(f: &mut Frame, lyt: &AppLayout, disk: &DisksList) {
    // [][] - which line, left/right
    let l = &lyt.disk_details;

    // kind
    f.render_widget(Paragraph::new("Kind:"), l[0][0]);
    f.render_widget(Paragraph::new(disk.kind.clone()).right_aligned(), l[0][1]);

    // fs
    f.render_widget(Paragraph::new("File system:"), l[1][0]);
    f.render_widget(Paragraph::new(disk.file_system.clone()).right_aligned(), l[1][1]);

    // mnt
    f.render_widget(Paragraph::new("Mount point:"), l[2][0]);
    f.render_widget(Paragraph::new(disk.mount_point.clone()).right_aligned(), l[2][1]);

    // ws
    f.render_widget(Paragraph::new("Write speed:"), l[3][0]);
    f.render_widget(Paragraph::new(format!("{}/s", size::parse_full(disk.write_speed))).right_aligned(), l[3][1]);

    // rs
    f.render_widget(Paragraph::new("Read speed:"), l[4][0]);
    f.render_widget(Paragraph::new(format!("{}/s", size::parse_full(disk.read_speed))).right_aligned(), l[4][1]);

    // usage
    f.render_widget(Paragraph::new("Usage:"), l[6][0]);

    let ratio = disk.used_space / disk.total_space;
    if ratio >= 0.0 && ratio <= 1.0 && !ratio.is_nan() {
        f.render_widget(charts::disk_gauge(ratio), lyt.disk_usage_gauge[0]);
        f.render_widget(charts::disk_gauge_label(ratio).into_right_aligned_line(), lyt.disk_usage_gauge[1]);
    } else {
        f.render_widget(charts::disk_gauge(0.0), lyt.disk_usage_gauge[0]);
        f.render_widget(Paragraph::new(" - ").right_aligned(), lyt.disk_usage_gauge[1]);
    }

    // used
    f.render_widget(Paragraph::new("Used:"), l[8][0]);
    f.render_widget(Paragraph::new(size::parse_full(disk.used_space)).right_aligned(), l[8][1]);

    // free
    f.render_widget(Paragraph::new("Available:"), l[9][0]);
    f.render_widget(Paragraph::new(size::parse_full(disk.available_space)).right_aligned(), l[9][1]);

    // total
    f.render_widget(Paragraph::new("Total:"), l[10][0]);
    f.render_widget(Paragraph::new(size::parse_full(disk.total_space)).right_aligned(), l[10][1]);
}
