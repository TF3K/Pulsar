use super::synth::Synth;

use rodio::Source;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;

pub struct SynthSource {
    synth:          Arc<Mutex<Synth>>,
    sample_rate:    u32,
}

impl SynthSource {
    pub fn new(synth: Arc<Mutex<Synth>>, sample_rate : u32) -> Self {
        SynthSource {
            synth,
            sample_rate,
        }
    }

    pub fn soft_clip(x: f32) -> f32 {
        let threshold = 0.95;
        if x.abs() > threshold {
            threshold * (x / x.abs())
        } else {
            x
        }
    }
}

impl Iterator for SynthSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut synth = self.synth.lock();

        let scaling_factor = synth.get_polyphonic_scaling_factor();

        let mut active_keys = Vec::new();
        for (key, envelope) in synth.key_envelopes.iter_mut() {
            envelope.update();
            if !envelope.is_finished() {
                active_keys.push(*key);
            }
        }

        let sample = active_keys.iter()
            .map(|&key| synth.generate_waveform(key))
            .sum::<f32>();

        synth.increment_sample_clock();

        synth.key_envelopes.retain(|_, envelope| !envelope.is_finished());

        let keys_to_retain: Vec<_> = synth.key_envelopes.keys().cloned().collect();

        synth.active_keys.retain(|key| keys_to_retain.contains(key));

        synth.oscillators.retain(|key, _| keys_to_retain.contains(key));

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

