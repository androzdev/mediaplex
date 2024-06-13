const { OpusEncoder: OpusEncoderNative, getOpusVersion, CodecType, probe, probeSync } = require('./js-binding')

class OpusEncoder {
  constructor(sampleRate, channels) {
    return OpusEncoderNative.create(sampleRate, channels)
  }

  static create(sampleRate, channels) {
    return OpusEncoderNative.create(sampleRate, channels)
  }

  static [Symbol.hasInstance](target) {
    return target instanceof OpusEncoderNative
  }
}

module.exports = {
  OpusEncoder,
  getOpusVersion,
  CodecType,
  probe,
  probeSync
}