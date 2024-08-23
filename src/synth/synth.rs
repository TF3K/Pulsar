use crate::synth::waveform::WaveForm;
use crate::synth::envelope::Envelope;

use device_query::Keycode;
use std::collections::{HashSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Synth {
    pub sample_rate: f32,
    pub sample_clock: AtomicU64,
    pub active_keys: HashSet<Keycode>,
    pub waveform: WaveForm,
    pub key_envelopes: HashMap<Keycode, Envelope>,
    pub detune: f32,
    pub num_oscillators: u32,
    pub envelope: Envelope,
    pub master_volume: f32,
}

impl Synth {
    pub fn new(sample_rate: f32, attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Synth {
            sample_rate,
            sample_clock: AtomicU64::new(0),
            active_keys: HashSet::new(),
            waveform: WaveForm::Sine,
            key_envelopes: HashMap::new(),
            detune: 0.0,
            num_oscillators: 3,
            envelope: Envelope::new(attack, decay, sustain, release),
            master_volume: 1.0,
        }
    }

    pub fn frequency(&self, key: Keycode) -> Option<f32> {
        match key {
            //white keys
            Keycode::W => Some(130.81), // C3
            Keycode::X => Some(146.83), // D3
            Keycode::C => Some(164.81), // E3
            Keycode::V => Some(174.61), // F3
            Keycode::B => Some(196.00), // G3
            Keycode::N => Some(220.00), // A3
            Keycode::Comma => Some(246.94), // B3
            Keycode::Dot => Some(261.63), // C4
            Keycode::A => Some(261.63), // C4
            Keycode::Z => Some(293.66), // D4
            Keycode::E => Some(329.63), // E4
            Keycode::R => Some(349.23), // F4
            Keycode::T => Some(392.00), // G4
            Keycode::Y => Some(440.00), // A4
            Keycode::U => Some(493.88), // B4
            Keycode::I => Some(523.25), // C5
            Keycode::O => Some(587.33), // D5
            Keycode::P => Some(659.25), // E5

            // black keys
            Keycode::S => Some(138.59), // C#3
            Keycode::D => Some(155.56), // D#3
            Keycode::G => Some(185.00), // F#3
            Keycode::H => Some(207.65), // G#3
            Keycode::J => Some(233.08), // A#3
            Keycode::Key2 => Some(277.18), // C#4
            Keycode::Key3 => Some(311.13), // D#4
            Keycode::Key5 => Some(369.99), // F#4
            Keycode::Key6 => Some(415.30), // G#4
            Keycode::Key7 => Some(466.16), // A#4
            Keycode::Key9 => Some(554.37), // C#5
            Keycode::Key0 => Some(622.25), // D#5
            _ => None,
        }
    }

    pub fn get_sample_clock(&self) -> f32 {
        self.sample_clock.load(Ordering::Relaxed) as f32 / self.sample_rate
    }

    pub fn get_polyphonic_scaling_factor(&self) -> f32 {
        let num_active_keys = self.active_keys.len() as f32;
        if num_active_keys <= 1.0 {
            1.0
        } else {
            1.0 / num_active_keys.powf(0.5)
        }
    }

    pub fn get_detuned_frequencies(&self, base_freq: f32) -> Vec<f32> {
        if self.num_oscillators == 1 {
            return vec![base_freq];
        }

        let detune_factor = self.detune / 100.0;
        let detune_range = base_freq * detune_factor;

        let step = detune_range / (self.num_oscillators as f32 - 1.0);

        (0..self.num_oscillators)
            .map(|i| base_freq + (i as f32 * step) - (detune_range / 2.0))
            .collect()
    }

    pub fn increment_sample_clock(&self) {
        self.sample_clock.fetch_add(1, Ordering::Relaxed);
    }

    pub fn generate_waveform(&self, frequency: f32, t: f32) -> f32 {
        let detuned_frequencies = self.get_detuned_frequencies(frequency);

        let waveform_value = if self.num_oscillators == 1 {
            self.waveform.generate(detuned_frequencies[0], t)
        } else {
            detuned_frequencies
                .iter()
                .map(|&freq| self.waveform.generate(freq, t))
                .sum::<f32>() / (self.num_oscillators as f32).max(1.0)
        };

        let normalized_value = waveform_value / (self.num_oscillators as f32).sqrt();

        normalized_value * self.envelope.amplitude * self.master_volume
    }

    pub fn set_detune(&mut self, detune: f32) {
        self.detune = detune;
    }

    pub fn update_envelope(&mut self) {
        for envelope in self.key_envelopes.values_mut() {
            envelope.update();
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn add_note(&mut self, key: Keycode) {
        if self.active_keys.insert(key) {
            self.envelope.trigger_attack();
        }
    }

    pub fn remove_note(&mut self, key: Keycode) {
        if self.active_keys.remove(&key) && self.active_keys.is_empty() {
            self.envelope.trigger_release();
        }
    }
}