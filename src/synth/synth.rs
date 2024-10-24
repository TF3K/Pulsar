use super::waveform::WaveForm;
use super::envelope::Envelope;
use super::adsr::ADSR;
use super::oscillator::Oscillator;
use super::key_mapping::{Note, PitchClass, get_pitch_class};

use device_query::Keycode;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU64, Ordering};
use wide::f32x4;

pub struct Synth {
    pub sample_rate:                f32,
    pub sample_clock:               AtomicU64,
    pub active_keys:                HashSet<Keycode>,
    pub key_envelopes:              HashMap<Keycode, Envelope>,
    pub oscillators:                HashMap<Keycode, Vec<Oscillator>>,
    pub current_waveform:           WaveForm,
    pub adsr:                       ADSR,
    pub detune:                     f32,
    pub num_oscillators:            u32,
    pub master_volume:              f32,
}

impl Synth {
    pub fn new(sample_rate: f32, adsr: ADSR) -> Self {
        Synth {
            sample_rate,
            sample_clock:           AtomicU64::new(0),
            active_keys:            HashSet::new(),
            key_envelopes:          HashMap::new(),
            oscillators:            HashMap::new(),
            current_waveform:       WaveForm::Sine,
            adsr,                 
            detune:                 0.0,
            num_oscillators:        4,
            master_volume:          1.0,
        }
    }

    pub fn frequency(&self, key: Keycode) -> Option<f32> {
        get_pitch_class(&key)
            .and_then(|pitch_class| FREQUENCY_MAP.get(pitch_class))
            .cloned()
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

        let step = detune_range / (self.num_oscillators as f32 - 1.0).max(1.0);

        (0..self.num_oscillators)
            .map(|i| {
                let offset = (i as f32 * step) - (detune_range / 2.0);
                base_freq * (1.0 + offset / base_freq)
            })
            .collect()
    }

    pub fn increment_sample_clock(&self) {
        self.sample_clock.fetch_add(1, Ordering::Relaxed);
    }

    pub fn generate_waveform(&self, key: Keycode) -> f32 {
        let t = self.get_sample_clock();
    
        if let Some(oscillators) = self.oscillators.get(&key) {
            let mut waveform_value = 0.0;
            let num_oscillators = oscillators.len();
    
            // SIMD processing for groups of 4 oscillators
            for chunk in oscillators.chunks_exact(4) {
                let samples = f32x4::from([
                    chunk[0].next_sample(t),
                    chunk[1].next_sample(t),
                    chunk[2].next_sample(t),
                    chunk[3].next_sample(t),
                ]);
                waveform_value += samples.reduce_add(); // Sum the elements of f32x4
            }
    
            // Handle any remaining oscillators that couldn't be processed in groups of 4
            let remainder = oscillators.chunks_exact(4).remainder();
            for osc in remainder {
                waveform_value += osc.next_sample(t);
            }
    
            let normalized_value = waveform_value / (num_oscillators as f32).max(1.0).sqrt();
    
            let envelope_amplitude = self.key_envelopes.get(&key)
                .map(|env| env.amplitude)
                .unwrap_or(0.0);
    
            normalized_value * envelope_amplitude * self.master_volume
        } else {
            0.0
        }
    }

    pub fn set_detune(&mut self, detune: f32) {
        self.detune = detune;
        
        let updates: Vec<(Keycode, Vec<f32>)> = self.oscillators.keys()
            .filter_map(|&key| {
                self.frequency(key).map(|base_freq| {
                    let detuned_frequencies = self.get_detuned_frequencies(base_freq);
                    (key, detuned_frequencies)
                })
            })
            .collect();
    
        for (key, detuned_frequencies) in updates {
            if let Some(oscillators) = self.oscillators.get_mut(&key) {
                for (osc, &freq) in oscillators.iter_mut().zip(detuned_frequencies.iter()) {
                    osc.set_frequency(freq);
                }
            }
        }
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
            let mut new_envelope = Envelope::new(self.adsr);
            new_envelope.trigger_attack();
            self.key_envelopes.insert(key, new_envelope);
    
            if let Some(frequency) = self.frequency(key) {
                let detuned_frequencies = self.get_detuned_frequencies(frequency);
                let oscillators = detuned_frequencies
                    .into_iter()
                    .map(|freq| Oscillator::new(freq, self.current_waveform.clone()))
                    .collect();
                self.oscillators.insert(key, oscillators);
            }
        }
    }

    pub fn get_sample_clock(&self) -> f32 {
        self.sample_clock.load(Ordering::Relaxed) as f32 / self.sample_rate
    }
    
    pub fn remove_note(&mut self, key: Keycode) {
        if self.active_keys.remove(&key) {
            if let Some(envelope) = self.key_envelopes.get_mut(&key) {
                envelope.trigger_release();
            }
            self.oscillators.remove(&key);
        }
    }

    pub fn toggle_waveform(&mut self) {
        self.current_waveform.toggle();
        
        for oscillators in self.oscillators.values_mut() {
            for osc in oscillators.iter_mut() {
                osc.set_waveform(self.current_waveform.clone());
            }
        }
    }
}

lazy_static! {
    static ref FREQUENCY_MAP: HashMap<PitchClass, f32> = {
        let mut m = HashMap::new();
        let base_frequency = 440.0; // A4
        let a4 = PitchClass::new(Note::A, 4);

        for octave in 0..10 {
            for note in [Note::C, Note::CSharp, Note::D, Note::DSharp, Note::E, Note::F, 
                        Note::FSharp, Note::G, Note::GSharp, Note::A, Note::ASharp, Note::B].iter() {
                let pitch_class = PitchClass::new(*note, octave);
                let semitone_difference = (pitch_class.octave - a4.octave) * 12 +
                    (*note as i32 - Note::A as i32);
                let frequency = base_frequency * 2f32.powf(semitone_difference as f32 / 12.0);
                m.insert(pitch_class, frequency);
            }
        }
        m
    };
}