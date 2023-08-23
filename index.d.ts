import type { ProbeResult } from './js-binding';
import type { Readable } from 'stream';

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

export * from './js-binding';
export function probeStream(
  stream: Readable,
  options?: ProbeStreamOptions
): Promise<StreamProbeResult>;
export function readMetadata(result: ProbeResult): Metadata;
