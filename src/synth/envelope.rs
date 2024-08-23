use std::time::Instant;

pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Envelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub stage: EnvelopeStage,
    pub start_time: Instant,
    pub amplitude: f32,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Envelope {
            attack,
            decay,
            sustain,
            release,
            stage: EnvelopeStage::Attack,
            start_time: Instant::now(),
            amplitude: 1.0,
        }
    }

    pub fn update(&mut self) {
        let elapsed = Instant::now() - self.start_time;
        let elapsed_secs = elapsed.as_secs_f32();

        self.amplitude = match self.stage {
            EnvelopeStage::Attack => {
                if elapsed_secs >= self.attack {
                    // Move to the decay stage
                    self.stage = EnvelopeStage::Decay;
                    self.start_time = Instant::now();
                    1.0
                } else {
                    // Linearly increase the amplitude
                    elapsed_secs / self.attack
                }
            }
            EnvelopeStage::Decay => {
                if elapsed_secs >= self.decay {
                    // Move to the sustain stage
                    self.stage = EnvelopeStage::Sustain;
                    self.start_time = Instant::now();
                    self.sustain
                } else {
                    // Linearly decrease the amplitude to the sustain level
                    let decay_progress = elapsed_secs / self.decay;
                    1.0 + decay_progress * (self.sustain - 1.0)
                }
            }
            EnvelopeStage::Sustain => {
                // Maintain the sustain level
                self.sustain
            }
            EnvelopeStage::Release => {
                if elapsed_secs >= self.release {
                    // End of the release phase
                    0.0
                } else {
                    // Linearly decrease the amplitude to 0.0
                    self.sustain * (1.0 - elapsed_secs / self.release)
                }
            }
        };
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