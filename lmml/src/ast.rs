use crate::timeline::{Element, LmmlTimeline, Note, NoteType};

#[derive(Debug, PartialEq, Clone)]
pub struct LmmlAst(pub Vec<LmmlCommand>);

#[derive(Debug, PartialEq, Clone)]
pub enum LmmlCommand {
    Note {
        note: NoteChar,
        modifier: NoteModifier,
        length: Option<u32>,
        is_dotted: bool,
    },
    Rest {
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

impl LmmlAst {
    pub fn to_timeline(&self) -> LmmlTimeline {
        let mut elements = Vec::new();
        let mut octave = 4;
        let mut length = 4;
        let mut tempo = 120;
        let mut volume = 10;

        for command in self.0.iter() {
            match command {
                LmmlCommand::Note {
                    note,
                    modifier,
                    length: l,
                    is_dotted,
                } => elements.push(Element::Note(Note {
                    note_type: NoteType::Single {
                        hz: note.to_hz(*modifier, octave),
                        volume: volume as f32,
                    },
                    length_ms: length_to_ms(tempo, l.unwrap_or(length), *is_dotted),
                })),
                LmmlCommand::Rest {
                    length: l,
                    is_dotted,
                } => elements.push(Element::Note(Note {
                    note_type: NoteType::Rest,
                    length_ms: length_to_ms(tempo, l.unwrap_or(length), *is_dotted),
                })),
                LmmlCommand::SetOctave(o) => octave = *o as i32,
                LmmlCommand::SetLength(l) => length = *l,
                LmmlCommand::SetVolume(v) => volume = *v,
                LmmlCommand::SetTempo(t) => tempo = *t,
                LmmlCommand::IncreaseOctave => octave += 1,
                LmmlCommand::DecreaseOctave => octave -= 1,
            }
        }

        LmmlTimeline { timeline: elements }
    }
}

fn length_to_ms(tempo: u32, length: u32, is_dotted: bool) -> u32 {
    let length = length as f32;
    let length = if is_dotted { length * 1.5 } else { length };
    (4.0 / length * 60.0 / tempo as f32 * 1000.0) as u32
}

impl NoteChar {
    pub fn to_hz(&self, modifier: NoteModifier, octave: i32) -> f32 {
        let notenumber = self.to_notenumber(modifier, octave);
        440.0 * 2.0_f32.powf((notenumber - 69) as f32 / 12.0)
    }

    pub fn to_notenumber(&self, modifier: NoteModifier, octave: i32) -> i32 {
        let base: i32 = match self {
            NoteChar::C => 0,
            NoteChar::D => 2,
            NoteChar::E => 4,
            NoteChar::F => 5,
            NoteChar::G => 7,
            NoteChar::A => 9,
            NoteChar::B => 11,
        };
        let modifier = match modifier {
            NoteModifier::Sharp => 1,
            NoteModifier::Flat => -1,
            NoteModifier::Natural => 0,
        };
        base + modifier + ((octave + 1) * 12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_notenumber() {
        assert_eq!(NoteChar::C.to_notenumber(NoteModifier::Natural, 4), 60);
        assert_eq!(NoteChar::C.to_notenumber(NoteModifier::Sharp, 4), 61);
        assert_eq!(NoteChar::C.to_notenumber(NoteModifier::Natural, -1), 0);
    }
}
