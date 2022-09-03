mod io;
mod reader;

pub use self::io::*;
pub use self::reader::*;

pub struct MidiFile {
    pub format: u16,
    pub ticks_per_quarter: u16,
    pub tracks: Vec<MidiTrack>,
    pub tempo: Vec<MidiTempo>
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
    pub name: Option<String>, // Matches meta track name
    pub events: Vec<MidiEvent>,
}

pub struct MidiInfo {
    pub format: u16,
    pub ticks_per_quarter: u16, // Usually 480
}

#[derive(Clone, Debug)]
pub enum MidiEvent {
    Note(MidiNote),
    Meta(MidiText),
    SysEx(MidiSysex)
}

impl MidiEvent {
    pub fn get_pos(&self) -> u64 {
        match self {
            MidiEvent::Note(note) => note.pos,
            MidiEvent::Meta(meta) => meta.pos,
            MidiEvent::SysEx(sys) => sys.pos
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MidiNote {
    pub pos: u64,
    pub pos_realtime: Option<f64>, // Milliseconds
    pub length: u64,
    pub length_realtime: Option<f64>, // Milliseconds
    pub pitch: u8,
    pub channel: u8,
    pub velocity: u8
}

#[derive(Clone, Debug)]
pub struct MidiText {
    pub pos: u64,
    pub pos_realtime: Option<f64>, // Milliseconds
    pub text: MidiTextType,
}

#[derive(Clone, Debug)]
pub enum MidiTextType {
    Event(Box<[u8]>),
    Lyric(Box<[u8]>)
}

impl MidiText {
    pub fn is_lyric(&self) -> bool {
        match self.text {
            MidiTextType::Lyric(_) => true,
            _ => false,
        }
    }

    pub fn as_str<'a>(&'a self) -> Option<&'a str> {
        match &self.text {
            MidiTextType::Lyric(text) => std::str::from_utf8(text).ok(),
            MidiTextType::Event(text) => std::str::from_utf8(text).ok(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MidiSysex {
    pub pos: u64,
    pub pos_realtime: Option<f64>, // Milliseconds
    pub data: Box<[u8]>
}