use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::pi::utils;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Parameter {
    Detune = 0,
}

impl TryFrom<u32> for Parameter {
    type Error = ();

    fn try_from(id: u32) -> Result<Self, Self::Error> {
        if id == Parameter::Detune as u32 {
            Ok(Parameter::Detune)
        } else {
            Err(())
        }
    }
}

pub struct ParamIter(u32);

impl ParamIter {
    pub fn new() -> ParamIter {
        ParamIter(Parameter::Detune as u32)
    }
}

impl Iterator for ParamIter {
    type Item = Parameter;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(p) = Parameter::try_from(self.0) {
            self.0 += 1;
            Some(p)
        } else {
            None
        }
    }
}

impl Parameter {
    pub fn iter() -> ParamIter {
        ParamIter::new()
    }
}

pub trait Parametric<Parameter> {
    fn set_param(&mut self, param: &Parameter, value: f64);
    fn get_param(&self, param: &Parameter) -> f64;
}

#[derive(Clone, Copy, Debug)]
pub enum ParameterType {
    Integer,
}

pub trait Normalizable<T> {
    fn denormalize(&self, normalized: f64) -> T;
    fn normalize(&self, plain: T) -> f64;
    fn format(&self, normalized: f64) -> String;
    fn parse(&self, string: &str) -> Option<f64>;
}

#[derive(Clone, Copy)]
pub struct IntegerParameter {
    min: i32,
    max: i32,
}

impl Normalizable<f64> for IntegerParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        utils::linear_denormalize(normalized, self.min as f64, self.max as f64) as i64 as f64
    }

    fn normalize(&self, plain: f64) -> f64 {
        utils::linear_normalize(plain, self.min as f64, self.max as f64)
    }

    fn format(&self, normalized: f64) -> String {
        format!("{:.2}", self.denormalize(normalized))
    }

    fn parse(&self, string: &str) -> Option<f64> {
        if let Some(vs) = string.split(' ').nth(0) {
            if let Ok(v) = vs.parse() {
                Some(self.normalize(v))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct ListParameter {
    elements: &'static [&'static str],
}

impl Normalizable<f64> for ListParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        normalized * (self.elements.len() - 1) as f64
    }

    fn normalize(&self, plain: f64) -> f64 {
        plain / (self.elements.len() - 1) as f64
    }

    fn format(&self, normalized: f64) -> String {
        if let Some(s) = self.elements.get(self.denormalize(normalized) as usize) {
            format!("{}", s)
        } else {
            "".to_string()
        }
    }

    fn parse(&self, string: &str) -> Option<f64> {
        if let Ok(v) = self.elements.binary_search(&string) {
            Some(self.normalize(v as f64))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub union ParameterInfo {
    pub int: IntegerParameter,
}

#[derive(Clone)]
pub struct PiParameter {
    pub r#type: ParameterType,
    pub parameter: ParameterInfo,
    pub title: String,
    pub short_title: String,
    pub unit_name: String,
    pub step_count: i32,
    pub default_value: f64,
}

impl Normalizable<f64> for PiParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        match self.r#type {
            ParameterType::Integer => unsafe { self.parameter.int.denormalize(normalized) },
        }
    }

    fn normalize(&self, plain: f64) -> f64 {
        match self.r#type {
            ParameterType::Integer => unsafe { self.parameter.int.normalize(plain) },
        }
    }

    fn format(&self, normalized: f64) -> String {
        let s = match self.r#type {
            ParameterType::Integer => unsafe { self.parameter.int.format(normalized) },
        };
        format!("{} {}", s, self.unit_name)
    }

    fn parse(&self, string: &str) -> Option<f64> {
        match self.r#type {
            ParameterType::Integer => unsafe { self.parameter.int.parse(string) },
        }
    }
}

pub fn make_parameter_info() -> HashMap<Parameter, PiParameter> {
    let mut params = HashMap::new();

    static GLOBAL_DETUNE: IntegerParameter = IntegerParameter {
        min: -200,
        max: 200,
    };
    params.insert(
        Parameter::Detune,
        PiParameter {
            r#type: ParameterType::Integer,
            parameter: ParameterInfo { int: GLOBAL_DETUNE },
            title: "Detune".to_string(),
            short_title: "Detune".to_string(),
            unit_name: "cent".to_string(),
            step_count: GLOBAL_DETUNE.max.abs() + GLOBAL_DETUNE.min.abs(),
            default_value: 0.0,
        },
    );

    params
}
