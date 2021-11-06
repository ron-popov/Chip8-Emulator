mod cpu;
mod memory;
mod consts;
mod errors;
mod stack;
mod delay_timer;

use cpu::CPU;
use memory::Memory;

use std::io::Read;
use std::fs::File;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[macro_use] extern crate log;
use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, Color, ColorChoice};

extern crate clap;
use clap::{Arg, App};

fn emulate() -> Result<(), String> {
    // Parse command line arguments
    let command_line_args = App::new("Chip8 Emulator")
                            .author("Ron Popov AKA DirtyAxe")
                            .arg(Arg::with_name("Rom File")
                                .short("f")
                                .long("rom-file")
                                .value_name("FILE_PATH")
                                .help("Path of a rom file to run")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("Verbosity")
                                .short("v")
                                .long("verbosity")
                                .value_name("Log Verbosity Level")
                                .help("Log Verbosity Level")
                                .takes_value(true)
                                .required(false))
                            .get_matches();

    let terminal_log_level_filter: LevelFilter = match command_line_args.value_of("Verbosity") {
        Some(level) => match level.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => {
                panic!("Unkown verbosity level specified");
            }
        },
        None => LevelFilter::Info,
    };

    // Initialize logger
    let mut config_builder = ConfigBuilder::new();
    config_builder.set_level_color(Level::Info, Some(Color::Green));
    config_builder.set_location_level(LevelFilter::Off);
    config_builder.set_target_level(LevelFilter::Error);

    let config = config_builder.build();
    let term_logger = TermLogger::new(terminal_log_level_filter, config.clone(), TerminalMode::Mixed, ColorChoice::Auto);
    let mut logging_vector: Vec<Box<dyn simplelog::SharedLogger>> = vec![term_logger];
    logging_vector.push(WriteLogger::new(LevelFilter::Trace, config.clone(), File::create("Chip8.log").unwrap()));
    let logger_init_result = CombinedLogger::init(logging_vector);

    if logger_init_result.is_err() {
        println!("Failed initializing logger : {}", logger_init_result.unwrap_err());
        return Ok(());
    }

    // Logger inialized and arguments parsed, PARTY
    info!("Starting Chip8");

    // Get rom file path from command line args
    let rom_file_path: String = command_line_args.value_of("Rom File").unwrap_or_else(|| {
        error!("Command line args are {:?}", command_line_args);
        panic!("Failed unwrapping rom file path");
    }).to_string();

    info!("Rom file path is \"{}\"", rom_file_path);

    // Initialize memory
    let rom_file: File = File::open(rom_file_path).expect("Failed opening rom file");

    let rom_content = rom_file.bytes().map(|value| {
        value.expect("Failed reading rom file")
    }).collect();

    let memory: Memory = Memory::new_from_rom(rom_content);

    // Initialize sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Chip8 Emulator", consts::DISPLAY_WIDTH as u32 * consts::SCALE_FACTOR as u32, consts::DISPLAY_HEIGHT as u32 * consts::SCALE_FACTOR as u32).build().unwrap();

    let mut canvas : Canvas<Window> = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    canvas.set_scale(consts::SCALE_FACTOR as f32, consts::SCALE_FACTOR as f32)?;
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;

    // Initialize cpu
    let mut cpu = CPU::new(memory, canvas);

    // Main loop
    'main_loop: loop {
        let keys: Vec<Scancode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            // .filter_map(Keycode::from_scancode)
            .collect();

        let cpu_result = cpu.execute_instruction(keys);
        if cpu_result.is_err() {
            error!("Leaving main loop, Got cpu error : {:?}", cpu_result.unwrap_err());
            break 'main_loop;
        }

        match event_pump.poll_event() {
            Some(event) => {
                match event {
                    Event::Quit {..} => {
                        error!("Got quit event");
                        break 'main_loop;
                    },
                    _ => {}
                }
            },
            None => {}
        }
    }

    return Ok(());
}

fn main() {
    let return_val = emulate();
    if return_val.is_err() {
        error!("Stopping due to {}", return_val.unwrap_err());
    }
}
