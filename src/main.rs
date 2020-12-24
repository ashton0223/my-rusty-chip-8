mod cpu;
mod display;
mod timers;

use std::sync::{Arc, Mutex};
use std::env;
use std::process;

fn main() {
    let display_mutex = Arc::new(
        Mutex::new([[false; 32]; 64])
    );
    let keys_mutex = Arc::new(Mutex::new([false; 16]));
    let delay_mutex = Arc::new(Mutex::new(0));
    let sound_mutex = Arc::new(Mutex::new(0));

    let display_mutex_processor_copy = Arc::clone(&display_mutex);
    let keys_mutex_processor_copy = Arc::clone(&keys_mutex);
    let delay_mutex_processor_copy = Arc::clone(&delay_mutex);
    let sound_mutex_processor_copy = Arc::clone(&sound_mutex);
    let mut processor = cpu::Cpu {
        mem: [0; 4096],
        display: display_mutex_processor_copy,
        pc: 0x200,
        i: 0,
        delay_timer: delay_mutex_processor_copy,
        sound_timer: sound_mutex_processor_copy,
        v: [0; 16],
        sp: 0,
        stack: [0; 16],
        keys_pressed: keys_mutex_processor_copy,
    };

    processor.load_font();

    let display_mutex_copy = Arc::clone(&display_mutex);
    let keys_mutex_copy = Arc::clone(&keys_mutex);
    display::start_display(display_mutex_copy, keys_mutex_copy);

    let delay_mutex_copy = Arc::clone(&delay_mutex);
    let sound_mutex_copy = Arc::clone(&sound_mutex);
    timers::start_timers(delay_mutex_copy, sound_mutex_copy);

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [ROM]", args[0]);
        process::exit(1);
    }
    let rom = args[1].clone();
    processor.load_program(rom);
    processor.run_program();
}
