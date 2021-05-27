use crate::mem::*;
use crate::ops::ops::*;
use crate::reg::{api::*, *};
use crate::utils::*;
use lazy_regex::regex;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use regex::Regex;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::process::exit;
use std::str::FromStr;

#[derive(PartialEq, FromPrimitive)]
enum Cmd {
    NI,
    Res,
    PMRr,
    PMNn,
    SFb,
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
    edit: Editor<()>,
    debug: bool,
    brks: Vec<u16>,
    sbys: bool,
}

impl<'a> Debugger {
    pub fn new(debug: bool) -> Debugger {
        Debugger {
            rgxs: vec![
                regex!(r#"^$"#i),
                regex!(r#"^!$"#i),
                regex!(r#"^([[:digit:]]*)\s*\(((?:af)|(?:bc)|(?:de)|(?:hl)|(?:pc)|(?:sp))\)$"#i),
                regex!(r#"^([[:digit:]]*)\s*\(0x([[:xdigit:]]{1,4})\)$"#i),
                regex!(r#"^([znh]|(?:cy))\s*=\s*(true|false)$"#i),
                regex!(r#"^((?:af)|(?:bc)|(?:de)|(?:hl)|(?:pc)|(?:sp))\s*=\s*0x([[:xdigit:]]{1,4})$"#i),
                regex!(r#"^([afbcdehl])\s*=\s*0x([[:xdigit:]]{1,2})$"#i),
                regex!(r#"^\(((?:af)|(?:bc)|(?:de)|(?:hl)|(?:pc)|(?:sp))\)\s*=\s*0x([[:xdigit:]]{1,2})$"#i),
                regex!(r#"^\(0x([[:xdigit:]]{1,4})\)\s*=\s*0x([[:xdigit:]]{1,2})$"#i),
                regex!(r#"^b$"#i),
                regex!(r#"^b 0x([[:xdigit:]]{1,4})$"#i),
                regex!(r#"^b del ([[:digit:]]+)$"#i),
                regex!(r#"^n$"#i),
                regex!(r#"^r$"#i),
                regex!(r#"^ra$"#i),
                regex!(r#"^exit$"#i),
            ],
            edit: Editor::new(),
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

    fn f_by_nm(nm: &str) -> Option<u16> {
        match nm {
            "z" => Some(Z),
            "n" => Some(N),
            "h" => Some(H),
            "cy" => Some(CY),
            _ => None,
        }
    }

    fn mem_dump(m: &Mem, addr: u16, len: usize) {
        println!("-------------------------------------------------------");
        for i in 0..len as u16 {
            if i % 16 == 0 {
                print!("{}0x{:04x}: ", if i != 0 { "\n" } else { "" }, i + addr);
            }
            print!("{:02x} ", m.get(i + addr));
        }
        println!("\n-------------------------------------------------------");
    }

    fn get_cmd(&mut self, m: &mut Mem, r: &mut Regs) -> bool {
        let mut line: Result<String, ReadlineError>;
        let mut entry: String;

        loop {
            line = self.edit.readline("> ");
            entry = match line {
                Ok(s) => s,
                _ => continue,
            };
            if entry.len() != 0 {
                self.edit.add_history_entry(&entry);
            }
            if let Some((cmd, par)) = self.parse_cmd(&entry[..]) {
                match cmd {
                    Cmd::NI => break,
                    Cmd::Res => return false,
                    Cmd::SFb => {
                        let tmp = Debugger::f_by_nm(&par[0]).unwrap();
                        sf((&mut r.af, tmp), bool::from_str(&par[1]).unwrap());
                    }
                    Cmd::PMRr => Debugger::mem_dump(
                        m,
                        grr(Debugger::rr_by_nm(r, &par[1]).unwrap()),
                        if par[0].len() == 0 {
                            1
                        } else {
                            usize::from_str_radix(&par[0], 10).unwrap()
                        },
                    ),
                    Cmd::PMNn => Debugger::mem_dump(
                        m,
                        u16::from_str_radix(&par[1], 16).unwrap(),
                        if par[0].len() == 0 {
                            1
                        } else {
                            usize::from_str_radix(&par[0], 10).unwrap()
                        },
                    ),
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
                        println!("-------------------------------------------------------");
                        if self.brks.len() == 0 {
                            println!("None");
                        }
                        for (idx, brk) in self.brks.iter().enumerate() {
                            println!("{}: 0x{:04x}", idx, brk);
                        }
                        println!("-------------------------------------------------------");
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
        true
    }

    pub fn run(&mut self, m: &mut Mem, r: &mut Regs, op: &Op, p: u16) -> bool {
        if self.debug {
            if self.sbys || self.brks.contains(&grr(&r.pc)) {
                self.sbys = true;

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
                return self.get_cmd(m, r);
            }
        }
        true
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
            "!",
            "   ",
            "\t",
            "100 (0xff)",
            "(0xff)",
            "100 (0xfffff)",
            "af (0xfffff)",
            "(0xffffff)",
            "100 (af)",
            "(hl)",
            "100 (a)",
            "af (rn)",
            "(rn)",
            "Z =false",
            "cy= true",
            "h=true",
            "r = true",
            "n=tru",
            "=(0xdead)",
            "af =0xdead",
            "af= 0xdaaad",
            "bc = 0xd",
            "de=  0x",
            "d =0x",
            "d=   0x00",
            "e =0x0f",
            "s =0x0f",
            "e = 0x0fa",
            "e = 0x0fdead",
            "(sp)=0x00",
            "(af) =0xfffff",
            "(ac) = 0x10",
            "(0xff)=  0x10",
            "(0xfffff) =0x10",
            "(0x)=0x10",
            "(0xff) =0xfff",
            "(0xff) =0x",
            "b",
            " b",
            "b sp",
            "b 0xff",
            "b 0xfffff",
            "b 0x",
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
            (true, Cmd::Res, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::PMNn, vec!["100", "ff"]),
            (true, Cmd::PMNn, vec!["", "ff"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::PMRr, vec!["100", "af"]),
            (true, Cmd::PMRr, vec!["", "hl"]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (false, Cmd::Unknown, vec![]),
            (true, Cmd::SFb, vec!["z", "false"]),
            (true, Cmd::SFb, vec!["cy", "true"]),
            (true, Cmd::SFb, vec!["h", "true"]),
            (false, Cmd::Unknown, vec![]),
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
                    panic!("nb:{}", idx);
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
