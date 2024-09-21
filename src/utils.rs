use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::thread;

// audio signal useful in the case of unavailable display
pub fn beep(secs: f32, added_beeps: u32) {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap(); // we have to handle this
        let sink = Sink::try_new(&stream_handle).unwrap();

        let high = SineWave::new(440.0)
            .take_duration(std::time::Duration::from_secs_f32(secs))
            .amplify(0.20);

        let mid = SineWave::new(220.0)
            .take_duration(std::time::Duration::from_secs_f32(secs))
            .amplify(0.20);

        sink.append(high.clone());

        for _ in 0..added_beeps {
            sink.append(mid.clone());
            sink.append(high.clone());
        }

        sink.sleep_until_end();
    });
}
