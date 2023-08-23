use napi::bindgen_prelude::*;
use napi::{Result, Task};
use symphonia::core::meta::MetadataRevision;
use symphonia::core::{
  audio::Channels, codecs::*, io::MediaSourceStream, probe::Hint, units::TimeBase,
};

#[napi(object, js_name = "ProbeResult")]
pub struct JsProbeResult {
  /// The number of channels
  pub channels: u32,
  /// The sample rate
  pub sample_rate: u32,
  /// The number of frames per block
  pub frames_per_block: u32,
  /// The codec type
  #[napi(ts_type = "CodecType")]
  pub codec: JsCodecType,
  /// The number of frames
  pub n_frames: u32,
  /// The approximate duration of this media in seconds
  pub duration: u32,
  /// The metadata object
  pub metadata: Vec<MetadataField>,
}

#[napi(js_name = "CodecType")]
pub enum JsCodecType {
  // Unknown
  UNKNOWN = 0,
  // PCM
  PCM_S32LE = 0x100,
  PCM_S32LE_PLANAR = 0x101,
  PCM_S32BE = 0x102,
  PCM_S32BE_PLANAR = 0x103,
  PCM_S24LE = 0x104,
  PCM_S24LE_PLANAR = 0x105,
  PCM_S24BE = 0x106,
  PCM_S24BE_PLANAR = 0x107,
  PCM_S16LE = 0x108,
  PCM_S16LE_PLANAR = 0x109,
  PCM_S16BE = 0x10a,
  PCM_S16BE_PLANAR = 0x10b,
  PCM_S8 = 0x10c,
  PCM_S8_PLANAR = 0x10d,
  PCM_U32LE = 0x10e,
  PCM_U32LE_PLANAR = 0x10f,
  PCM_U32BE = 0x110,
  PCM_U32BE_PLANAR = 0x111,
  PCM_U24LE = 0x112,
  PCM_U24LE_PLANAR = 0x113,
  PCM_U24BE = 0x114,
  PCM_U24BE_PLANAR = 0x115,
  PCM_U16LE = 0x116,
  PCM_U16LE_PLANAR = 0x117,
  PCM_U16BE = 0x118,
  PCM_U16BE_PLANAR = 0x119,
  PCM_U8 = 0x11a,
  PCM_U8_PLANAR = 0x11b,
  PCM_F32LE = 0x11c,
  PCM_F32LE_PLANAR = 0x11d,
  PCM_F32BE = 0x11e,
  PCM_F32BE_PLANAR = 0x11f,
  PCM_F64LE = 0x120,
  PCM_F64LE_PLANAR = 0x121,
  PCM_F64BE = 0x122,
  PCM_F64BE_PLANAR = 0x123,
  PCM_ALAW = 0x124,
  PCM_MULAW = 0x125,
  // ADPCM
  ADPCM_G722 = 0x200,
  ADPCM_G726 = 0x201,
  ADPCM_G726LE = 0x202,
  ADPCM_MS = 0x203,
  ADPCM_IMA_WAV = 0x204,
  ADPCM_IMA_QT = 0x205,
  // Vorbis
  VORBIS = 0x1000,
  // MPEG Layer 1 (MP1)
  MP1 = 0x1001,
  // MPEG Layer 2 (MP2)
  MP2 = 0x1002,
  // MPEG Layer 3 (MP3)
  MP3 = 0x1003,
  // Advanced Audio Coding (AAC)
  AAC = 0x1004,
  // Opus
  OPUS = 0x1005,
  // Speex
  SPEEX = 0x1006,
  // Musepack
  MUSEPACK = 0x1007,
  // Adaptive Transform Acoustic Coding (ATRAC1)
  ATRAC1 = 0x1008,
  // Adaptive Transform Acoustic Coding 3 (ATRAC3)
  ATRAC3 = 0x1009,
  // Adaptive Transform Acoustic Coding 3+ (ATRAC3+)
  ATRAC3PLUS = 0x100a,
  // Adaptive Transform Acoustic Coding 9 (ATRAC9)
  ATRAC9 = 0x100b,
  // AC-3, E-AC-3, Dolby Digital (ATSC A/52)
  EAC3 = 0x100c,
  // Dolby AC-4 (ETSI TS 103 190)
  AC4 = 0x100d,
  // DTS Coherent Acoustics (DCA/DTS)
  DCA = 0x100e,
  // Windows Media Audio
  WMA = 0x100f,
  // FLAC
  FLAC = 0x2000,
  // WavPack
  WAVPACK = 0x2001,
  // Monkey's Audio (APE)
  MONKEYS_AUDIO = 0x2002,
  // Apple Lossless Audio Codec (ALAC)
  ALAC = 0x2003,
  // True Audio (TTA)
  TTA = 0x2004,
}

#[napi(object)]
pub struct MetadataField {
  pub name: String,
  pub value: String,
}

#[napi(catch_unwind, ts_return_type = "Promise<ProbeResult>")]
pub fn probe(data: Buffer) -> AsyncTask<ProbeTask> {
  return AsyncTask::new(ProbeTask { input: data });
}

#[napi(catch_unwind)]
pub fn probe_sync(data: Buffer) -> Result<JsProbeResult> {
  return probe_inner(data.to_vec());
}

pub struct ProbeTask {
  input: Buffer,
}

impl Task for ProbeTask {
  type Output = JsProbeResult;
  type JsValue = JsProbeResult;

  fn compute(&mut self) -> Result<Self::Output> {
    probe_inner(self.input.to_vec())
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

fn probe_inner(data: Vec<u8>) -> Result<JsProbeResult> {
  // Create a Cursor over the data.
  let cursor = std::io::Cursor::new(data.to_vec());

  // Create a MediaSourceStream from the Cursor.
  let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

  let hint = Hint::new();

  // Use the default options for metadata and format readers.
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

  let metadata: Vec<MetadataField>;

  if let Some(metadata_rev) = format.metadata().current() {
    metadata = parse_metadata(&metadata_rev);
  } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
    metadata = parse_metadata(&metadata_rev);
  } else {
    metadata = Vec::new();
  }

  let track = format
    .tracks()
    .iter()
    .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
    .ok_or(Error::new(Status::GenericFailure, "no audio tracks found"))?;

  let codec_params = &track.codec_params;

  Ok(JsProbeResult {
    channels: codec_params
      .channels
      .unwrap_or(Channels::FRONT_CENTRE)
      .count() as u32,
    sample_rate: codec_params.sample_rate.unwrap_or(44100),
    frames_per_block: codec_params.frames_per_block.unwrap_or(0) as u32,
    codec: match codec_params.codec {
      CODEC_TYPE_PCM_S32LE => JsCodecType::PCM_S32LE,
      CODEC_TYPE_PCM_S32LE_PLANAR => JsCodecType::PCM_S32LE_PLANAR,
      CODEC_TYPE_PCM_S32BE => JsCodecType::PCM_S32BE,
      CODEC_TYPE_PCM_S32BE_PLANAR => JsCodecType::PCM_S32BE_PLANAR,
      CODEC_TYPE_PCM_S24LE => JsCodecType::PCM_S24LE,
      CODEC_TYPE_PCM_S24LE_PLANAR => JsCodecType::PCM_S24LE_PLANAR,
      CODEC_TYPE_PCM_S24BE => JsCodecType::PCM_S24BE,
      CODEC_TYPE_PCM_S24BE_PLANAR => JsCodecType::PCM_S24BE_PLANAR,
      CODEC_TYPE_PCM_S16LE => JsCodecType::PCM_S16LE,
      CODEC_TYPE_PCM_S16LE_PLANAR => JsCodecType::PCM_S16LE_PLANAR,
      CODEC_TYPE_PCM_S16BE => JsCodecType::PCM_S16BE,
      CODEC_TYPE_PCM_S16BE_PLANAR => JsCodecType::PCM_S16BE_PLANAR,
      CODEC_TYPE_PCM_S8 => JsCodecType::PCM_S8,
      CODEC_TYPE_PCM_S8_PLANAR => JsCodecType::PCM_S8_PLANAR,
      CODEC_TYPE_PCM_U32LE => JsCodecType::PCM_U32LE,
      CODEC_TYPE_PCM_U32LE_PLANAR => JsCodecType::PCM_U32LE_PLANAR,
      CODEC_TYPE_PCM_U32BE => JsCodecType::PCM_U32BE,
      CODEC_TYPE_PCM_U32BE_PLANAR => JsCodecType::PCM_U32BE_PLANAR,
      CODEC_TYPE_PCM_U24LE => JsCodecType::PCM_U24LE,
      CODEC_TYPE_PCM_U24LE_PLANAR => JsCodecType::PCM_U24LE_PLANAR,
      CODEC_TYPE_PCM_U24BE => JsCodecType::PCM_U24BE,
      CODEC_TYPE_PCM_U24BE_PLANAR => JsCodecType::PCM_U24BE_PLANAR,
      CODEC_TYPE_PCM_U16LE => JsCodecType::PCM_U16LE,
      CODEC_TYPE_PCM_U16LE_PLANAR => JsCodecType::PCM_U16LE_PLANAR,
      CODEC_TYPE_PCM_U16BE => JsCodecType::PCM_U16BE,
      CODEC_TYPE_PCM_U16BE_PLANAR => JsCodecType::PCM_U16BE_PLANAR,
      CODEC_TYPE_PCM_U8 => JsCodecType::PCM_U8,
      CODEC_TYPE_PCM_U8_PLANAR => JsCodecType::PCM_U8_PLANAR,
      CODEC_TYPE_PCM_F32LE => JsCodecType::PCM_F32LE,
      CODEC_TYPE_PCM_F32LE_PLANAR => JsCodecType::PCM_F32LE_PLANAR,
      CODEC_TYPE_PCM_F32BE => JsCodecType::PCM_F32BE,
      CODEC_TYPE_PCM_F32BE_PLANAR => JsCodecType::PCM_F32BE_PLANAR,
      CODEC_TYPE_PCM_F64LE => JsCodecType::PCM_F64LE,
      CODEC_TYPE_PCM_F64LE_PLANAR => JsCodecType::PCM_F64LE_PLANAR,
      CODEC_TYPE_PCM_F64BE => JsCodecType::PCM_F64BE,
      CODEC_TYPE_PCM_F64BE_PLANAR => JsCodecType::PCM_F64BE_PLANAR,
      CODEC_TYPE_PCM_ALAW => JsCodecType::PCM_ALAW,
      CODEC_TYPE_PCM_MULAW => JsCodecType::PCM_MULAW,
      CODEC_TYPE_ADPCM_G722 => JsCodecType::ADPCM_G722,
      CODEC_TYPE_ADPCM_G726 => JsCodecType::ADPCM_G726,
      CODEC_TYPE_ADPCM_G726LE => JsCodecType::ADPCM_G726LE,
      CODEC_TYPE_ADPCM_MS => JsCodecType::ADPCM_MS,
      CODEC_TYPE_ADPCM_IMA_WAV => JsCodecType::ADPCM_IMA_WAV,
      CODEC_TYPE_ADPCM_IMA_QT => JsCodecType::ADPCM_IMA_QT,
      CODEC_TYPE_VORBIS => JsCodecType::VORBIS,
      CODEC_TYPE_MP1 => JsCodecType::MP1,
      CODEC_TYPE_MP2 => JsCodecType::MP2,
      CODEC_TYPE_MP3 => JsCodecType::MP3,
      CODEC_TYPE_AAC => JsCodecType::AAC,
      CODEC_TYPE_OPUS => JsCodecType::OPUS,
      CODEC_TYPE_SPEEX => JsCodecType::SPEEX,
      CODEC_TYPE_MUSEPACK => JsCodecType::MUSEPACK,
      CODEC_TYPE_ATRAC1 => JsCodecType::ATRAC1,
      CODEC_TYPE_ATRAC3 => JsCodecType::ATRAC3,
      CODEC_TYPE_ATRAC3PLUS => JsCodecType::ATRAC3PLUS,
      CODEC_TYPE_ATRAC9 => JsCodecType::ATRAC9,
      CODEC_TYPE_EAC3 => JsCodecType::EAC3,
      CODEC_TYPE_AC4 => JsCodecType::AC4,
      CODEC_TYPE_DCA => JsCodecType::DCA,
      CODEC_TYPE_WMA => JsCodecType::WMA,
      CODEC_TYPE_FLAC => JsCodecType::FLAC,
      CODEC_TYPE_WAVPACK => JsCodecType::WAVPACK,
      CODEC_TYPE_MONKEYS_AUDIO => JsCodecType::MONKEYS_AUDIO,
      CODEC_TYPE_ALAC => JsCodecType::ALAC,
      CODEC_TYPE_TTA => JsCodecType::TTA,
      _ => JsCodecType::UNKNOWN,
    },
    n_frames: codec_params.n_frames.unwrap_or(0) as u32,
    duration: TimeBase::calc_time(
      &codec_params
        .time_base
        .unwrap_or(TimeBase { numer: 1, denom: 1 }),
      codec_params.n_frames.unwrap_or(0),
    )
    .seconds as u32,
    metadata: metadata,
  })
}

fn parse_metadata(rev: &MetadataRevision) -> Vec<MetadataField> {
  let tags = rev.tags();
  tags
    .iter()
    .map(|tag| MetadataField {
      name: tag.key.to_string(),
      value: tag.value.to_string(),
    })
    .collect()
}
