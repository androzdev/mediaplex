import b from 'benny';
import { createDjsEncoder, createMediaplexEncoder, createOpusScriptAsmEncoder, createOpusScriptWasmEncoder, generatePCMSample, createEvanOpusEncoder, createEvanOpusEncoderWasm } from './common.mjs';

const config = {
    FRAME_SIZE: 960,
    SAMPLE_RATE: 48000,
    CHANNELS: 2,
};

const mediaplexEncoder = createMediaplexEncoder(config);
const nativeEncoder = createDjsEncoder(config);
const wasmEncoder = createOpusScriptWasmEncoder(config);
const asmEncoder = createOpusScriptAsmEncoder(config);
const evanOpus = createEvanOpusEncoder(config);
const evanOpusWasm = createEvanOpusEncoderWasm(config);

const SAMPLE = generatePCMSample(config.FRAME_SIZE * config.CHANNELS * 6);

b.suite(
    'OpusEncoder Benchmark',
    b.add('mediaplex', () => {
        mediaplexEncoder.encode(SAMPLE);
    }),
    b.add('@discordjs/opus', () => {
        nativeEncoder.encode(SAMPLE);
    }),
    b.add('@evan/opus', () => {
        evanOpus.encode(SAMPLE);
    }),
    b.add('@evan/opus (wasm)', () => {
        evanOpusWasm.encode(SAMPLE);
    }),
    b.add('opusscript', () => {
        wasmEncoder.encode(SAMPLE, config.FRAME_SIZE);
    }),
    b.add('opusscript (no wasm)', () => {
        asmEncoder.encode(SAMPLE, config.FRAME_SIZE);
    }),
    b.cycle(),
    b.complete(),
);