use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color};

// this file is ugly. but it works

pub fn processes_list<'a>() -> Line<'a> {
    let text_beg = Span::raw(" Processes  [");
    let d = Span::styled(
        "C", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_mid = Span::raw("] up  [");
    let c = Span::styled(
        "D", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_end = Span::raw("] down ");

    Line::from(vec![text_beg, c, text_mid, d, text_end])
}


pub fn core_list<'a>() -> Line<'a> {
    let text_beg = Span::raw(" Thread usage  [");
    let f = Span::styled(
        "F", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_mid = Span::raw("] up  [");
    let v = Span::styled(
        "V", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_end = Span::raw("] down ");

    Line::from(vec![text_beg, f, text_mid, v, text_end])
}


pub fn disks_list<'a>() -> Line<'a> {
    let text_beg = Span::raw(" Disks  [");
    let g = Span::styled(
        "G", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_mid = Span::raw("] up [");
    let b = Span::styled(
        "B", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_end = Span::raw("] down ");
    
    Line::from(vec![text_beg, g, text_mid, b, text_end])
}


pub fn networks_list<'a>() -> Line<'a> {
    let text_beg = Span::raw(" Networks  [");
    let h = Span::styled(
        "H", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_mid = Span::raw("] up [");
    let n = Span::styled(
        "N", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_end = Span::raw("] down ");
    
    Line::from(vec![text_beg, h, text_mid, n, text_end])
}


pub fn networks_details<'a>() -> Line<'a> {
    let text_beg = Span::raw(" Disks  [");
    let up = Span::styled(
        "\u{2191}", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_mid = Span::raw("] up [");
    let down = Span::styled(
        "\u{2193}", Style::default().fg(Color::Rgb(255, 80, 80))
    );

    let text_end = Span::raw("] down ");
    
    Line::from(vec![text_beg, up, text_mid, down, text_end])
}
