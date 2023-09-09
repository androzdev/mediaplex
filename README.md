# Mediaplex

Mediaplex is a tiny media transcoding utility for Node.js that allows you to probe media files for metadata and use an Opus encoder/decoder.

> 🏗️ Please note that this package is a work in progress.

# Installation

To install Mediaplex, run the following command:

```sh
$ npm install --save mediaplex
```

# Current Features

Mediaplex currently supports the following features:

- Probe media files for metadata
- Opus encoder/decoder

More features are coming soon.

# Examples

## Probe metadata

You can use Mediaplex to probe media files for metadata. Here's an example:

```js
const mediaplex = require('mediaplex');

const stream = createReadStream('./media.mp3');
const { result } = await mediaplex.probeStream(stream);

console.log(result);

/* Sample Output */
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
```

This will output an object containing information about the media file, including the number of channels, sample rate, codec, duration, and metadata.
The default probe size is `10MB`, but you can adjust this as needed by passing the second argument to `probeStream`:

```js
// probe only 1024 bytes
const { result } = await mediaplex.probeStream(stream, 1024);

// probe 5 MB
const { result } = await mediaplex.probeStream(stream, 5 * 1024 * 1024);
```

// add a note

> ⚠️
> Probing a stream is non-blocking as it is done in a worker thread. But probing a buffer is a blocking operation.

## Opus Encoder

Mediaplex also includes an Opus encoder/decoder. Here's an example:

```js
const { OpusEncoder, getOpusVersion } = require('mediaplex');

console.log(getOpusVersion()); // libopus xxx

const encoder = new OpusEncoder(48000, 2);

const encoded = encoder.encode(buffer);
const decoded = encoder.decode(encoded);
```

You can use `OpusEncoder` to encode pcm data to opus and decode opus data to pcm format. Stream interface is provided by [@discord-player/opus](https://npm.im/@discord-player/opus) package.

#### Opus Benchmarks

Mediaplex includes benchmarks for the opus encoder/decoder. Here are the results of the benchmarks on a Windows 11 machine with an i7-8700 3.2GHz processor:

```js
$ yarn benchmark

Running "OpusEncoder Benchmark" suite...

  mediaplex:
    3 575 ops/s, ±0.75%   | 14.82% slower

  @discordjs/opus:
    3 169 ops/s, ±0.43%   | 24.49% slower

  @evan/opus:
    3 310 ops/s, ±0.18%   | 21.13% slower

  @evan/opus (wasm):
    2 259 ops/s, ±0.17%   | 46.18% slower

  opusscript:
    4 197 ops/s, ±0.52%   | fastest

  opusscript (no wasm):
    266 ops/s, ±0.55%     | slowest, 93.66% slower

Finished 6 cases!
  Fastest: opusscript
  Slowest: opusscript (no wasm)
Running "OpusDecoder Benchmark" suite...

  mediaplex:
    9 951 ops/s, ±0.42%    | 16.12% slower

  @discordjs/opus:
    11 864 ops/s, ±0.49%   | fastest

  @evan/opus:
    11 470 ops/s, ±0.39%   | 3.32% slower

  @evan/opus (wasm):
    7 436 ops/s, ±0.35%    | 37.32% slower

  opusscript:
    6 101 ops/s, ±0.31%    | 48.58% slower

  opusscript (no wasm):
    2 261 ops/s, ±0.24%    | slowest, 80.94% slower

Finished 6 cases!
  Fastest: @discordjs/opus
  Slowest: opusscript (no wasm)
```

These benchmarks compare the performance of Mediaplex's Opus encoder/decoder to other popular Opus libraries.
