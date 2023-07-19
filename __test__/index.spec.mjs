import test from 'ava'
import { probe } from '../index.js'
import { readFile } from 'fs/promises';

test('should probe mp3 buffer', async (t) => {
  const chunk = await readFile('./__test__/data/head.mp3');
  const probeResult = probe(chunk);

  t.deepEqual(probeResult, {
    channels: 2,
    sampleRate: 44100,
    framesPerBlock: 0,
    codec: 4099,
    nFrames: 796032,
    duration: 18,
    metadata: probeResult.metadata.slice()
  });
})
