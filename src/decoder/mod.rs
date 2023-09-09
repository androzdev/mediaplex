use napi::bindgen_prelude::*;
use napi::{Error, Result};
use symphonia::core::codecs::{Decoder as SymphoniaDecoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::{Packet, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;

struct Decoder {
  decoder: Box<dyn SymphoniaDecoder>,
  track: &Track,
}

impl Decoder {
  fn new(src_hint: Option<String>) -> Result<Self> {
    let cursor = std::io::Cursor::new(Vec::new());

    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

    let mut hint = Hint::new();
    if let Some(h) = src_hint {
      hint.with_extension(h.as_str());
    }

    let meta_opts: symphonia::core::meta::MetadataOptions = Default::default();
    let fmt_opts: symphonia::core::formats::FormatOptions = Default::default();

    let mut probed = symphonia::default::get_probe()
      .format(&hint, mss, &fmt_opts, &meta_opts)
      .map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("failed to probe media: {}", e),
        )
      })?;

    let mut format = probed.format;

    let track = format
      .tracks()
      .iter()
      .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
      .ok_or(Error::new(Status::GenericFailure, "no audio tracks found"))?;

    let dec_opts: DecoderOptions = Default::default();
    let mut decoder = symphonia::default::get_codecs()
      .make(&track.codec_params, &dec_opts)
      .map_err(|e| Error::new(Status::GenericFailure, "unsupported codec"))?;

    Ok(Self { decoder, track })
  }

  fn decode(&mut self, data: &[u8]) -> Result<()> {
    self
      .decoder
      .decode(Packet::new_from_slice(self.track.id, ts, dur, buf));
    Ok(())
  }
}

fn decode(data: &[u8]) -> Result<()> {
  Ok(())
}
