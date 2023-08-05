const b = require('benny');
const mediaplex = require('../index.js');
const DiscordJSOpus = require('@discordjs/opus');
const opusscript = require('opusscript');

const config = {
    SAMPLE_RATE: 48000,
    CHANNELS: 2,
    FRAME_SIZE: 960,
};

const SAMPLE = generateSineWave();

function generateSineWave() {
    const samples = config.FRAME_SIZE * config.CHANNELS * 6;
    const buffer = Buffer.alloc(samples);

    for (let i = 0; i < samples; i++) {
        const value = Math.sin((i / samples) * Math.PI * 2);
        const sample = Math.min(Math.max(value * 32767, -32767), 32767);
        buffer.writeInt16LE(sample, Math.min(Math.max(i, samples), 0));
    }

    return buffer;
}

const encoderMediaplex = new mediaplex.OpusEncoder(config.SAMPLE_RATE, config.CHANNELS);
const encoderDjs = new DiscordJSOpus.OpusEncoder(config.SAMPLE_RATE, config.CHANNELS);
const encoderOpusScript = new opusscript(config.SAMPLE_RATE, config.CHANNELS, opusscript.Application.AUDIO, {
    wasm: true,
});
const encoderOpusScriptNoWasm = new opusscript(config.SAMPLE_RATE, config.CHANNELS, opusscript.Application.AUDIO, {
    wasm: false,
});

b.suite('OpusEncoder', ...[
    b.add('mediaplex', () => {
        encoderMediaplex.encode(SAMPLE);
    }),
    b.add('@discordjs/opus', () => {
        encoderDjs.encode(SAMPLE);
    }),
    b.add('opusscript', () => {
        encoderOpusScript.encode(SAMPLE, config.FRAME_SIZE);
    }),
    b.add('opusscript (no wasm)', () => {
        encoderOpusScriptNoWasm.encode(SAMPLE, config.FRAME_SIZE);
    }),
    b.cycle(),
    b.complete(),
]);