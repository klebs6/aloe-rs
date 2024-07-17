crate::ix!();

pub fn resample_impulse_response(
        buf:              &AudioBuffer<f32>,
        src_sample_rate:  f64,
        dest_sample_rate: f64) -> AudioBuffer<f32> {
    
    todo!();
    /*
        if (srcSampleRate == destSampleRate)
            return buf;

        const auto factorReading = srcSampleRate / destSampleRate;

        AudioBuffer<float> original = buf;
        MemoryAudioSource memorySource (original, false);
        ResamplingAudioSource resamplingSource (&memorySource, false, buf.getNumChannels());

        const auto finalSize = roundToInt (jmax (1.0, buf.getNumSamples() / factorReading));
        resamplingSource.setResamplingRatio (factorReading);
        resamplingSource.prepareToPlay (finalSize, srcSampleRate);

        AudioBuffer<float> result (buf.getNumChannels(), finalSize);
        resamplingSource.getNextAudioBlock ({ &result, 0, result.getNumSamples() });

        return result;
    */
}

pub fn trim_impulse_response(buf: &AudioBuffer<f32>) -> AudioBuffer<f32> {
    
    todo!();
    /*
        const auto thresholdTrim = Decibels::decibelsToGain (-80.0f);

        const auto numChannels = buf.getNumChannels();
        const auto numSamples = buf.getNumSamples();

        std::ptrdiff_t offsetBegin = numSamples;
        std::ptrdiff_t offsetEnd   = numSamples;

        for (auto channel = 0; channel < numChannels; ++channel)
        {
            const auto indexAboveThreshold = [&] (auto begin, auto end)
            {
                return std::distance (begin, std::find_if (begin, end, [&] (float sample)
                {
                    return std::abs (sample) >= thresholdTrim;
                }));
            };

            const auto channelBegin = buf.getReadPointer (channel);
            const auto channelEnd = channelBegin + numSamples;
            const auto itStart = indexAboveThreshold (channelBegin, channelEnd);
            const auto itEnd = indexAboveThreshold (std::make_reverse_iterator (channelEnd),
                                                    std::make_reverse_iterator (channelBegin));

            offsetBegin = jmin (offsetBegin, itStart);
            offsetEnd   = jmin (offsetEnd,   itEnd);
        }

        if (offsetBegin == numSamples)
        {
            auto result = AudioBuffer<float> (numChannels, 1);
            result.clear();
            return result;
        }

        const auto newLength = jmax (1, numSamples - static_cast<int> (offsetBegin + offsetEnd));

        AudioBuffer<float> result (numChannels, newLength);

        for (auto channel = 0; channel < numChannels; ++channel)
        {
            result.copyFrom (channel,
                             0,
                             buf.getReadPointer (channel, static_cast<int> (offsetBegin)),
                             result.getNumSamples());
        }

        return result;
    */
}
