use crate::mem::*;
use crate::reg::api::*;

pub struct Sprite {
    pos: (isize, isize),
    tile: u16,
    under: bool,
    flip: (bool, bool),
    pal: u8,
}

impl Sprite {
    fn new(m: My, id: usize) -> Sprite {
        let addr: u16 = 0xfe00 | id as u16 * 4;
        let attr = m.su_get(addr + 3) & 0xf0;

        Sprite {
            pos: (
                m.su_get(addr + 1) as isize - 8,
                m.su_get(addr) as isize - 16,
            ),
            tile: 0x8000
                | (m.su_get(addr + 2)
                    & if m.su_get(LCDC) & 0x4 != 0 {
                        0xfe
                    } else {
                        0xff
                    }) as u16
                    * 16,
            under: attr & 0x80 != 0,
            flip: (attr & 0x20 != 0, attr & 0x40 != 0),
            pal: m.su_get(if attr & 0x10 != 0 { OBP1 } else { OBP0 }),
        }
    }

    pub fn get_pix(&self, m: My, x: usize) -> Option<(u8, u8, bool)> {
        if m.su_get(LCDC) & 0x2 == 0 || !(self.pos.0..(self.pos.0 + 8)).contains(&(x as isize)) {
            return None;
        }

        let (spr_x, spr_y) = (
            (x as isize - self.pos.0) as usize,
            (m.su_get(LY) as isize - self.pos.1) as usize,
        );
        let lst_l = if m.su_get(LCDC) & 0x4 != 0 { 15 } else { 7 };
        let byte = self.tile
            + (if self.flip.1 {
                ((spr_y as isize - lst_l) * -1) as usize
            } else {
                spr_y
            } * 2) as u16;
        let i = if self.flip.0 {
            spr_x
        } else {
            ((spr_x as isize - 7) * -1) as usize
        };
        let bit1 = (m.su_get(byte as u16) >> i) & 0x1;
        let bit2 = ((m.su_get(byte as u16 + 1) >> i) & 0x1) << 1;
        let result = bit1 | bit2;
        match result {
            0 => None,
            _ => Some((bit1 | bit2, self.pal, self.under)),
        }
    }

    pub fn update(sprites: &mut Vec<Sprite>, m: My) {
        let mut spr: Sprite;
        let spr_h = if m.su_get(LCDC) & 0x4 != 0 { 16 } else { 8 };

        sprites.clear();
        for i in 0..40 {
            spr = Sprite::new(m, i);
            if m.su_get(LY) as isize >= spr.pos.1 && (m.su_get(LY) as isize) < spr.pos.1 + spr_h {
                sprites.push(spr);
            }
            if sprites.len() == 10 {
                break;
            }
        }
        sprites.sort_by(|a, b| a.pos.0.partial_cmp(&b.pos.0).unwrap());
    }
}
