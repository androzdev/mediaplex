use napi::bindgen_prelude::*;

#[napi]
pub enum PCMFormat {
  S16LE,
  S16BE,
  S32LE,
  S32BE,
}

#[napi]
pub enum AudioChannel {
  Mono,
  Stereo,
}

mod equalizer;
mod volume;
