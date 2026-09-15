use crate::{SortProcessesBy, SystemContext};
use crate::ui::{charts, size};
use sysinfo::{System, Pid};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;
use ratatui::style::Style;

pub struct Process {
    pid: Pid,
    name: String,
    exec_path: String,
    memory: u64,
    cpu: f32,
}

impl Process {
    pub fn update(system: &System, sort_by: &SortProcessesBy) -> Vec<Self> {
        // processes list
        let mut list: Vec<Self> = Vec::new();
        for (pid, proc) in system.processes() {
            // clearing duplicates (proc.thread_kind() is None only if this is a main process)
            if proc.thread_kind().is_some() {
                continue;
            }

            // adding process to the list
            list.push(Self {
                pid: *pid,
                name: proc.name().to_string_lossy().to_string(),
                exec_path: match proc.exe() {
                    Some(path) => path.to_string_lossy().to_string(),
                    None => String::from("-"),
                },
                memory: proc.memory(),
                cpu: proc.cpu_usage(),
            });
        }

        // sorting processes
        match sort_by {
            SortProcessesBy::Cpu => {
                list.sort_unstable_by(|a, b| {
                    b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortProcessesBy::Memory => {
                list.sort_unstable_by(|a, b| {
                    b.memory.partial_cmp(&a.memory).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }

        list
    }
}

pub fn list_items<'a>(proc_list: &Vec<Process>, context: &SystemContext) -> Vec<ListItem<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    for proc in proc_list {
        let pid_7_chars: String = proc.pid.to_string().chars().take(7).collect();
        let pid_span = Span::from(format!("{:>7}  ", pid_7_chars));

        let name_20_chars: String = proc.name.chars().take(20).collect();
        let name = Span::from(format!("{:<20}  ", name_20_chars));

        let exec_path_50_chars: String = proc.exec_path.chars().take(50).collect();
        let exec_path = Span::from(format!("{:<50}  ", exec_path_50_chars));
            
        let cpu_usage_ratio = proc.cpu / context.cpu_model.total_threads as f32;
        let cpu_usage_color = charts::dynamic_color(&(cpu_usage_ratio as f64 / 100.0));
        let cpu_usage = Span::styled(format!("  {:>5.1}%   ", cpu_usage_ratio), Style::default().fg(cpu_usage_color));

        let ram_usage_b = proc.memory as f64;
        let ram_usage_ratio = ram_usage_b / context.total_ram as f64;
        let ram_usage_color = charts::dynamic_color(&ram_usage_ratio);
        let ram_usage = Span::styled(format!("   {:>5}", size::parse_short(ram_usage_b)), Style::default().fg(ram_usage_color));

        lines.push(Line::from(vec![
            pid_span,
            name,
            exec_path,
            cpu_usage,
            ram_usage,
        ]));
    }

    let list_items: Vec<ListItem> = lines
        .into_iter()
        .map(ListItem::from)
        .collect();

    list_items
}
