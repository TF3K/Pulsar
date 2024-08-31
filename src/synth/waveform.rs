use std::f32::consts::PI;
use rand::Rng;

#[derive(Clone)]
pub enum WaveForm {
    Sine,
    Saw,
    Square,
    Pulse,
    Triangle,
    WhiteNoise,
}

impl WaveForm {
    pub fn generate(&self, frequency: f32, t: f32) -> f32 {
        match self {
            WaveForm::Sine => (t * frequency * 2.0 * PI).sin(),
            WaveForm::Saw => {
                let phase = t * frequency % 1.0;
                2.0 * phase - 1.0
            },
            WaveForm::Square => {
                let phase = t * frequency % 1.0;
                if phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            },
            WaveForm::Pulse => {
                let phase = t * frequency % 1.0;
                if phase < 0.25 {
                    1.0
                } else {
                    -1.0
                }
            },
            WaveForm::Triangle => {
                let phase = t * frequency % 1.0;
                if phase < 0.25 {
                    4.0 * phase
                } else if phase < 0.75 {
                    2.0 - 4.0 * phase
                } else {
                    4.0 * phase - 4.0
                }
            }
            WaveForm::WhiteNoise => {
                let mut rng = rand::thread_rng();
                rng.gen_range(-1.0..1.0)
            },
        }
    } 

    pub fn toggle(&mut self) {
        *self = match self {
            WaveForm::Sine => WaveForm::Saw,
            WaveForm::Saw => WaveForm::Square,
            WaveForm::Square => WaveForm::Pulse,
            WaveForm::Pulse => WaveForm::Triangle,
            WaveForm::Triangle => WaveForm::WhiteNoise,
            WaveForm::WhiteNoise => WaveForm::Sine,
        }
    }
}