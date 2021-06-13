use crate::mem::*;
use crate::utils::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

const SAMPLE_RATE: u32 = 44100;

pub struct Audio {
    stream: cpal::Stream,
    chunk: Vec<(f32, f32)>,
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
            chunk: Vec::new(),
            oscs: oscs_cln,
        }
    }

    pub fn update(&mut self, m: MMy) {
        self.oscs.lock().unwrap().update(m);
    }
}

fn stream_thrd(out_buff: &mut [f32], oscs: &Arc<Mutex<Oscillators>>) {
    let mut oscs = oscs.lock().unwrap();
    let mut sample;

    for i in 0..out_buff.len() / 2 {
        sample = oscs.next();

        out_buff[i * 2] = cpal::Sample::from(&sample.0);
        out_buff[i * 2 + 1] = cpal::Sample::from(&sample.1);
    }
}

//add osc trait to make an osc array
struct Oscillators {
    osc1: Square,
    osc2: Square,
    osc3: Square,
    cycle: usize,
}

impl Oscillators {
    fn new() -> Oscillators {
        Oscillators {
            osc1: Square::new((0xff12, 0xff13)),
            osc2: Square::new((0xff18, 0xff19)),
            osc3: Square::new((0xff1d, 0xff1e)),
            cycle: 0,
        }
    }

    fn next(&mut self) -> (f32, f32) {
        let (l1, r1) = self.osc1.next();
        let (l2, r2) = self.osc2.next();
        let (l3, r3) = self.osc3.next();
        (l1 + l2 + l3, r1 + r2 + r3)
    }

    fn update(&mut self, m: MMy) {
        self.osc1.update(m);
        self.osc2.update(m);
        self.osc3.update(m);
    }
}

struct Square {
    on: bool,
    per: f64,
    idx: f64,
    freq_addr: (u16, u16),
}

impl Square {
    fn new(freq_addr: (u16, u16)) -> Square {
        Square {
            on: false,
            per: 1.,
            idx: 0.,
            freq_addr,
        }
    }

    fn update(&mut self, m: MMy) {
        let per_tmp = SAMPLE_RATE as f64
            / 2.
            / (131072.
                / (2048
                    - (m.su_get(self.freq_addr.0) as usize
                        | ((m.su_get(self.freq_addr.1) as usize & 0x7) << 8)))
                    as f64);

        if per_tmp != self.per {
            self.per = per_tmp;
            self.idx = 0.;
        }
    }

    fn next(&mut self) -> (f32, f32) {
        let result = if self.idx < self.per / 2. {
            (-0.2, -0.2)
        } else {
            (0.2, 0.2)
        };

        self.idx = (self.idx + 1.) % self.per.round();
        result
    }
}

/*
#[test]
fn audio_test() {
    let mut audio = Audio::new();
    let mut count1 = 0;
    let mut count2 = 0;
    let mut buf_l = vec![0_f32; CHUNK_SZ];
    let mut buf_r = vec![0_f32; CHUNK_SZ];

    for j in 0..10000000 {
        if let Some(_) = audio.push_sample(
            if count1 < 100 { -0.1 } else { 0.1 },
            if count2 < 75 { -0.1 } else { 0.1 },
        ) {
            count1 = (count1 + 1) % 200;
            count2 = (count2 + 1) % 150;
        }
    }
}
*/
