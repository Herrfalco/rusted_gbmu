use crate::mem::*;
use crate::reg::api::*;

const DIV_T: usize = 256;

pub struct Timer {
    div_cy: usize,
    tima_cy: usize,
    tima_cy_sav: usize,
    tac_sav: u8,
}

impl<'a> Timer {
    pub fn new(m: MMy) -> Timer {
        let mut result = Timer {
            div_cy: DIV_T,
            tima_cy: 0xff,
            tima_cy_sav: 0xff,
            tac_sav: 0xff,
        };

        result.new_tima_cy(m);
        result
    }

    fn new_tima_cy(&mut self, m: My) {
        let tmp = m.su_get(TAC) & 0x3;

        if self.tac_sav != tmp {
            self.tac_sav = tmp;
            self.tima_cy_sav = match m.su_get(TAC) & 0x3 {
                0 => 1024,
                1 => 16,
                2 => 64,
                3 => 256,
                _ => 0,
            };
            self.tima_cy = self.tima_cy_sav;
        }
    }

    pub fn update(&mut self, m: MMy, cy: usize) {
        if cy >= self.div_cy {
            self.div_cy = DIV_T - (cy - self.div_cy);
            m.su_set(DIV, m.su_get(DIV).wrapping_add(1));
        } else {
            self.div_cy -= cy;
        }
        self.new_tima_cy(m);
        if m.su_get(TAC) & 0x4 != 0 {
            if cy >= self.tima_cy {
                let mut tmp = m.su_get(TIMA).wrapping_add(1);

                self.tima_cy = self.tima_cy_sav - ((cy - self.tima_cy) % self.tima_cy_sav);
                if tmp == 0 {
                    tmp = m.su_get(TMA);
                    m.su_set(IF, m.su_get(IF) | 0x4);
                }
                m.su_set(TIMA, tmp);
            } else {
                self.tima_cy -= cy;
            }
        }
    }
}
