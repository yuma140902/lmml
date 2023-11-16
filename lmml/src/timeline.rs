use std::{fmt::Display, time::Duration};

use rodio::{Sink, Source};

use crate::oscillator::{self, SawWave, SineWave, SquareWave, TriangleWave, SAMPLE_RATE};

#[derive(Debug, PartialEq, Clone)]
pub struct LmmlTimeline {
    pub timeline: Vec<Element>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Note(Note),
    Event(Event),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub length_ms: u32,
    pub note_type: NoteType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NoteType {
    Single { hz: f32, volume: f32, waveform: u32 },
    Rest,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    ChangeTempo(u32),
}

impl LmmlTimeline {
    pub fn play(&self, sink: &Sink) {
        for element in self.timeline.iter() {
            match element {
                Element::Note(note) => match note.note_type {
                    NoteType::Single {
                        hz,
                        volume,
                        waveform,
                    } => {
                        if waveform == 1 {
                            let mut source = SquareWave::new(hz, 0.01 * volume, 0.5)
                                .take_duration(Duration::from_millis(note.length_ms as u64));
                            source.set_filter_fadeout();
                            sink.append(source);
                        } else if waveform == 2 {
                            let mut source = SquareWave::new(hz, 0.01 * volume, 0.1)
                                .take_duration(Duration::from_millis(note.length_ms as u64));
                            source.set_filter_fadeout();
                            sink.append(source);
                        } else if waveform == 3 {
                            let mut source = TriangleWave::new(hz, 0.01 * volume)
                                .take_duration(Duration::from_millis(note.length_ms as u64));
                            source.set_filter_fadeout();
                            sink.append(source);
                        } else if waveform == 4 {
                            let mut source = SineWave::new(hz, 0.01 * volume)
                                .take_duration(Duration::from_millis(note.length_ms as u64));
                            source.set_filter_fadeout();
                            sink.append(source);
                        } else {
                            let mut source = SawWave::new(hz, 0.01 * volume)
                                .take_duration(Duration::from_millis(note.length_ms as u64));
                            source.set_filter_fadeout();
                            sink.append(source);
                        }
                    }
                    NoteType::Rest => {
                        let source = rodio::source::Zero::<f32>::new(1, SAMPLE_RATE)
                            .take_duration(Duration::from_millis(note.length_ms as u64));
                        sink.append(source);
                    }
                },
                Element::Event(event) => match event {
                    Event::ChangeTempo(_) => { /* do nothing */ }
                },
            }
        }
    }
}

impl Display for LmmlTimeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.timeline.iter() {
            match element {
                Element::Note(note) => match note.note_type {
                    NoteType::Single {
                        hz,
                        volume,
                        waveform,
                    } => {
                        write!(
                            f,
                            "Note: {} Hz, {} ms, volume {}, waveform {}",
                            hz, note.length_ms, volume, waveform
                        )?;
                    }
                    NoteType::Rest => {
                        write!(f, "Rest: {} ms", note.length_ms)?;
                    }
                },
                Element::Event(event) => match event {
                    Event::ChangeTempo(tempo) => {
                        write!(f, "Event ChangeTempo: {}", tempo)?;
                    }
                },
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
