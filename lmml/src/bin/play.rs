use std::{thread, time::Duration};

use lmml::oscillator::SquareWave;
use rodio::OutputStream;

pub fn main() {
    println!("hoge");
    let square = SquareWave::new(440.0, 0.5);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(square).unwrap();

    thread::sleep(Duration::from_secs(5));
}
