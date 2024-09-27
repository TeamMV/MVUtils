use std::thread;
use std::time::Duration;
use crate::utils::Time;

pub struct Clock {
    tps: u16,
    current_start: u128,
    running: bool,
    to_pass: u128,
}

impl Clock {
    pub fn new(tps: u16) -> Self {
        Self {
            tps,
            current_start: 0,
            running: false,
            to_pass: 1000u128 / tps as u128,
        }
    }

    pub fn new_and_start(tps: u16) -> Self {
        Self {
            tps,
            current_start: u128::time_millis(),
            running: true,
            to_pass: 1000u128 / tps as u128,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.current_start = u128::time_millis();
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    fn rdy(&mut self) -> bool {
        if !self.running { return false; }

        let time = u128::time_millis();
        if time - self.current_start > self.to_pass {
            self.current_start = time;
            return true;
        }
        false
    }

    pub fn tick(&mut self) -> bool {
        self.rdy()
    }

    pub fn wait_tick(&mut self, delay: u64) {
        while !self.rdy() {
            if delay > 0 {
                thread::sleep(Duration::from_millis(delay))
            }
        }
    }
}