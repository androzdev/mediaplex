import type { ProbeResult } from './js-binding'
import type { Readable } from 'stream'

export type StreamProbeResult = {
  stream: Readable;
  result: ProbeResult;
};

export interface Metadata {
  title: string | null;
  author: string | null;
  genre: string | null;
}

export * from './js-binding';
export function probeStream(
  stream: Readable,
  probeSize?: number
): Promise<StreamProbeResult>;
export function readMetadata(result: ProbeResult): Metadata;