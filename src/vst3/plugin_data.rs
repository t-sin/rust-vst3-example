pub const VST3_VENDOR: &str = "t-sin";
pub const VST3_VERSION: &str = "0.1.0";
pub const VST3_URL: &str = "https://github.com/t-sin/rust-vst3-example/";
pub const VST3_EMAIL: &str = "shinichi.tanaka45@gmail.com";

pub const VST3_CLASS_NAME: &str = "Pi Synth";
pub const VST3_CLASS_CATEGORY: &str = "Audio Module Class";
pub const VST3_CLASS_SUBCATEGORIES: &str = "Instrument|Synth";

pub const VST3_CID: [u8; 16] = [
    0xa4, 0x13, 0x81, 0x7a, 0x8d, 0x44, 0x40, 0x98, 0x92, 0xdd, 0x72, 0xaf, 0x62, 0x0c, 0xc7, 0x41,
];

pub const VST3_CONTROLLER_CLASS_NAME: &str = "PiSynth Controller";
pub const VST3_CONTROLLER_CLASS_CATEGORY: &str = "Component Controller Class";
pub const VST3_CONTROLLER_CLASS_SUBCATEGORIES: &str = "";

pub const VST3_CONTROLLER_CID: [u8; 16] = [
    0xef, 0x43, 0xad, 0xbe, 0x5e, 0x22, 0x49, 0x8d, 0x81, 0x4c, 0x96, 0x2f, 0x6f, 0x8b, 0xf2, 0xd6,
];
