mod instances;
mod ui;
mod update;
mod system;
mod cpu;
mod memory;
mod disks;
mod processes;
mod networks;

use crate::system::data::StaticOsInfo;
use crate::update::SystemDetails;
use crate::cpu::data::CpuModel;
use crate::instances::{Instances, Refreshes};
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time::Duration};
use std::sync::{{Mutex, Arc}, atomic::{AtomicBool, Ordering}, mpsc};
use arc_swap::ArcSwap;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use sysinfo::System;
use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;

// structures
#[derive(Clone)]
struct SystemContext {
    cpu_model: CpuModel,
    total_ram: u64,
    total_swap: u64,
    static_os_info: StaticOsInfo,
}

struct AppInitArguments<'a> {
    system_details: &'a Arc<ArcSwap<SystemDetails>>,
    context: &'a SystemContext,
    sort: Arc<Mutex<SortProcessesBy>>,
}

struct ListStates {
    cores: ListState,
    disks: ListState,
    processes: ListState,
    networks_list: ListState,
    networks_details: u16,
}

// sorting processes list options
#[derive(Clone, Copy)]
pub enum SortProcessesBy {
    Cpu,
    Memory,
}

// global quit flag
static QUIT: AtomicBool = AtomicBool::new(false);

fn main() -> color_eyre::Result<()> {

    // shared state instances
    let refreshes = Refreshes::new();
    let instances = Instances::new(refreshes.system_refresh, refreshes.disks_refresh);

    // safely exit the program if it fails to fetch CPU core count
    let total_cores = System::physical_core_count().unwrap_or_else(|| std::process::exit(1));

    // creating CPU model struct and safe exit if it fails
    let cpu = instances.system.cpus().first().unwrap_or_else(|| std::process::exit(1));
    let cpu_model = CpuModel {
        vendor_id: cpu.vendor_id().to_string(),
        brand: cpu.brand().to_string(),
        total_cores,
        total_threads: instances.system.cpus().len(),
    };

    // context (immutable system information)
    let context = SystemContext {
        cpu_model,
        total_ram: instances.system.total_memory(),
        total_swap: instances.system.total_swap(),
        static_os_info: StaticOsInfo::get(&instances.system),
    };

    // shared mutable state for sorting processes
    let sort_processes = Arc::new(Mutex::new(SortProcessesBy::Cpu));
    let initial_sort = *sort_processes.lock().unwrap();

    // initial details without waiting for the first refresh
    let initial_details = update::SystemDetails::update(&instances, &initial_sort);

    let system_details_shared = Arc::new(ArcSwap::from_pointee(initial_details));
    let system_details_backend = Arc::clone(&system_details_shared);
    let system_details_tui = Arc::clone(&system_details_shared);

    // mpsc channels
    let (tx, rx) = mpsc::channel::<()>();
    let tx_crossterm = tx.clone();
    let tx_backend = tx.clone();

    let (tx_force, rx_force) = mpsc::channel::<()>();
    let tx_force_crossterm = tx_force.clone();

    // backend thread for updating system data once per second
    let sorting_processes_backend = Arc::clone(&sort_processes);
    thread::spawn(move || {
        let mut instances_local = instances;
        let refreshes = Refreshes::new();

        loop {
            match rx_force.recv_timeout(Duration::from_secs(1)) {
                Ok(_) => {}
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break, // safe exit if the channel closes
            }

            instances_local.system.refresh_specifics(refreshes.system_refresh);
            instances_local.disks.refresh_specifics(true, refreshes.disks_refresh);
            instances_local.networks.refresh(true);
            instances_local.components.refresh(true);

            let active_sort = *sorting_processes_backend.lock().unwrap();
            let updated_details = update::SystemDetails::update(&instances_local, &active_sort);

            system_details_backend.store(Arc::new(updated_details));

            let _ = tx_backend.send(()); // draw UI
        };
    });

    // shared mutable state for list states
    let list_states = Arc::new(Mutex::new(ListStates {
        cores: ListState::default().with_selected(Some(0)),
        disks: ListState::default().with_selected(Some(0)),
        processes: ListState::default(),
        networks_list: ListState::default().with_selected(Some(0)),
        networks_details: 0,
    }));

    // creating app() argument
    let args = AppInitArguments {
        system_details: &system_details_tui,
        context: &context,
        sort: Arc::clone(&sort_processes),
    };

    // init
    color_eyre::install()?;
    ratatui::run(|terminal| {
        app(terminal, args, &list_states, tx_crossterm, tx_force_crossterm, rx)
    })?;
    Ok(())
}

fn app(
    terminal: &mut DefaultTerminal,
    args: AppInitArguments,
    list_states: &Arc<Mutex<ListStates>>,
    tx_crossterm: Sender<()>,
    tx_force_crossterm: Sender<()>,
    rx: Receiver<()>
) -> std::io::Result<()> {
    
    // cloning references to pass them into separate thread
    let list_states_clone = Arc::clone(list_states);
    let sort_by_clone = Arc::clone(&args.sort);

    // crossterm event handling
    thread::spawn(move || {
        loop {
            if let Ok(event) = crossterm::event::read() {
                let mut states = list_states_clone.lock().unwrap();
                let mut should_render = false;

                match event {
                    Event::Key(key) => {
                        match (key.code, key.modifiers) {
                            // quit (esc / ctrl+c)
                            (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
                                QUIT.store(true, Ordering::Relaxed);
                                should_render = true;
                            }

                            // processes list (d / c)
                            (KeyCode::Char('d'), _) => {
                                states.processes.select_previous();
                                should_render = true;
                            }
                            (KeyCode::Char('c'), _) => {
                                states.processes.select_next();
                                should_render = true;
                            }

                            // cores list (f / v)
                            (KeyCode::Char('f'), _) => {
                                states.cores.select_previous();
                                should_render = true;
                            }
                            (KeyCode::Char('v'), _) => {
                                states.cores.select_next();
                                should_render = true;
                            }

                            // disks (g / b)
                            (KeyCode::Char('g'), _) => {
                                states.disks.select_previous();
                                should_render = true;
                            }
                            (KeyCode::Char('b'), _) => {
                                states.disks.select_next();
                                should_render = true;
                            }

                            // networks list (h / n)
                            (KeyCode::Char('h'), _) => {
                                states.networks_list.select_previous();
                                should_render = true;
                            }
                            (KeyCode::Char('n'), _) => {
                                states.networks_list.select_next();
                                should_render = true;
                            }

                            // network details (up / down)
                            (KeyCode::Up, _) => {
                                if states.networks_details > 0 {
                                    states.networks_details -= 1;
                                    should_render = true;
                                }
                            }
                            (KeyCode::Down, _) => {
                                states.networks_details += 1;
                                should_render = true;
                            }

                            // sorting processes (1 / 2)
                            (KeyCode::Char('1'), _) => {
                                let mut new_sort = sort_by_clone.lock().unwrap();
                                *new_sort = SortProcessesBy::Cpu;
                                should_render = true;
                                let _ = tx_force_crossterm.send(());
                            }
                            (KeyCode::Char('2'), _) => {
                                let mut new_sort = sort_by_clone.lock().unwrap();
                                *new_sort = SortProcessesBy::Memory;
                                should_render = true;
                                let _ = tx_force_crossterm.send(());
                            }


                            _ => ()
                        }
                    }

                    Event::Resize(_, _) => { should_render = true; }
                    _ => ()
                }

                if should_render {
                    // call main thread
                    let _ = tx_crossterm.send(());
                    thread::sleep(Duration::from_millis(16));
                }
            }
        }
    });

    // rendering loop
    loop {
        if QUIT.load(Ordering::Relaxed) {
            break Ok(());
        }

        // calling draw() with arguments
        let system_details_guard = args.system_details.load();
        terminal.draw(|frame| {
            crate::ui::draw(frame, &system_details_guard, args.context, list_states);
        })?;

        // waiting for a signal to draw ui / safe exit in case the channel closes
        if rx.recv().is_err() {
            break Ok(());
        }
    }
}
