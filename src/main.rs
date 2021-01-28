use rodio::Source;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Take argv1
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // Load a sound from a file
    let file = File::open(filename).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    // Press ctrl + C to stop the process once you're done.
    loop {}
}
