use crate::ui::blocks_labels;
use ratatui::layout::Alignment;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::Block;
use ratatui::style::{Style, Color};

pub struct BorderedBlocks<'a> {
    pub system: Block<'a>,
    pub cpu: Block<'a>,
    pub cpu_core_list: Block<'a>,
    pub ram: Block<'a>,
    pub processes: Block<'a>,
    pub disks_list: Block<'a>,
    pub disks_details: Block<'a>,
    pub networks_list: Block<'a>,
    pub networks_details: Block<'a>,
}

// function to create colored blocks - theyre placed on screen in layout.rs
impl BorderedBlocks<'_> {
    pub fn new() -> Self {

        let system = Block::bordered()
            .title(" System ")
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(255, 124, 124)));

        let cpu = Block::bordered()
            .title(" CPU ")
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(156, 255, 242)))
            .merge_borders(MergeStrategy::Exact);

        let cpu_core_list = Block::bordered()
            .title(blocks_labels::core_list())
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(156, 255, 242)))
            .merge_borders(MergeStrategy::Exact);

        let ram = Block::bordered()
            .title(" Memory ")
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(151, 252, 163)));

        let processes = Block::bordered()
            .title(blocks_labels::processes_list())
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(207, 159, 255)));


        let disks_list = Block::bordered()
            .title(blocks_labels::disks_list())
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(255, 253, 171)))
            .merge_borders(MergeStrategy::Exact);

        let disks_details = Block::bordered()
            .title(" Details ")
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(255, 253, 171)))
            .merge_borders(MergeStrategy::Exact);

        let networks_list = Block::bordered()
            .title(blocks_labels::networks_list())
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(126, 131, 247)))
            .merge_borders(MergeStrategy::Exact);

        let networks_details = Block::bordered()
            .title(blocks_labels::networks_details())
            .title_alignment(Alignment::Center)
            .style(Style::new().white().on_black())
            .border_style(Style::new().fg(Color::Rgb(126, 131, 247)))
            .merge_borders(MergeStrategy::Exact);

        
        Self {
            system,
            cpu,
            cpu_core_list,
            ram,
            processes,
            disks_list,
            disks_details,
            networks_list,
            networks_details,
        }
    }
}
