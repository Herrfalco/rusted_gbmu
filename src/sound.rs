use crate::mem::*;
use crate::utils::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const SAMPLE_RATE: u32 = 44100;
const OSC_T: usize = 32;
const SND_DIV: f32 = 6.;
const FILT_SZ: usize = 2;

pub struct Audio {
    stream: cpal::Stream,
    sample_rate: u32,
    oscs: Arc<Mutex<Oscillators>>,
}

impl Audio {
    pub fn new() -> Audio {
        let device = cpal::default_host()
            .default_output_device()
            .unwrap_or_else(|| fatal_err("Can't find output device", 23));
        let wanted_samplerate = cpal::SampleRate(SAMPLE_RATE);

        let mut supported_config = None;
        for f in device
            .supported_output_configs()
            .unwrap_or_else(|_| fatal_err("Can't find any output configuration", 24))
        {
            if f.channels() == 2
                && f.sample_format() == cpal::SampleFormat::F32
                && f.min_sample_rate() <= wanted_samplerate
                && wanted_samplerate <= f.max_sample_rate()
            {
                supported_config = Some(f.with_sample_rate(wanted_samplerate));
                break;
            }
        }

        let config: cpal::StreamConfig = supported_config
            .unwrap_or_else(|| fatal_err("Can't find suitable output configuration", 25))
            .into();
        let err_fn = |_| fatal_err("An error occurred on the output audio stream", 26);
        let oscs = Arc::new(Mutex::new(Oscillators::new()));
        let oscs_cln = oscs.clone();
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| stream_thrd(data, &oscs),
                err_fn,
            )
            .unwrap_or_else(|_| fatal_err("Can't create output stream", 27));
        stream
            .play()
            .unwrap_or_else(|_| fatal_err("Can't run output stream", 28));
        Audio {
            stream,
            sample_rate: SAMPLE_RATE,
            oscs: oscs_cln,
        }
    }

    pub fn update(&mut self, m: MMy, cy: usize) {
        self.oscs.lock().unwrap().update(m, cy);
    }
}

fn stream_thrd(out_buff: &mut [f32], oscs: &Arc<Mutex<Oscillators>>) {
    let mut oscs = oscs.lock().unwrap();
    let mut sample;

    for i in 0..out_buff.len() / 2 {
        sample = oscs.next();

        oscs.filt_buff.0.pop_front();
        oscs.filt_buff.1.pop_front();
        oscs.filt_buff.0.push_back(sample.0 * 2. - sample.0);
        oscs.filt_buff.1.push_back(sample.1 * 2. - sample.1);
        out_buff[i * 2] =
            cpal::Sample::from(&(oscs.filt_buff.0.iter().sum::<f32>() / FILT_SZ as f32));
        out_buff[i * 2 + 1] =
            cpal::Sample::from(&(oscs.filt_buff.1.iter().sum::<f32>() / FILT_SZ as f32));
    }
}

//add osc trait to make an osc array
struct Oscillators {
    osc1: Square,
    osc1_pan: (f32, f32),
    osc2: Square,
    osc2_pan: (f32, f32),
    osc3: Wave,
    osc3_pan: (f32, f32),
    osc4: Noise,
    osc4_pan: (f32, f32),
    glob_vol: f32,
    glob_pan: (f32, f32),
    cy: usize,
    filt_buff: (VecDeque<f32>, VecDeque<f32>),
}

impl Oscillators {
    fn new() -> Oscillators {
        Oscillators {
            osc1: Square::new((0xff13, 0xff14), 0xff11, 0xff12, 0x1, true),
            osc1_pan: (0., 0.),
            osc2: Square::new((0xff18, 0xff19), 0xff16, 0xff17, 0x2, false),
            osc2_pan: (0., 0.),
            osc3: Wave::new((0xff1d, 0xff1e)),
            osc3_pan: (0., 0.),
            osc4: Noise::new(),
            osc4_pan: (0., 0.),
            glob_vol: 0.,
            glob_pan: (0., 0.),
            cy: OSC_T,
            filt_buff: (
                VecDeque::from(vec![0.; FILT_SZ]),
                VecDeque::from(vec![0.; FILT_SZ]),
            ),
        }
    }

    fn next(&mut self) -> (f32, f32) {
        let osc1 = self.osc1.next() * self.glob_vol;
        let osc2 = self.osc2.next() * self.glob_vol;
        let osc3 = self.osc3.next() * self.glob_vol;
        let osc4 = self.osc4.next() * self.glob_vol;

        (
            (osc1 * self.osc1_pan.0)
//                + osc2 * self.osc2_pan.0
//                + osc3 * self.osc3_pan.0
//                + osc4 * self.osc4_pan.0)
                * self.glob_pan.0,
            (osc1 * self.osc1_pan.1)
//                + osc2 * self.osc2_pan.1
//                + osc3 * self.osc3_pan.1
//                + osc4 * self.osc4_pan.1)
                * self.glob_pan.1,
        )
    }

    fn update(&mut self, m: MMy, cy: usize) {
        /*
        if cy > self.cy {
        */
        self.osc1.update(m);
        self.osc2.update(m);
        self.osc3.update(m);
        self.osc4.update(m);
        self.glob_vol = if m.su_get(0xff26) & 0x80 != 0 { 1. } else { 0. };
        self.glob_pan = (
            ((m.su_get(0xff24) & 0x7) as f32 / 7.),
            (((m.su_get(0xff24) & 0x70) >> 4) as f32 / 7.),
        );
        self.osc1_pan = match m.su_get(0xff25) & 0x11 {
            0x1 => (0.1, 0.9),
            0x10 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
        self.osc2_pan = match m.su_get(0xff25) & 0x22 {
            0x2 => (0.1, 0.9),
            0x20 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
        self.osc3_pan = match m.su_get(0xff25) & 0x44 {
            0x4 => (0.1, 0.9),
            0x40 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
        self.osc4_pan = match m.su_get(0xff25) & 0x88 {
            0x8 => (0.1, 0.9),
            0x80 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
        /*
            self.cy = OSC_T - (cy - self.cy);
        } else {
            self.cy -= cy;
        }
            */
    }
}

struct Square {
    per: f32,
    idx: f32,
    len: f32,
    len_on: bool,
    ratio: f32,
    env_s_len: f32,
    env_s_hi: f32,
    env_idx: f32,
    env_vol: f32,
    freq_addr: (u16, u16),
    wave_addr: u16,
    env_addr: u16,
    play_mask: u8,
    play: bool,
    sweep: bool,
    sweep_on: bool,
    sweep_per: f32,
    sweep_idx: f32,
    sweep_val: f32,
}

impl Square {
    fn new(
        freq_addr: (u16, u16),
        wave_addr: u16,
        env_addr: u16,
        play_mask: u8,
        sweep: bool,
    ) -> Square {
        Square {
            per: 1.,
            idx: 0.,
            len: 0.,
            len_on: false,
            ratio: 0.5,
            env_s_len: 0.,
            env_s_hi: 0.,
            env_idx: 0.,
            env_vol: 0.,
            freq_addr,
            wave_addr,
            env_addr,
            play_mask,
            play: false,
            sweep,
            sweep_on: false,
            sweep_per: 0.,
            sweep_idx: 0.,
            sweep_val: 0.,
        }
    }

    fn update(&mut self, m: MMy) {
        if !self.play {
            m.su_set(0xff26, m.su_get(0xff26) & !self.play_mask);
        }
        if m.su_get(self.freq_addr.1) & 0x80 != 0 {
            m.su_set(self.freq_addr.1, m.su_get(self.freq_addr.1) & !0x80);
            m.su_set(0xff26, m.su_get(0xff26) | self.play_mask);
            self.play = true;

            self.idx = 0.;
            self.env_idx = 0.;
            self.sweep_idx = 0.;
            self.env_vol = ((m.su_get(self.env_addr) & 0xf0) >> 4) as f32 / 15. / SND_DIV;
        }
        self.per = SAMPLE_RATE as f32
            / (131072.
                / (2048
                    - (m.su_get(self.freq_addr.0) as usize
                        | ((m.su_get(self.freq_addr.1) as usize & 0x7) << 8)))
                    as f32);
        self.len = (64 - (m.su_get(self.wave_addr) & 0x3f)) as f32 * SAMPLE_RATE as f32 / 256.;
        self.len_on = m.su_get(self.freq_addr.1) & 0x40 != 0;
        self.ratio = match (m.su_get(self.wave_addr) & 0xc0) >> 6 {
            0 => 1. / 8.,
            1 => 1. / 4.,
            3 => 3. / 4.,
            2 | _ => 1. / 2.,
        };
        self.env_s_len = (m.su_get(self.env_addr) & 0x7) as f32 / 64. * SAMPLE_RATE as f32;
        self.env_s_hi = if m.su_get(self.env_addr) & 0x8 != 0 {
            1.
        } else {
            -1.
        } / 15.
            / SND_DIV;
        if self.sweep {
            self.sweep_per = ((m.su_get(0xff10) & 0x70) >> 4) as f32 * SAMPLE_RATE as f32 / 128.;
            self.sweep_val = if m.su_get(0xff10) & 0x8 != 0 { -1. } else { 1. }
                / 2_f32.powf((m.su_get(0xff10) & 0x7) as f32);
            self.sweep_on = m.su_get(0xff10) & 0x7 != 0;
        }
    }

    fn next(&mut self) -> f32 {
        let env_on = self.env_s_len != 0.;

        if env_on {
            if self.env_idx >= self.env_s_len {
                self.env_vol += self.env_s_hi * (self.env_idx / self.env_s_len);
                if self.env_vol > 1. {
                    self.env_vol = 1.;
                } else if self.env_vol < 0. {
                    self.env_vol = 0.;
                }
                self.env_idx %= self.env_s_len;
            }
            self.env_idx += 1.;
        }
        if self.sweep && self.sweep_on {
            if self.sweep_idx > self.sweep_per {
                let sweep = self.per * self.sweep_val;

                self.per += sweep;
                self.sweep_idx %= self.sweep_per;
            } else {
                self.sweep_idx += 1.;
            }
        }
        let result = if self.idx < self.per * self.ratio {
            self.env_vol
        } else {
            0.
        };

        if self.len > 0. {
            self.len -= 1.;
        } else if self.len_on {
            self.play = false;
            return 0.;
        }
        self.idx = (self.idx + 1.) % self.per;
        result
    }
}

struct Wave {
    on: bool,
    per: f32,
    idx: f32,
    len: f32,
    len_on: bool,
    vol: f32,
    freq_addr: (u16, u16),
    ram: Vec<f32>,
    play: bool,
}

impl Wave {
    fn new(freq_addr: (u16, u16)) -> Wave {
        Wave {
            on: false,
            per: 1.,
            idx: 0.,
            len: 0.,
            len_on: false,
            vol: 0.,
            freq_addr,
            ram: Vec::new(),
            play: false,
        }
    }

    fn update(&mut self, m: MMy) {
        if !self.play {
            m.su_set(0xff26, m.su_get(0xff26) & !0x4);
        }
        if m.su_get(self.freq_addr.1) & 0x80 != 0 {
            m.su_set(self.freq_addr.1, m.su_get(self.freq_addr.1) & !0x80);
            m.su_set(0xff26, m.su_get(0xff26) | 0x4);
            self.play = true;
            self.idx = 0.;
        }

        self.on = m.su_get(0xff1a) != 0;
        self.per = SAMPLE_RATE as f32
            / (65536.
                / (2048
                    - (m.su_get(self.freq_addr.0) as usize
                        | ((m.su_get(self.freq_addr.1) as usize & 0x7) << 8)))
                    as f32);
        self.len = (256. - m.su_get(0xff1b) as f32) * SAMPLE_RATE as f32 / 256.;
        self.len_on = m.su_get(self.freq_addr.1) & 0x40 != 0;
        self.vol = match (m.su_get(0xff1c) & 0x60) >> 5 {
            1 => 1.,
            2 => 0.5,
            3 => 0.25,
            0 | _ => 0.,
        };
        self.ram.clear();
        let mut spl;
        for i in (0x0..=0xf).rev() {
            spl = m.su_get(0xff30 | i);

            self.ram.push((spl & 0xf) as f32);
            self.ram.push(((spl & 0xf0) >> 4) as f32);
        }
    }

    fn next(&mut self) -> f32 {
        if !self.on {
            return 0.;
        }
        let ram_idx = (self.idx * 31. / self.per) % self.ram.len() as f32;
        let result = self.ram[ram_idx as usize] * self.vol / 15. / SND_DIV;

        if self.len > 0. {
            self.len -= 1.;
        } else if self.len_on {
            self.play = false;
            return 0.;
        }
        self.idx = (self.idx + 1.) % self.per;
        result
    }
}

struct Noise {
    per: f32,
    idx: f32,
    len: f32,
    len_on: bool,
    tables: (Vec<f32>, Vec<f32>),
    cur_table: usize,
    idx_table: usize,
    env_s_len: f32,
    env_s_hi: f32,
    env_idx: f32,
    env_vol: f32,
    play: bool,
}

impl Noise {
    fn new() -> Noise {
        Noise {
            per: 0.,
            idx: 0.,
            len: 0.,
            len_on: false,
            tables: (Noise::gen_table(15), Noise::gen_table(7)),
            cur_table: 0,
            idx_table: 0,
            env_s_len: 0.,
            env_s_hi: 0.,
            env_idx: 0.,
            env_vol: 0.,
            play: false,
        }
    }

    fn gen_table(size: usize) -> Vec<f32> {
        let mut data: u16;
        let feeder: u16;
        let len: usize;

        match size {
            7 => {
                data = 0x7f;
                feeder = 0x40;
                len = 0x7f;
            }
            15 | _ => {
                data = 0x7fff;
                feeder = 0x4000;
                len = 0x7fff;
            }
        }

        let mut result = Vec::new();

        for _ in 0..len {
            let val = data & 0x1;

            data >>= 1;
            if val ^ data & 0x1 != 0 {
                data |= feeder;
            }
            result.push(val as f32);
        }
        result
    }

    fn update(&mut self, m: MMy) {
        if !self.play {
            m.su_set(0xff26, m.su_get(0xff26) & !0x8);
        }
        if m.su_get(0xff23) & 0x80 != 0 {
            m.su_set(0xff23, m.su_get(0xff23) & !0x80);
            m.su_set(0xff26, m.su_get(0xff26) | 0x8);
            self.play = true;

            self.idx = 0.;
            self.env_idx = 0.;
            self.idx_table = 0;
            self.env_vol = ((m.su_get(0xff21) & 0xf0) >> 4) as f32 / 15. / SND_DIV;
        }

        let mut r = (m.su_get(0xff22) & 0x7) as f32;

        if r == 0. {
            r = 0.5;
        }
        self.per =
            44100. / (524288. / r / 2_u32.pow(((m.su_get(0xff22) & 0xf0) as u32 >> 4) + 1) as f32);
        self.len = (64 - (m.su_get(0xff20) & 0x3f)) as f32 * SAMPLE_RATE as f32 / 256.;
        self.len_on = m.su_get(0xff23) & 0x40 != 0;
        self.cur_table = if m.su_get(0xff22) & 0x8 != 0 { 1 } else { 0 };
        self.env_s_len = (m.su_get(0xff21) & 0x7) as f32 / 64. * SAMPLE_RATE as f32;
        self.env_s_hi = if m.su_get(0xff21) & 0x8 != 0 { 1. } else { -1. } / 15. / SND_DIV;
    }

    fn next(&mut self) -> f32 {
        let env_on = self.env_s_len != 0.;

        if env_on {
            if self.env_idx > self.env_s_len {
                self.env_vol += self.env_s_hi * (self.env_idx / self.env_s_len);
                if self.env_vol > 1. {
                    self.env_vol = 1.;
                } else if self.env_vol < 0. {
                    self.env_vol = 0.;
                }
                self.env_idx %= self.env_s_len;
            }
            self.env_idx += 1.;
        }
        let tab = if self.cur_table == 0 {
            &self.tables.0
        } else {
            &self.tables.1
        };
        if self.idx >= self.per {
            self.idx %= 1.;
            self.idx_table = (self.idx_table + 1) % tab.len();
        }
        let cur_val = tab[self.idx_table] * self.env_vol;

        if self.len > 0. {
            self.len -= 1.;
        } else if self.len_on {
            self.play = false;
            return 0.;
        }
        self.idx += 1.;
        cur_val
    }
}
