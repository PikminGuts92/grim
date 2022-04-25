mod io;

pub use self::io::*;

#[derive(Default)]
pub struct MidiFile {
    pub format: u16,
    pub ticks_per_quarter: u16,
    pub tracks: Vec<MidiTrack>,
    pub tempo: Vec<MidiTempo>,
}

pub struct MidiTempo {
    pub pos: u64,
    pub pos_realtime: f64, // Milliseconds
    pub mpq: u32,
    pub bpm: f64,
}

pub struct MidiTrack {
    pub name: Option<String>,
    pub notes: Vec<MidiNote>,
    pub texts: Vec<MidiText>,
}

pub enum MidiTextType {
    Event(String),
    Lyric(String)
}

pub struct MidiText {
    pub pos: u64,
    pub text: MidiTextType,
}

impl MidiText {
    pub fn is_lyric(&self) -> bool {
        match self.text {
            MidiTextType::Lyric(_) => true,
            _ => false,
        }
    }

    pub fn get_text<'a>(&'a self) -> &'a String {
        match &self.text {
            MidiTextType::Lyric(text) => text,
            MidiTextType::Event(text) => text,
        }
    }
}

pub struct MidiNote {
    pub pos: u64,
    pub pos_realtime: f64, // Milliseconds
    pub length: u64,
    pub length_realtime: f64, // Milliseconds
    pub pitch: u8,
    pub channel: u8,
    pub velocity: u8
}