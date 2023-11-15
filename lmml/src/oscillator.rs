pub use saw::*;
pub use square::*;
pub use triangle::*;

mod saw;
mod square;

pub const SAMPLE_RATE: u32 = 44100;

mod triangle {
    use rodio::Source;

    use super::SAMPLE_RATE;

    pub struct TriangleWave {
        frame: usize,
        pub frequency: f32,
        pub amplitude: f32,
    }

    impl TriangleWave {
        pub fn new(frequency: f32, amplitude: f32) -> Self {
            Self {
                frame: 0,
                frequency,
                amplitude,
            }
        }
    }

    impl Source for TriangleWave {
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

    impl Iterator for TriangleWave {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
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
    }
}
