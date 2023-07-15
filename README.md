# Mediaplex

Tiny media transcoding utility for node.

> 🏗️ This package is a work in progress.

# Installation

```sh
$ npm install --save mediaplex
```

# Example

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
    duration: 18 // seconds
}
*/
```