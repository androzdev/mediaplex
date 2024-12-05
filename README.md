# `mediaplex`

![https://github.com/napi-rs/tar/actions](https://github.com/napi-rs/tar/workflows/CI/badge.svg)
[![install size](https://packagephobia.com/badge?p=mediaplex)](https://packagephobia.com/result?p=mediaplex)
[![Downloads](https://img.shields.io/npm/dm/mediaplex.svg?sanitize=true)](https://npmcharts.com/compare/mediaplex?minimal=true)

Media processing library for Node.js

## Install this package

```
yarn add mediaplex
pnpm add mediaplex
bun add mediaplex
npm install mediaplex
```

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
const { OpusEncoder, getOpusVersion } = require("mediaplex");

console.log(getOpusVersion()); // libopus xxx

const encoder = new OpusEncoder(48000, 2);

const encoded = encoder.encode(buffer);
const decoded = encoder.decode(encoded);
```

You can use `OpusEncoder` to encode pcm data to opus and decode opus data to pcm format. Stream interface is provided by [@discord-player/opus](https://npm.im/@discord-player/opus) package.
