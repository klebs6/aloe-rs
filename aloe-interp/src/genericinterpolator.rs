crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_GenericInterpolator.h]

/**
  | An interpolator base class for resampling
  | streams of floats.
  | 
  | Note that the resamplers are stateful,
  | so when there's a break in the continuity
  | of the input stream you're feeding it,
  | you should call reset() before feeding
  | it any new data. And like with any other
  | stateful filter, if you're resampling
  | multiple channels, make sure each one
  | uses its own interpolator object.
  | 
  | @see LagrangeInterpolator, CatmullRomInterpolator,
  | WindowedSincInterpolator, LinearInterpolator,
  | ZeroOrderHoldInterpolator
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct GenericInterpolator<T: interpolators::InterpolatorTraits,const MEMORY_SIZE: usize> {
    last_input_samples: [f32; MEMORY_SIZE],
    sub_sample_pos:     f64, // default = 1.0
    index_buffer:       i32, // default = 0
    phantom:            std::marker::PhantomData<T>,
}

impl<T: interpolators::InterpolatorTraits,const MEMORY_SIZE: usize> Default for GenericInterpolator<T,MEMORY_SIZE> {
    
    fn default() -> Self {
        todo!();
        /*


            reset()
        */
    }
}

impl<T: interpolators::InterpolatorTraits,const MEMORY_SIZE: usize> GenericInterpolator<T,MEMORY_SIZE> {

    /**
      | Returns the latency of the interpolation
      | algorithm in isolation.
      | 
      | In the context of resampling the total
      | latency of a process using the interpolator
      | is the base latency divided by the speed
      | ratio.
      |
      */
    pub fn get_base_latency() -> f32 {
        
        todo!();
        /*
            return InterpolatorTraits::algorithmicLatency;
        */
    }

    /**
      | Resets the state of the interpolator.
      | 
      | Call this when there's a break in the
      | continuity of the input data stream.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            indexBuffer = 0;
            subSamplePos = 1.0;
            std::fill (std::begin (lastInputSamples), std::end (lastInputSamples), 0.0f);
        */
    }

    /**
      | Resamples a stream of samples.
      | 
      | -----------
      | @param speedRatio
      | 
      | the number of input samples to use for
      | each output sample
      | ----------
      | @param inputSamples
      | 
      | the source data to read from. This must
      | contain at least (speedRatio * numOutputSamplesToProduce)
      | samples.
      | ----------
      | @param outputSamples
      | 
      | the buffer to write the results into
      | ----------
      | @param numOutputSamplesToProduce
      | 
      | the number of output samples that should
      | be created
      | 
      | -----------
      | @return
      | 
      | the actual number of input samples that
      | were used
      |
      */
    pub fn process(&mut self, 
        speed_ratio:                   f64,
        input_samples:                 *const f32,
        output_samples:                *mut f32,
        num_output_samples_to_produce: i32) -> i32 {
        
        todo!();
        /*
            return interpolate (speedRatio, inputSamples, outputSamples, numOutputSamplesToProduce);
        */
    }

    /**
      | Resamples a stream of samples.
      | 
      | -----------
      | @param speedRatio
      | 
      | the number of input samples to use for
      | each output sample
      | ----------
      | @param inputSamples
      | 
      | the source data to read from. This must
      | contain at least (speedRatio * numOutputSamplesToProduce)
      | samples.
      | ----------
      | @param outputSamples
      | 
      | the buffer to write the results into
      | ----------
      | @param numOutputSamplesToProduce
      | 
      | the number of output samples that should
      | be created
      | ----------
      | @param numInputSamplesAvailable
      | 
      | the number of available input samples.
      | If it needs more samples than available,
      | it either wraps back for wrapAround
      | samples, or it feeds zeroes
      | ----------
      | @param wrapAround
      | 
      | if the stream exceeds available samples,
      | it wraps back for wrapAround samples.
      | If wrapAround is set to 0, it will feed
      | zeroes.
      | 
      | -----------
      | @return
      | 
      | the actual number of input samples that
      | were used
      |
      */
    pub fn process_with_wrap(
        &mut self, 
        speed_ratio:                   f64,
        input_samples:                 *const f32,
        output_samples:                *mut f32,
        num_output_samples_to_produce: i32,
        num_input_samples_available:   i32,
        wrap_around:                   i32) -> i32 {
        
        todo!();
        /*
            return interpolate (speedRatio, inputSamples, outputSamples,
                                numOutputSamplesToProduce, numInputSamplesAvailable, wrapAround);
        */
    }

    /**
      | Resamples a stream of samples, adding
      | the results to the output data with a
      | gain.
      | 
      | -----------
      | @param speedRatio
      | 
      | the number of input samples to use for
      | each output sample
      | ----------
      | @param inputSamples
      | 
      | the source data to read from. This must
      | contain at least (speedRatio * numOutputSamplesToProduce)
      | samples.
      | ----------
      | @param outputSamples
      | 
      | the buffer to write the results to - the
      | result values will be added to any pre-existing
      | data in this buffer after being multiplied
      | by the gain factor
      | ----------
      | @param numOutputSamplesToProduce
      | 
      | the number of output samples that should
      | be created
      | ----------
      | @param gain
      | 
      | a gain factor to multiply the resulting
      | samples by before adding them to the
      | destination buffer
      | 
      | -----------
      | @return
      | 
      | the actual number of input samples that
      | were used
      |
      */
    pub fn process_adding(&mut self, 
        speed_ratio:                   f64,
        input_samples:                 *const f32,
        output_samples:                *mut f32,
        num_output_samples_to_produce: i32,
        gain:                          f32) -> i32 {
        
        todo!();
        /*
            return interpolateAdding (speedRatio, inputSamples, outputSamples, numOutputSamplesToProduce, gain);
        */
    }

    /**
      | Resamples a stream of samples, adding
      | the results to the output data with a
      | gain.
      | 
      | -----------
      | @param speedRatio
      | 
      | the number of input samples to use for
      | each output sample
      | ----------
      | @param inputSamples
      | 
      | the source data to read from. This must
      | contain at least (speedRatio * numOutputSamplesToProduce)
      | samples.
      | ----------
      | @param outputSamples
      | 
      | the buffer to write the results to - the
      | result values will be added to any pre-existing
      | data in this buffer after being multiplied
      | by the gain factor
      | ----------
      | @param numOutputSamplesToProduce
      | 
      | the number of output samples that should
      | be created
      | ----------
      | @param numInputSamplesAvailable
      | 
      | the number of available input samples.
      | If it needs more samples than available,
      | it either wraps back for wrapAround
      | samples, or it feeds zeroes
      | ----------
      | @param wrapAround
      | 
      | if the stream exceeds available samples,
      | it wraps back for wrapAround samples.
      | If wrapAround is set to 0, it will feed
      | zeroes.
      | ----------
      | @param gain
      | 
      | a gain factor to multiply the resulting
      | samples by before adding them to the
      | destination buffer
      | 
      | -----------
      | @return
      | 
      | the actual number of input samples that
      | were used
      |
      */
    pub fn process_adding_with_wrap(
        &mut self, 
        speed_ratio:                   f64,
        input_samples:                 *const f32,
        output_samples:                *mut f32,
        num_output_samples_to_produce: i32,
        num_input_samples_available:   i32,
        wrap_around:                   i32,
        gain:                          f32) -> i32 {
        
        todo!();
        /*
            return interpolateAdding (speedRatio, inputSamples, outputSamples,
                                      numOutputSamplesToProduce, numInputSamplesAvailable, wrapAround, gain);
        */
    }
    
    #[inline(always)] pub fn push_interpolation_sample(&mut self, new_value: f32)  {
        
        todo!();
        /*
            lastInputSamples[indexBuffer] = newValue;

            if (++indexBuffer == memorySize)
                indexBuffer = 0;
        */
    }
    
    #[inline(always)] pub fn push_interpolation_samples(&mut self, 
        input:                         *const f32,
        num_output_samples_to_produce: i32)  {
        
        todo!();
        /*
            if (numOutputSamplesToProduce >= memorySize)
            {
                const auto* const offsetInput = input + (numOutputSamplesToProduce - memorySize);

                for (int i = 0; i < memorySize; ++i)
                    pushInterpolationSample (offsetInput[i]);
            }
            else
            {
                for (int i = 0; i < numOutputSamplesToProduce; ++i)
                    pushInterpolationSample (input[i]);
            }
        */
    }
    
    #[inline(always)] pub fn push_interpolation_samples_with_wrap(
        &mut self, 
        input:                         *const f32,
        num_output_samples_to_produce: i32,
        num_input_samples_available:   i32,
        wrap_around:                   i32)  {
        
        todo!();
        /*
            if (numOutputSamplesToProduce >= memorySize)
            {
                if (numInputSamplesAvailable >= memorySize)
                {
                    pushInterpolationSamples (input,
                                              numOutputSamplesToProduce);
                }
                else
                {
                    pushInterpolationSamples (input + ((numOutputSamplesToProduce - numInputSamplesAvailable) - 1),
                                              numInputSamplesAvailable);

                    if (wrapAround > 0)
                    {
                        numOutputSamplesToProduce -= wrapAround;

                        pushInterpolationSamples (input + ((numOutputSamplesToProduce - (memorySize - numInputSamplesAvailable)) - 1),
                                                  memorySize - numInputSamplesAvailable);
                    }
                    else
                    {
                        for (int i = numInputSamplesAvailable; i < memorySize; ++i)
                            pushInterpolationSample (0.0f);
                    }
                }
            }
            else
            {
                if (numOutputSamplesToProduce > numInputSamplesAvailable)
                {
                    for (int i = 0; i < numInputSamplesAvailable; ++i)
                        pushInterpolationSample (input[i]);

                    const auto extraSamples = numOutputSamplesToProduce - numInputSamplesAvailable;

                    if (wrapAround > 0)
                    {
                        const auto* const offsetInput = input + (numInputSamplesAvailable - wrapAround);

                        for (int i = 0; i < extraSamples; ++i)
                            pushInterpolationSample (offsetInput[i]);
                    }
                    else
                    {
                        for (int i = 0; i < extraSamples; ++i)
                            pushInterpolationSample (0.0f);
                    }
                }
                else
                {
                    for (int i = 0; i < numOutputSamplesToProduce; ++i)
                        pushInterpolationSample (input[i]);
                }
            }
        */
    }
    
    pub fn interpolate(&mut self, 
        speed_ratio:                   f64,
        input:                         *const f32,
        output:                        *mut f32,
        num_output_samples_to_produce: i32) -> i32 {
        
        todo!();
        /*
            auto pos = subSamplePos;
            int numUsed = 0;

            while (numOutputSamplesToProduce > 0)
            {
                while (pos >= 1.0)
                {
                    pushInterpolationSample (input[numUsed++]);
                    pos -= 1.0;
                }

                *output++ = InterpolatorTraits::valueAtOffset (lastInputSamples, (float) pos, indexBuffer);
                pos += speedRatio;
                --numOutputSamplesToProduce;
            }

            subSamplePos = pos;
            return numUsed;
        */
    }
    
    pub fn interpolate_with_wrap(&mut self, 
        speed_ratio:                   f64,
        input:                         *const f32,
        output:                        *mut f32,
        num_output_samples_to_produce: i32,
        num_input_samples_available:   i32,
        wrap:                          i32) -> i32 {
        
        todo!();
        /*
            auto originalIn = input;
            auto pos = subSamplePos;
            bool exceeded = false;

            if (speedRatio < 1.0)
            {
                for (int i = numOutputSamplesToProduce; --i >= 0;)
                {
                    if (pos >= 1.0)
                    {
                        if (exceeded)
                        {
                            pushInterpolationSample (0.0f);
                        }
                        else
                        {
                            pushInterpolationSample (*input++);

                            if (--numInputSamplesAvailable <= 0)
                            {
                                if (wrap > 0)
                                {
                                    input -= wrap;
                                    numInputSamplesAvailable += wrap;
                                }
                                else
                                {
                                    exceeded = true;
                                }
                            }
                        }

                        pos -= 1.0;
                    }

                    *output++ = InterpolatorTraits::valueAtOffset (lastInputSamples, (float) pos, indexBuffer);
                    pos += speedRatio;
                }
            }
            else
            {
                for (int i = numOutputSamplesToProduce; --i >= 0;)
                {
                    while (pos < speedRatio)
                    {
                        if (exceeded)
                        {
                            pushInterpolationSample (0);
                        }
                        else
                        {
                            pushInterpolationSample (*input++);

                            if (--numInputSamplesAvailable <= 0)
                            {
                                if (wrap > 0)
                                {
                                    input -= wrap;
                                    numInputSamplesAvailable += wrap;
                                }
                                else
                                {
                                    exceeded = true;
                                }
                            }
                        }

                        pos += 1.0;
                    }

                    pos -= speedRatio;
                    *output++ = InterpolatorTraits::valueAtOffset (lastInputSamples, jmax (0.0f, 1.0f - (float) pos), indexBuffer);
                }
            }

            subSamplePos = pos;

            if (wrap == 0)
                return (int) (input - originalIn);

            return ((int) (input - originalIn) + wrap) % wrap;
        */
    }
    
    pub fn interpolate_adding_with_wrap(
        &mut self, 
        speed_ratio:                   f64,
        input:                         *const f32,
        output:                        *mut f32,
        num_output_samples_to_produce: i32,
        num_input_samples_available:   i32,
        wrap:                          i32,
        gain:                          f32) -> i32 {
        
        todo!();
        /*
            auto originalIn = input;
            auto pos = subSamplePos;
            bool exceeded = false;

            if (speedRatio < 1.0)
            {
                for (int i = numOutputSamplesToProduce; --i >= 0;)
                {
                    if (pos >= 1.0)
                    {
                        if (exceeded)
                        {
                            pushInterpolationSample (0.0);
                        }
                        else
                        {
                            pushInterpolationSample (*input++);

                            if (--numInputSamplesAvailable <= 0)
                            {
                                if (wrap > 0)
                                {
                                    input -= wrap;
                                    numInputSamplesAvailable += wrap;
                                }
                                else
                                {
                                    numInputSamplesAvailable = true;
                                }
                            }
                        }

                        pos -= 1.0;
                    }

                    *output++ += gain * InterpolatorTraits::valueAtOffset (lastInputSamples, (float) pos, indexBuffer);
                    pos += speedRatio;
                }
            }
            else
            {
                for (int i = numOutputSamplesToProduce; --i >= 0;)
                {
                    while (pos < speedRatio)
                    {
                        if (exceeded)
                        {
                            pushInterpolationSample (0.0);
                        }
                        else
                        {
                            pushInterpolationSample (*input++);

                            if (--numInputSamplesAvailable <= 0)
                            {
                                if (wrap > 0)
                                {
                                    input -= wrap;
                                    numInputSamplesAvailable += wrap;
                                }
                                else
                                {
                                    exceeded = true;
                                }
                            }
                        }

                        pos += 1.0;
                    }

                    pos -= speedRatio;
                    *output++ += gain * InterpolatorTraits::valueAtOffset (lastInputSamples, jmax (0.0f, 1.0f - (float) pos), indexBuffer);
                }
            }

            subSamplePos = pos;

            if (wrap == 0)
                return (int) (input - originalIn);

            return ((int) (input - originalIn) + wrap) % wrap;
        */
    }
    
    pub fn interpolate_adding(&mut self, 
        speed_ratio:                   f64,
        input:                         *const f32,
        output:                        *mut f32,
        num_output_samples_to_produce: i32,
        gain:                          f32) -> i32 {
        
        todo!();
        /*
            auto pos = subSamplePos;
            int numUsed = 0;

            while (numOutputSamplesToProduce > 0)
            {
                while (pos >= 1.0)
                {
                    pushInterpolationSample (input[numUsed++]);
                    pos -= 1.0;
                }

                *output++ += gain * InterpolatorTraits::valueAtOffset (lastInputSamples, (float) pos, indexBuffer);
                pos += speedRatio;
                --numOutputSamplesToProduce;
            }

            subSamplePos = pos;
            return numUsed;
        */
    }
}
