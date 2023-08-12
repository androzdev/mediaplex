const fs = require('fs');
const { probeStream, readMetadata } = require('../index.js');

const url = "https://cdn.discordapp.com/attachments/1128739211701395518/1134561530885705738/16905709686973736.mp3";
// const stream = fs.createReadStream(`${__dirname}/test.mp3`);
const stream = new Promise((resolve, reject) => {
    require('https').get(url, resolve);
});

/**
 * @param {Buffer} chunk 
 */
async function main() {
    const s = await stream;
    const start = performance.now();
    const probeResult = await probeStream(s);
    const end = performance.now() - start;
    console.log(probeResult.result, readMetadata(probeResult.result), `Took ${end.toFixed(2)}ms`);
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