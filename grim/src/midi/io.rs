use midly::{Format as MidiFormat, Header as MidiHeader, MetaMessage, MidiMessage, Smf, Timing as MidiTiming, Track, TrackEvent, TrackEventKind};
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

        // Parse tempo track
        if let Some(tempo_track) = smf.tracks.get(0) {
            let mut abs_pos = 0;

            for ev in tempo_track.iter() {
                abs_pos += ev.delta.as_int() as u64;

                match ev.kind {
                    TrackEventKind::Meta(MetaMessage::Tempo(tempo)) => {
                        mid.tempo.push(MidiTempo {
                            pos: abs_pos,
                            pos_realtime: None,
                            mpq: tempo.as_int()
                        });
                    },
                    TrackEventKind::Meta(MetaMessage::TimeSignature(num, dem, clocks_per_click, notes_per_quarter_32)) => {
                        // TODO: Save time sig changes
                    },
                    _ => continue
                }
            }
        }

        // TODO: Parse tempo track individually. Maybe use accumulator too.
        for track in smf.tracks.iter().skip(1) {
            let mut abs_pos = 0;
            let mut mid_track_events = Vec::new();
            let mut track_name = None;

            // TODO: Parse track name event
            for ev in track.iter() {
                abs_pos += ev.delta.as_int() as u64;

                let mut pending_notes: [Option<MidiNote>; 0x80] = [(); 0x80].map(|_| None);

                match &ev.kind {
                    TrackEventKind::Meta(meta) => match meta {
                        MetaMessage::TrackName(raw_track_name) => {
                            // Parse track name and assign
                            if let Ok(name) = String::from_utf8(raw_track_name.to_vec()) {
                                track_name = Some(name);
                            }
                        },
                        MetaMessage::Lyric(lyric) => {
                            mid_track_events.push(MidiEvent::Meta(MidiText {
                                pos: abs_pos,
                                pos_realtime: None,
                                text: MidiTextType::Lyric(lyric.to_vec().into_boxed_slice())
                            }));
                        },
                        MetaMessage::Text(text) => {
                            mid_track_events.push(MidiEvent::Meta(MidiText {
                                pos: abs_pos,
                                pos_realtime: None,
                                text: MidiTextType::Event(text.to_vec().into_boxed_slice())
                            }));
                        },
                        _ => {}
                    },
                    TrackEventKind::Midi { channel, message: MidiMessage::NoteOn { key, vel } } => {
                        let index = key.as_int() as usize;

                        // If note exists, ignore
                        if let Some(mut note) = pending_notes[index].take() {
                            if vel.as_int() == 0 {
                                // Treat as off note
                                let length = abs_pos - note.pos;
                                if length == 0 {
                                    continue;
                                }

                                // Edit length and add note
                                note.length = length;
                                mid_track_events.push(MidiEvent::Note(note));
                            } else {
                                // Restore
                                pending_notes[index] = Some(note);
                            }
                            continue;
                        }

                        // Otherwise add note
                        pending_notes[index] = Some(MidiNote {
                            pos: abs_pos,
                            length: 0,
                            pitch: key.as_int(),
                            channel: channel.as_int(),
                            velocity: vel.as_int() as u8,
                            ..Default::default()
                        });
                    },
                    TrackEventKind::Midi { channel: _, message: MidiMessage::NoteOff { key, vel: _ } } => {
                        let index = key.as_int() as usize;

                        if let Some(mut note) = pending_notes[index].take() {
                            let length = abs_pos - note.pos;
                            if length == 0 {
                                continue;
                            }

                            // Edit length and add note
                            note.length = length;
                            mid_track_events.push(MidiEvent::Note(note));
                        }
                    },
                    TrackEventKind::SysEx(sysex) => {
                        mid_track_events.push(MidiEvent::SysEx(MidiSysex {
                            pos: abs_pos,
                            pos_realtime: None,
                            data: sysex.to_vec().into_boxed_slice()
                        }));
                    },
                    _ => {}
                }

                // Add remaining notes
                for mut note in pending_notes {
                    if let Some(mut note) = note.take() {
                        let length = abs_pos - note.pos;
                        if length == 0 {
                            continue;
                        }

                        // Edit length and add note
                        note.length = length;
                        mid_track_events.push(MidiEvent::Note(note));
                    }
                }
            }

            mid.tracks.push(MidiTrack {
                name: track_name,
                events: mid_track_events
            });
        }

        // Update realtime offsets
        mid.calculate_realtime_pos();

        Some(mid)
    }

    pub fn write_to_file<T: AsRef<Path>>(&self, path: T) {
        let path = path.as_ref();

        let format = match self.format {
            0 => MidiFormat::SingleTrack,
            1 => MidiFormat::Parallel,
            _ => MidiFormat::Sequential
        };

        let header = MidiHeader::new(format, MidiTiming::Metrical(self.ticks_per_quarter.into()));
        let mut smf = Smf::new(header);

        // Add tempo track
        // TODO: Iterate and add tempo changes
        smf.tracks.push(
            vec![
                TrackEvent {
                    delta: 0.into(),
                    kind: TrackEventKind::Meta(midly::MetaMessage::TrackName(b"tempo"))
                },
                TrackEvent {
                    delta: 0.into(),
                    kind: TrackEventKind::Meta(midly::MetaMessage::EndOfTrack)
                }
            ]
        );

        // Write tracks
        for track in &self.tracks {
            let mut events = Vec::new();

            // Add track name
            if let Some(track_name) = track.name.as_ref().map(|n| n.as_bytes()) {
                events.push(TrackEvent {
                    delta: 0.into(),
                    kind: TrackEventKind::Meta(midly::MetaMessage::TrackName(track_name))
                });
            }

            // TODO: Implement writing midi notes

            // Add end event
            events.push(TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(midly::MetaMessage::EndOfTrack)
            });

            smf.tracks.push(events);
        }

        // Write midi file
        let file = std::fs::File::create(path).unwrap();
        midly::write_std(&smf.header, &smf.tracks, file).unwrap();
    }
}