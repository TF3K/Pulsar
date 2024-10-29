use crate::synth::adsr::ADSR;
use std::time::Instant;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Finished,
}

pub struct Envelope {
    pub adsr: ADSR,
    pub stage: EnvelopeStage,
    pub start_time: Instant,
    pub amplitude: f32,
    pub release_start_amplitude: f32,
}

impl Envelope {
    pub fn new(adsr: ADSR) -> Self {
        Envelope {
            adsr,
            stage: EnvelopeStage::Attack,  // Start with Attack stage
            start_time: Instant::now(),
            amplitude: 0.0,  // Start at zero amplitude
            release_start_amplitude: 0.0,
        }
    }

    pub fn update(&mut self) {
        let elapsed = Instant::now().duration_since(self.start_time);
        
        self.amplitude = match self.stage {
            EnvelopeStage::Attack => {
                let amp = self.adsr.calculate_amplitude(self.stage, elapsed, 0.0);
                if elapsed >= self.adsr.attack {
                    self.stage = EnvelopeStage::Decay;
                    self.start_time = Instant::now();
                }
                amp
            }
            EnvelopeStage::Decay => {
                let amp = self.adsr.calculate_amplitude(self.stage, elapsed, 0.0);
                if elapsed >= self.adsr.decay {
                    self.stage = EnvelopeStage::Sustain;
                }
                amp
            }
            EnvelopeStage::Sustain => {
                self.adsr.calculate_amplitude(self.stage, elapsed, 0.0)
            }
            EnvelopeStage::Release => {
                let amp = self.adsr.calculate_amplitude(self.stage, elapsed, self.release_start_amplitude);
                if elapsed >= self.adsr.release {
                    self.stage = EnvelopeStage::Finished;
                }
                amp
            }
            EnvelopeStage::Finished => 0.0,
        };
    }

    pub fn is_finished(&self) -> bool {
        self.stage == EnvelopeStage::Finished
    }

    pub fn trigger_attack(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.start_time = Instant::now();
        self.amplitude = 0.0;
    }

    pub fn trigger_release(&mut self) {
        if self.stage != EnvelopeStage::Release && self.stage != EnvelopeStage::Finished {
            self.stage = EnvelopeStage::Release;
            self.start_time = Instant::now();
            self.release_start_amplitude = self.amplitude;
        }
    }
}