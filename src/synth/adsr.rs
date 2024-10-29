use std::time::Duration;
use crate::synth::envelope::EnvelopeStage;

#[derive(Debug, Clone, Copy)]
pub struct ADSR {
    pub attack: Duration,
    pub decay: Duration,
    pub sustain: f32,
    pub release: Duration,
}

impl ADSR {
    pub fn new(attack: u32, decay: u32, sustain: f32, release: u32) -> Self {
        ADSR {
            attack: Duration::from_millis(attack as u64),
            decay: Duration::from_millis(decay as u64),
            sustain,
            release: Duration::from_millis(release as u64),
        }
    }

    // This method calculates the amplitude based on the current time and stage
    pub fn calculate_amplitude(&self, stage: EnvelopeStage, elapsed: Duration, start_amplitude: f32) -> f32 {
        match stage {
            EnvelopeStage::Attack => {
                if elapsed >= self.attack {
                    1.0 // Full amplitude at the end of attack
                } else {
                    elapsed.as_secs_f32() / self.attack.as_secs_f32() // Linear ramp up
                }
            }
            EnvelopeStage::Decay => {
                if elapsed >= self.decay {
                    self.sustain // Hold at sustain level
                } else {
                    let decay_progress = elapsed.as_secs_f32() / self.decay.as_secs_f32();
                    1.0 - decay_progress * (1.0 - self.sustain) // Ramp down to sustain
                }
            }
            EnvelopeStage::Sustain => self.sustain, // Constant sustain level
            EnvelopeStage::Release => {
                if elapsed >= self.release {
                    0.0 // End of release
                } else {
                    let release_progress = elapsed.as_secs_f32() / self.release.as_secs_f32();
                    start_amplitude * (1.0 - release_progress) // Ramp down from current amplitude
                }
            }
            EnvelopeStage::Finished => 0.0,
        }
    }
}