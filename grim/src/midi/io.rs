use midly::{Format as MidiFormat, Header as MidiHeader, Smf, Timing as MidiTiming, Track, TrackEvent, TrackEventKind};
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