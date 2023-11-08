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
    Single { hz: f64, volume: f32 },
    Rest,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    ChangeTempo(u32),
}
