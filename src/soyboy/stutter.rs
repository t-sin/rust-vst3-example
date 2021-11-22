use crate::soyboy::{
    event::{Event, Triggered},
    parameters::{Parameter, Parametric},
};

pub struct NoteStutter {
    time_count: f64,
    note: u16,
    velocity: f64,

    time: f64,
    depth: f64,
}

impl NoteStutter {
    pub fn new() -> Self {
        NoteStutter {
            time_count: 0.0,
            note: 0,
            velocity: 0.0,

            time: 0.07,
            depth: 0.0,
        }
    }

    pub fn trigger(&mut self, event: &Event, triggered: &mut [&mut dyn Triggered]) {
        match event {
            Event::NoteOn { note, velocity } => {
                self.note = *note;
                self.velocity = *velocity;

                for t in triggered.iter_mut() {
                    t.trigger(&Event::NoteOn {
                        note: self.note,
                        velocity: self.velocity,
                    });
                }
            }
            Event::NoteOff { note } => {
                self.velocity = 0.0;
                for t in triggered.iter_mut() {
                    t.trigger(&Event::NoteOff { note: *note });
                }
            }
            Event::PitchBend { ratio: _ } => {}
        }
    }

    pub fn process(&mut self, sample_rate: f64, triggered: &mut [&mut dyn Triggered]) -> () {
        self.time_count += 1.0 / sample_rate;

        if self.depth != 0.0 && self.time_count > self.time {
            self.time_count = 0.0;
            self.velocity -= 1.0 - self.depth;

            if self.velocity < 0.0 {
                self.velocity = 0.0;
                self.trigger(&Event::NoteOff { note: self.note }, triggered)
            } else {
                self.trigger(
                    &Event::NoteOn {
                        note: self.note,
                        velocity: self.velocity,
                    },
                    triggered,
                );
            }
        }
    }
}

impl Parametric<Parameter> for NoteStutter {
    fn set_param(&mut self, param: &Parameter, value: f64) {
        match param {
            Parameter::StutterTime => self.time = value,
            Parameter::StutterDepth => self.depth = value,
            _ => (),
        }
    }

    fn get_param(&self, param: &Parameter) -> f64 {
        match param {
            Parameter::StutterTime => self.time,
            Parameter::StutterDepth => self.depth,
            _ => 0.0,
        }
    }
}