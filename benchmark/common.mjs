import { readFileSync } from 'node:fs';
import djs from '@discordjs/opus';
import opusscript from 'opusscript';
import mediaplex from '../index.js';
import * as evanOpus from '@evan/opus';
import * as evanOpusWasm from '@evan/opus/wasm/index.mjs';
import * as simdEvanOpus from '@evan/wasm/target/opus/node.mjs';

export const generatePCMSample = (sampleSize) => {
    return readFileSync(new URL(`./data/sample.pcm`, import.meta.url)).subarray(0, sampleSize);
}

export const generateOpusSample = () => {
    return readFileSync(new URL(`./data/sample.opus`, import.meta.url));
}

export const createMediaplexEncoder = (config) => new mediaplex.OpusEncoder(config.SAMPLE_RATE, config.CHANNELS);
export const createDjsEncoder = (config) => new djs.OpusEncoder(config.SAMPLE_RATE, config.CHANNELS);
export const createOpusScriptWasmEncoder = (config) => new opusscript(config.SAMPLE_RATE, config.CHANNELS, opusscript.Application.AUDIO, {
    wasm: true
});
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
export const createSimdEvanOpusEncoder = (config) => new simdEvanOpus.Encoder({
    application: 'voip',
    channels: config.CHANNELS,
    sample_rate: config.SAMPLE_RATE
});
export const createSimdEvanOpusDecoder = (config) => new simdEvanOpus.Decoder({
    channels: config.CHANNELS,
    sample_rate: config.SAMPLE_RATE
});