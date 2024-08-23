
use super::synth::Synth;

use rodio::Source;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct SynthSource {
    synth: Arc<Mutex<Synth>>,
    sample_rate: u32,
}

impl SynthSource {
    pub fn new(synth: Arc<Mutex<Synth>>, sample_rate : u32) -> Self {
        SynthSource {
            synth,
            sample_rate,
        }
    }

    pub fn soft_clip(x: f32) -> f32 {
        if x.abs() <= 1.0 {
            x
        } else {
            x.signum() * (1.0 -  (-x.abs()).exp())
        }
    } 
}

impl Iterator for SynthSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let synth = self.synth.lock().unwrap();
        let t = synth.get_sample_clock();

        let scaling_factor = synth.get_polyphonic_scaling_factor();

        let sample = synth.active_keys.iter()
            .filter_map(|&key| synth.frequency(key))
            .map(|freq| synth.generate_waveform(freq, t))
            .sum::<f32>();

        synth.increment_sample_clock();

        Some(Self::soft_clip(sample * scaling_factor))
    }
}

impl Source for SynthSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

