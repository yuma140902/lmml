use std::f32::consts::PI;

use rodio::Source;

pub const SAMPLE_RATE: u32 = 44100;

#[derive(Debug, Clone, Copy)]
pub enum Waveform {
    Zero,
    Saw,
    Square(f32),
    Triangle,
    Sine,
}

pub struct Wave {
    frame: usize,
    waveform: Waveform,
    frequency: f32,
    amplitude: f32,
}

impl Wave {
    pub fn new(waveform: Waveform, frequency: f32, amplitude: f32) -> Self {
        Self {
            frame: 0,
            waveform,
            frequency,
            amplitude,
        }
    }
}

impl Source for Wave {
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

impl Iterator for Wave {
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

pub struct MixedWave(Vec<Wave>);

impl MixedWave {
    pub fn new(waves: Vec<Wave>) -> Self {
        Self(waves)
    }
}

impl Source for MixedWave {
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

impl Iterator for MixedWave {
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
