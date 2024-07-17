crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ResamplingAudioSource.h]

pub struct ResamplingAudioSourceFilterState
{
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

/**
  | A type of AudioSource that takes an input
  | source and changes its sample rate.
  | 
  | @see AudioSource, LagrangeInterpolator,
  | CatmullRomInterpolator
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ResamplingAudioSource {
    input:             OptionalScopedPointer<dyn AudioSource>,
    ratio:             f64, // default = 1.0
    last_ratio:        f64, // default = 1.0
    buffer:            AudioBuffer<f32>,
    buffer_pos:        i32, // default = 0
    samps_in_buffer:   i32, // default = 0
    sub_sample_offset: f64, // default = 0.0
    coefficients:      [f64; 6],
    ratio_lock:        SpinLock,
    callback_lock:     CriticalSection,
    num_channels:      i32,
    dest_buffers:      HeapBlock<*mut f32>,
    src_buffers:       HeapBlock<*const f32>,
    filter_states:     HeapBlock<ResamplingAudioSourceFilterState>,
}

impl AudioSource for ResamplingAudioSource {}

impl PrepareToPlayAudioSource for ResamplingAudioSource {

    fn prepare_to_play(
        &mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64
    ) {
        
        todo!();
        /*
            const SpinLock::ScopedLockType sl (ratioLock);

        auto scaledBlockSize = roundToInt (samplesPerBlockExpected * ratio);
        input->prepareToPlay (scaledBlockSize, sampleRate * ratio);

        buffer.setSize (numChannels, scaledBlockSize + 32);

        filterStates.calloc (numChannels);
        srcBuffers.calloc (numChannels);
        destBuffers.calloc (numChannels);
        createLowPass (ratio);

        flushBuffers();
        */
    }
}
    
impl ReleaseResources for ResamplingAudioSource {

    fn release_resources(&mut self)  {
        
        todo!();
        /*
            input->releaseResources();
        buffer.setSize (numChannels, 0);
        */
    }
}
    
impl GetNextAudioBlock for ResamplingAudioSource {

    fn get_next_audio_block(&mut self, info: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

        double localRatio;

        {
            const SpinLock::ScopedLockType ratioSl (ratioLock);
            localRatio = ratio;
        }

        if (lastRatio != localRatio)
        {
            createLowPass (localRatio);
            lastRatio = localRatio;
        }

        const int sampsNeeded = roundToInt (info.numSamples * localRatio) + 3;

        int bufferSize = buffer.getNumSamples();

        if (bufferSize < sampsNeeded + 8)
        {
            bufferPos %= bufferSize;
            bufferSize = sampsNeeded + 32;
            buffer.setSize (buffer.getNumChannels(), bufferSize, true, true);
        }

        bufferPos %= bufferSize;

        int endOfBufferPos = bufferPos + sampsInBuffer;
        const int channelsToProcess = jmin (numChannels, info.buffer->getNumChannels());

        while (sampsNeeded > sampsInBuffer)
        {
            endOfBufferPos %= bufferSize;

            int numToDo = jmin (sampsNeeded - sampsInBuffer,
                                bufferSize - endOfBufferPos);

            AudioSourceChannelInfo readInfo (&buffer, endOfBufferPos, numToDo);
            input->getNextAudioBlock (readInfo);

            if (localRatio > 1.0001)
            {
                // for down-sampling, pre-apply the filter..

                for (int i = channelsToProcess; --i >= 0;)
                    applyFilter (buffer.getWritePointer (i, endOfBufferPos), numToDo, filterStates[i]);
            }

            sampsInBuffer += numToDo;
            endOfBufferPos += numToDo;
        }

        for (int channel = 0; channel < channelsToProcess; ++channel)
        {
            destBuffers[channel] = info.buffer->getWritePointer (channel, info.startSample);
            srcBuffers[channel] = buffer.getReadPointer (channel);
        }

        int nextPos = (bufferPos + 1) % bufferSize;

        for (int m = info.numSamples; --m >= 0;)
        {
            jassert (sampsInBuffer > 0 && nextPos != endOfBufferPos);

            const float alpha = (float) subSampleOffset;

            for (int channel = 0; channel < channelsToProcess; ++channel)
                *destBuffers[channel]++ = srcBuffers[channel][bufferPos]
                                            + alpha * (srcBuffers[channel][nextPos] - srcBuffers[channel][bufferPos]);

            subSampleOffset += localRatio;

            while (subSampleOffset >= 1.0)
            {
                if (++bufferPos >= bufferSize)
                    bufferPos = 0;

                --sampsInBuffer;

                nextPos = (bufferPos + 1) % bufferSize;
                subSampleOffset -= 1.0;
            }
        }

        if (localRatio < 0.9999)
        {
            // for up-sampling, apply the filter after transposing..
            for (int i = channelsToProcess; --i >= 0;)
                applyFilter (info.buffer->getWritePointer (i, info.startSample), info.numSamples, filterStates[i]);
        }
        else if (localRatio <= 1.0001 && info.numSamples > 0)
        {
            // if the filter's not currently being applied, keep it stoked with the last couple of samples to avoid discontinuities
            for (int i = channelsToProcess; --i >= 0;)
            {
                const float* const endOfBuffer = info.buffer->getReadPointer (i, info.startSample + info.numSamples - 1);
                ResamplingAudioSourceFilterState& fs = filterStates[i];

                if (info.numSamples > 1)
                {
                    fs.y2 = fs.x2 = *(endOfBuffer - 1);
                }
                else
                {
                    fs.y2 = fs.y1;
                    fs.x2 = fs.x1;
                }

                fs.y1 = fs.x1 = *endOfBuffer;
            }
        }

        jassert (sampsInBuffer >= 0);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ResamplingAudioSource.cpp]
impl ResamplingAudioSource {

    /**
      | Returns the current resampling ratio.
      | 
      | This is the value that was set by setResamplingRatio().
      |
      */
    pub fn get_resampling_ratio(&self) -> f64 {
        
        todo!();
        /*
            return ratio;
        */
    }
    
    /**
      | Creates a ResamplingAudioSource for
      | a given input source.
      | 
      | -----------
      | @param inputSource
      | 
      | the input source to read from
      | ----------
      | @param deleteInputWhenDeleted
      | 
      | if true, the input source will be deleted
      | when this object is deleted
      | ----------
      | @param numChannels
      | 
      | the number of channels to process
      |
      */
    pub fn new(
        input_source:              *mut dyn AudioSource,
        delete_input_when_deleted: bool,
        channels:                  Option<i32>) -> Self {

        let channels = channels.unwrap_or(2);
    
        todo!();
        /*
        : input(inputSource, deleteInputWhenDeleted),
        : num_channels(channels),

            jassert (input != nullptr);
        zeromem (coefficients, sizeof (coefficients));
        */
    }
    
    /**
      | Changes the resampling ratio.
      | 
      | (This value can be changed at any time,
      | even while the source is running).
      | 
      | -----------
      | @param samplesInPerOutputSample
      | 
      | if set to 1.0, the input is passed through;
      | higher values will speed it up; lower
      | values will slow it down. The ratio must
      | be greater than 0
      |
      */
    pub fn set_resampling_ratio(&mut self, samples_in_per_output_sample: f64)  {
        
        todo!();
        /*
            jassert (samplesInPerOutputSample > 0);

        const SpinLock::ScopedLockType sl (ratioLock);
        ratio = jmax (0.0, samplesInPerOutputSample);
        */
    }
    
    /**
      | Clears any buffers and filters that
      | the resampler is using.
      |
      */
    pub fn flush_buffers(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

        buffer.clear();
        bufferPos = 0;
        sampsInBuffer = 0;
        subSampleOffset = 0.0;
        resetFilters();
        */
    }

    pub fn create_low_pass(&mut self, frequency_ratio: f64)  {
        
        todo!();
        /*
            const double proportionalRate = (frequencyRatio > 1.0) ? 0.5 / frequencyRatio
                                                               : 0.5 * frequencyRatio;

        const double n = 1.0 / std::tan (MathConstants<double>::pi * jmax (0.001, proportionalRate));
        const double nSquared = n * n;
        const double c1 = 1.0 / (1.0 + MathConstants<double>::sqrt2 * n + nSquared);

        setFilterCoefficients (c1,
                               c1 * 2.0f,
                               c1,
                               1.0,
                               c1 * 2.0 * (1.0 - nSquared),
                               c1 * (1.0 - MathConstants<double>::sqrt2 * n + nSquared));
        */
    }
    
    pub fn set_filter_coefficients(&mut self, 
        c1: f64,
        c2: f64,
        c3: f64,
        c4: f64,
        c5: f64,
        c6: f64)  {
        
        todo!();
        /*
            const double a = 1.0 / c4;

        c1 *= a;
        c2 *= a;
        c3 *= a;
        c5 *= a;
        c6 *= a;

        coefficients[0] = c1;
        coefficients[1] = c2;
        coefficients[2] = c3;
        coefficients[3] = c4;
        coefficients[4] = c5;
        coefficients[5] = c6;
        */
    }
    
    pub fn reset_filters(&mut self)  {
        
        todo!();
        /*
            if (filterStates != nullptr)
            filterStates.clear ((size_t) numChannels);
        */
    }
    
    pub fn apply_filter(&mut self, 
        samples: *mut f32,
        num:     i32,
        fs:      &mut ResamplingAudioSourceFilterState)  {
        
        todo!();
        /*
            while (--num >= 0)
        {
            const double in = *samples;

            double out = coefficients[0] * in
                         + coefficients[1] * fs.x1
                         + coefficients[2] * fs.x2
                         - coefficients[4] * fs.y1
                         - coefficients[5] * fs.y2;

           #if ALOE_INTEL
            if (! (out < -1.0e-8 || out > 1.0e-8))
                out = 0;
           #endif

            fs.x2 = fs.x1;
            fs.x1 = in;
            fs.y2 = fs.y1;
            fs.y1 = out;

            *samples++ = (float) out;
        }
        */
    }
}
