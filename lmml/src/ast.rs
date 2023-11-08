#[derive(Debug, PartialEq, Clone)]
pub enum LmmlAst {
    Note {
        note: NoteChar,
        modifier: NoteModifier,
        length: Option<u32>,
        is_dotted: bool,
    },
    SetOctave(u32),
    SetLength(u32),
    SetVolume(u32),
    SetTempo(u32),
    IncreaseOctave,
    DecreaseOctave,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NoteChar {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NoteModifier {
    Sharp,
    Flat,
    Natural,
}
