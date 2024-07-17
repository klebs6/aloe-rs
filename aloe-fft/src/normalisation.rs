crate::ix!();

pub fn calculate_normalisation_factor(sum_squared_magnitude: f32) -> f32 {
    
    todo!();
    /*
        if (sumSquaredMagnitude < 1e-8f)
            return 1.0f;

        return 0.125f / std::sqrt (sumSquaredMagnitude);
    */
}

pub fn normalise_impulse_response(buf: &mut AudioBuffer<f32>)  {
    
    todo!();
    /*
        const auto numChannels = buf.getNumChannels();
        const auto numSamples  = buf.getNumSamples();
        const auto channelPtrs = buf.getArrayOfWritePointers();

        const auto maxSumSquaredMag = std::accumulate (channelPtrs, channelPtrs + numChannels, 0.0f, [numSamples] (auto max, auto* channel)
        {
            return jmax (max, std::accumulate (channel, channel + numSamples, 0.0f, [] (auto sum, auto samp)
            {
                return sum + (samp * samp);
            }));
        });

        const auto normalisationFactor = calculateNormalisationFactor (maxSumSquaredMag);

        std::for_each (channelPtrs, channelPtrs + numChannels, [normalisationFactor, numSamples] (auto* channel)
        {
            FloatVectorOperations::multiply (channel, normalisationFactor, numSamples);
        });
    */
}
