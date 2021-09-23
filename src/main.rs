mod cpu;
mod memory;
mod types;
mod consts;

use cpu::CPU;
use types::Double;
use types::Byte;

use std::fs::File;

#[macro_use] extern crate log;
use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, Color, ColorChoice};

fn draw() {
    trace!("Drawing the screen");
}

fn main() { 
    // Initialize logger
    let mut config_builder = ConfigBuilder::new();
    config_builder.set_level_color(Level::Info, Some(Color::Green));
    config_builder.set_location_level(LevelFilter::Off);
    config_builder.set_target_level(LevelFilter::Error);

    let config = config_builder.build();
    let mut logging_vector: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(LevelFilter::Info, config.clone(), TerminalMode::Mixed, ColorChoice::Auto)];
    logging_vector.push(WriteLogger::new(LevelFilter::Trace, config.clone(), File::create("Chip8.log").unwrap()));
    let logger_init_result = CombinedLogger::init(logging_vector);

    if logger_init_result.is_err() {
        println!("Failed initializing logger : {}", logger_init_result.unwrap_err());
        return;
    }

    // Initialize memory and cpu
    info!("Starting Chip8");
    
    let cpu = CPU::new(draw);
}
