const fs = require('fs');
const { probeStream, readMetadata } = require('../index.js');

const stream = fs.createReadStream(`${__dirname}/demo.pcm`);

/**
 * @param {Buffer} chunk 
 */
async function main() {
    const probeResult = await probeStream(stream);
    console.log(probeResult.result);
    // console.log(readMetadata(probeResult.result));
    /*
    {
        channels: 2,
        sampleRate: 44100,
        framesPerBlock: 0,
        codec: 4099,
        nFrames: 796032,
        duration: 18
    }
    */
}

main();