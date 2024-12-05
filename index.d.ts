import type { ProbeResult } from "./js-binding";
import type { Readable } from "stream";

export class OpusEncoder {
  public constructor(sampleRate: number, channels: number);
  public encode(data: Buffer): Buffer;
  public decode(data: Buffer): Buffer;
  public setBitrate(bitrate: number): void;
  public getBitrate(): number;
  public applyEncoderCtl(request: number, value: number): void;
  public applyDecoderCtl(request: number, value: number): void;
  public get version(): string;
  public hasEncoder(): boolean;
  public hasDecoder(): boolean;
  public static create(sampleRate: number, channels: number): OpusEncoder;
}

export {
  CodecType,
  MetadataField,
  ProbeResult,
  getOpusVersion,
  probe,
  probeSync,
} from "./js-binding";

export type StreamProbeResult = {
  stream: Readable;
  result: ProbeResult;
};

export interface Metadata {
  title: string | null;
  author: string | null;
  genre: string | null;
  album: string | null;
  year: string | null;
  duration: number | null;
  composer: string | null;
  bpm: number | null;
}

export interface ProbeStreamOptions {
  /**
   * If true, mediaplex will probe the stream synchronously. Defaults to false.
   */
  sync?: boolean;
  /**
   * The maximum number of bytes to read from the stream. Defaults to 2MB.
   */
  probeSize?: number;
}

export function probeStream(
  stream: Readable,
  options?: ProbeStreamOptions,
): Promise<StreamProbeResult>;
export function readMetadata(result: ProbeResult): Metadata;
