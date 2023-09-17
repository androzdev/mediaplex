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
The default probe size is `2MB`, but you can adjust this as needed by passing the second argument to `probeStream`:

```js
// probe only 1024 bytes
const { result } = await mediaplex.probeStream(stream, 1024);

// probe 5 MB
const { result } = await mediaplex.probeStream(stream, 5 * 1024 * 1024);
```

## Opus Encoder

Mediaplex also includes an Opus encoder/decoder, which can be used as a drop-in replacement for [`@discordjs/opus`](https://github.com/discordjs/opus). Here's an example on how to use it:

```js
const { OpusEncoder, getOpusVersion } = require('mediaplex');

console.log(getOpusVersion()); // libopus xxx

const encoder = new OpusEncoder(48000, 2);

const encoded = encoder.encode(buffer);
const decoded = encoder.decode(encoded);
```

You can use `OpusEncoder` to encode pcm data to opus and decode opus data to pcm format. Stream interface is provided by [@discord-player/opus](https://npm.im/@discord-player/opus) package.

#### Opus benchmark with other equivalent libraries

```js
$ yarn benchmark

cpu: Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz
runtime: node v20.6.1 (x64-win32)

benchmark                 time (avg)             (min … max)       p75       p99      p995
------------------------------------------------------------ -----------------------------
• OpusEncoder
------------------------------------------------------------ -----------------------------
mediaplex             522.67 µs/iter   (506.8 µs … 816.2 µs)  521.5 µs  645.6 µs  726.6 µs
@discordjs/opus       679.61 µs/iter   (671.3 µs … 799.9 µs)  677.8 µs  750.2 µs  770.9 µs
opusscript            306.86 µs/iter   (281.9 µs … 894.8 µs)  299.3 µs  619.8 µs  695.5 µs
opusscript (no wasm)    4.36 ms/iter     (4.09 ms … 6.08 ms)   4.49 ms   4.94 ms   6.08 ms
@evan/opus            494.32 µs/iter   (479.6 µs … 754.2 µs)  495.2 µs  632.9 µs  694.2 µs
@evan/opus (wasm)     850.82 µs/iter    (800.7 µs … 1.93 ms)  841.3 µs   1.46 ms   1.61 ms
@evan/wasm            869.24 µs/iter    (821.7 µs … 1.61 ms)  852.4 µs   1.39 ms   1.48 ms

summary for OpusEncoder
  opusscript
   1.61x faster than @evan/opus
   1.7x faster than mediaplex
   2.21x faster than @discordjs/opus
   2.77x faster than @evan/opus (wasm)
   2.83x faster than @evan/wasm
   14.22x faster than opusscript (no wasm)

cpu: Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz
runtime: node v20.6.1 (x64-win32)

benchmark                 time (avg)             (min … max)       p75       p99      p995
------------------------------------------------------------ -----------------------------
• OpusDecoder
------------------------------------------------------------ -----------------------------
mediaplex              57.54 µs/iter    (49.6 µs … 944.4 µs)   55.3 µs  103.4 µs  116.5 µs
@discordjs/opus        49.85 µs/iter    (45.7 µs … 483.6 µs)   48.5 µs  134.7 µs  140.5 µs
opusscript             82.25 µs/iter      (75 µs … 540.6 µs)   81.1 µs  143.4 µs    157 µs
opusscript (no wasm)  230.88 µs/iter    (202.9 µs … 2.05 ms)  215.3 µs  532.3 µs  594.9 µs
@evan/opus             49.16 µs/iter    (45.7 µs … 827.6 µs)   47.7 µs  132.6 µs    137 µs
@evan/opus (wasm)      71.71 µs/iter    (64.5 µs … 426.9 µs)   67.6 µs  208.1 µs  230.4 µs
@evan/wasm              77.3 µs/iter    (71.9 µs … 383.6 µs)   74.4 µs  162.4 µs  169.4 µs

summary for OpusDecoder
  @evan/opus
   1.01x faster than @discordjs/opus
   1.17x faster than mediaplex
   1.46x faster than @evan/opus (wasm)
   1.57x faster than @evan/wasm
   1.67x faster than opusscript
   4.7x faster than opusscript (no wasm)
```
