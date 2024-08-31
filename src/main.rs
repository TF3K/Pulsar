mod synth;

use std:: {collections::HashSet, io::stdout, sync::Arc, time::Duration};
use parking_lot::Mutex;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::{Sink, OutputStream};
use synth::{adsr::ADSR, Slider, Synth, SynthSource};
use crossterm::{cursor::{Hide, MoveTo, Show}, event::{self, DisableMouseCapture, EnableMouseCapture, Event, MouseButton, MouseEvent, MouseEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

fn main(){
    let slider_width = 100;
    let mut detune_slider = Slider::new(0.0, 1.0, slider_width);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    enable_raw_mode().unwrap();
    execute!(stdout(), EnableMouseCapture, Hide).unwrap();

    let sample_rate = 48000;
    

    let adsr = ADSR::new(0.1, 0.1, 0.7, 3.0);
    let synth = Arc::new(Mutex::new(Synth::new(
        sample_rate as f32,
        adsr,
    )));
    let source = SynthSource::new(synth.clone(), sample_rate);

    sink.set_speed(1.0);
    sink.append(source);
    sink.play();

    let device_state = DeviceState::new();
    let mut last_keys: HashSet<Keycode> = HashSet::new();
    let mut is_dragging = false;
    
    loop {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::CurrentLine)).unwrap();
        detune_slider.draw();

        if event::poll(Duration::from_millis(10)).unwrap() {
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
                    if key_event.code == event::KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }

        let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();
        let mut synth = synth.lock();
        synth.set_master_volume(0.8);
        synth.update_envelope();  
        
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
    }

    execute!(stdout(), DisableMouseCapture, Show).unwrap();
    disable_raw_mode().unwrap();
}