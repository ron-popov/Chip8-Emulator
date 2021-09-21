mod cpu;
mod memory;
mod types;
mod consts;

pub use cpu::CPU;
pub use memory::{Memory, init_empty_memory};
pub use types::Double;
pub use types::Byte;

#[macro_use] extern crate log;
use simplelog::{ConfigBuilder, Level, CombinedLogger, TermLogger, LevelFilter, TerminalMode, Color, ColorChoice};

fn main() { 
    // Initialize logger
    let mut config_builder = ConfigBuilder::new();
    config_builder.set_level_color(Level::Info, Some(Color::Green));
    config_builder.set_location_level(LevelFilter::Off);
    config_builder.set_target_level(LevelFilter::Error);

    let config = config_builder.build();
    let logging_vector: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(LevelFilter::Info, config.clone(), TerminalMode::Mixed, ColorChoice::Auto)];
    // logging_vector.push(WriteLogger::new(LevelFilter::Trace, config.clone(), File::create("nessy.log").unwrap()));

    let _ = CombinedLogger::init(logging_vector);

    // Initialize memory and cpu
    info!("Starting Chip8");
    
    let cpu = CPU{memory_space: init_empty_memory()};
}
