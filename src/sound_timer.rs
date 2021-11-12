use crate::consts;
use std::time::{Duration};

use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};


pub struct SoundTimer {}

impl SoundTimer {
    pub fn init_timer() -> SoundTimer {
        return SoundTimer{};
    }
    
    pub fn set_value(&mut self, value: u8) {
        info!("Sound timer value set to {}", value);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        // let source = SineWave::new(440).take_duration(
        //     Duration::from_millis((consts::TIMER_TICK_MILLIS * value as f32) as u64)).amplify(1.20);
        let source = SineWave::new(440).take_duration(Duration::from_secs(5)).amplify(1.20);
        let play_res = stream_handle.play_raw(source);
        if play_res.is_err() {
            warn!("Failed playing sound due to : {}", play_res.unwrap_err());
        } else {
            info!("No error while trying to play sound");
        }
    }
}