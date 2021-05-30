use crate::mem::*;
use crate::reg::api::*;
use crate::utils::*;
use minifb::{Window, WindowOptions};
use std::process::exit;

const LCD_W: usize = 160;
const LCD_H: usize = 144;

const OAM_T: usize = 80;
const DRAW_T: usize = 172;
const H_BLK_T: usize = 204;
const V_BLK_T: usize = 456;

enum State {
    Oam,
    Draw,
    HBlank,
    VBlank,
}

//use std::time::Instant;

pub struct Display {
    cycles: usize,
    state: State,
    buff: Vec<u32>,
    win: Window,
    //    time: Instant,
    //   time_v: Vec<u128>,
}

impl Display {
    pub fn new() -> Display {
        let mut result = Display {
            cycles: 80,
            state: State::Oam,
            buff: vec![COLORS[0]; LCD_W * LCD_H],
            win: Window::new(
                "Falco's GBMU",
                LCD_W * 4,
                LCD_H * 4,
                WindowOptions::default(),
            )
            .unwrap_or_else(|_| fatal_err("Can't open game window", 10)),
            //           time: Instant::now(),
            //           time_v: Vec::new(),
        };
        result
            .win
            .limit_update_rate(Some(std::time::Duration::from_millis(17)));
        result
    }

    fn update_ly(m: MMy) {
        m.set(LY, (m.get(LY) + 1) % 154);
        if m.get(LY) == m.get(LYC) {
            m.set(STAT, m.get(STAT) | 0x4);
            if m.get(STAT) & 0x40 != 0 {
                m.set(IF, m.get(IF) | 0x2);
            }
        } else if m.get(STAT) & 0x4 != 0 {
            m.set(STAT, m.get(STAT) & !0x4);
        }
    }

    fn update_stat(m: MMy, st: State) -> State {
        m.set(STAT, m.get(STAT) & !0x3);
        m.set(
            STAT,
            m.get(STAT)
                | match st {
                    State::Oam => 0x2,
                    State::Draw => 0x3,
                    State::HBlank => 0x0,
                    State::VBlank => 0x1,
                },
        );
        if m.get(STAT)
            & match st {
                State::Oam => 0x10,
                State::HBlank => 0x4,
                State::VBlank => 0x8,
                _ => 0x0,
            }
            != 0
        {
            m.set(IF, m.get(IF) | 0x2);
        }
        st
    }

    pub fn get_bg_pix(m: My, x: usize) -> u32 {
        let (bg_x, bg_y) = (
            (x + m.get(SCX) as usize) % 256,
            ((m.get(LY) + m.get(SCY)) as usize) % 256,
        );
        let (tile_x, tile_y) = (bg_x % 8, bg_y % 8);
        let tile_n = m.get(
            (bg_y / 8 * 32
                + bg_x / 8
                + if m.get(LCDC) & 0x8 == 0 {
                    0x9800
                } else {
                    0x9c00
                }) as u16,
        );
        let tile_b = tile_n as usize * 16
            + if m.get(LCDC) & 0x10 == 0 && tile_n < 128 {
                0x9000
            } else {
                0x8000
            }
            + tile_y * 2;
        let i = ((tile_x as isize - 7) * -1) as usize;
        COLORS[(((m.get(tile_b as u16) >> i) & 0x1)
            | (((m.get(tile_b as u16 + 1) >> i) & 0x1) << 1)) as usize
            + 1]
    }

    pub fn update(&mut self, m: MMy, cy: usize) {
        if cy >= self.cycles {
            let rem = cy - self.cycles;

            match self.state {
                State::Oam => {
                    self.state = Display::update_stat(m, State::Draw);
                    self.cycles = DRAW_T - rem;
                }
                State::Draw => {
                    let y = m.get(LY) as usize * LCD_W;

                    for x in 0..160 {
                        self.buff[y + x] = Display::get_bg_pix(m, x);
                    }
                    self.state = Display::update_stat(m, State::HBlank);
                    self.cycles = H_BLK_T - rem;
                }
                State::HBlank => {
                    if m.get(LY) == 143 {
                        self.state = Display::update_stat(m, State::VBlank);
                        self.cycles = V_BLK_T - rem;
                        m.set(IF, m.get(IF) | 0x1);
                    } else {
                        self.state = Display::update_stat(m, State::Oam);
                        self.cycles = OAM_T - rem;
                    }
                    Display::update_ly(m);
                }
                State::VBlank => {
                    if m.get(LY) == 153 {
                        /*
                        let elap = self.time.elapsed();
                        self.time_v.push(elap.as_millis());
                        if self.time_v.len() == 60 {
                            println!(
                                "{}",
                                1000. / (self.time_v.iter().sum::<u128>() as f32 / 60.)
                            );
                            self.time_v.clear();
                        }
                        self.time = Instant::now();
                        */
                        self.win
                            .update_with_buffer(&self.buff, LCD_W, LCD_H)
                            .unwrap();
                        if !self.win.is_open() {
                            exit(0);
                        }
                        Display::update_ly(m);
                        self.state = Display::update_stat(m, State::Oam);
                        self.cycles = OAM_T - rem;
                    } else {
                        Display::update_ly(m);
                        self.cycles = V_BLK_T - rem;
                    }
                }
            }
        } else {
            self.cycles -= cy;
        }
    }
}
