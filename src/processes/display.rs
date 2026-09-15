use crate::SystemContext;
use crate::processes::data::{Process, list_items};
use ratatui::widgets::{Block, List, Paragraph};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn display_list<'a>(processes: &Vec<Process>, context: &SystemContext) -> List<'a> {
    let items = list_items(processes, context);

    List::new(items)
        .block(Block::default())
        .highlight_style(Style::new().bg(Color::Rgb(60, 60, 60)))
}

pub fn legend<'a>() -> Paragraph<'a> {
    let line = Line::from(vec![
        Span::from(format!("{:<7}  ", String::from("Pid:"))),
        Span::from(format!("{:<20}  ", String::from("Name:"))),
        Span::from(format!("{:<50}  ", String::from("Exec path:"))),
        Span::from(format!("  {:>6}   ", String::from("Cpu:"))),
        Span::from(format!("   {:>5}  ", String::from("RAM:"))),
    ]);

    Paragraph::new(line)
}

pub fn sorting<'a>() -> Paragraph<'a> {
    let line = Line::from(vec![
        Span::raw(format!("{:<87}", String::from(""))),
        // [1]
        Span::styled("[", Style::new().fg(Color::Rgb(207, 159, 255))),
        Span::styled("1", Style::new().fg(Color::Rgb(255, 80, 80))),
        Span::styled("]", Style::new().fg(Color::Rgb(207, 158, 255))),
        Span::raw(format!("{:<8}", String::from(""))),
        // [2]
        Span::styled("[", Style::new().fg(Color::Rgb(207, 159, 255))),
        Span::styled("2", Style::new().fg(Color::Rgb(255, 80, 80))),
        Span::styled("]", Style::new().fg(Color::Rgb(207, 158, 255))),
        
    ]);

    Paragraph::new(line)
}
