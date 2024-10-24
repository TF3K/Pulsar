#[derive(Clone, Copy)]
pub struct ADSR {
    pub attack:     f32,
    pub decay:      f32,
    pub sustain:    f32,
    pub release:    f32,
}

impl ADSR {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        ADSR {
            attack,
            decay,
            sustain,
            release,
        }
    }
}