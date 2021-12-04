const NOTE_NUMBER_OF_440_HZ: i16 = 69;

/// https://steinbergmedia.github.io/vst3_doc/vstinterfaces/structSteinberg_1_1Vst_1_1NoteOnEvent.html の pitch の項目
pub fn frequency_from_note_number(note_num: u16) -> f64 {
    440.0 * 2.0_f64.powf((note_num as i16 - NOTE_NUMBER_OF_440_HZ) as f64 / 12.0)
}

pub fn ratio_from_cents(cents: i16) -> f64 {
    2.0f64.powf(cents as f64 / 1200.0)
}

pub fn linear_denormalize(v: f64, min: f64, max: f64) -> f64 {
    let range = max.abs() + min.abs();
    v * range + min
}

pub fn linear_normalize(x: f64, min: f64, max: f64) -> f64 {
    let range = max.abs() + min.abs();
    (x - min) / range
}
