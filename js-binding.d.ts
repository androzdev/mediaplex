/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function getOpusVersion(): string
export interface ProbeResult {
  /** The number of channels */
  channels: number
  /** The sample rate */
  sampleRate: number
  /** The number of frames per block */
  framesPerBlock: number
  /** The codec type */
  codec: CodecType
  /** The number of frames */
  nFrames: number
  /** The approximate duration of this media in seconds */
  duration: number
  /** The metadata object */
  metadata: Array<MetadataField>
}
export const enum CodecType {
  UNKNOWN = 0,
  PCM_S32LE = 256,
  PCM_S32LE_PLANAR = 257,
  PCM_S32BE = 258,
  PCM_S32BE_PLANAR = 259,
  PCM_S24LE = 260,
  PCM_S24LE_PLANAR = 261,
  PCM_S24BE = 262,
  PCM_S24BE_PLANAR = 263,
  PCM_S16LE = 264,
  PCM_S16LE_PLANAR = 265,
  PCM_S16BE = 266,
  PCM_S16BE_PLANAR = 267,
  PCM_S8 = 268,
  PCM_S8_PLANAR = 269,
  PCM_U32LE = 270,
  PCM_U32LE_PLANAR = 271,
  PCM_U32BE = 272,
  PCM_U32BE_PLANAR = 273,
  PCM_U24LE = 274,
  PCM_U24LE_PLANAR = 275,
  PCM_U24BE = 276,
  PCM_U24BE_PLANAR = 277,
  PCM_U16LE = 278,
  PCM_U16LE_PLANAR = 279,
  PCM_U16BE = 280,
  PCM_U16BE_PLANAR = 281,
  PCM_U8 = 282,
  PCM_U8_PLANAR = 283,
  PCM_F32LE = 284,
  PCM_F32LE_PLANAR = 285,
  PCM_F32BE = 286,
  PCM_F32BE_PLANAR = 287,
  PCM_F64LE = 288,
  PCM_F64LE_PLANAR = 289,
  PCM_F64BE = 290,
  PCM_F64BE_PLANAR = 291,
  PCM_ALAW = 292,
  PCM_MULAW = 293,
  ADPCM_G722 = 512,
  ADPCM_G726 = 513,
  ADPCM_G726LE = 514,
  ADPCM_MS = 515,
  ADPCM_IMA_WAV = 516,
  ADPCM_IMA_QT = 517,
  VORBIS = 4096,
  MP1 = 4097,
  MP2 = 4098,
  MP3 = 4099,
  AAC = 4100,
  OPUS = 4101,
  SPEEX = 4102,
  MUSEPACK = 4103,
  ATRAC1 = 4104,
  ATRAC3 = 4105,
  ATRAC3PLUS = 4106,
  ATRAC9 = 4107,
  EAC3 = 4108,
  AC4 = 4109,
  DCA = 4110,
  WMA = 4111,
  FLAC = 8192,
  WAVPACK = 8193,
  MONKEYS_AUDIO = 8194,
  ALAC = 8195,
  TTA = 8196
}
export interface MetadataField {
  name: string
  value: string
}
export function probe(data: Buffer): Promise<ProbeResult>
export function probeSync(data: Buffer): ProbeResult
export interface Mp3DecodedFrame {
  frameBytes: number
  frameOffset: number
  channels: number
  hz: number
  layer: number
  bitrateKbps: number
  data: Buffer
}
export type JsOpusEncoder = OpusEncoder
export class OpusEncoder {
  constructor(sampleRate: number, channels: number)
  encode(data: Buffer): Buffer
  decode(data: Buffer): Buffer
  setBitrate(bitrate: number): void
  getBitrate(): number
  applyEncoderCtl(request: number, value: number): void
  applyDecoderCtl(request: number, value: number): void
  get version(): string
}
export class Mp3Decoder {
  constructor()
  decode(data: Buffer): Mp3DecodedFrame
}
