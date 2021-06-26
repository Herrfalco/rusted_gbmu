use crate::mem::*;
use crate::utils::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::FairMutex;
use std::collections::VecDeque;
use std::sync::Arc;

const SAMPLE_RATE: u32 = 96000;
const SND_DIV: f32 = 6.;
const FILT_SZ: usize = 4;

pub struct Audio {
    _stream: cpal::Stream,
    _sample_rate: u32,
    oscs: Arc<FairMutex<Oscillators>>,
}

impl Audio {
    pub fn new(snd_mem: SM) -> Audio {
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
        let oscs = Arc::new(FairMutex::new(Oscillators::new(snd_mem)));
        let oscs_clone = oscs.clone();
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    stream_thrd(data, oscs_clone.clone())
                },
                err_fn,
            )
            .unwrap_or_else(|_| fatal_err("Can't create output stream", 27));
        stream
            .play()
            .unwrap_or_else(|_| fatal_err("Can't run output stream", 28));
        Audio {
            _stream: stream,
            _sample_rate: SAMPLE_RATE,
            oscs,
        }
    }

    pub fn update(&mut self) {
        self.oscs.lock().update();
    }
}

fn stream_thrd(out_buff: &mut [f32], oscs: Arc<FairMutex<Oscillators>>) {
    let mut sample;

    for i in 0..out_buff.len() / 2 {
        let mut oscs = oscs.lock();
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
    filt_buff: (VecDeque<f32>, VecDeque<f32>),
    snd_mem: SM,
}

impl Oscillators {
    fn new(snd_mem: SM) -> Oscillators {
        Oscillators {
            osc1: Square::new(snd_mem.clone(), (0xff13, 0xff14), 0xff11, 0xff12, 0x1, true),
            osc1_pan: (0., 0.),
            osc2: Square::new(
                snd_mem.clone(),
                (0xff18, 0xff19),
                0xff16,
                0xff17,
                0x2,
                false,
            ),
            osc2_pan: (0., 0.),
            osc3: Wave::new(snd_mem.clone(), (0xff1d, 0xff1e)),
            osc3_pan: (0., 0.),
            osc4: Noise::new(snd_mem.clone()),
            osc4_pan: (0., 0.),
            glob_vol: 0.,
            glob_pan: (0., 0.),
            filt_buff: (
                VecDeque::from(vec![0.; FILT_SZ]),
                VecDeque::from(vec![0.; FILT_SZ]),
            ),
            snd_mem,
        }
    }

    fn update(&mut self) {
        self.osc1.minit();
        self.osc2.minit();
        self.osc3.minit();
        self.osc4.minit();
    }

    fn next(&mut self) -> (f32, f32) {
        self.mload();

        let osc1 = self.osc1.next() * self.glob_vol;
        let osc2 = self.osc2.next() * self.glob_vol;
        let osc3 = self.osc3.next() * self.glob_vol;
        let osc4 = self.osc4.next() * self.glob_vol;

        (
            (osc1 * self.osc1_pan.0
                + osc2 * self.osc2_pan.0
                + osc3 * self.osc3_pan.0
                + osc4 * self.osc4_pan.0)
                * self.glob_pan.0,
            (osc1 * self.osc1_pan.1
                + osc2 * self.osc2_pan.1
                + osc3 * self.osc3_pan.1
                + osc4 * self.osc4_pan.1)
                * self.glob_pan.1,
        )
    }

    fn mload(&mut self) {
        let m = self.snd_mem.read();

        self.glob_vol = if m.get(0xff26) & 0x80 != 0 { 1. } else { 0. };
        self.glob_pan = (
            ((m.get(0xff24) & 0x7) as f32 / 7.),
            (((m.get(0xff24) & 0x70) >> 4) as f32 / 7.),
        );
        self.osc1_pan = match m.get(0xff25) & 0x11 {
            0x1 => (0.1, 0.9),
            0x10 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
        self.osc2_pan = match m.get(0xff25) & 0x22 {
            0x2 => (0.1, 0.9),
            0x20 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
        self.osc3_pan = match m.get(0xff25) & 0x44 {
            0x4 => (0.1, 0.9),
            0x40 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
        self.osc4_pan = match m.get(0xff25) & 0x88 {
            0x8 => (0.1, 0.9),
            0x80 => (0.9, 0.1),
            _ => (0.5, 0.5),
        };
    }
}

struct Square {
    init: bool,
    freq: u16,
    ratio: f32,
    per_idx: f32,
    stop: bool,

    len: f32,
    len_on: bool,
    len_idx: f32,

    env_s_len: f32,
    env_s_hi: f32,
    env_vol: f32,
    env_idx: f32,

    sweep_per: f32,
    sweep_up: bool,
    sweep_on: bool,
    sweep_n: u16,
    sweep_change: bool,
    sweep_idx: f32,

    sweep: bool,
    snd_mem: SM,
    freq_addr: (u16, u16),
    wave_addr: u16,
    env_addr: u16,
    play_mask: u8,
}

impl Square {
    fn new(
        snd_mem: SM,
        freq_addr: (u16, u16),
        wave_addr: u16,
        env_addr: u16,
        play_mask: u8,
        sweep: bool,
    ) -> Square {
        Square {
            init: false,
            freq: 0,
            ratio: 0.5,
            per_idx: 0.,
            stop: false,

            len: 0.,
            len_on: false,
            len_idx: 0.,

            env_s_len: 0.,
            env_s_hi: 0.,
            env_vol: 0.,
            env_idx: 0.,

            sweep_per: 0.,
            sweep_up: false,
            sweep_on: false,
            sweep_n: 0,
            sweep_change: false,
            sweep_idx: 0.,

            sweep,
            snd_mem,
            freq_addr,
            wave_addr,
            env_addr,
            play_mask,
        }
    }

    fn minit(&mut self) {
        let m = self.snd_mem.read();

        self.init = m.get(self.freq_addr.1) & 0x80 != 0;
        if self.init {
            self.per_idx = 0.;
            self.len_idx = 0.;
            self.env_idx = 0.;
            self.sweep_idx = 0.;
            self.env_vol = ((m.get(self.env_addr) & 0xf0) >> 4) as f32 / 15. / SND_DIV;
        }
    }

    fn mload(&mut self) {
        let m = self.snd_mem.read();

        self.freq = m.get(self.freq_addr.0) as u16 | ((m.get(self.freq_addr.1) as u16 & 0x7) << 8);
        self.len = (64 - (m.get(self.wave_addr) & 0x3f)) as f32 * SAMPLE_RATE as f32 / 256.;
        self.len_on = m.get(self.freq_addr.1) & 0x40 != 0;
        self.ratio = match (m.get(self.wave_addr) & 0xc0) >> 6 {
            0 => 1. / 8.,
            1 => 1. / 4.,
            3 => 3. / 4.,
            2 | _ => 1. / 2.,
        };
        self.env_s_len = (m.get(self.env_addr) & 0x7) as f32 / 64. * SAMPLE_RATE as f32;
        self.env_s_hi = if m.get(self.env_addr) & 0x8 != 0 {
            1.
        } else {
            -1.
        } / 15.
            / SND_DIV;
        if self.sweep {
            self.sweep_per = ((m.get(0xff10) & 0x70) >> 4) as f32 * SAMPLE_RATE as f32 / 128.;
            self.sweep_up = m.get(0xff10) & 0x8 == 0;
            self.sweep_n = (m.get(0xff10) & 0x7) as u16;
            self.sweep_on = m.get(0xff10) & 0x70 != 0;
        }
        self.stop = m.get(self.env_addr) == 0 || self.freq == 0x7ff;
    }

    fn msave(&mut self) {
        let mut m = self.snd_mem.write();
        let mut tmp = m.get(0xff26);

        if self.init {
            m.set(0xff26, tmp | self.play_mask);
            tmp = m.get(self.freq_addr.1);
            m.set(self.freq_addr.1, tmp & !0x80);
            self.init = false;
        }
        if self.len_idx < self.len && self.len_on {
            tmp = m.get(0xff26);
            m.set(0xff26, tmp & !self.play_mask);
        }
        if self.sweep_change {
            m.set(self.freq_addr.0, self.freq as u8);
            tmp = m.get(self.freq_addr.1);
            m.set(
                self.freq_addr.1,
                (tmp & !0x7) | ((self.freq >> 8) & 0x7) as u8,
            );
            self.sweep_change = false;
        }
    }

    fn next(&mut self) -> f32 {
        self.mload();
        let env_on = self.env_s_len != 0.;

        if self.stop {
            self.env_vol = 0.;
        } else if env_on {
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
                let sweep = self.freq >> self.sweep_n;

                if (self.sweep_up && self.freq + sweep <= 0x7ff)
                    || (!self.sweep_up && sweep <= self.freq)
                {
                    self.freq = if self.sweep_up {
                        self.freq + sweep
                    } else {
                        self.freq - sweep
                    };
                } else {
                    self.freq = if self.sweep_up { 0x800 } else { 0 };
                }
                self.sweep_change = true;
                self.sweep_idx %= self.sweep_per;
            } else {
                self.sweep_idx += 1.;
            }
        }
        let per = SAMPLE_RATE as f32 / (131072. / (2048. - self.freq as f32));
        let result = if self.per_idx < per * self.ratio {
            self.env_vol
        } else {
            0.
        };

        if self.len_idx < self.len {
            self.len_idx += 1.;
        } else if self.len_on {
            return 0.;
        }
        self.per_idx = (self.per_idx + 1.) % per;
        self.msave();
        result
    }
}

struct Wave {
    init: bool,
    freq: u16,
    on: bool,
    vol: f32,
    per_idx: f32,

    len: f32,
    len_on: bool,
    len_idx: f32,

    freq_addr: (u16, u16),
    snd_mem: SM,
    ram: Vec<f32>,
}

impl Wave {
    fn new(snd_mem: SM, freq_addr: (u16, u16)) -> Wave {
        Wave {
            init: false,
            freq: 0,
            on: false,
            vol: 0.,
            per_idx: 1.,

            len: 0.,
            len_on: false,
            len_idx: 0.,

            freq_addr,
            snd_mem,
            ram: Vec::new(),
        }
    }

    fn minit(&mut self) {
        let m = self.snd_mem.read();

        self.init = m.get(self.freq_addr.1) & 0x80 != 0;
        if self.init {
            self.per_idx = 0.;
            self.len_idx = 0.;
        }
    }

    fn mload(&mut self) {
        let m = self.snd_mem.read();

        self.on = m.get(0xff1a) != 0;
        self.freq = m.get(self.freq_addr.0) as u16 | ((m.get(self.freq_addr.1) as u16 & 0x7) << 8);
        self.len = (256. - m.get(0xff1b) as f32) * SAMPLE_RATE as f32 / 256.;
        self.len_on = m.get(self.freq_addr.1) & 0x40 != 0;
        self.vol = match (m.get(0xff1c) & 0x60) >> 5 {
            1 => 1.,
            2 => 0.5,
            3 => 0.25,
            0 | _ => 0.,
        };
        self.ram.clear();
        let mut spl;
        for i in (0x0..=0xf).rev() {
            spl = m.get(0xff30 | i);
            self.ram.push((spl & 0xf) as f32);
            self.ram.push(((spl & 0xf0) >> 4) as f32);
        }
    }

    fn msave(&mut self) {
        let mut m = self.snd_mem.write();
        let mut tmp = m.get(0xff26);

        if self.init {
            m.set(0xff26, tmp | 0x4);
            tmp = m.get(self.freq_addr.1);
            m.set(self.freq_addr.1, tmp & !0x80);
            self.init = false;
        }
        if self.len_idx < self.len && self.len_on {
            tmp = m.get(0xff26);
            m.set(0xff26, tmp & !0x4);
        }
    }

    fn next(&mut self) -> f32 {
        self.mload();

        if !self.on {
            return 0.;
        }
        let per = SAMPLE_RATE as f32 / (65536. / (2048. - self.freq as f32));
        let ram_idx = (self.per_idx * 31. / per) % self.ram.len() as f32;
        let result = self.ram[ram_idx as usize] * self.vol / 15. / SND_DIV;

        if self.len_idx < self.len {
            self.len_idx += 1.;
        } else if self.len_on {
            return 0.;
        }
        self.per_idx = (self.per_idx + 1.) % per;
        self.msave();
        result
    }
}

struct Noise {
    init: bool,
    freq: f32,
    per_idx: f32,

    len: f32,
    len_on: bool,
    len_idx: f32,

    tables: (Vec<f32>, Vec<f32>),
    cur_table: usize,
    idx_table: usize,

    env_s_len: f32,
    env_s_hi: f32,
    env_vol: f32,
    env_idx: f32,
    env_stop: bool,

    snd_mem: SM,
}

impl Noise {
    fn new(snd_mem: SM) -> Noise {
        Noise {
            init: false,
            freq: 0.,
            per_idx: 0.,

            len: 0.,
            len_on: false,
            len_idx: 0.,

            tables: (Noise::gen_table(15), Noise::gen_table(7)),
            cur_table: 0,
            idx_table: 0,

            env_s_len: 0.,
            env_s_hi: 0.,
            env_idx: 0.,
            env_vol: 0.,
            env_stop: false,

            snd_mem,
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

    fn minit(&mut self) {
        let m = self.snd_mem.read();

        self.init = m.get(0xff23) & 0x80 != 0;
        if self.init {
            self.per_idx = 0.;
            self.len_idx = 0.;
            self.env_idx = 0.;
            self.idx_table = 0;
            self.env_vol = ((m.get(0xff21) & 0xf0) >> 4) as f32 / 15. / SND_DIV;
        }
    }

    fn mload(&mut self) {
        let m = self.snd_mem.read();

        let mut r = (m.get(0xff22) & 0x7) as f32;
        r = if r != 0. { r } else { 0.5 };
        self.freq = 524288. / r / 2_f32.powf((((m.get(0xff22) & 0xf0) >> 4) + 1) as f32);
        self.len = (64 - (m.get(0xff20) & 0x3f)) as f32 * SAMPLE_RATE as f32 / 256.;
        self.len_on = m.get(0xff23) & 0x40 != 0;
        self.env_s_len = (m.get(0xff21) & 0x7) as f32 / 64. * SAMPLE_RATE as f32;
        self.env_s_hi = if m.get(0xff21) & 0x8 != 0 { 1. } else { -1. } / 15. / SND_DIV;
        self.cur_table = if m.get(0xff22) & 0x8 != 0 { 1 } else { 0 };
        self.env_stop = m.get(0xff21) == 0;
    }

    fn msave(&mut self) {
        let mut m = self.snd_mem.write();
        let mut tmp = m.get(0xff26);

        if self.init {
            m.set(0xff26, tmp | 0x8);
            tmp = m.get(0xff23);
            m.set(0xff23, tmp & !0x80);
            self.init = false;
        }
        if self.len_idx < self.len && self.len_on {
            tmp = m.get(0xff26);
            m.set(0xff26, tmp & !0x8);
        }
    }

    fn next(&mut self) -> f32 {
        self.mload();
        let env_on = self.env_s_len != 0.;

        if self.env_stop {
            self.env_vol = 0.;
        } else if env_on {
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
        self.idx_table %= tab.len();
        let per = SAMPLE_RATE as f32 / self.freq;
        let result = tab[self.idx_table] * self.env_vol;

        if self.len_idx < self.len {
            self.len_idx += 1.;
        } else if self.len_on {
            return 0.;
        }

        self.per_idx += 1.;
        if self.per_idx >= per {
            self.idx_table = (self.idx_table + 1) % tab.len();
            self.per_idx %= per;
        }
        self.msave();
        result
    }
}
