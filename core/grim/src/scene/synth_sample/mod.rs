mod io;

pub use io::*;
use grim_macros::*;
use grim_traits::scene::*;

#[derive(Default)]
pub struct SampleData {
    pub encoding: i32,
    pub sample_count: i32,
    pub sample_rate: i32,
    pub unknown: bool,
    pub data: Vec<u8>
}

#[milo]
pub struct SynthSample {
    pub file: String,
    pub looped: bool,
    pub loop_start_sample: i32,
    pub loop_end_sample: i32,
    pub sample_data: SampleData,
}

impl Default for SynthSample {
    fn default() -> SynthSample {
        SynthSample {
            // Base object
            name: String::default(),
            type2: String::default(),
            note: String::default(),

            // SynthSample object
            file: String::default(),
            looped: false,
            loop_start_sample: 0,
            loop_end_sample: -1,
            sample_data: SampleData::default()
        }
    }
}