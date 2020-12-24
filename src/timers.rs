extern crate soloud;

use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::thread;

use soloud::*;


pub fn start_timers(delay: Arc<Mutex<u8>>, sound: Arc<Mutex<u8>>) {
    thread::spawn(move || {
        let sl = Soloud::default().unwrap();
        let mut wav = audio::Wav::default();
        wav.load_mem(include_bytes!("../res/beep.wav")).unwrap();
        loop {
            let start = Instant::now();
            let mut delay_data = delay.lock().unwrap();
            if *delay_data > 0 { *delay_data -= 1 };
            drop(delay_data);

            let mut sound_data = sound.lock().unwrap();
            if *sound_data > 0 {
                println!("Playing sound!");
                *sound_data -= 1;
                sl.play(&wav);
            };
            drop(sound_data);

            thread::sleep(Duration::from_millis((1000 / 60) - start.elapsed().as_millis() as u64));
        }
    });
}