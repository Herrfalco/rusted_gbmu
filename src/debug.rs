use crate::mem::*;
use crate::ops::ops::*;
use crate::reg::{api::*, *};
use crate::utils::*;
use lazy_regex::regex;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use regex::{Captures, Regex};
use std::process::exit;
use text_io::read;

/*
fn main() {
    let entry = "g (0x0245)";

    if let Some(p) = g_r.captures(entry) {
    } else if let Some(p) = g_rr.captures(entry) {
    } else if let Some(p) = g_arr.captures(entry) {
    } else if let Some(p) = g_ann.captures(entry) {
    } else if let Some(p) = s_rr_nn.captures(entry) {
    } else if let Some(p) = s_r_n.captures(entry) {
    } else if let Some(p) = s_arr_n.captures(entry) {
    } else if let Some(p) = s_ann_n.captures(entry) {
    } else if let Some(_) = b_lst.captures(entry) {
    } else if let Some(p) = b_add.captures(entry) {
    } else if let Some(p) = b_del.captures(entry) {
    } else if let Some(_) = b_nxt.captures(entry) {
    } else if let Some(_) = r_shw.captures(entry) {
    } else if let Some(_) = r_a_shw.captures(entry) {
    } else if let Some(_) = exit.captures(entry) {
    }
}
*/

#[derive(PartialEq, FromPrimitive)]
enum Cmd {
    NI,
    GR,
    GRr,
    GArr,
    GAnn,
    SRrNn,
    SRN,
    SArrN,
    SAnnN,
    BLst,
    BAdd,
    BDel,
    BNxt,
    RShw,
    RAShw,
    Exit,
    Unknown,
}

pub struct Debugger {
    rgxs: Vec<&'static Regex>,
    debug: bool,
    brks: Vec<u16>,
    sbys: bool,
}

impl<'a> Debugger {
    pub fn new(debug: bool) -> Debugger {
        Debugger {
            rgxs: vec![
                regex!(r#"^$"#i),
                regex!(r#"^g ([afbcdehl])$"#i),
                regex!(r#"^g ((?:af)|(?:bc)|(?:de)|(?:hl)|(?:pc)|(?:sp))$"#i),
                regex!(r#"^g \(((?:af)|(?:bc)|(?:de)|(?:hl)|(?:pc)|(?:sp))\)$"#i),
                regex!(r#"^g \(0x([[:xdigit:]]{1,4})\)$"#i),
                regex!(r#"^s ((?:af)|(?:bc)|(?:de)|(?:hl)|(?:pc)|(?:sp)) 0x([[:xdigit:]]{1,4})$"#i),
                regex!(r#"^s ([afbcdehl]) 0x([[:xdigit:]]{1,2})$"#i),
                regex!(r#"^s \(((?:af)|(?:bc)|(?:de)|(?:hl)|(?:pc)|(?:sp))\) 0x([[:xdigit:]]{1,2})$"#i),
                regex!(r#"^s \(0x([[:xdigit:]]{1,4})\) 0x([[:xdigit:]]{1,2})$"#i),
                regex!(r#"^b$"#i),
                regex!(r#"^b add 0x([[:xdigit:]]{1,4})$"#i),
                regex!(r#"^b del ([[:digit:]]+)$"#i),
                regex!(r#"^n$"#i),
                regex!(r#"^r$"#i),
                regex!(r#"^ra$"#i),
                regex!(r#"^exit$"#i),
            ],
            debug,
            brks: Vec::new(),
            sbys: true,
        }
    }

    fn parse_cmd(&self, s: &'a str) -> Option<(Cmd, Vec<String>)> {
        for (i, r) in self.rgxs.iter().enumerate() {
            if let Some(c) = r.captures(s) {
                return Some((
                    FromPrimitive::from_usize(i).unwrap(),
                    c.iter()
                        .skip(1)
                        .map(|e| e.unwrap().as_str().to_lowercase())
                        .collect::<Vec<String>>(),
                ));
            }
        }
        None
    }

    fn rr_by_nm(r: &'a mut Regs, nm: &str) -> Option<MRR<'a>> {
        match nm {
            "af" => Some(&mut r.af),
            "bc" => Some(&mut r.bc),
            "de" => Some(&mut r.de),
            "hl" => Some(&mut r.hl),
            "sp" => Some(&mut r.sp),
            "pc" => Some(&mut r.pc),
            _ => None,
        }
    }

    fn r_by_nm(r: &'a mut Regs, nm: &str) -> Option<MR<'a>> {
        match nm {
            "a" => Some((&mut r.af, U)),
            "f" => Some((&mut r.af, D)),
            "b" => Some((&mut r.bc, U)),
            "c" => Some((&mut r.bc, D)),
            "d" => Some((&mut r.de, U)),
            "e" => Some((&mut r.de, D)),
            "h" => Some((&mut r.hl, U)),
            "l" => Some((&mut r.hl, D)),
            _ => None,
        }
    }

    fn get_cmd(&mut self, m: &mut Mem, r: &mut Regs) {
        let mut entry: String;

        loop {
            pflush("> ");
            entry = read!("{}\n");
            if let Some((cmd, par)) = self.parse_cmd(&entry[..]) {
                match cmd {
                    Cmd::NI => break,
                    Cmd::GR => {
                        let tmp = Debugger::r_by_nm(r, &par[0]).unwrap();
                        println!("0x{:02x}", gr((tmp.0, tmp.1)));
                    }
                    Cmd::GRr => {
                        let tmp = Debugger::rr_by_nm(r, &par[0]).unwrap();
                        println!("0x{:04x}", grr(tmp));
                    }
                    Cmd::GArr => {
                        let tmp = Debugger::rr_by_nm(r, &par[0]).unwrap();
                        println!("0x{:02x}", m.get(grr(tmp)));
                    }
                    Cmd::GAnn => {
                        let tmp = u16::from_str_radix(&par[0], 16).unwrap();
                        println!("0x{:02x}", m.get(tmp));
                    }
                    Cmd::SRrNn => {
                        let tmp1 = Debugger::rr_by_nm(r, &par[0]).unwrap();
                        let tmp2 = u16::from_str_radix(&par[1], 16).unwrap();
                        srr(tmp1, tmp2);
                    }
                    Cmd::SRN => {
                        let tmp1 = Debugger::r_by_nm(r, &par[0]).unwrap();
                        let tmp2 = u8::from_str_radix(&par[1], 16).unwrap();
                        sr(tmp1, tmp2);
                    }
                    Cmd::SArrN => {
                        let tmp1 = Debugger::rr_by_nm(r, &par[0]).unwrap();
                        let tmp2 = u8::from_str_radix(&par[1], 16).unwrap();
                        m.set(grr(tmp1), tmp2);
                    }
                    Cmd::SAnnN => {
                        let tmp1 = u16::from_str_radix(&par[0], 16).unwrap();
                        let tmp2 = u8::from_str_radix(&par[1], 16).unwrap();
                        m.set(tmp1, tmp2);
                    }
                    Cmd::BLst => {
                        for (idx, brk) in self.brks.iter().enumerate() {
                            println!("{}: 0x{:04x}", idx, brk);
                        }
                    }
                    Cmd::BAdd => {
                        let tmp = u16::from_str_radix(&par[0], 16).unwrap();
                        self.brks.push(tmp);
                    }
                    Cmd::BDel => {
                        let tmp = usize::from_str_radix(&par[0], 10).unwrap();
                        if tmp >= self.brks.len() {
                            println!("Error: Wrong breakpoint ID");
                        } else {
                            self.brks.remove(tmp);
                        }
                    }
                    Cmd::BNxt => {
                        self.sbys = false;
                        break;
                    }
                    Cmd::RShw => {
                        println!("{}", r);
                    }
                    Cmd::RAShw => println!("Error: Not implemented yet..."),
                    Cmd::Exit => {
                        exit(0);
                    }
                    _ => println!("Error: Unknown command"),
                }
            } else {
                println!("Error: Unknown command");
            }
        }
    }

    pub fn run(&mut self, m: &mut Mem, r: &mut Regs, op: &Op, p: u16) {
        if self.debug {
            let fm_par: String;

            fm_par = match op.len() {
                1 => String::from(""),
                2 => format!("0x{:02x}", p),
                3 => format!("0x{:04x}", p),
                _ => fatal_err("Wrong operation length", 4),
            };
            println!(
                "0x{:04x}:  {}",
                grr(&r.pc),
                format!("{}", op).replace("#", &fm_par)
            );
            if self.sbys || self.brks.contains(&grr(&r.pc)) {
                self.sbys = true;
                self.get_cmd(m, r);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex() {
        let dbg = Debugger::new(true);
        let ents = vec![
            "",
            "   ",
            "\t",
            " g a",
            "g a",
            "g ",
            "ga",
            "g f",
            "g e",
            "g s",
            "g sp",
            "g sp ",
            "g pc",
            "h pc",
            "g (sp)",
            "g (af)",
            "g (ac)",
            "g (a)",
            "g (ff)",
            "g (0xff)",
            "g 0x(ff)",
            "g 0x()",
            "g (0x)",
            "g (0xdead)",
            "g (0xdead0)",
            "s (0xdead)",
            "s af 0xdead",
            "s af 0xdaaad",
            "s bc 0xd",
            "s de 0x",
            "s d 0x",
            "s d 0x00",
            "s e 0x0f",
            "s s 0x0f",
            "s e 0x0fa",
            "s e 0x0fdead",
            "s (sp) 0x00",
            "s (af) 0xfffff",
            "s (ac) 0x10",
            "s (0xff) 0x10",
            "s (0xfffff) 0x10",
            "s (0x) 0x10",
            "s (0xff) 0xfff",
            "s (0xff) 0x",
            "b",
            " b",
            "b add sp",
            "b add 0xff",
            "b add 0xfffff",
            "b add 0x",
            "b del 10",
            "b del 0",
            "b del f",
            "n",
            "n ",
            "r",
            "r ",
            "ra",
            " ra",
            "exit",
            " exit",
            "q",
        ];
        let res = vec![
            (true, Cmd::NI, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::GR, vec!["a"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::GR, vec!["f"]),
            (true, Cmd::GR, vec!["e"]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::GRr, vec!["sp"]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::GRr, vec!["pc"]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::GArr, vec!["sp"]),
            (true, Cmd::GArr, vec!["af"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::GAnn, vec!["ff"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::GAnn, vec!["dead"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::SRrNn, vec!["af", "dead"]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::SRrNn, vec!["bc", "d"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::SRN, vec!["d", "00"]),
            (true, Cmd::SRN, vec!["e", "0f"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::SArrN, vec!["sp", "00"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::SAnnN, vec!["ff", "10"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::BLst, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::BAdd, vec!["ff"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::BDel, vec!["10"]),
            (true, Cmd::BDel, vec!["0"]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::BNxt, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::RShw, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::RAShw, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::Exit, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
        ];
        for (idx, entry) in ents.iter().enumerate() {
            if let Some((cmd, par)) = dbg.parse_cmd(&entry[..]) {
                if !res[idx].0 || res[idx].1 != cmd {
                    panic!();
                }
                for (i, p) in par.iter().enumerate() {
                    assert_eq!(&res[idx].2[i], p);
                }
            } else if res[idx].0 {
                panic!("nb:{}", idx);
            }
        }
    }
}
