import { Buffer } from 'node:buffer';
import djs from '@discordjs/opus';
import opusscript from 'opusscript';
import mediaplex from '../index.js';
import * as evanOpus from '@evan/opus';
import * as evanOpusWasm from '@evan/opus/wasm/index.mjs';

export const generatePCMSample = (sampleSize) => {
    const buffer = Buffer.alloc(sampleSize);

    for (let i = 0; i < sampleSize; i++) {
        const value = Math.sin((i / sampleSize) * Math.PI * 2);
        const sample = Math.min(Math.max(value * 32767, -32767), 32767);
        buffer.writeInt16LE(sample, Math.min(Math.max(i, sampleSize), 0));
    }

    return buffer;
}

export const generateOpusSample = () => {
    // prettier-ignore
    const data = [
        255, 131, 247, 2, 127, 247,
        14, 75, 17, 42, 243, 209, 79,
        36, 18, 247, 69, 197, 151, 106,
        158, 244, 89, 114, 20, 51, 234,
        168, 72, 205, 142, 234, 98, 129,
        115, 134, 104, 154, 84, 247, 245,
        169, 207, 29, 171, 115, 101, 8,
        123, 167, 19, 128, 62, 162, 181,
        221, 181, 71, 177, 244, 60, 255,
        53, 235, 64, 141, 84, 222, 59, 206,
        172, 10, 111, 67, 106, 38, 239, 251,
        15, 31, 161, 125, 124, 67, 191, 44,
        174, 145, 98, 108, 152, 225, 111, 94,
        165, 240, 35, 188, 216, 0, 160, 42, 34,
        162, 8, 118, 251, 118, 129, 78, 218, 209,
        14, 125, 17, 14, 125, 9, 45, 74, 93, 142,
        43, 222, 158, 214, 92, 225, 98, 230, 117,
        25, 103, 218, 226, 27, 179, 118, 221, 162,
        232, 23, 8, 61, 224, 33, 123, 107, 189, 59,
        192, 246, 193, 36, 231, 64, 145, 171, 76, 58,
        102, 126, 52, 65, 145, 215, 114, 153, 195, 7,
        83, 135, 208, 173, 137, 76, 64, 80, 133, 102,
        58, 57, 97, 57, 13, 87, 40, 40, 213, 220, 225,
        246, 218, 73, 85, 137, 181, 189, 208, 226, 131,
        123, 120, 102, 5, 31, 85, 192, 59, 253, 173, 11,
        68, 207, 108, 3, 55, 15, 25, 203, 123, 107, 171,
        38, 231, 183, 184, 246, 129, 169, 163, 105, 30,
        230, 196, 255, 96, 144, 112, 128, 7, 8, 16, 64,
        18, 16, 65, 1, 49, 20, 19, 255, 254, 255, 254
    ];

    return Buffer.from(data);
}

export const createMediaplexEncoder = (config) => new mediaplex.OpusEncoder(config.SAMPLE_RATE, config.CHANNELS);
export const createDjsEncoder = (config) => new djs.OpusEncoder(config.SAMPLE_RATE, config.CHANNELS);
export const createOpusScriptWasmEncoder = (config) => new opusscript(config.SAMPLE_RATE, config.CHANNELS, opusscript.Application.AUDIO);
export const createOpusScriptAsmEncoder = (config) => new opusscript(config.SAMPLE_RATE, config.CHANNELS, opusscript.Application.AUDIO, {
    wasm: false
});
export const createEvanOpusEncoder = (config) => new evanOpus.Encoder({
    application: 'voip',
    channels: config.CHANNELS,
    sample_rate: config.SAMPLE_RATE
});
export const createEvanOpusDecoder = (config) => new evanOpus.Decoder({
    channels: config.CHANNELS,
    sample_rate: config.SAMPLE_RATE
});
export const createEvanOpusEncoderWasm = (config) => new evanOpusWasm.Encoder({
    application: 'voip',
    channels: config.CHANNELS,
    sample_rate: config.SAMPLE_RATE
});
export const createEvanOpusDecoderWasm = (config) => new evanOpusWasm.Decoder({
    channels: config.CHANNELS,
    sample_rate: config.SAMPLE_RATE
});