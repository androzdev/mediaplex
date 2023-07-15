import type { ProbeResult } from './lib/binding'
import type { Readable } from 'stream'

export type StreamProbeResult = {
  stream: Readable;
  result: ProbeResult;
};

export * from './lib/binding';
export function probeStream(
  stream: Readable,
  probeSize?: number
): Promise<StreamProbeResult>;