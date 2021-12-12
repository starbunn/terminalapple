use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source}

pub fn start_audio() {
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let soundFile = BufReader.new(File::open("src/audio/bad_apple.mp3").unwrap());
  let source = Decoder::new(soundFile).unwrap();
  stream_handle.play_raw(source.convert_samples());
}