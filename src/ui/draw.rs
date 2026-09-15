use crate::update::SystemDetails;
use crate::{ListStates, system, cpu, memory, processes, disks, networks};
use crate::ui::layout::AppLayout;
use crate::ui::blocks::BorderedBlocks;
use crate::SystemContext;
use std::sync::{Arc, Mutex};
use ratatui::Frame;
use ratatui::widgets::Paragraph;

// main function to render blocks, charts and information

pub fn draw(f: &mut Frame, system: &SystemDetails, context: &SystemContext, list_states: &Arc<Mutex<ListStates>>) {
    // creating layout
    let layout = AppLayout::generate(f.area());

    // rendering bordered blocks
    let blocks = BorderedBlocks::new();

    f.render_widget(blocks.system, layout.borders.system);
    f.render_widget(blocks.cpu, layout.borders.cpu_info);
    f.render_widget(blocks.cpu_core_list, layout.borders.cpu_thread_list);
    f.render_widget(blocks.ram, layout.borders.memory);
    f.render_widget(blocks.processes, layout.borders.processes);
    f.render_widget(blocks.disks_list, layout.borders.disks_list);
    f.render_widget(blocks.disks_details, layout.borders.disk_details);
    f.render_widget(blocks.networks_list, layout.borders.networks_list);
    f.render_widget(blocks.networks_details, layout.borders.network_details);

    // rendering system information
    let system_info = system::display(system, &context.static_os_info);

    f.render_widget(system_info, layout.system);

    // rendering CPU information
    let cpu_info = cpu::display_cpu_info(system, context);

    f.render_widget(cpu_info.model, layout.cpu_info[0]);
    f.render_widget(cpu_info.cores, layout.cpu_info[1]);
    f.render_widget(cpu_info.global_usage_label, layout.cpu_global_usage[0]);
    f.render_widget(cpu_info.global_usage, layout.cpu_global_usage[1]);
    f.render_widget(cpu_info.temperature, layout.cpu_info[3]);

    { // cores list
        let mut state = list_states.lock().unwrap();
        let core_list = cpu::display_core_list(system);

        f.render_stateful_widget(core_list, layout.cpu_thread_list, &mut state.cores);
    }

    // rendering memory information
    let memory = memory::display(system, context);

    f.render_widget(memory.ram_usage, layout.memory[0]);
    f.render_widget(memory.ram_usage_gauge, layout.memory[1]);
    f.render_widget(memory.available_ram, layout.memory[2]);
    f.render_widget(memory.swap_usage, layout.memory[4]);
    f.render_widget(memory.swap_usage_gauge, layout.memory[5]);
    f.render_widget(memory.available_swap, layout.memory[6]);

    // rendering processes list
    {
        let mut state = list_states.lock().unwrap();
        let processes_list = processes::display_list(&system.processes_list, context);

        f.render_widget(processes::display::sorting(), layout.processes[0]);
        f.render_widget(processes::display::legend(), layout.processes[1]);
        f.render_stateful_widget(processes_list, layout.processes[2], &mut state.processes);
    }

    // rendering disks information
    {
        // disks list
        let mut state = list_states.lock().unwrap();
        let disks_list = disks::display_list(&system.disks);
        
        f.render_stateful_widget(disks_list, layout.disks_list, &mut state.disks);

        // disk details
        let position = state.disks.selected().or(Some(0)).unwrap();

        disks::display_details(f, &layout, &system.disks[position]);
    }

    // rendering networks information
    if system.networks.len() > 0 {
        // networks list
        let mut state = list_states.lock().unwrap();
        let networks_list = networks::display_list(&system.networks);

        f.render_stateful_widget(networks_list, layout.networks_list, &mut state.networks_list);

        // network details
        let position = state.networks_list.selected().or(Some(0)).unwrap();
        let viewport_height = layout.network_details.height;
        let network_details = networks::display_details(&system.networks[position], viewport_height);

        let scroll = state.networks_details.min(network_details.max_scroll);
        state.networks_details = scroll;  // updating global list state

        f.render_widget(network_details.content.scroll((scroll, 0)), layout.network_details);
    } else {
        f.render_widget(Paragraph::new("No networks found."), layout.networks_list);
    }
}
