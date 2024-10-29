use super::synth::Synth;
use rodio::Source;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;

pub struct SynthSource {
    synth: Arc<Mutex<Synth>>,
    sample_rate: u32,
    buffer: Vec<f32>,
    buffer_pos: usize,
}

impl SynthSource {
    pub fn new(synth: Arc<Mutex<Synth>>, sample_rate: u32) -> Self {
        SynthSource {
            synth,
            sample_rate,
            buffer: Vec::with_capacity(256), // Small buffer size
            buffer_pos: 0,
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

    fn fill_buffer(&mut self) {
        self.buffer.clear();
        let mut synth = self.synth.lock();

        let scaling_factor = synth.get_polyphonic_scaling_factor();
        
        // Pre-filter active keys for faster processing
        let active_keys: Vec<_> = synth.key_envelopes.iter_mut()
            .filter_map(|(key, envelope)| {
                envelope.update();
                if !envelope.is_finished() {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();

        // Generate samples in bulk for better performance
        for _ in 0..256 {
            let sample = active_keys.iter()
                .map(|&key| synth.generate_waveform(key))
                .sum::<f32>();

            self.buffer.push(Self::soft_clip(sample * scaling_factor));
            synth.increment_sample_clock();
        }

        // Cleanup finished envelopes and keys
        // In SynthSource::fill_buffer()
        synth.key_envelopes.retain(|_, envelope| !envelope.is_finished());
        let keys_to_retain: Vec<_> = synth.key_envelopes.keys().cloned().collect();
        synth.active_keys.retain(|key| keys_to_retain.contains(key));
        synth.oscillators.retain(|key, _| keys_to_retain.contains(key));
        
        self.buffer_pos = 0;
    }
}

impl Iterator for SynthSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer_pos >= self.buffer.len() {
            self.fill_buffer();
        }

        let sample = self.buffer[self.buffer_pos];
        self.buffer_pos += 1;
        Some(sample)
    }
}

impl Source for SynthSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(256) // Match the minimal buffer size
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