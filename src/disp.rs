use crate::mem::*;
use crate::reg::api::*;

enum State {
    Oam,
    Draw,
    HBlank,
    VBlank,
}

pub struct Display {
    cycles: usize,
    state: State,
}

impl Display {
    pub fn new() -> Display {
        Display {
            cycles: 20,
            state: State::Oam,
        }
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

    pub fn update(&mut self, m: MMy) {
        self.cycles -= 1;
        if self.cycles == 0 {
            match self.state {
                State::Oam => {
                    self.state = Display::update_stat(m, State::Draw);
                    self.cycles = 43;
                }
                State::Draw => {
                    self.state = Display::update_stat(m, State::HBlank);
                    self.cycles = 51;
                }
                State::HBlank => {
                    if m.get(LY) == 143 {
                        self.state = Display::update_stat(m, State::VBlank);
                        self.cycles = 114;
                        m.set(IF, m.get(IF) | 0x1);
                    } else {
                        self.state = Display::update_stat(m, State::Oam);
                        self.cycles = 20;
                    }
                    Display::update_ly(m);
                }
                State::VBlank => {
                    if m.get(LY) == 153 {
                        Display::update_ly(m);
                        self.state = Display::update_stat(m, State::Oam);
                        self.cycles = 20;
                    } else {
                        Display::update_ly(m);
                        self.cycles = 114;
                    }
                }
            }
        }
    }
}
