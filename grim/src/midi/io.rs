use midly::{Format as MidiFormat, Smf, Timing as MidiTiming};
use std::fs;
use std::path::Path;
use super::*;

impl MidiFile {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Option<MidiFile> {
        // TODO: Use result w/ custom error
        let path = path.as_ref();
        let mut mid = MidiFile::default();

        // Load midi file using lib
        let mid_bytes = fs::read(path).ok()?;
        let smf = Smf::parse(&mid_bytes).ok()?;

        // TODO: Don't bother re-mapping and just re-export enum
        mid.format = match &smf.header.format {
            MidiFormat::SingleTrack => 0,
            MidiFormat::Parallel => 1,
            MidiFormat::Sequential => 2
        };

        mid.ticks_per_quarter = match &smf.header.timing {
            MidiTiming::Metrical(time) => time.as_int(),
            MidiTiming::Timecode(_, _) => panic!("\"Timecode\" not supported for reading in midi file"),
        };

        Some(mid)
    }
}