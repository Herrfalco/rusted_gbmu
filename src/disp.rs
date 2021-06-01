use crate::input::*;
use crate::mem::*;
use crate::reg::api::*;
use crate::sprite::*;
use crate::utils::*;
use minifb::{Window, WindowOptions};

const LCD_W: usize = 160;
const LCD_H: usize = 144;

const ZOOM: usize = 6;

const OAM_T: usize = 80;
const DRAW_T: usize = 172;
const H_BLK_T: usize = 204;
const V_BLK_T: usize = 456;

const OFF_T: usize = 70224;

enum State {
    Oam,
    Draw,
    HBlank,
    VBlank,
}

/*
use std::time::Instant;
*/

pub struct Display {
    cycles: usize,
    state: State,
    buff: Vec<u32>,
    win: Window,
    sprites: Vec<Sprite>,
    off_cy: usize,
    win_y: usize,
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
                LCD_W * ZOOM,
                LCD_H * ZOOM,
                WindowOptions::default(),
            )
            .unwrap_or_else(|_| fatal_err("Can't open game window", 10)),
            sprites: Vec::new(),
            off_cy: OFF_T,
            win_y: 0,
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

    fn update_ly(&mut self, m: MMy, res_win_y: bool) {
        let lcdc = m.su_get(LCDC);
        if res_win_y {
            self.win_y = 0;
        } else if lcdc & 0x1 != 0 && lcdc & 0x20 != 0 {
            self.win_y += 1;
        }
        m.su_set(LY, (m.su_get(LY) + 1) % 154);
        if m.su_get(LY) == m.su_get(LYC) {
            m.su_set(STAT, m.su_get(STAT) | 0x4);
            if m.su_get(STAT) & 0x40 != 0 {
                m.su_set(IF, m.su_get(IF) | 0x2);
            }
        } else if m.su_get(STAT) & 0x4 != 0 {
            m.su_set(STAT, m.su_get(STAT) & !0x4);
        }
    }

    fn update_stat(m: MMy, st: State) -> State {
        m.su_set(STAT, m.su_get(STAT) & !0x3);
        m.su_set(
            STAT,
            m.su_get(STAT)
                | match st {
                    State::Oam => 0x2,
                    State::Draw => 0x3,
                    State::HBlank => 0x0,
                    State::VBlank => 0x1,
                },
        );
        if m.su_get(STAT)
            & match st {
                State::Oam => 0x10,
                State::HBlank => 0x4,
                State::VBlank => 0x8,
                _ => 0x0,
            }
            != 0
        {
            m.su_set(IF, m.su_get(IF) | 0x2);
        }
        st
    }

    fn get_win_pix(&self, m: My, x: usize) -> Option<u8> {
        let lcdc = m.su_get(LCDC);
        let (pos_x, pos_y) = (m.su_get(WX).wrapping_sub(7), m.su_get(WY));
        if lcdc & 0x1 == 0 || lcdc & 0x20 == 0 || x < pos_x as usize || m.su_get(LY) < pos_y {
            return None;
        }
        let (win_x, win_y) = (x - pos_x as usize, self.win_y);

        Some(self.get_pix(m, (win_x, win_y), false))
    }

    fn get_bg_pix(&self, m: My, x: usize) -> Option<u8> {
        if m.su_get(LCDC) & 0x1 == 0 {
            return None;
        }
        let (bg_x, bg_y) = (
            (x as u8).wrapping_add(m.su_get(SCX)) as usize,
            m.su_get(LY).wrapping_add(m.su_get(SCY)) as usize,
        );

        Some(self.get_pix(m, (bg_x, bg_y), true))
    }

    fn get_pix(&self, m: My, (x, y): (usize, usize), bg: bool) -> u8 {
        let (tile_x, tile_y) = (x % 8, y % 8);
        let tile_n = m.su_get(
            (y / 8 * 32
                + x / 8
                + if m.su_get(LCDC) & if bg { 0x8 } else { 0x40 } == 0 {
                    0x9800
                } else {
                    0x9c00
                }) as u16,
        );
        let tile_b = tile_n as usize * 16
            + if m.su_get(LCDC) & 0x10 == 0 && tile_n < 128 {
                0x9000
            } else {
                0x8000
            }
            + tile_y * 2;
        let i = ((tile_x as isize - 7) * -1) as usize;
        let bit1 = (m.su_get(tile_b as u16) >> i) & 0x1;
        let bit2 = ((m.su_get(tile_b as u16 + 1) >> i) & 0x1) << 1;
        bit1 | bit2
    }

    fn pix_mix(&self, m: My, x: usize) -> u32 {
        let mut s_pix = None;
        let win_pix = self.get_win_pix(m, x);
        let bg_pix = self.get_bg_pix(m, x);
        let mut result = 0;
        let mut pal = m.su_get(BGP);

        for s in &self.sprites {
            if let Some(px) = s.get_pix(m, x) {
                s_pix = Some(px);
                break;
            }
        }

        if let Some(bp) = bg_pix {
            result = bp;
        }
        if let Some(wp) = win_pix {
            result = wp;
        }
        if let Some((sp, spal, udr)) = s_pix {
            if sp != 0 && !(udr && result > 0) {
                result = sp;
                pal = spal;
            }
        }
        COLORS[((pal >> result * 2) & 0x3) as usize + 1]
    }

    pub fn update(&mut self, m: MMy, cy: usize) {
        if m.su_get(LCDC) & 0x80 == 0 {
            self.lcd_off(m, cy);
            return;
        }
        if cy >= self.cycles {
            let rem = cy - self.cycles;

            match self.state {
                State::Oam => {
                    Sprite::update(&mut self.sprites, m);
                    self.state = Display::update_stat(m, State::Draw);
                    self.cycles = DRAW_T - rem;
                }
                State::Draw => {
                    for x in 0..160 {
                        self.buff[m.su_get(LY) as usize * LCD_W + x] = self.pix_mix(m, x)
                    }
                    self.state = Display::update_stat(m, State::HBlank);
                    self.cycles = H_BLK_T - rem;
                }
                State::HBlank => {
                    if m.su_get(LY) == 143 {
                        self.state = Display::update_stat(m, State::VBlank);
                        self.cycles = V_BLK_T - rem;
                        m.su_set(IF, m.su_get(IF) | 0x1);
                    } else {
                        self.state = Display::update_stat(m, State::Oam);
                        self.cycles = OAM_T - rem;
                    }
                    self.update_ly(m, false);
                }
                State::VBlank => {
                    if m.su_get(LY) == 153 {
                        /*
                        let elap = self.time.elapsed();
                        self.time_v.push(elap.as_millis());
                        if self.time_v.len() == 100 {
                            println!(
                                "{}",
                                1000. / (self.time_v.iter().sum::<u128>() as f32 / 100.)
                            );
                            self.time_v.clear();
                        }
                        self.time = Instant::now();
                        */
                        self.win
                            .update_with_buffer(&self.buff, LCD_W, LCD_H)
                            .unwrap();
                        if !self.win.is_open() {
                            quit::with_code(0);
                        }
                        Inputs::up_keys(m, &self.win);
                        self.update_ly(m, true);
                        self.state = Display::update_stat(m, State::Oam);
                        self.cycles = OAM_T - rem;
                    } else {
                        self.update_ly(m, false);
                        self.cycles = V_BLK_T - rem;
                    }
                }
            }
        } else {
            self.cycles -= cy;
        }
    }

    fn lcd_off(&mut self, m: MMy, cy: usize) {
        if cy >= self.off_cy {
            self.win
                .update_with_buffer(&vec![COLORS[0]; LCD_W * LCD_H], LCD_W, LCD_H)
                .unwrap();
            if !self.win.is_open() {
                quit::with_code(0);
            }
            Inputs::up_keys(m, &self.win);
            self.off_cy = OFF_T - (cy - self.off_cy);
        } else {
            self.off_cy -= cy;
        }
    }
}
