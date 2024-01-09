use std::f32::consts::PI;

use rodio::{
    source::{TakeDuration, Zero},
    Source,
};

pub const SAMPLE_RATE: u32 = 44100;

#[derive(Debug, Clone, Copy)]
pub enum Waveform {
    Zero,
    Saw,
    Square(f32),
    Triangle,
    Sine,
}

pub struct NoteWave {
    frame: usize,
    waveform: Waveform,
    frequency: f32,
    amplitude: f32,
}

impl NoteWave {
    pub fn new(waveform: Waveform, frequency: f32, amplitude: f32) -> Self {
        Self {
            frame: 0,
            waveform,
            frequency,
            amplitude,
        }
    }
}

impl Source for NoteWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for NoteWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.waveform {
            Waveform::Zero => Some(0.0),
            Waveform::Saw => {
                self.frame += 1;
                if self.frame > (SAMPLE_RATE as f32 / self.frequency) as usize {
                    self.frame = 0;
                }
                Some(
                    (self.frame as f32 / (SAMPLE_RATE as f32 / self.frequency) - 0.5)
                        * 2.0
                        * self.amplitude,
                )
            }
            Waveform::Square(pulse_width) => {
                self.frame += 1;
                if self.frame > (SAMPLE_RATE as f32 / self.frequency) as usize {
                    self.frame = 0;
                }
                if self.frame < ((SAMPLE_RATE as f32 / self.frequency) * pulse_width) as usize {
                    Some(self.amplitude)
                } else {
                    Some(-self.amplitude)
                }
            }
            Waveform::Triangle => {
                self.frame += 1;
                if self.frame > (SAMPLE_RATE as f32 / self.frequency) as usize {
                    self.frame = 0;
                }
                if self.frame < ((SAMPLE_RATE as f32 / self.frequency) * 0.5) as usize {
                    Some(
                        (2.0 * self.frame as f32 / (SAMPLE_RATE as f32 / self.frequency) - 0.5)
                            * 2.0
                            * self.amplitude,
                    )
                } else {
                    Some(
                        (1.5 - 2.0 * self.frame as f32 / (SAMPLE_RATE as f32 / self.frequency))
                            * 2.0
                            * self.amplitude,
                    )
                }
            }
            Waveform::Sine => {
                self.frame = self.frame.wrapping_add(1);
                let value = 2.0 * PI * self.frequency * self.frame as f32 / SAMPLE_RATE as f32;
                Some(self.amplitude * value.sin())
            }
        }
    }
}

pub struct ChordWave(Vec<NoteWave>);

impl ChordWave {
    pub fn new(waves: Vec<NoteWave>) -> Self {
        Self(waves)
    }
}

impl Source for ChordWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for ChordWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return Some(0.0);
        }
        let sum: f32 = self
            .0
            .iter_mut()
            .map(|wave| wave.next().unwrap_or(0.0))
            .sum();
        Some(sum)
    }
}

pub enum ScoreWave {
    Note(TakeDuration<NoteWave>),
    Chord(TakeDuration<ChordWave>),
    Rest(TakeDuration<Zero<f32>>),
}

impl Source for ScoreWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for ScoreWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ScoreWave::Note(note) => note.next(),
            ScoreWave::Chord(chord) => chord.next(),
            ScoreWave::Rest(rest) => rest.next(),
        }
    }
}

pub struct ChannelWave {
    waves: Vec<ScoreWave>,
    index: usize,
}

impl ChannelWave {
    pub fn new(waves: Vec<ScoreWave>) -> Self {
        Self { waves, index: 0 }
    }
}

impl Source for ChannelWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for ChannelWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.waves.len() {
            return None;
        }

        let wave = &mut self.waves[self.index];
        match wave.next() {
            Some(value) => Some(value),
            None => {
                self.index += 1;
                self.next()
            }
        }
    }
}

pub struct MusicWave(Vec<ChannelWave>);

impl MusicWave {
    pub fn new(waves: Vec<ChannelWave>) -> Self {
        Self(waves)
    }
}

impl Source for MusicWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for MusicWave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut samples = self.0.iter_mut().filter_map(|wave| wave.next());
        if let Some(first) = samples.next() {
            let sum = samples.sum::<f32>() + first;
            Some(sum)
        } else {
            None
        }
    }
}
