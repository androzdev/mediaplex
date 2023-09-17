import * as mitata from 'mitata';

// prettier-ignore
import {
    createDjsEncoder,
    createMediaplexEncoder,
    createOpusScriptAsmEncoder,
    createOpusScriptWasmEncoder,
    generateOpusSample,
    createEvanOpusDecoder,
    createEvanOpusDecoderWasm,
    createSimdEvanOpusDecoder
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
const evanOpus = createEvanOpusDecoder(config);
const evanOpusWasm = createEvanOpusDecoderWasm(config);
const evanWasmOpus = createSimdEvanOpusDecoder(config);

const SAMPLE = generateOpusSample();

mitata.group('OpusDecoder', () => {
    mitata.bench('mediaplex', () => {
        mediaplexEncoder.decode(SAMPLE);
    });
    mitata.bench('@discordjs/opus', () => {
        nativeEncoder.decode(SAMPLE);
    });
    mitata.bench('opusscript', () => {
        wasmEncoder.decode(SAMPLE);
    });
    mitata.bench('opusscript (no wasm)', () => {
        asmEncoder.decode(SAMPLE);
    });
    mitata.bench('@evan/opus', () => {
        evanOpus.decode(SAMPLE);
    });
    mitata.bench('@evan/opus (wasm)', () => {
        evanOpusWasm.decode(SAMPLE);
    });
    mitata.bench('@evan/wasm', () => {
        evanWasmOpus.decode(SAMPLE);
    });
});

await mitata.run();