export class OpusEncoder {
  public constructor(sampleRate: number, channels: number)
  public encode(data: Buffer): Buffer
  public decode(data: Buffer): Buffer
  public setBitrate(bitrate: number): void
  public getBitrate(): number
  public applyEncoderCtl(request: number, value: number): void
  public applyDecoderCtl(request: number, value: number): void
  public get version(): string
  public hasEncoder(): boolean
  public hasDecoder(): boolean
  public static create(sampleRate: number, channels: number): OpusEncoder
}

export { CodecType, MetadataField, ProbeResult, getOpusVersion, probe, probeSync } from './js-binding'
