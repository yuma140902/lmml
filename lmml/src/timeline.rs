use std::{fmt::Display, time::Duration};

use rodio::{Sink, Source};

use crate::oscillator::{MixedWave, Wave, Waveform, SAMPLE_RATE};

#[derive(Debug, PartialEq, Clone)]
pub struct LmmlTimeline {
    pub timeline: [Vec<Element>; 16],
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
    Single {
        hz: f32,
        volume: f32,
        waveform: u32,
    },
    Chord {
        hzs: Vec<f32>,
        volume: f32,
        waveform: u32,
    },
    Rest,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    ChangeTempo(u32),
}

impl LmmlTimeline {
    pub fn play_channel(&self, i: usize, sink: &Sink) {
        for element in self.timeline[i].iter() {
            match element {
                Element::Note(note) => match note.note_type {
                    NoteType::Single {
                        hz,
                        volume,
                        waveform,
                    } => {
                        let waveform = match waveform {
                            1 => Waveform::Square(0.5),
                            2 => Waveform::Square(0.1),
                            3 => Waveform::Triangle,
                            4 => Waveform::Sine,
                            _ => Waveform::Saw,
                        };

                        let mut source = Wave::new(waveform, hz, 0.01 * volume)
                            .take_duration(Duration::from_millis(note.length_ms as u64));
                        source.set_filter_fadeout();
                        sink.append(source);
                    }
                    NoteType::Chord {
                        ref hzs,
                        volume,
                        waveform,
                    } => {
                        let waveform = match waveform {
                            1 => Waveform::Square(0.5),
                            2 => Waveform::Square(0.1),
                            3 => Waveform::Triangle,
                            4 => Waveform::Sine,
                            _ => Waveform::Saw,
                        };
                        let mut source = MixedWave::new(
                            hzs.iter()
                                .map(|hz| Wave::new(waveform, *hz, 0.01 * volume))
                                .collect(),
                        )
                        .take_duration(Duration::from_millis(note.length_ms as u64));
                        source.set_filter_fadeout();
                        sink.append(source);
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

    pub fn play(&self, sink: &[Sink; 16]) {
        for i in 0..16 {
            self.play_channel(i, &sink[i]);
        }
    }

    fn fmt_channel(&self, i: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("--- CH{} ---", i);
        for element in self.timeline[i].iter() {
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
                    NoteType::Chord {
                        ref hzs,
                        volume,
                        waveform,
                    } => {
                        write!(f, "Chord: [")?;
                        for hz in hzs {
                            write!(f, "{} ", hz)?;
                        }
                        write!(
                            f,
                            "] Hz, {} ms, volume {}, waveform {}",
                            note.length_ms, volume, waveform
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

impl Display for LmmlTimeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..16 {
            self.fmt_channel(i, f)?;
        }
        Ok(())
    }
}
