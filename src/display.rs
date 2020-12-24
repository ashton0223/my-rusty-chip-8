extern crate sdl2;

use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::thread;


use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const MULTIPLIER: i32 = 16;

pub fn start_display(mutex: Arc<Mutex<[[bool; 32]; 64]>>, keys_mutex: Arc<Mutex<[bool; 16]>>) {
    thread::spawn(move || {
        let sdl_content = sdl2::init().unwrap();
        let video_subsystem = sdl_content.video().unwrap();
    
        let window = video_subsystem.window(
             "CHIP-8 Interpreter",
             64 * MULTIPLIER as u32,
             32 * MULTIPLIER as u32
            )
            .position_centered()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().build().unwrap();

        let mut event_pump = sdl_content.event_pump().unwrap();

        canvas.present();
    
        'display: loop {
            let start = Instant::now();
            let mut key_data = keys_mutex.lock().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'display
                    }
                    Event::KeyDown { keycode: key,.. } => {
                        match key {
                            Some(Keycode::Num1) => {
                                key_data[0x1] = true;
                            }
                            Some(Keycode::Num2) => {
                                key_data[0x2] = true;
                            }
                            Some(Keycode::Num3) => {
                                key_data[0x3] = true;
                            }
                            Some(Keycode::Num4) => {
                                key_data[0xC] = true;
                            }
                            Some(Keycode::Q) => {
                                key_data[0x4] = true;
                            }
                            Some(Keycode::W) => {
                                key_data[0x5] = true;
                            }
                            Some(Keycode::E) => {
                                key_data[0x6] = true;
                            }
                            Some(Keycode::R) => {
                                key_data[0xD] = true;
                            }
                            Some(Keycode::A) => {
                                key_data[0x7] = true;
                            }
                            Some(Keycode::S) => {
                                key_data[0x8] = true;
                            }
                            Some(Keycode::D) => {
                                key_data[0x9] = true;
                            }
                            Some(Keycode::F) => {
                                key_data[0xE] = true;
                            }
                            Some(Keycode::Z) => {
                                key_data[0xA] = true;
                            }
                            Some(Keycode::X) => {
                                key_data[0x0] = true;
                            }
                            Some(Keycode::C) => {
                                key_data[0xB] = true;
                            }
                            Some(Keycode::V) => {
                                key_data[0xF] = true;
                            }
                            _ => {}
                        }
                    }
                    Event::KeyUp { keycode: key, .. } => {
                        match key {
                            Some(Keycode::Num1) => {
                                key_data[0x1] = false;
                            }
                            Some(Keycode::Num2) => {
                                key_data[0x2] = false;
                            }
                            Some(Keycode::Num3) => {
                                key_data[0x3] = false;
                            }
                            Some(Keycode::Num4) => {
                                key_data[0xC] = false;
                            }
                            Some(Keycode::Q) => {
                                key_data[0x4] = false;
                            }
                            Some(Keycode::W) => {
                                key_data[0x5] = false;
                            }
                            Some(Keycode::E) => {
                                key_data[0x6] = false;
                            }
                            Some(Keycode::R) => {
                                key_data[0xD] = false;
                            }
                            Some(Keycode::A) => {
                                key_data[0x7] = false;
                            }
                            Some(Keycode::S) => {
                                key_data[0x8] = false;
                            }
                            Some(Keycode::D) => {
                                key_data[0x9] = false;
                            }
                            Some(Keycode::F) => {
                                key_data[0xE] = false;
                            }
                            Some(Keycode::Z) => {
                                key_data[0xA] = false;
                            }
                            Some(Keycode::X) => {
                                key_data[0x0] = false;
                            }
                            Some(Keycode::C) => {
                                key_data[0xB] = false;
                            }
                            Some(Keycode::V) => {
                                key_data[0xF] = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            drop(key_data);

            // Clear screen, then set color to white to draw each pixel
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));

            // Get display data from mutex, then unlock
            let data = mutex.lock().unwrap();
            for (i, i_item) in data.iter().enumerate() {
                for (j, j_item) in i_item.iter().enumerate() {
                    if *j_item {
                        canvas.fill_rect(Rect::new(
                             i as i32 * MULTIPLIER,
                             j as i32 * MULTIPLIER,
                             1 * MULTIPLIER as u32,
                             1 * MULTIPLIER as u32,
                            ))
                            .expect("Drawing failed");
                        
                    }
                }
            }
            drop(data);

            canvas.present();

            let delay: i32 = (1000 / 60) as i32 - start.elapsed().as_millis() as i32;

            if delay > 0 {
                thread::sleep(Duration::from_millis(delay as u64));
            }
        }
    });
}