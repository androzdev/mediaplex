const binding = require('./js-binding');
const { Readable } = require('stream');

/**
 * @typedef StreamProbeResult
 * @property {Readable} stream The source stream
 * @property {binding.ProbeResult} result The probe result
 */

/**
 * Attempt to probe a Readable stream
 * @param {Readable} stream The readable stream to probe
 * @param {Object} options The options
 * @param {number} [options.probeSize=2 * 1024 * 1024] The amount of bytes to read from the stream. Defaults to 2 MB.
 * @param {boolean} [options.sync=false] Whether to probe synchronously. Defaults to false.
 * @returns {Promise<StreamProbeResult>}
 */
async function probeStream(
    stream,
    options = {
        probeSize: 2 * 1024 * 1024,
        sync: false,
    }
) {
    const { probeSize = 2 * 1024 * 1024, sync = false } = options;
    return new Promise((resolve, reject) => {
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
                const probeInner = sync ? probeSync : probe;
                const result = await probeInner(readBuffer);
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

const { CodecType, probe, probeSync, OpusEncoder, getOpusVersion } = binding;
const { version } = require('./package.json');

module.exports = {
    CodecType,
    probe,
    probeSync,
    probeStream,
    readMetadata,
    OpusEncoder,
    getOpusVersion,
    version
};