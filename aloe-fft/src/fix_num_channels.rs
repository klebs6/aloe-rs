crate::ix!();

pub fn fix_num_channels(
    buf:    &AudioBuffer<f32>,
    stereo: ConvolutionStereo

) -> AudioBuffer<f32> {
    
    todo!();
    /*
        const auto numChannels = jmin (buf.getNumChannels(), stereo == Convolution::ConvolutionStereo::yes ? 2 : 1);
        const auto numSamples = buf.getNumSamples();

        AudioBuffer<float> result (numChannels, buf.getNumSamples());

        for (auto channel = 0; channel != numChannels; ++channel)
            result.copyFrom (channel, 0, buf.getReadPointer (channel), numSamples);

        if (result.getNumSamples() == 0 || result.getNumChannels() == 0)
        {
            result.setSize (1, 1);
            result.setSample (0, 0, 1.0f);
        }

        return result;
    */
}

