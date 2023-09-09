import b from 'benny';
import { createDjsEncoder, createMediaplexEncoder, createOpusScriptAsmEncoder, createOpusScriptWasmEncoder, generateOpusSample, createEvanOpusDecoder, createEvanOpusDecoderWasm } from './common.mjs';

const config = {
    FRAME_SIZE: 960,
    SAMPLE_RATE: 48000,
    CHANNELS: 2,
};

const mediaplexEncoder = createMediaplexEncoder(config);
const nativeEncoder = createDjsEncoder(config);
const wasmEncoder = createOpusScriptWasmEncoder(config);
const asmEncoder = createOpusScriptAsmEncoder(config);
const evanOpus = createEvanOpusDecoder(config);
const evanOpusWasm = createEvanOpusDecoderWasm(config);

const SAMPLE = generateOpusSample();

b.suite(
    'OpusDecoder Benchmark',
    b.add('mediaplex', () => {
        mediaplexEncoder.decode(SAMPLE);
    }),
    b.add('@discordjs/opus', () => {
        nativeEncoder.decode(SAMPLE);
    }),
    b.add('@evan/opus', () => {
        evanOpus.decode(SAMPLE);
    }),
    b.add('@evan/opus (wasm)', () => {
        evanOpusWasm.decode(SAMPLE);
    }),
    b.add('opusscript', () => {
        wasmEncoder.decode(SAMPLE);
    }),
    b.add('opusscript (no wasm)', () => {
        asmEncoder.decode(SAMPLE);
    }),
    b.cycle(),
    b.complete(),
);