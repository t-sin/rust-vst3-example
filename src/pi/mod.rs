mod sine;
mod types;
mod utils;

pub mod event;
pub mod parameters;

pub use types::AudioProcessor;

use crate::pi::{
    event::{Event, Triggered},
    parameters::{Parameter, Parametric},
    sine::SineOscillator,
    utils::ratio_from_cents,
};

pub type Signal = (f64, f64);

pub struct PiSynth {
    osc: SineOscillator,
    note_on: bool, // 音が鳴ってるかを保持している。

    detune: i16, // ピッチをノブでぐりぐり変えられるパラメータ。
}

impl PiSynth {
    pub fn new() -> Self {
        Self {
            osc: SineOscillator::new(),
            note_on: false,

            detune: 0,
        }
    }
}

impl Triggered for PiSynth {
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::NoteOn {
                note: _,
                velocity: _,
            } => {
                self.note_on = true;
                self.osc.trigger(event);
            }
            Event::NoteOff { note: _ } => {
                self.note_on = false;
            }
            Event::PitchBend { ratio: _ } => {
                self.osc.trigger(event);
            }
        }
    }
}

impl Parametric<Parameter> for PiSynth {
    fn set_param(&mut self, param: &Parameter, value: f64) {
        match param {
            Parameter::Detune => {
                self.detune = value as i16;
                let ratio = ratio_from_cents(self.detune);
                self.trigger(&Event::PitchBend { ratio: ratio });
            }
        }
    }

    fn get_param(&self, param: &Parameter) -> f64 {
        match param {
            Parameter::Detune => self.detune as f64,
        }
    }
}

impl AudioProcessor<Signal> for PiSynth {
    fn process(&mut self, sample_rate: f64) -> Signal {
        let osc = self.osc.process(sample_rate);
        let v = if self.note_on { 0.3 * osc } else { 0.0 };

        (v, v)
    }
}
