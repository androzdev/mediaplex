const binding = require('./js-binding');
const { Readable } = require('stream');
const { Worker } = require('worker_threads');

/**
 * @typedef StreamProbeResult
 * @property {Readable} stream The source stream
 * @property {binding.ProbeResult} result The probe result
 */

/**
 * Attempt to probe a Readable stream
 * @param {Readable} stream The readable stream to probe
 * @param {number} [probeSize] The number of bytes to read. Defaults to 10 MB
 * @returns {Promise<StreamProbeResult>}
 */
async function probeStream(
    stream,
    probeSize = 1024 * 1024 * 10
) {
    return new Promise((resolve, reject) => {
        // preconditions
        if (stream.readableObjectMode) {
            reject(new Error('Cannot probe a readable stream in object mode'));
            return;
        }

        if (stream.readableEnded) {
            reject(new Error('Cannot probe a stream that has ended'));
            return;
        }

        let readBuffer = Buffer.alloc(0);

        /**
         * @type {binding.ProbeResult}
        */
        let resolved = null;

        const finish = (data) => {
            stream.off('data', onData);
            stream.off('close', onClose);
            stream.off('end', onClose);
            stream.pause();

            resolved = data;

            if (stream.readableEnded) {
                resolve({
                    stream: Readable.from(readBuffer),
                    result: data,
                });
            } else {
                if (readBuffer.length > 0) {
                    stream.push(readBuffer);
                }

                resolve({
                    stream,
                    result: data,
                });
            }
        };

        const onClose = async () => {
            try {
                const result = await toThread(readBuffer);
                if (result != null) resolved = result;
            } catch { }

            finish(resolved || null);
        };

        const onData = (buffer) => {
            readBuffer = Buffer.concat([readBuffer, buffer]);

            if (readBuffer.length >= probeSize) {
                stream.off('data', onData);
                stream.pause();
                process.nextTick(onClose);
            }
        };

        stream.once('error', reject);
        stream.on('data', onData);
        stream.once('close', onClose);
        stream.once('end', onClose);
    });
}

async function toThread(buffer) {
    return new Promise((resolve, reject) => {
        const workerScript = ';('.concat((function MEDIAPLEX_PROBE_METADATA() {
            const { probe } = require('./js-binding');
            const { parentPort, workerData } = require('worker_threads');

            if (!parentPort || !workerData) {
                throw new Error('Could not probe on worker thread');
            }

            try {
                const result = probe(workerData);
                parentPort.postMessage({
                    error: false,
                    value: result
                });
            } catch (err) {
                parentPort.postMessage({
                    error: true,
                    value: err
                });
            } finally {
                process.exit(0);
            }
        }).toString()).concat(')();');

        const worker = new Worker(workerScript, {
            eval: true,
            workerData: buffer
        });

        worker.unref();

        const terminator = setTimeout(() => {
            if (markAsResolved) return;
            worker.terminate().catch(() => null);
        }, 5000).unref();

        const markAsResolved = () => clearTimeout(terminator);

        worker.once('message', (result) => {
            markAsResolved();
            if (result.error) {
                reject(result.value);
            } else {
                resolve(result.value);
            }

            worker.terminate().catch(() => null);
        });

        worker.once('error', (e) => {
            markAsResolved();
            reject(e);
            worker.terminate().catch(() => null);
        });
    });
}

/**
 * Reads metadata tags
 * @param {import('./js-binding').ProbeResult} result The probe result to read metadata from
 */
function readMetadata(result) {
    const res = {
        title: null,
        author: null,
        album: null,
        genre: null,
        year: null,
        duration: null,
        composer: null,
        bpm: null
    };

    if (!result.metadata?.length) return res;

    result.metadata.forEach(m => {
        if (!m.value) return;

        if (m.name === 'TIT2') {
            res.title = m.value;
        } else if (m.name === 'TPE1' || m.name === 'TPUB') {
            res.author = m.value;
        } else if (m.name === 'TALB') {
            res.album = m.value;
        } else if (m.name === 'TCON') {
            res.genre = m.value;
        } else if (m.name === 'TYER') {
            res.year = m.value;
        } else if (m.name === 'TLEN') {
            res.duration = parseInt(m.value);
        } else if (m.name === 'TCOM') {
            res.composer = m.value;
        } else if (m.name === 'TBPM') {
            res.bpm = parseInt(m.value);
        }
    });

    return res;
}

const { CodecType, probe, OpusEncoder, getOpusVersion } = binding;
const { version } = require('./package.json');

module.exports = {
    CodecType,
    probe,
    probeStream,
    readMetadata,
    OpusEncoder,
    getOpusVersion,
    version
};