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
 * @param {number} [probeSize] The number of bytes to read
 * @returns {Promise<StreamProbeResult>}
 */
async function probeStream(
    stream,
    probeSize = 1024
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

        const onClose = () => {
            if (!resolved) {
                finish(null);
            }
        };

        const onData = (buffer) => {
            readBuffer = Buffer.concat([readBuffer, buffer]);

            try {
                const result = binding.probe(readBuffer);
                return finish(result);
            } catch { }

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
        genre: null
    };

    if (!result.metadata?.length) return res;

    result.metadata.forEach(m => {
        if (!m.value) return;

        if ((m.name === 'TPE1' || m.name === 'TPUB') && !res['author']) {
            res['author'] = m.value;
        } else if (m.name === 'TIT2') {
            res['title'] = m.value;
        } else if ((m.name === 'TALB' || m.name === 'TCON') && !res['genre']) {
            res['genre'] = m.value;
        }
    });

    return res;
}

const { CodecType, probe } = binding;

module.exports = {
    CodecType,
    probe,
    probeStream,
    readMetadata
};