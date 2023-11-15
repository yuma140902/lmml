use std::{thread, time::Duration};

use lmml::oscillator::{SawWave, SquareWave};
use rodio::OutputStream;

pub fn main() {
    println!("hoge");
    let square = SquareWave::new(440.0, 0.1);
    let saw = SawWave::new(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(saw).unwrap();
    thread::sleep(Duration::from_secs(5));
}
