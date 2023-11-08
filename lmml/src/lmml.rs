pub struct Lmml {
    pub timeline: Vec<Element>,
}

pub enum Element {
    Note(Note),
    Event(Event),
}

pub struct Note {
    pub length_ms: u32,
    pub note_type: NoteType,
}

pub enum NoteType {
    Single { hz: f64, volume: u32 },
    Rest,
}

pub enum Event {
    ChangeTempo(u32),
}
