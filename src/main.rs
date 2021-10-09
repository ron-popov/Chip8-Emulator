mod cpu;
mod memory;
mod consts;
mod errors;
mod stack;
mod display;

use cpu::CPU;
use memory::Memory;
use errors::Chip8Error;
use display::Display;

use std::io::Read;
use std::fs::File;

#[macro_use] extern crate log;
use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, Color, ColorChoice};

extern crate clap;
use clap::{Arg, App};

fn main() {
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
                            .get_matches();

    // Initialize logger
    let mut config_builder = ConfigBuilder::new();
    config_builder.set_level_color(Level::Info, Some(Color::Green));
    config_builder.set_location_level(LevelFilter::Off);
    config_builder.set_target_level(LevelFilter::Error);

    let config = config_builder.build();
    let mut logging_vector: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(LevelFilter::Trace, config.clone(), TerminalMode::Mixed, ColorChoice::Auto)];
    logging_vector.push(WriteLogger::new(LevelFilter::Trace, config.clone(), File::create("Chip8.log").unwrap()));
    let logger_init_result = CombinedLogger::init(logging_vector);

    if logger_init_result.is_err() {
        println!("Failed initializing logger : {}", logger_init_result.unwrap_err());
        return;
    }

    // Logger inialized and arguments parsed, PARTY
    info!("Starting Chip8");

    // Get rom file path from command line args
    let rom_file_path: String = command_line_args.value_of("Rom File").unwrap_or_else(|| {
        debug!("Command line args are {:?}", command_line_args);
        panic!("Failed unwrapping rom file path");
    }).to_string();

    info!("Rom file path is \"{}\"", rom_file_path);

    // Initialize memory
    let rom_file: File = File::open(rom_file_path).expect("Failed opening rom file");

    let rom_content = rom_file.bytes().map(|value| {
        value.expect("Failed reading rom file")
    }).collect();

    let memory: Memory = Memory::new_from_rom(rom_content);

    // Initialize display
    let display = Display::new();
    if display.is_err() {
        error!("Failed initializing display");
        return;
    }

    // Initialize cpu
    let mut cpu = CPU::new(memory, display.unwrap());

    // Main CPU Loop
    let mut cpu_result: Result<(), Chip8Error> = Ok(());
    while cpu_result.is_ok() {
        cpu_result = cpu.execute_instruction();
    }

    error!("Left main cpu, Got error : {:?}", cpu_result.unwrap_err());
}
