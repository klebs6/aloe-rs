crate::ix!();

pub fn convert_fixed_to_float(
    channels:     *const *const i32,
    num_channels: i32,
    num_samples:  i32

) {
    todo!();

        /*
            constexpr auto scaleFactor = 1.0f / static_cast<float> (0x7fffffff);

        for (int i = 0; i < numChannels; ++i)
            if (auto d = channels[i])
                FloatVectorOperations::convertFixedToFloat (reinterpret_cast<float*> (d), d, scaleFactor, numSamples);
        */
}

pub fn convert_floats_to_ints(
    dest:        *mut i32,
    src:         *const f32,
    num_samples: i32

) {
    todo!();

        /*
            while (--numSamples >= 0)
        {
            const double samp = *src++;

            if (samp <= -1.0)
                *dest = std::numeric_limits<int>::min();
            else if (samp >= 1.0)
                *dest = std::numeric_limits<int>::max();
            else
                *dest = roundToInt (std::numeric_limits<int>::max() * samp);

            ++dest;
        }
        */
}
