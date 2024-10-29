mod synth;

use std::{
    collections::HashSet,
    io::stdout,
    sync::Arc,
    time::Duration,
    thread,
};
use parking_lot::Mutex;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::{Sink, OutputStream};
use synth::{adsr::ADSR, Slider, Synth, SynthSource};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event,
        MouseButton, MouseEvent, MouseEventKind, KeyCode
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};

fn main() {
    let slider_width = 100;
    let mut detune_slider = Slider::new(0.0, 1.0, slider_width);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    enable_raw_mode().unwrap();
    execute!(stdout(), EnableMouseCapture, Hide).unwrap();

    // Clear the terminal screen at the start
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    let sample_rate = 44100;
    
    // Set ADSR with duration values; ensure `ADSR` struct handles `Duration` correctly if needed
    let adsr = ADSR::new(
        100,      // Attack (ms)
        1000,     // Decay (ms)
        1.0,      // Sustain level
        350,      // Release (ms)
    );
    
    let synth = Arc::new(Mutex::new(Synth::new(
        sample_rate as f32,
        adsr,
    )));
    
    // Audio thread to handle SynthSource with Rodio Sink
    let audio_synth = Arc::clone(&synth);
    let audio_thread = thread::Builder::new()
        .name("audio_processing".to_string())
        .spawn(move || {
            let source = SynthSource::new(audio_synth, sample_rate);
            sink.set_volume(1.0);
            sink.append(source);
            sink.play();
            
            while !sink.empty() {
                thread::sleep(Duration::from_micros(500));
            }
        })
        .unwrap();

    let device_state = DeviceState::new();
    let mut last_keys: HashSet<Keycode> = HashSet::new();
    let mut is_dragging = false;
    
    // Input loop handling keys, mouse, and envelope updates
    'main: loop {
        if event::poll(Duration::from_micros(100)).unwrap() {
            execute!(stdout(), MoveTo(0, 0), Clear(ClearType::CurrentLine)).unwrap();

            detune_slider.draw("Detune");

            match event::read().unwrap() {
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    match kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            is_dragging = true;
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            is_dragging = false;
                        }
                        _ => {}
                    }
                    
                    if is_dragging {
                        let min_column = 1;
                        let max_column = min_column + slider_width as u16;

                        if column >= min_column && column <= max_column && row == 0 {
                            let pos = (column - min_column) as usize;
                            detune_slider.update_value(pos);
                            let detune_value = detune_slider.get_value();
                            let mut synth = synth.lock();
                            synth.set_detune(detune_value);
                        }
                    }
                }
                Event::Key(key_event) => {
                    if let KeyCode::Esc = key_event.code {
                        break 'main;
                    }
                }
                _ => {}
            }
        }

        // Handle key inputs
        let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();
        let mut synth = synth.lock();
        
        // Direct envelope updates
        synth.update_envelope();
        synth.set_master_volume(0.8);
        
        // Add or remove notes based on key differences
        for key in keys.difference(&last_keys) {
            synth.add_note(*key);
        }
        
        for key in last_keys.difference(&keys) {
            synth.remove_note(*key);
        }
        
        if keys.contains(&Keycode::Space) && !last_keys.contains(&Keycode::Space) {
            synth.toggle_waveform();
        }

        last_keys = keys;
        drop(synth);
        
        thread::sleep(Duration::from_micros(100));
    }

    // Cleanup
    execute!(stdout(), DisableMouseCapture, Show).unwrap();
    disable_raw_mode().unwrap();
    
    // Clear active keys and envelopes before exiting
    let mut synth = synth.lock();
    synth.active_keys.clear();
    synth.key_envelopes.clear();
    drop(synth);
    
    audio_thread.join().unwrap();
}