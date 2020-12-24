use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_timers(delay: Arc<Mutex<u8>>, sound: Arc<Mutex<u8>>) {
    thread::spawn(move || {
        loop {
            let start = Instant::now();
            let mut delay_data = delay.lock().unwrap();
            if *delay_data > 0 { *delay_data -= 1 };
            drop(delay_data);

            let mut sound_data = sound.lock().unwrap();
            if *sound_data > 0 { *sound_data -= 1; };
            drop(sound_data);

            thread::sleep(Duration::from_millis((1000 / 60) - start.elapsed().as_millis() as u64));
        }
    });
}