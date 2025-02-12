use std::time::{Duration, Instant};
use mvutils_proc_macro::Savable;

use crate as mvutils;

#[derive(Clone, Savable, Debug)]
pub struct Clock {
    last_tick: Instant,
    tick: Duration,
    enabled: bool,
}

impl Clock {
    pub fn new(tps: u16) -> Self {
        Clock {
            last_tick: Instant::now(),
            tick: Duration::from_micros(1_000_000 / tps as u64),
            enabled: true,
        }
    }

    pub fn new_disabled(tps: u16) -> Self {
        Clock {
            last_tick: Instant::now(),
            tick: Duration::from_micros(1_000_000 / tps as u64),
            enabled: false,
        }
    }

    pub fn set_tps(&mut self, tps: u16) {
        self.tick = Duration::from_micros(1_000_000 / tps as u64);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn ready_and_tick(&mut self) -> bool {
        if self.ready() {
            self.tick();
            true
        } else { false }
    }

    pub fn ready(&self) -> bool {
        if !self.enabled { return false; }
        self.last_tick.elapsed() >= self.tick
    }

    pub fn tick(&mut self) {
        if !self.enabled { return; }
        self.last_tick = Instant::now();
    }
}