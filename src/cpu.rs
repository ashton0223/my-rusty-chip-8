extern crate rand;

use std::sync::{Arc, Mutex};
use std::fs;
use std::time::{Instant, Duration};
use std::thread;
use std::num::Wrapping;
use std::process;

use rand::Rng;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
pub struct Cpu {
    pub mem: [u8; 4096],
    pub display: Arc<Mutex<[[bool; 32]; 64]>>,
    pub pc: u16,
    pub i: u16,
    pub delay_timer: Arc<Mutex<u8>>,
    pub sound_timer: Arc<Mutex<u8>>,
    pub v: [u8; 16],
    pub sp: usize,
    pub stack: [u16; 16],
    pub keys_pressed: Arc<Mutex<[bool; 16]>>,
}

impl Cpu {
    pub fn load_font(&mut self) {
        for index in 0..80 {
            self.mem[index+80] = FONT[index];
        }
    }

    pub fn load_program(&mut self, filename: String) {
        let file = fs::read(&filename);
        match file {
            Ok(v) => { 
                println!("Loading {}", filename);
                let file_vect = v;
                for (i, byte) in file_vect.iter().enumerate() {
                    self.mem[0x200 + i] = *byte;
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                process::exit(2);
            }
        }
    }

    fn push_stack(&mut self, value: u16) {
        self.sp += 1;
        self.stack[self.sp-1] = value;
    }

    fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp]
    }

    pub fn run_program(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let start = Instant::now();
            // Grab two bytes and combine into one opcode
            let op = ((self.mem[self.pc as usize] as u16) << 8) + self.mem[self.pc as usize + 1] as u16;
            self.pc += 2;
            let x = (op & 0x0F00) as usize >> 8;
            let y = (op & 0x00F0) as usize >> 4;
            let n = (op & 0x000F) as u8;
            let nn = (op & 0x00FF) as u8;
            let nnn = op & 0x0FFF;
            match op & 0xF000 {
                0x0000 => {
                    match op {
                        0x00E0 => {
                            let mut data = self.display.lock().unwrap();
                            for i in 0..data.len() {
                                data[i] = [false; 32]
                            }
                            drop(data);
                        }
                        0x00EE => {
                            self.pc = self.pop_stack();
                        }
                        _ => {}
                    }
                }
                0x1000 => {
                    self.pc = nnn;
                }
                0x2000 => {
                    self.push_stack(self.pc);
                    self.pc = nnn;
                }
                0x3000 => {
                    if self.v[x] == nn {
                        self.pc += 2;
                    }
                }
                0x4000 => {
                    if self.v[x] != nn {
                        self.pc += 2;
                    }
                }
                0x5000 => {
                    if self.v[x] == self.v[y] {
                        self.pc += 2;
                    }
                }
                0x6000 => {
                    self.v[x] = nn;
                }
                0x7000 => {
                    let register = Wrapping(self.v[x]);
                    let add_num = Wrapping(nn);
                    self.v[x] = (register + add_num).0;
                }
                0x8000 => {
                    match op & 0x000F {
                        0x0000 => {
                            self.v[x] = self.v[y];
                        }
                        0x0001 => {
                            self.v[x] = self.v[x] | self.v[y];
                        }
                        0x0002 => {
                            self.v[x] = self.v[x] & self.v[y];
                        }
                        0x0003 => {
                            self.v[x] = self.v[x] ^ self.v[y];
                        }
                        0x0004 => {
                            let sum = self.v[x] as u16 + self.v[y] as u16;
                            if sum > 255 {
                                self.v[0xF] = 1;
                            } else {
                                self.v[0xF] = 0;
                            }
                            self.v[x] = (sum % 256) as u8;
                        }
                        0x0005 => {
                            let reg_x = Wrapping(self.v[x]);
                            let reg_y = Wrapping(self.v[y]);
                            if reg_x > reg_y {
                                self.v[0xF] = 1;
                            } else  {
                                self.v[0xF] = 0;
                            }
                            self.v[x] = (reg_x - reg_y).0;
                        }
                        0x0006 => {
                            self.v[0xF] = (self.v[x] << 7) >> 7;
                            self.v[x] = self.v[x] >> 1;
                        }
                        0x0007 => {
                            let reg_x = Wrapping(self.v[x]);
                            let reg_y = Wrapping(self.v[y]);
                            if reg_y > reg_x {
                                self.v[0xF] = 1;
                            } else {
                                self.v[0xF] = 0;
                            }
                            self.v[x] = (reg_y - reg_x).0
                        }
                        0x000E => {
                            self.v[0xF] = self.v[x] >> 7;
                            self.v[x] = self.v[x] << 1;
                        }
                        _ => {
                            println!("{:x?}", op);
                            panic!("Not implemented"); 
                        }
                    }
                }
                0x9000 => {
                    if self.v[x] != self.v[y] {
                        self.pc += 2;
                    }
                }
                0xA000 => {
                    self.i = nnn;
                }
                0xB000 => {
                    let mut addr = nnn;
                    addr += self.v[0] as u16;
                    self.pc = addr;
                }
                0xC000 => {
                    let random: u8 = rng.gen();
                    self.v[x] = random & nn;
                }
                0xD000 => {
                    self.v[0xF] = 0;
                    let cdts = ((self.v[x as usize] % 64) as usize, (self.v[y as usize] % 32) as usize);
                    for i in 0..n {
                        let row = self.mem[(self.i + i as u16) as usize];
                        for j in 0..8 {
                            let to_and = 0x1 << j;
                            if row & to_and != 0 {
                                let mut data = self.display.lock().unwrap();
                                let mut x = cdts.0+7-j;
                                let mut y = cdts.1+i as usize;
                                if x >= 64 { x -= 64 };
                                if y >= 32 { y -= 32 };
                                data[x][y] = !data[x][y];
                                if !data[x][y] {
                                    self.v[0xF] = 1;
                                }
                            }
                        }
                    }
                }
                0xE000 => {
                    let key_data = self.keys_pressed.lock().unwrap();
                    match nn {
                        0x009E => {
                            if key_data[self.v[x] as usize] {
                                self.pc += 2;
                            }
                        }
                        0x00A1 => {
                            if !key_data[self.v[x] as usize] {
                                self.pc += 2;
                            }
                        }
                        _ => {}
                    }
                }
                0xF000 => {
                    match nn {
                        0x0007 => {
                            let delay_data = self.delay_timer.lock().unwrap();
                            self.v[x] = *delay_data;
                        },
                        0x0015 => {
                            let mut delay_data = self.delay_timer.lock().unwrap();
                            *delay_data = self.v[x];
                        },
                        0x0018 => {
                            let mut sound_data = self.delay_timer.lock().unwrap();
                            *sound_data = self.v[x];
                        },
                        0x001E => {
                            self.i += self.v[x] as u16;
                            if self.i > 0x0FFF {
                                self.v[0xF] = 1;
                            }
                        },
                        0x000A => {
                            'block: loop {
                                let key_data = self.keys_pressed.lock().unwrap();
                                for (i, item) in (*key_data).iter().enumerate() {
                                    if *item {
                                        self.v[x] = i as u8;
                                        break 'block
                                    }
                                }
                            }
                        },
                        0x0029 => {
                            self.i = 0x50 + ((self.v[x] & 0x0F) * 5) as u16; 
                        },
                        0x0033 => {
                            let num = self.v[x];
                            let num_hundreds = num / 100;
                            let num_tens = (num - (num_hundreds * 100)) / 10;
                            let num_ones = num - (num_hundreds * 100) - (num_tens * 10);
                            self.mem[self.i as usize] = num_hundreds;
                            self.mem[self.i as usize+1] = num_tens;
                            self.mem[self.i as usize+2] = num_ones;
                        },
                        0x0055 => {
                            for i in 0..x+1 {
                                self.mem[self.i as usize+i as usize] = self.v[i as usize];
                            }
                        },
                        0x0065 => {
                            for i in 0..x+1 {
                                self.v[i as usize] = self.mem[self.i as usize+i as usize];
                            }
                        },
                        _ => {}
                    }
                }
                _ => { println!("Not implemented yet, {:x?}", op); }
            }

            let elapsed_ms = start.elapsed().as_millis() as u64;
            if elapsed_ms < 2 { thread::sleep(Duration::from_millis(2 - elapsed_ms)); }
        }
    }
}