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

/*
use std::time::Instant;
*/

struct Sprite {
    id: usize,
    pos: (isize, isize),
    tile: u16,
    under: bool,
    flip: (bool, bool),
    pal: u16,
}

impl Sprite {
    fn new(m: My, id: usize) -> Sprite {
        let addr: u16 = 0xfe00 + id as u16 * 4;
        let attr = m.get(addr + 3) & 0xf0;

        Sprite {
            id,
            pos: (m.get(addr + 1) as isize - 8, m.get(addr) as isize - 16),
            tile: 0x8000 + m.get(addr + 2) as u16 * 16,
            under: attr & 0x80 != 0,
            flip: (attr & 0x20 != 0, attr & 0x40 != 0),
            pal: if attr & 0x10 != 0 { OBP1 } else { OBP0 },
        }
    }
}

pub struct Display {
    cycles: usize,
    state: State,
    buff: Vec<u32>,
    win: Window,
    sprites: Vec<Sprite>,
    /*
    time: Instant,
    time_v: Vec<u128>,
    */
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
            sprites: Vec::new(),
            /*
            time: Instant::now(),
            time_v: Vec::new(),
            */
        };
        result
            .win
            .limit_update_rate(Some(std::time::Duration::from_millis(17)));
        result
            .win
            .update_with_buffer(&result.buff, LCD_W, LCD_H)
            .unwrap();
        result
    }

    pub fn reset(&mut self) {
        self.cycles = 80;
        self.state = State::Oam;
        self.buff = vec![COLORS[0]; LCD_W * LCD_H];
        self.sprites = Vec::new();
        self.win
            .update_with_buffer(&self.buff, LCD_W, LCD_H)
            .unwrap();
    }

    fn update_spr(&mut self, m: My) {
        let mut spr: Sprite;
        let spr_h = if m.get(LCDC) & 0x4 != 0 { 16 } else { 8 };

        self.sprites.clear();
        for i in 0..40 {
            spr = Sprite::new(m, i);
            if m.get(LY) as isize >= spr.pos.1 && (m.get(LY) as isize) < spr.pos.1 + spr_h {
                self.sprites.push(spr);
            }
            if self.sprites.len() == 10 {
                break;
            }
        }
        self.sprites
            .sort_by(|a, b| a.pos.0.partial_cmp(&b.pos.0).unwrap());
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

    pub fn get_bg_pix(m: My, x: usize) -> u8 {
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
        let bit1 = (m.get(tile_b as u16) >> i) & 0x1;
        let bit2 = ((m.get(tile_b as u16 + 1) >> i) & 0x1) << 1;
        bit1 | bit2
    }

    pub fn update(&mut self, m: MMy, cy: usize) {
        if cy >= self.cycles {
            let rem = cy - self.cycles;

            match self.state {
                State::Oam => {
                    self.update_spr(m);
                    self.state = Display::update_stat(m, State::Draw);
                    self.cycles = DRAW_T - rem;
                }
                State::Draw => {
                    let y = m.get(LY) as usize * LCD_W;

                    /*
                    if self.sprites.len() != 0 {
                        for s in &self.sprites {
                            if s.pos.0 > 0 {
                                println!("{}", s.id);
                            }
                        }
                    }
                    */
                    for x in 0..160 {
                        self.buff[y + x] = COLORS[Display::get_bg_pix(m, x) as usize + 1];
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
