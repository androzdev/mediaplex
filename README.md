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
mediaplex             282.31 µs/iter   (274.4 µs … 746.9 µs)  279.5 µs  378.7 µs  391.3 µs
@discordjs/opus       315.47 µs/iter     (310 µs … 460.4 µs)  314.1 µs  384.6 µs  396.8 µs
@evan/opus            304.59 µs/iter   (299.9 µs … 483.7 µs)  303.5 µs  357.4 µs  376.9 µs
@evan/opus (wasm)     469.27 µs/iter     (440.1 µs … 1.3 ms)    447 µs  964.1 µs   1.03 ms
opusscript            249.68 µs/iter   (235.3 µs … 692.1 µs)  240.7 µs  499.5 µs  551.3 µs
opusscript (no wasm)    4.01 ms/iter     (3.84 ms … 4.69 ms)    4.1 ms   4.55 ms   4.69 ms

summary for OpusEncoder
  opusscript
   1.13x faster than mediaplex
   1.22x faster than @evan/opus
   1.26x faster than @discordjs/opus
   1.88x faster than @evan/opus (wasm)
   16.06x faster than opusscript (no wasm)


cpu: Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz
runtime: node v20.6.1 (x64-win32)

benchmark                 time (avg)             (min … max)       p75       p99      p995
------------------------------------------------------------ -----------------------------
• OpusDecoder
------------------------------------------------------------ -----------------------------
mediaplex              92.94 µs/iter     (84.8 µs … 1.22 ms)   90.8 µs  143.5 µs  161.8 µs
@discordjs/opus        87.75 µs/iter      (81 µs … 376.1 µs)   86.3 µs  130.2 µs    142 µs
@evan/opus             91.22 µs/iter    (84.7 µs … 320.2 µs)   89.9 µs  133.4 µs    142 µs
@evan/opus (wasm)     146.92 µs/iter   (130.3 µs … 966.5 µs)  137.7 µs  298.7 µs  309.3 µs
opusscript            169.56 µs/iter   (159.5 µs … 448.9 µs)  167.2 µs  232.6 µs  258.6 µs
opusscript (no wasm)  482.66 µs/iter    (443.7 µs … 1.56 ms)  456.5 µs  965.8 µs  995.3 µs

summary for OpusDecoder
  @discordjs/opus
   1.04x faster than @evan/opus
   1.06x faster than mediaplex
   1.67x faster than @evan/opus (wasm)
   1.93x faster than opusscript
   5.5x faster than opusscript (no wasm)
```
