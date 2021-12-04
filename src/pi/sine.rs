use crate::pi::{
    event::{Event, Triggered},
    types::AudioProcessor,
    utils::frequency_from_note_number,
};

pub struct SineOscillator {
    phase: f64, // オシレータの進行具合。sin(x)のxを保持してる。
    freq: f64,  // 鳴らしている音の周波数。
    pitch: f64, // ピッチベンド幅を保持しておくフィールド。
}

impl SineOscillator {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            freq: 440.0,
            pitch: 0.0,
        }
    }
}

impl Triggered for SineOscillator {
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::NoteOn { note, velocity: _ } => {
                self.freq = frequency_from_note_number(*note);
            }
            Event::NoteOff { note: _ } => {}
            Event::PitchBend { ratio } => {
                self.pitch = *ratio;
            }
        }
    }
}

impl AudioProcessor<f64> for SineOscillator {
    fn process(&mut self, sample_rate: f64) -> f64 {
        let phase_diff = (self.freq * self.pitch) * 2.0 * std::f64::consts::PI / sample_rate;
        self.phase += phase_diff;

        self.phase.sin()
    }
}
