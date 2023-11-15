use std::time::Duration;

use lmml::oscillator::{SawWave, SquareWave, TriangleWave};
use rodio::{OutputStream, Sink, Source};

pub fn main() {
    println!("hoge");
    let square = SquareWave::new(440.0, 0.1, 0.5);
    let saw = SawWave::new(440.0, 0.1);
    let triangle = TriangleWave::new(440.0, 0.1);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(square.take_duration(Duration::from_secs(3)));
    sink.append(saw.take_duration(Duration::from_secs(3)));
    sink.append(triangle.take_duration(Duration::from_secs(3)));
    sink.sleep_until_end();
}
