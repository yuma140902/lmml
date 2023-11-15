use rodio::Source;

pub enum Waveform {
    Square,
}

pub struct SquareWave {
    frame: usize,
    pub frequency: f64,
    pub pulse_width: f64,
}

impl SquareWave {
    pub fn new(frequency: f64, pulse_width: f64) -> Self {
        Self {
            frame: 0,
            frequency,
            pulse_width,
        }
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.frame += 1;
        if self.frame > (48000_f64 / self.frequency) as usize {
            self.frame = 0;
        }
        if self.frame > ((48000_f64 / self.frequency) * self.pulse_width) as usize {
            Some(0.1)
        } else {
            Some(-0.1)
        }
    }
}
