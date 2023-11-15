use std::f32::consts::PI;

use rodio::Source;

use super::SAMPLE_RATE;

pub struct SineWave {
    frame: u64,
    pub frequency: f32,
    pub amplitude: f32,
}

impl SineWave {
    pub fn new(frequency: f32, amplitude: f32) -> Self {
        Self {
            frame: 0,
            frequency,
            amplitude,
        }
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        super::SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.frame = self.frame.wrapping_add(1);

        let value = 2.0 * PI * self.frequency * self.frame as f32 / SAMPLE_RATE as f32;
        Some(self.amplitude * value.sin())
    }
}
