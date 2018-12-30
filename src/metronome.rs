use std::time::{Duration, Instant};
use std::thread;

pub struct Metronome<F> {
    func: F,
    interval: Duration,
    base: Instant,
    tick: u64,
}

impl<F> Metronome<F>
    where F: FnMut(u64) -> Option<Duration>,
{
    pub fn new(func: F, interval: Duration) -> Metronome<F> {
        Metronome {
            func: func,
            interval: interval,
            base: Instant::now(),
            tick: 0,
        }
    }

    pub fn run(&mut self) {
        self.base = Instant::now();
        loop {
            let delay_opt = (self.func)(self.tick);
            if let Some(delay) = delay_opt {
                self.base += delay;
            }
            self.tick += 1;
            let till = self.base + self.interval * self.tick as u32;
            thread::sleep(till.duration_since(Instant::now()));
        }
    }
}