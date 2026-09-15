use std::rc::Rc;
use ratatui::layout::{Constraint, Layout, Spacing, Flex, Rect, Margin};

// struct to define borders & make rects list more readable
pub struct Rects {
    pub system: Rect,
    pub cpu_info: Rect,
    pub cpu_thread_list: Rect,
    pub memory: Rect,
    pub processes: Rect,
    pub disks_list: Rect,
    pub disk_details: Rect,
    pub networks_list: Rect,
    pub network_details: Rect,
}

// app layout
pub struct AppLayout {
    pub borders: Rects,
    pub system: Rect,
    pub cpu_info: Rc<[Rect]>,
    pub cpu_global_usage: Rc<[Rect]>,
    pub cpu_thread_list: Rect,
    pub memory: Rc<[Rect]>,
    pub processes: Rc<[Rect]>,
    pub disks_list: Rect,
    pub disk_details: Vec<Rc<[Rect]>>,
    pub disk_usage_gauge: Rc<[Rect]>,
    pub networks_list: Rect,
    pub network_details: Rect,
}

impl AppLayout {
    pub fn generate(window_size: Rect) -> Self {

        // layout definitions - they define how rects are split on screen
        // vertical
        let ld_vertical_35_65 = Layout::vertical([
            Constraint::Percentage(35), Constraint::Fill(1)
        ]);
        let ld_vertical_50_50 = Layout::vertical([
            Constraint::Fill(1); 2
        ]);
        let ld_vertical_50_50_overlap = Layout::vertical([
            Constraint::Fill(1); 2
        ]).spacing(Spacing::Overlap(1));

        // horizontal
        let ld_horizontal_3 = Layout::horizontal([
            Constraint::Fill(1); 3
        ]);
        let ld_horizontal_60_40 = Layout::horizontal([
            Constraint::Percentage(60), Constraint::Fill(1)
        ]);
        let ld_horizontal_50_50 = Layout::horizontal([
            Constraint::Fill(1); 2
        ]);
        let ld_horizontal_50_50_overlap = Layout::horizontal([
            Constraint::Fill(1); 2
        ]).spacing(Spacing::Overlap(1));
        let ld_horizontal_40_60_overlap = Layout::horizontal([
            Constraint::Percentage(40), Constraint::Fill(1)
        ]).spacing(Spacing::Overlap(1));


        // rects - these are separate areas rendered in the terminal - effectively splitting
        // terminal window into separate blocks as defined in layout definitions
        let rects_terminal_35_65 = ld_vertical_35_65.split(window_size);
        let rects_terminal_top = ld_horizontal_3.split(rects_terminal_35_65[0]);
        let rects_terminal_bottom = ld_horizontal_60_40.split(rects_terminal_35_65[1]);
        let rects_terminal_bottom_right = ld_vertical_50_50.split(rects_terminal_bottom[1]);

        let rects_cpu = ld_vertical_50_50_overlap.split(rects_terminal_top[1]);
        let rects_disks = ld_horizontal_50_50_overlap.split(rects_terminal_bottom_right[0]);
        let rects_networks = ld_horizontal_40_60_overlap.split(rects_terminal_bottom_right[1]);

        // helper structs init
        let rects = Rects {
            system: rects_terminal_top[0],
            cpu_info: rects_cpu[0],
            cpu_thread_list: rects_cpu[1],
            memory: rects_terminal_top[2],
            processes: rects_terminal_bottom[0],
            disks_list: rects_disks[0],
            disk_details: rects_disks[1],
            networks_list: rects_networks[0],
            network_details: rects_networks[1],
        };
        

        // blocks - these are usable areas created directly from rects - they also have set up
        // margins, flex directions and sub-areas (e.g. Vec<> of lines)
        // system
        let block_system = Layout::vertical([
            Constraint::Length(8)
        ]).flex(Flex::Center).split(
            rects.system.inner(Margin::new(8, 0))
        );

        // cpu
        let block_cpu_info = Layout::vertical([
            Constraint::Length(1); 4
        ]).split(
            rects.cpu_info.inner(Margin::new(3, 2))
        );
        let block_cpu_usage_line = Layout::horizontal([
            Constraint::Length(13), Constraint::Fill(1)
        ]).split(
            block_cpu_info[2]
        );
        let block_cpu_thread_list = rects.cpu_thread_list.inner(Margin::new(3, 1));

        // memory
        let block_memory = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
        ]).flex(Flex::Center).split(
            rects.memory.inner(Margin::new(5, 3))
        );

        // processes
        let block_processes = Layout::vertical([
            Constraint::Length(1), Constraint::Length(2), Constraint::Fill(1)
        ]).split(
            rects.processes.inner(Margin::new(4, 1))
        );

        // disks
        let block_disks_list = rects.disks_list.inner(Margin::new(4, 2));
        let block_disk_details = Layout::vertical([
            Constraint::Length(1); 11
        ]).split(
            Layout::vertical([
                Constraint::Length(11)
            ]).split(
                rects.disk_details.inner(Margin::new(4, 2))
            )[0]
        );
        let block_disk_usage_gauge = Layout::horizontal([
            Constraint::Fill(1), Constraint::Length(8)
        ]).split(block_disk_details[7]);
        let block_disk_details_lines: Vec<_> = block_disk_details
            .iter()
            .map(|&row| ld_horizontal_50_50.split(row))
            .collect();

        // networks 
        let block_networks_list = Layout::vertical([
            Constraint::Fill(1)
        ]).split(
            rects.networks_list.inner(Margin::new(4, 2))
        );
        let block_network_details = rects.network_details.inner(Margin::new(4, 1));

        
        // returning AppLayout
        Self {
            borders: rects,
            system: block_system[0],
            cpu_info: block_cpu_info,
            cpu_global_usage: block_cpu_usage_line,
            cpu_thread_list: block_cpu_thread_list,
            memory: block_memory,
            processes: block_processes,
            disks_list: block_disks_list,
            disk_usage_gauge: block_disk_usage_gauge,
            disk_details: block_disk_details_lines,
            networks_list: block_networks_list[0],
            network_details: block_network_details
        }
    }
}
