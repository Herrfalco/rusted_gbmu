use crate::mem::*;
use crate::reg::api::*;

pub struct Timer {
    div_cy: usize,
    tima_cy: usize,
    tima_cy_sav: usize,
    tac_sav: u8,
}

impl Timer {
    pub fn new(m: MMy) -> Timer {
        let mut result = Timer {
            div_cy: 64,
            tima_cy: 0xff,
            tima_cy_sav: 0xff,
            tac_sav: 0xff,
        };

        result.new_tima_cy(m);
        result
    }

    fn new_tima_cy(&mut self, m: My) {
        let tmp = m.get(TAC) & 0x3;

        if self.tac_sav != tmp {
            self.tac_sav = tmp;
            self.tima_cy_sav = match m.get(TAC) & 0x3 {
                0 => 256,
                1 => 4,
                2 => 16,
                3 => 64,
                _ => 0,
            };
            self.tima_cy = self.tima_cy_sav;
        }
    }

    pub fn update(&mut self, m: MMy) {
        self.div_cy -= 1;
        if self.div_cy == 0 {
            self.div_cy = 64;
            m.set(DIV, m.get(DIV).wrapping_add(1));
        }
        self.new_tima_cy(m);
        if m.get(TAC) & 0x4 != 0 {
            self.tima_cy -= 1;
            if self.tima_cy == 0 {
                let mut tmp = m.get(TIMA).wrapping_add(1);

                self.tima_cy = self.tima_cy_sav;
                if tmp == 0 {
                    tmp = m.get(TMA);
                    m.set(IF, m.get(IF) | 0x4);
                }
                m.set(TIMA, tmp);
            }
        }
    }
}
