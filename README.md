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
    3 538 ops/s, ±0.63%   | 15.96% slower

  @discordjs/opus:
    3 168 ops/s, ±0.53%   | 24.75% slower

  @evan/opus:
    3 283 ops/s, ±0.25%   | 22.02% slower

  @evan/opus (wasm):
    2 242 ops/s, ±0.39%   | 46.75% slower

  opusscript:
    4 210 ops/s, ±0.20%   | fastest

  opusscript (no wasm):
    245 ops/s, ±3.69%     | slowest, 94.18% slower

Finished 6 cases!
  Fastest: opusscript
  Slowest: opusscript (no wasm)

Running "OpusDecoder Benchmark" suite...

  mediaplex:
    10 726 ops/s, ±0.50%   | 9.42% slower

  @discordjs/opus:
    11 841 ops/s, ±0.55%   | fastest

  @evan/opus:
    11 382 ops/s, ±0.46%   | 3.88% slower

  @evan/opus (wasm):
    7 461 ops/s, ±0.32%    | 36.99% slower

  opusscript:
    6 138 ops/s, ±0.38%    | 48.16% slower

  opusscript (no wasm):
    2 252 ops/s, ±0.73%    | slowest, 80.98% slower

Finished 6 cases!
  Fastest: @discordjs/opus
  Slowest: opusscript (no wasm)
```

These benchmarks compare the performance of Mediaplex's Opus encoder/decoder to other popular Opus libraries.
