use std::time::Duration;

use rodio::{Sink, Source};

use crate::oscillator::{self, SAMPLE_RATE};

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
    Single { hz: f32, volume: f32 },
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
                    NoteType::Single { hz, volume } => {
                        let mut source = oscillator::SawWave::new(hz, 0.1 * volume)
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
}
