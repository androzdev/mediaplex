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

Mediaplex includes benchmarks for the Opus encoder/decoder. Here are the results of the benchmarks on a Windows 11 machine with an i7-8700 3.2GHz processor:

```js
$ yarn benchmark

Running "OpusEncoder Benchmark" suite...
Progress: 100%

  mediaplex:
    3 502 ops/s, ±0.84%   | 16.04% slower

  @discordjs/opus:
    3 185 ops/s, ±0.17%   | 23.64% slower

  opusscript:
    4 171 ops/s, ±0.34%   | fastest

  opusscript (no wasm):
    261 ops/s, ±0.85%     | slowest, 93.74% slower

Finished 4 cases!
  Fastest: opusscript
  Slowest: opusscript (no wasm)

Running "OpusDecoder Benchmark" suite...
Progress: 100%

  mediaplex:
    9 838 ops/s, ±0.38%    | 16.96% slower

  @discordjs/opus:
    11 848 ops/s, ±0.40%   | fastest

  opusscript:
    6 100 ops/s, ±0.23%    | 48.51% slower

  opusscript (no wasm):
    2 589 ops/s, ±0.20%    | slowest, 78.15% slower

Finished 4 cases!
  Fastest: @discordjs/opus
  Slowest: opusscript (no wasm)
```

These benchmarks compare the performance of Mediaplex's Opus encoder/decoder to other popular Opus libraries.
