import * as mitata from 'mitata';
// prettier-ignore
import {
    createDjsEncoder,
    createMediaplexEncoder,
    createOpusScriptAsmEncoder,
    createOpusScriptWasmEncoder,
    generatePCMSample,
    createEvanOpusEncoder,
    createEvanOpusEncoderWasm,
    createSimdEvanOpusEncoder
} from './common.mjs';

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
const evanWasmOpus = createSimdEvanOpusEncoder(config);

const SAMPLE = generatePCMSample(config.FRAME_SIZE * config.CHANNELS * 6);

mitata.group('OpusEncoder', () => {
    mitata.bench('mediaplex', () => {
        mediaplexEncoder.encode(SAMPLE);
    });
    mitata.bench('@discordjs/opus', () => {
        nativeEncoder.encode(SAMPLE);
    });
    mitata.bench('opusscript', () => {
        wasmEncoder.encode(SAMPLE, config.FRAME_SIZE);
    });
    mitata.bench('opusscript (no wasm)', () => {
        asmEncoder.encode(SAMPLE, config.FRAME_SIZE);
    });
    mitata.bench('@evan/opus', () => {
        evanOpus.encode(SAMPLE);
    });
    mitata.bench('@evan/opus (wasm)', () => {
        evanOpusWasm.encode(SAMPLE);
    });
    mitata.bench('@evan/wasm', () => {
        evanWasmOpus.encode(SAMPLE);
    });
});

await mitata.run();