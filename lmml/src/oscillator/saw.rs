use rodio::Source;

use super::SAMPLE_RATE;

pub struct SawWave {
    frame: usize,
    pub frequency: f32,
    pub amplitude: f32,
}

impl SawWave {
    pub fn new(frequency: f32, amplitude: f32) -> Self {
        Self {
            frame: 0,
            frequency,
            amplitude,
        }
    }
}

impl Source for SawWave {
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

impl Iterator for SawWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.frame += 1;
        if self.frame > (SAMPLE_RATE as f32 / self.frequency) as usize {
            self.frame = 0;
        }
        Some(
            (self.frame as f32 / (SAMPLE_RATE as f32 / self.frequency as f32) - 0.5)
                * 2.0
                * self.amplitude,
        )
    }
}
