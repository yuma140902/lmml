use std::fmt::Display;

use crate::timeline::{Element, Event, LmmlTimeline, Note, NoteType};

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
    Chord {
        notes: Vec<(NoteChar, NoteModifier)>,
        length: Option<u32>,
        is_dotted: bool,
    },
    NoteNumber(u32),
    SetOctave(u32),
    SetLength(u32, bool),
    SetVolume(u32),
    SetTempo(u32),
    SetWaveform(u32),
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

fn resolve_length(l_cmd_num: u32, l_cmd_dot: bool, num: Option<u32>, dot: bool) -> (u32, bool) {
    let m = l_cmd_num;
    match (l_cmd_dot, num, dot) {
        (false, None, d) => (m, d),
        (false, Some(n), d) => (n, d),
        (true, None, _) => (m, true),
        (true, Some(n), d) => (n, d),
    }
}

fn length_to_ms(tempo: u32, (length, is_dotted): (u32, bool)) -> u32 {
    let length = length as f32;
    let dot = if is_dotted { 1.5 } else { 1.0 };
    ((4.0 / length * 60.0 / tempo as f32 * 1000.0) * dot) as u32
}

#[derive(Debug, Clone)]
pub struct EvalEnv {
    octave: i32,
    length: u32,
    is_dotted: bool,
    tempo: u32,
    volume: u32,
    waveform: u32,
}

impl Display for EvalEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "@{} v{} t{} l{}{} o{}",
            self.waveform,
            self.volume,
            self.tempo,
            self.length,
            if self.is_dotted { "." } else { "" },
            self.octave
        )
    }
}

impl Default for EvalEnv {
    fn default() -> Self {
        Self {
            octave: 4,
            length: 4,
            is_dotted: false,
            tempo: 120,
            volume: 20,
            waveform: 0,
        }
    }
}

impl LmmlAst {
    pub fn to_timeline(&self, env: &mut EvalEnv) -> LmmlTimeline {
        let mut elements = Vec::new();

        elements.push(Element::Event(Event::ChangeTempo(env.tempo)));
        for command in self.0.iter() {
            match command {
                LmmlCommand::Note {
                    note,
                    modifier,
                    length: l,
                    is_dotted,
                } => elements.push(Element::Note(Note {
                    note_type: NoteType::Single {
                        hz: note.to_hz(*modifier, env.octave),
                        volume: env.volume as f32,
                        waveform: env.waveform,
                    },
                    length_ms: length_to_ms(
                        env.tempo,
                        resolve_length(env.length, env.is_dotted, *l, *is_dotted),
                    ),
                })),
                LmmlCommand::Rest {
                    length: l,
                    is_dotted,
                } => elements.push(Element::Note(Note {
                    note_type: NoteType::Rest,
                    length_ms: length_to_ms(
                        env.tempo,
                        resolve_length(env.length, env.is_dotted, *l, *is_dotted),
                    ),
                })),
                LmmlCommand::Chord {
                    notes,
                    length: l,
                    is_dotted,
                } => {
                    let mut notenumbers = notes
                        .iter()
                        .map(|(n, m)| n.to_notenumber(*m, env.octave))
                        .collect::<Vec<_>>();
                    for i in 1..notenumbers.len() {
                        while notenumbers[i - 1] >= notenumbers[i] {
                            notenumbers[i] += 12;
                        }
                    }
                    let hzs = notenumbers.iter().map(|n| notenumber_to_hz(*n)).collect();
                    elements.push(Element::Note(Note {
                        note_type: NoteType::Chord {
                            hzs,
                            volume: env.volume as f32,
                            waveform: env.waveform,
                        },
                        length_ms: length_to_ms(
                            env.tempo,
                            resolve_length(env.length, env.is_dotted, *l, *is_dotted),
                        ),
                    }))
                }
                LmmlCommand::NoteNumber(n) => {
                    elements.push(Element::Note(Note {
                        note_type: NoteType::Single {
                            hz: notenumber_to_hz(*n as i32),
                            volume: env.volume as f32,
                            waveform: env.waveform,
                        },
                        length_ms: length_to_ms(env.tempo, (env.length, env.is_dotted)),
                    }));
                }
                LmmlCommand::SetOctave(o) => env.octave = *o as i32,
                LmmlCommand::SetLength(l, d) => {
                    env.length = *l;
                    env.is_dotted = *d;
                }
                LmmlCommand::SetVolume(v) => env.volume = *v,
                LmmlCommand::SetTempo(t) => {
                    env.tempo = *t;
                    elements.push(Element::Event(Event::ChangeTempo(*t)));
                }
                LmmlCommand::SetWaveform(n) => env.waveform = *n,
                LmmlCommand::IncreaseOctave => env.octave += 1,
                LmmlCommand::DecreaseOctave => env.octave -= 1,
            }
        }

        LmmlTimeline { timeline: elements }
    }
}

impl NoteChar {
    pub fn to_hz(&self, modifier: NoteModifier, octave: i32) -> f32 {
        let notenumber = self.to_notenumber(modifier, octave);
        notenumber_to_hz(notenumber)
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

pub fn notenumber_to_hz(notenumber: i32) -> f32 {
    440.0 * 2.0_f32.powf((notenumber - 69) as f32 / 12.0)
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
