# Mediaplex

Tiny media transcoding utility for node.

> 🏗️ This package is a work in progress.

# Installation

```sh
$ npm install --save mediaplex
```

# Current Features

- probe media files for metadata
- Opus encoder/decoder

More features coming soon.

# Examples

## Probe metadata

```js
const mediaplex = require('mediaplex');

const stream = createReadStream('./media.mp3');
const result = mediaplex.probe(stream);

console.log(result);

/*
{
    channels: 2,
    sampleRate: 44100,
    framesPerBlock: 0,
    codec: 4099, // use `CodecType` enum to validate this
    nFrames: 796032,
    duration: 18, // seconds
    metadata: [
        { name: 'TXXX:major_brand', value: 'mp42' },
        { name: 'TXXX:minor_version', value: '0' },
        { name: 'TXXX:compatible_brands', value: 'isommp42' },
        { name: 'TSSE', value: 'Lavf59.6.100' },
        { name: 'TIT2', value: "..." },
        { name: 'TPE1', value: '...' },
        { name: 'TALB', value: '...' },
        { name: 'TCON', value: '...' },
        { name: 'TPUB', value: '...' }
    ]
}
*/
```

## Opus Encoder

```js
const { OpusEncoder, getOpusVersion } = require('mediaplex');

console.log(getOpusVersion()); // libopus ...

const encoder = new OpusEncoder(48000, 2);

const encoded = encoder.encode(buffer);
const decoded = encoder.decode(encoded);
```

#### Opus Benchmarks

Tested on windows 11 i7-8700 3.2GHz

```js
$ yarn benchmark

Running "OpusEncoder Benchmark" suite...
Progress: 100%

  mediaplex:
    3 305 ops/s, ±0.15%   | 21.18% slower

  @discordjs/opus:
    3 174 ops/s, ±0.09%   | 24.3% slower

  opusscript:
    4 193 ops/s, ±0.16%   | fastest

  opusscript (no wasm):
    260 ops/s, ±0.55%     | slowest, 93.8% slower

Finished 4 cases!
  Fastest: opusscript
  Slowest: opusscript (no wasm)

Running "OpusDecoder Benchmark" suite...
Progress: 100%

  mediaplex:
    11 441 ops/s, ±0.99%   | 3.21% slower

  @discordjs/opus:
    11 821 ops/s, ±0.55%   | fastest

  opusscript:
    6 095 ops/s, ±0.27%    | 48.44% slower

  opusscript (no wasm):
    2 584 ops/s, ±0.52%    | slowest, 78.14% slower

Finished 4 cases!
  Fastest: @discordjs/opus
  Slowest: opusscript (no wasm)
```