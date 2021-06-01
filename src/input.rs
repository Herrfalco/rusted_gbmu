use crate::mem::*;
use crate::reg::api::*;
use minifb::{Key, Window};

pub struct Inputs {
    pub keys: Option<Vec<Key>>,
    acts: Vec<Key>,
    dirs: Vec<Key>,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            keys: None,
            acts: vec![Key::D, Key::F, Key::A, Key::S],
            dirs: vec![Key::L, Key::H, Key::K, Key::J],
        }
    }

    pub fn up_keys(m: MMy, win: &Window) {
        let new_keys = win.get_keys();

        if let Some(n_keys) = &new_keys {
            if n_keys.len() != 0 {
                if let Some(o_keys) = m.inputs.keys.take() {
                    for k in n_keys {
                        if !o_keys.contains(k) {
                            m.su_set(IF, m.su_get(IF) & 0x10);
                        }
                    }
                } else {
                    m.su_set(IF, m.su_get(IF) & 0x10);
                }
            }
        }
        m.inputs.keys = new_keys;
    }

    pub fn get_p1(m: My) -> u8 {
        let saved_p1 = m.su_get(P1);
        let mut result = (saved_p1 & 0x30) | !0x30;

        if let Some(keys) = &m.inputs.keys {
            if keys.len() != 0 {
                match result & 0x30 {
                    0x10 => {
                        for k in keys {
                            if m.inputs.acts.contains(k) {
                                result &= match k {
                                    Key::D => !0x1,
                                    Key::F => !0x2,
                                    Key::A => !0x4,
                                    Key::S => !0x8,
                                    _ => !0x0,
                                }
                            }
                        }
                    }
                    0x20 => {
                        for k in keys {
                            if m.inputs.dirs.contains(k) {
                                result &= match k {
                                    Key::L => !0x1,
                                    Key::H => !0x2,
                                    Key::K => !0x4,
                                    Key::J => !0x8,
                                    _ => !0x0,
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        result
    }
}
