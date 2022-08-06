mod io;
mod reader;

pub use self::io::*;
pub use self::reader::*;

pub struct MidiFile {
    pub format: u16,
    pub ticks_per_quarter: u16,
    pub tracks: Vec<MidiTrack>,
    pub tempo: Vec<MidiTempo>,
}

impl Default for MidiFile {
    fn default() -> Self {
        Self {
            format: 1,
            ticks_per_quarter: 480,
            tracks: Vec::new(),
            tempo: Vec::new()
        }
    }
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

pub struct MidiInfo {
    pub format: u16,
    pub ticks_per_quarter: u16, // Usually 480
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