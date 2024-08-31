use crate::synth::adsr::ADSR;
use std::time::Instant;

#[derive(PartialEq, Eq, Debug)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Finished
}

pub struct Envelope {
    pub adsr: ADSR,
    pub stage: EnvelopeStage,
    pub start_time: Instant,
    pub amplitude: f32,
    pub current_sample: usize,
    pub sample_rate: f32,
}

impl Envelope {
    pub fn new(adsr: ADSR, sample_rate: f32) -> Self {
        Envelope {
            adsr,
            stage: EnvelopeStage::Attack,
            start_time: Instant::now(),
            amplitude: 1.0,
            current_sample: 0,
            sample_rate,
        }
    }

    pub fn update(&mut self) {
        let elapsed = Instant::now() - self.start_time;
        let elapsed_secs = elapsed.as_secs_f32();

        self.current_sample += 1;
        self.amplitude = match self.stage {
            EnvelopeStage::Attack => {
                if elapsed_secs >= self.adsr.attack {
                    self.stage = EnvelopeStage::Decay;
                    self.start_time = Instant::now();
                    1.0
                } else {
                    elapsed_secs / self.adsr.attack
                }
            }
            EnvelopeStage::Decay => {
                if elapsed_secs >= self.adsr.decay {
                    self.stage = EnvelopeStage::Sustain;
                    self.start_time = Instant::now();
                    self.adsr.sustain
                } else {
                    let decay_progress = elapsed_secs / self.adsr.decay;
                    1.0 + decay_progress * (self.adsr.sustain - 1.0)
                }
            }
            EnvelopeStage::Sustain => self.adsr.sustain,
            EnvelopeStage::Release => {
                if elapsed_secs >= self.adsr.release {
                    self.stage = EnvelopeStage::Finished;
                    0.0
                } else {
                    let release_progress = elapsed_secs / self.adsr.release;
                    self.amplitude * (1.0 - release_progress)
                }
            },
            EnvelopeStage::Finished => 0.0,
        };
    }

    pub fn is_finished(&self) -> bool {
        self.stage == EnvelopeStage::Finished
    }

    pub fn trigger_attack(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.start_time = Instant::now();
    }

    pub fn trigger_release(&mut self) {
        self.stage = EnvelopeStage::Release;
        self.start_time = Instant::now();
    }
}