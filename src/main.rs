mod metronome;
mod stats;

use std::env;
use std::io::{self, Write};
use std::sync::mpsc;
//use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::metronome::Metronome;
use crate::stats::Stats;

const NUM_SUBTICKS: u64 = 32;
const TICK_LEN_NANOS: u64 = 1024_000_000_000 / (60 * NUM_SUBTICKS);
// For MH
//const FRAME_TO_GET: u32 = 459;
// For DC
//const FRAME_TO_GET: u32 = 640;
// For MH and WC
//const LOAD_DELAY: u32 = 480;
// For DC
//const LOAD_DELAY: u32 = 439;

fn main() {
    let mut args = env::args().skip(1);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("[debug] Hunting IL Metronome v{}", env!("CARGO_PKG_VERSION"));

    let load_delay: u32 = args.next()
        .map(|s| s.parse().unwrap())
        .unwrap_or_else(|| {
            loop {
                print!("[input] Load delay: ");
                stdout.flush().unwrap();

                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                match buf.trim().parse() {
                    Ok(val) => return val,
                    Err(e) => println!("[error] {}", e),
                }
            }
        });
    let target_frame: u32 = args.next()
        .map(|s| s.parse().unwrap())
        .unwrap_or_else(|| {
            loop {
                print!("[input] Target frame: ");
                stdout.flush().unwrap();

                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                match buf.trim().parse() {
                    Ok(val) => return val,
                    Err(e) => println!("[error] {}", e),
                }
            }
        });

    let (sender, receiver) = mpsc::channel();
//    let console_mutex = Arc::new(Mutex::new(()));
    let mut stats = Stats::new();
    let mut first = true;

//    let console_mutex_clone = console_mutex.clone();
    let mut metro = Metronome::new(|tick| {
        let sub_tick = tick % NUM_SUBTICKS;
//        if let Ok(_lock) = console_mutex_clone.try_lock() {
            if sub_tick == 0 {
                println!("[timer] ===TICK===")
            }
            else {
                println!("[timer] {}", sub_tick)
            }
//        }

        receiver.try_recv().ok().map(|frame: u32| {
            if !first {
                stats.add_sample((frame as i32 - target_frame as i32) as f32);
                let (n, mean, std_dev) = stats.stats(None);
                let (_, mean_10, std_dev_10) = stats.stats(Some(10));
                println!("[stats] Stats:");
                println!("[stats]   n: {}", n);
                println!("[stats]   mean: {}", mean);
                println!("[stats]   stdd: {}", std_dev);
                println!("[stats]   mean_10: {}", mean_10);
                println!("[stats]   stdd_10: {}", std_dev_10);
            }
            else {
                println!("[stats] Ignoring stats on tuning set");
                first = false;
            }
            let frames_to_wait = (target_frame - (frame + load_delay) + 1024 + 1024) % 1024;
            Duration::from_nanos(1_000_000_000 / 60) * frames_to_wait
        })
    }, Duration::from_nanos(TICK_LEN_NANOS));

    thread::spawn(move || {
        let mut buf = String::new();
        loop {
            buf.clear();
            // Enter press to enter input state.
            stdin.read_line(&mut buf).unwrap();

            // Fix a dumb bug with powershell that might have to do with
            // locking a mutex on another thread when a ^C has been received.
            if buf == "" {
                return;
            }

            buf.clear();
//            let _lock = console_mutex.lock().unwrap();

            print!("[input] Frame entered: ");
            stdout.flush().unwrap();
            stdin.read_line(&mut buf).unwrap();
            match buf.trim().parse() {
                Ok(frame) => sender.send(frame).unwrap(),
                Err(e) => println!("[error] {}", e),
            };
        }
    });

    println!("[debug] Starting timer.");
    println!("[debug] load_delay is {}", load_delay);
    println!("[debug] target frame is {}", target_frame);
    metro.run();
}
