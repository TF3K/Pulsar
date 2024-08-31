use super::waveform::WaveForm;

pub struct Oscillator {
    pub frequency: f32,
    pub sample_rate: f32,
    pub waveform: WaveForm,
}

impl Oscillator {
    pub fn new(frequency: f32, sample_rate: f32, waveform: WaveForm) -> Self {
        Oscillator {
            frequency,
            sample_rate,
            waveform,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_waveform(&mut self, waveform: WaveForm) {
        self.waveform = waveform;
    }

    pub fn next_sample(&self, t: f32) -> f32 {
        self.waveform.generate(self.frequency, t)
    }
}