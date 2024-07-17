crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_Interpolators.h]

/**
  | A collection of different interpolators
  | for resampling streams of floats.
  | 
  | @see GenericInterpolator, WindowedSincInterpolator,
  | LagrangeInterpolator, CatmullRomInterpolator,
  | LinearInterpolator, ZeroOrderHoldInterpolator
  | 
  | @tags{Audio}
  |
  */
pub mod interpolators {

    use super::*;

    pub trait InterpolatorTraits {

        const ALGORITHMIC_LATENCY: f32;

        fn value_at_offset(inputs: *const f32, offset: f32, index: i32) -> f32;
    }

    pub struct WindowedSincTraits;

    pub mod windowed_sinc {

        use super::*;

        lazy_static!{
            /*
               static const float lookupTable[10001];
               */
        }
    }

    impl WindowedSincTraits {

        #[inline(always)] pub fn windowed_sinc(
            first_frac: f32,
            index:      i32) -> f32 {
            
            todo!();
            /*
                auto index2 = index + 1;
                    auto frac = firstFrac;

                    auto value1 = lookupTable[index];
                    auto value2 = lookupTable[index2];

                    return value1 + (frac * (value2 - value1));
            */
        }
    }

    impl InterpolatorTraits for WindowedSincTraits {

        const ALGORITHMIC_LATENCY: f32 = 100.0;
        
        #[inline(always)] fn value_at_offset(
            inputs:       *const f32,
            offset:       f32,
            index_buffer: i32) -> f32 {
            
            todo!();
            /*
                const int numCrossings = 100;
                    const float floatCrossings = (float) numCrossings;
                    float result = 0.0f;

                    auto samplePosition = indexBuffer;
                    float firstFrac = 0.0f;
                    float lastSincPosition = -1.0f;
                    int index = 0, sign = -1;

                    for (int i = -numCrossings; i <= numCrossings; ++i)
                    {
                        auto sincPosition = (1.0f - offset) + (float) i;

                        if (i == -numCrossings || (sincPosition >= 0 && lastSincPosition < 0))
                        {
                            auto indexFloat = (sincPosition >= 0.f ? sincPosition : -sincPosition) * 100.0f;
                            auto indexFloored = std::floor (indexFloat);
                            index = (int) indexFloored;
                            firstFrac = indexFloat - indexFloored;
                            sign = (sincPosition < 0 ? -1 : 1);
                        }

                        if (sincPosition == 0.0f)
                            result += inputs[samplePosition];
                        else if (sincPosition < floatCrossings && sincPosition > -floatCrossings)
                            result += inputs[samplePosition] * windowedSinc (firstFrac, index);

                        if (++samplePosition == numCrossings * 2)
                            samplePosition = 0;

                        lastSincPosition = sincPosition;
                        index += 100 * sign;
                    }

                    return result;
            */
        }
    }

    pub struct LagrangeTraits;

    pub mod lagrange {

        //-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_LagrangeInterpolator.cpp]

        pub fn calc_coefficient<const K: usize>(input: &mut f32, offset: f32) -> f32 
        where 
            [(); {4 - K}]: ,
            [(); {3 - K}]: ,
            [(); {2 - K}]: ,
            [(); {1 - K}]: ,
            [(); {0 - K}]: ,
        {
            LagrangeResampleHelper::<{0 - K}>::calc(input, -2.0 - offset);
            LagrangeResampleHelper::<{1 - K}>::calc(input, -1.0 - offset);
            LagrangeResampleHelper::<{2 - K}>::calc(input, 0.0 - offset);
            LagrangeResampleHelper::<{3 - K}>::calc(input, 1.0 - offset);
            LagrangeResampleHelper::<{4 - K}>::calc(input, 2.0 - offset);
            *input
        }

        pub struct LagrangeResampleHelper<const K: usize>;

        impl<const K: usize> LagrangeResampleHelper<K> {

            #[inline]
            pub fn calc(a: &mut f32, b: f32) {
                if K == 0 {
                    return;
                }
                *a *= b * (1.0 / K as f32);
            }
        }
    }


    impl InterpolatorTraits for LagrangeTraits {

        const ALGORITHMIC_LATENCY: f32 = 2.0;

        fn value_at_offset(
            inputs: *const f32,
            offset: f32,
            index:  i32) -> f32 {

            todo!();
            /*
                float result = 0.0f;

            result += calcCoefficient<0> (inputs[index], offset); if (++index == 5) index = 0;
            result += calcCoefficient<1> (inputs[index], offset); if (++index == 5) index = 0;
            result += calcCoefficient<2> (inputs[index], offset); if (++index == 5) index = 0;
            result += calcCoefficient<3> (inputs[index], offset); if (++index == 5) index = 0;
            result += calcCoefficient<4> (inputs[index], offset);

            return result;
            */
        }
    }

    pub struct CatmullRomTraits;

    impl InterpolatorTraits for CatmullRomTraits {

        const ALGORITHMIC_LATENCY: f32 = 2.0;

        fn value_at_offset(inputs: *const f32, offset: f32, index: i32) -> f32 {
            todo!();
            /*
                auto y0 = inputs[index]; if (++index == 4) index = 0;
                        auto y1 = inputs[index]; if (++index == 4) index = 0;
                        auto y2 = inputs[index]; if (++index == 4) index = 0;
                        auto y3 = inputs[index];

                        auto halfY0 = 0.5f * y0;
                        auto halfY3 = 0.5f * y3;

                        return y1 + offset * ((0.5f * y2 - halfY0)
                                  + (offset * (((y0 + 2.0f * y2) - (halfY3 + 2.5f * y1))
                                  + (offset * ((halfY3 + 1.5f * y1) - (halfY0 + 1.5f * y2))))));
            */

        }
    }

    pub struct LinearTraits;

    impl InterpolatorTraits for LinearTraits {

        const ALGORITHMIC_LATENCY: f32 = 1.0;

        fn value_at_offset(inputs: *const f32, offset: f32, index: i32) -> f32 {

            todo!();
            /*
                auto y0 = inputs[index];
                        auto y1 = inputs[index == 0 ? 1 : 0];

                        return y1 * offset + y0 * (1.0f - offset);
            */
        }
    }

    pub struct ZeroOrderHoldTraits;

    impl InterpolatorTraits for ZeroOrderHoldTraits {

        const ALGORITHMIC_LATENCY: f32 = 0.0;

        fn value_at_offset(inputs: *const f32, _1: f32, _2: i32) -> f32 {

            todo!();
            /*
                return inputs[0];
            */
        }
    }

    pub type WindowedSinc  = GenericInterpolator<WindowedSincTraits,200>;
    pub type Lagrange      = GenericInterpolator<LagrangeTraits,5>;
    pub type CatmullRom    = GenericInterpolator<CatmullRomTraits,4>;
    pub type Linear        = GenericInterpolator<LinearTraits,2>;
    pub type ZeroOrderHold = GenericInterpolator<ZeroOrderHoldTraits,1>;

}

/**
  | An interpolator for resampling a stream
  | of floats using high order windowed
  | (hann) sinc interpolation, recommended
  | for high quality resampling.
  | 
  | Note that the resampler is stateful,
  | so when there's a break in the continuity
  | of the input stream you're feeding it,
  | you should call reset() before feeding
  | it any new data. And like with any other
  | stateful filter, if you're resampling
  | multiple channels, make sure each one
  | uses its own LinearInterpolator object.
  | 
  | @see GenericInterpolator
  | 
  | @see LagrangeInterpolator, CatmullRomInterpolator,
  | LinearInterpolator, ZeroOrderHoldInterpolator
  | 
  | @tags{Audio}
  |
  */
pub type WindowedSincInterpolator = interpolators::WindowedSinc;

/**
  | An interpolator for resampling a stream
  | of floats using 4-point lagrange interpolation.
  | 
  | Note that the resampler is stateful,
  | so when there's a break in the continuity
  | of the input stream you're feeding it,
  | you should call reset() before feeding
  | it any new data. And like with any other
  | stateful filter, if you're resampling
  | multiple channels, make sure each one
  | uses its own LagrangeInterpolator
  | object.
  | 
  | @see GenericInterpolator
  | 
  | @see CatmullRomInterpolator, WindowedSincInterpolator,
  | LinearInterpolator, ZeroOrderHoldInterpolator
  | 
  | @tags{Audio}
  |
  */
pub type LagrangeInterpolator = interpolators::Lagrange;

/**
  | An interpolator for resampling a stream
  | of floats using Catmull-Rom interpolation.
  | 
  | Note that the resampler is stateful,
  | so when there's a break in the continuity
  | of the input stream you're feeding it,
  | you should call reset() before feeding
  | it any new data. And like with any other
  | stateful filter, if you're resampling
  | multiple channels, make sure each one
  | uses its own CatmullRomInterpolator
  | object.
  | 
  | @see GenericInterpolator
  | 
  | @see LagrangeInterpolator, WindowedSincInterpolator,
  | LinearInterpolator, ZeroOrderHoldInterpolator
  | 
  | @tags{Audio}
  |
  */
pub type CatmullRomInterpolator = interpolators::CatmullRom;

/**
  | An interpolator for resampling a stream
  | of floats using linear interpolation.
  | 
  | Note that the resampler is stateful,
  | so when there's a break in the continuity
  | of the input stream you're feeding it,
  | you should call reset() before feeding
  | it any new data. And like with any other
  | stateful filter, if you're resampling
  | multiple channels, make sure each one
  | uses its own LinearInterpolator object.
  | 
  | @see GenericInterpolator
  | 
  | @see LagrangeInterpolator, CatmullRomInterpolator,
  | WindowedSincInterpolator, ZeroOrderHoldInterpolator
  | 
  | @tags{Audio}
  |
  */
pub type LinearInterpolator = interpolators::Linear;

/**
  | An interpolator for resampling a stream
  | of floats using zero order hold interpolation.
  | 
  | Note that the resampler is stateful,
  | so when there's a break in the continuity
  | of the input stream you're feeding it,
  | you should call reset() before feeding
  | it any new data. And like with any other
  | stateful filter, if you're resampling
  | multiple channels, make sure each one
  | uses its own ZeroOrderHoldInterpolator
  | object.
  | 
  | @see GenericInterpolator
  | 
  | @see LagrangeInterpolator, CatmullRomInterpolator,
  | WindowedSincInterpolator, LinearInterpolator
  | 
  | @tags{Audio}
  |
  */
pub type ZeroOrderHoldInterpolator = interpolators::ZeroOrderHold;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_Interpolators.cpp]

#[cfg(ALOE_UNIT_TESTS)]
#[test] fn interpolator_tests() {

    todo!();

    /*
    class InterpolatorTests  : public UnitTest
    {

        InterpolatorTests()
            : UnitTest ("InterpolatorTests", UnitTestCategories::audio)
        {
        }


        template <typename InterpolatorType>
        void runInterplatorTests (const String& interpolatorName)
        {
            auto createGaussian = [] (std::vector<float>& destination, float scale, float centreInSamples, float width)
            {
                for (size_t i = 0; i < destination.size(); ++i)
                {
                    auto x = (((float) i) - centreInSamples) * width;
                    destination[i] = std::exp (-(x * x));
                }

                FloatVectorOperations::multiply (destination.data(), scale, (int) destination.size());
            };

            auto findGaussianPeak = [] (const std::vector<float>& input) -> float
            {
                auto max = std::max_element (std::begin (input), std::end (input));
                auto maxPrev = max - 1;
                jassert (maxPrev >= std::begin (input));
                auto maxNext = max + 1;
                jassert (maxNext < std::end (input));
                auto quadraticMaxLoc = (*maxPrev - *maxNext) / (2.0f * ((*maxNext + *maxPrev) - (2.0f * *max)));
                return quadraticMaxLoc + (float) std::distance (std::begin (input), max);
            };

            auto expectAllElementsWithin = [this] (const std::vector<float>& v1, const std::vector<float>& v2, float tolerance)
            {
                expectEquals ((int) v1.size(), (int) v2.size());

                for (size_t i = 0; i < v1.size(); ++i)
                    expectWithinAbsoluteError (v1[i], v2[i], tolerance);
            };

            InterpolatorType interpolator;

            constexpr size_t inputSize = 1001;
            static_assert (inputSize > 800 + InterpolatorType::getBaseLatency(),
                           "The test InterpolatorTests input buffer is too small");

            std::vector<float> input (inputSize);
            constexpr auto inputGaussianMidpoint = (float) (inputSize - 1) / 2.0f;
            constexpr auto inputGaussianValueAtEnds = 0.000001f;
            const auto inputGaussianWidth = std::sqrt (-std::log (inputGaussianValueAtEnds)) / inputGaussianMidpoint;

            createGaussian (input, 1.0f, inputGaussianMidpoint, inputGaussianWidth);

            for (auto speedRatio : { 0.4, 0.8263, 1.0, 1.05, 1.2384, 1.6 })
            {
                const auto expectedGaussianMidpoint = (inputGaussianMidpoint + InterpolatorType::getBaseLatency()) / (float) speedRatio;
                const auto expectedGaussianWidth = inputGaussianWidth * (float) speedRatio;

                const auto outputBufferSize = (size_t) std::floor ((float) input.size() / speedRatio);

                for (int numBlocks : { 1, 5 })
                {
                    const auto inputBlockSize = (float) input.size() / (float) numBlocks;
                    const auto outputBlockSize = (int) std::floor (inputBlockSize / speedRatio);

                    std::vector<float> output (outputBufferSize, std::numeric_limits<float>::min());

                    beginTest (interpolatorName + " process " + String (numBlocks) + " blocks ratio " + String (speedRatio));

                    interpolator.reset();

                    {
                        auto* inputPtr = input.data();
                        auto* outputPtr = output.data();

                        for (int i = 0; i < numBlocks; ++i)
                        {
                            auto numInputSamplesRead = interpolator.process (speedRatio, inputPtr, outputPtr, outputBlockSize);
                            inputPtr += numInputSamplesRead;
                            outputPtr += outputBlockSize;
                        }
                    }

                    expectWithinAbsoluteError (findGaussianPeak (output), expectedGaussianMidpoint, 0.1f);

                    std::vector<float> expectedOutput (output.size());
                    createGaussian (expectedOutput, 1.0f, expectedGaussianMidpoint, expectedGaussianWidth);

                    expectAllElementsWithin (output, expectedOutput, 0.02f);

                    beginTest (interpolatorName + " process adding " + String (numBlocks) + " blocks ratio " + String (speedRatio));

                    interpolator.reset();

                    constexpr float addingGain = 0.7384f;

                    {
                        auto* inputPtr = input.data();
                        auto* outputPtr = output.data();

                        for (int i = 0; i < numBlocks; ++i)
                        {
                            auto numInputSamplesRead = interpolator.processAdding (speedRatio, inputPtr, outputPtr, outputBlockSize, addingGain);
                            inputPtr += numInputSamplesRead;
                            outputPtr += outputBlockSize;
                        }
                    }

                    expectWithinAbsoluteError (findGaussianPeak (output), expectedGaussianMidpoint, 0.1f);

                    std::vector<float> additionalOutput (output.size());
                    createGaussian (additionalOutput, addingGain, expectedGaussianMidpoint, expectedGaussianWidth);
                    FloatVectorOperations::add (expectedOutput.data(), additionalOutput.data(), (int) additionalOutput.size());

                    expectAllElementsWithin (output, expectedOutput, 0.02f);
                }

                beginTest (interpolatorName + " process wrap 0 ratio " + String (speedRatio));

                std::vector<float> doubleLengthOutput (2 * outputBufferSize, std::numeric_limits<float>::min());

                interpolator.reset();
                interpolator.process (speedRatio, input.data(), doubleLengthOutput.data(), (int) doubleLengthOutput.size(),
                                      (int) input.size(), 0);

                std::vector<float> expectedDoubleLengthOutput (doubleLengthOutput.size());
                createGaussian (expectedDoubleLengthOutput, 1.0f, expectedGaussianMidpoint, expectedGaussianWidth);

                expectAllElementsWithin (doubleLengthOutput, expectedDoubleLengthOutput, 0.02f);

                beginTest (interpolatorName + " process wrap double ratio " + String (speedRatio));

                interpolator.reset();
                interpolator.process (speedRatio, input.data(), doubleLengthOutput.data(), (int) doubleLengthOutput.size(),
                                      (int) input.size(), (int) input.size());

                std::vector<float> secondGaussian (doubleLengthOutput.size());
                createGaussian (secondGaussian, 1.0f, expectedGaussianMidpoint + (float) outputBufferSize, expectedGaussianWidth);
                FloatVectorOperations::add (expectedDoubleLengthOutput.data(), secondGaussian.data(), (int) expectedDoubleLengthOutput.size());

                expectAllElementsWithin (doubleLengthOutput, expectedDoubleLengthOutput, 0.02f);
            }
        }


        void runTest() override
        {
            runInterplatorTests<WindowedSincInterpolator> ("WindowedSincInterpolator");
            runInterplatorTests<LagrangeInterpolator>     ("LagrangeInterpolator");
            runInterplatorTests<CatmullRomInterpolator>   ("CatmullRomInterpolator");
            runInterplatorTests<LinearInterpolator>       ("LinearInterpolator");
        }
    };

    static InterpolatorTests interpolatorTests;

    */
}
