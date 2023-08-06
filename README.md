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

console.log(getOpusVersion()); // libopus 1.4-6-g9fc8fc4c

const encoder = new OpusEncoder(48000, 2);

const encoded = encoder.encode(buffer);
const decoded = encoder.decode(encoded);
```
