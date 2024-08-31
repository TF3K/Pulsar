use device_query::Keycode;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Note {
    C = 0,
    CSharp = 1,
    D = 2,
    DSharp = 3,
    E = 4,
    F = 5,
    FSharp = 6,
    G = 7,
    GSharp = 8,
    A = 9,
    ASharp = 10,
    B = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PitchClass {
    pub note: Note,
    pub octave: i32,
}

impl PitchClass {
    pub fn new(note: Note, octave: i32) -> Self {
        PitchClass {
            note,
            octave,
        }
    }
}

lazy_static! {
    pub static ref KEY_MAP: HashMap<Keycode, PitchClass> = {
        let mut m = HashMap::new();
        // White keys
        m.insert(Keycode::W, PitchClass::new(Note::C, 3));
        m.insert(Keycode::X, PitchClass::new(Note::D, 3));
        m.insert(Keycode::C, PitchClass::new(Note::E, 3));
        m.insert(Keycode::V, PitchClass::new(Note::F, 3));
        m.insert(Keycode::B, PitchClass::new(Note::G, 3));
        m.insert(Keycode::N, PitchClass::new(Note::A, 3));
        m.insert(Keycode::Comma, PitchClass::new(Note::B, 3));
        m.insert(Keycode::Dot, PitchClass::new(Note::C, 4));
        m.insert(Keycode::Slash, PitchClass::new(Note::D, 4));
        m.insert(Keycode::A, PitchClass::new(Note::C, 4));
        m.insert(Keycode::Z, PitchClass::new(Note::D, 4));
        m.insert(Keycode::E, PitchClass::new(Note::E, 4));
        m.insert(Keycode::R, PitchClass::new(Note::F, 4));
        m.insert(Keycode::T, PitchClass::new(Note::G, 4));
        m.insert(Keycode::Y, PitchClass::new(Note::A, 4));
        m.insert(Keycode::U, PitchClass::new(Note::B, 4));
        m.insert(Keycode::I, PitchClass::new(Note::C, 5));
        m.insert(Keycode::O, PitchClass::new(Note::D, 5));
        m.insert(Keycode::P, PitchClass::new(Note::E, 5));
        // Black keys
        m.insert(Keycode::S, PitchClass::new(Note::CSharp, 3));
        m.insert(Keycode::D, PitchClass::new(Note::DSharp, 3));
        m.insert(Keycode::G, PitchClass::new(Note::FSharp, 3));
        m.insert(Keycode::H, PitchClass::new(Note::GSharp, 3));
        m.insert(Keycode::J, PitchClass::new(Note::ASharp, 3));
        m.insert(Keycode::L, PitchClass::new(Note::CSharp, 4));
        m.insert(Keycode::Semicolon, PitchClass::new(Note::DSharp, 4));
        m.insert(Keycode::Key2, PitchClass::new(Note::CSharp, 4));
        m.insert(Keycode::Key3, PitchClass::new(Note::DSharp, 4));
        m.insert(Keycode::Key5, PitchClass::new(Note::FSharp, 4));
        m.insert(Keycode::Key6, PitchClass::new(Note::GSharp, 4));
        m.insert(Keycode::Key7, PitchClass::new(Note::ASharp, 4));
        m.insert(Keycode::Key9, PitchClass::new(Note::CSharp, 5));
        m.insert(Keycode::Key0, PitchClass::new(Note::DSharp, 5));
        m
    };
}

pub fn get_pitch_class(key: &Keycode) -> Option<&PitchClass> {
    KEY_MAP.get(key)
}