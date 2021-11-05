use crate::consts;
use std::time::{Instant};

pub struct DelayTimer {
    timer_value: u8,
    last_set_time: Instant
}

impl DelayTimer {
    pub fn init_timer() -> DelayTimer {
        return DelayTimer{timer_value: 0, last_set_time: Instant::now()};
    }

    pub fn set_value(&mut self, value: u8) {
        self.timer_value = value;
        self.last_set_time = Instant::now();
    }

    pub fn get_value(&mut self) -> u8 {
        let elapsed_millis = self.last_set_time.elapsed().as_millis();
        let ticks_ticked = (elapsed_millis as f32 / consts::DELAY_TIMER_TICK_MILLIS).floor() as u32;

        if ticks_ticked >= self.timer_value as u32 {
            self.set_value(0);
        } else {
            self.set_value(self.timer_value - ticks_ticked as u8)
        }

        return self.timer_value;
    }
}