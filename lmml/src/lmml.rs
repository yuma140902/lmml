pub struct Lmml {
    pub timeline: Vec<Element>,
}

pub enum Element {
    Note(Note),
    Event(Event),
}

pub enum Note {
    Single(SingleNote),
    Rest(Rest),
}

pub struct SingleNote {
    pub length_ms: u32,
    pub hz: f64,
    pub volume: u32,
}

pub struct Rest {
    pub length_ms: u32,
}

pub enum Event {
    Tempo(u32),
    Tone(Tone),
}

pub enum Tone {
    Sine,
    Square,
    Saw,
    Triangle,
}
