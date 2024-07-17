crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatWriter.h]

/**
  | Writes samples to an audio file stream.
  | 
  | A subclass that writes a specific type
  | of audio format will be created by an
  | AudioFormat object.
  | 
  | After creating one of these with the
  | AudioFormat::createWriterFor()
  | method you can call its write() method
  | to store the samples, and then delete
  | it.
  | 
  | @see AudioFormat, AudioFormatReader
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioFormatWriter<'a, W: Write> {

    /**
      | The sample rate of the stream.
      |
      */
    sample_rate:              f64,

    /**
      | The number of channels being written
      | to the stream.
      |
      */
    num_channels:             u32,

    /**
      | The bit depth of the file.
      |
      */
    bits_per_sample:          u32,

    /**
      | True if it's a floating-point format,
      | false if it's fixed-point.
      |
      */
    uses_floating_point_data: bool,

    /**
      | The audio channel layout that the writer
      | should use
      |
      */
    channel_layout:           AudioChannelSet,

    /**
      | The output stream for use by subclasses.
      |
      */
    output:                   &'a mut W,

    format_name:              String,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatWriter.cpp]
impl<'a, W: Write> Drop for AudioFormatWriter<'a, W> {

    fn drop(&mut self) {
        todo!();
        /*
            delete output;
        */
    }
}

impl<'a, W: Write> AudioFormatWriter<'a, W> {

    /**
      | Returns a description of what type of
      | format this is.
      | 
      | E.g. "AIFF file"
      |
      */
    pub fn get_format_name(&self) -> &String {
        
        todo!();
        /*
            return formatName;
        */
    }
    
    /**
      | Returns the sample rate being used.
      |
      */
    pub fn get_sample_rate(&self) -> f64 {
        
        todo!();
        /*
            return sampleRate;
        */
    }

    /**
      | Returns the number of channels being
      | written.
      |
      */
    pub fn get_num_channels(&self) -> i32 {
        
        todo!();
        /*
            return (int) numChannels;
        */
    }

    /**
      | Returns the bit-depth of the data being
      | written.
      |
      */
    pub fn get_bits_per_sample(&self) -> i32 {
        
        todo!();
        /*
            return (int) bitsPerSample;
        */
    }

    /**
      | Returns true if it's a floating-point
      | format, false if it's fixed-point.
      |
      */
    pub fn is_floating_point(&self) -> bool {
        
        todo!();
        /*
            return usesFloatingPointData;
        */
    }
    
    /**
      | Creates an AudioFormatWriter object.
      | 
      | -----------
      | @param destStream
      | 
      | the stream to write to - this will be deleted
      | by this object when it is no longer needed
      | ----------
      | @param formatName
      | 
      | the description that will be returned
      | by the getFormatName() method
      | ----------
      | @param sampleRate
      | 
      | the sample rate to use - the base class
      | just stores this value, it doesn't do
      | anything with it
      | ----------
      | @param numberOfChannels
      | 
      | the number of channels to write - the
      | base class just stores this value, it
      | doesn't do anything with it
      | ----------
      | @param bitsPerSample
      | 
      | the bit depth of the stream - the base
      | class just stores this value, it doesn't
      | do anything with it
      |
      */
    pub fn new(
        out:             &mut W,
        format_name:     &String,
        rate:            f64,
        num_channels:    u32,
        bits_per_sample: u32

    ) -> Self {
    
        todo!();
        /*


            : sampleRate (rate),
        numChannels (numChannels_),
        bitsPerSample (bitsPerSample_),
        usesFloatingPointData (false),
        channelLayout (AudioChannelSet::canonicalChannelSet(static_cast<int> (numChannels_))),
        output (out),
        formatName (formatName_)
        */
    }
    
    /**
      | Creates an AudioFormatWriter object.
      | 
      | -----------
      | @param destStream
      | 
      | the stream to write to - this will be deleted
      | by this object when it is no longer needed
      | ----------
      | @param formatName
      | 
      | the description that will be returned
      | by the getFormatName() method
      | ----------
      | @param sampleRate
      | 
      | the sample rate to use - the base class
      | just stores this value, it doesn't do
      | anything with it
      | ----------
      | @param audioChannelLayout
      | 
      | the channel layout to use for the writer
      | - the base class just stores this value,
      | it doesn't do anything with it
      | ----------
      | @param bitsPerSample
      | 
      | the bit depth of the stream - the base
      | class just stores this value, it doesn't
      | do anything with it
      |
      */
    pub fn new_with_channel_layout(
        out:             *mut W,
        format_name:     &String,
        rate:            f64,
        channel_layout:  &AudioChannelSet,
        bits_per_sample: u32

    ) -> Self {
    
        todo!();
        /*


            : sampleRate (rate),
        numChannels (static_cast<unsigned int> (channelLayout_.size())),
        bitsPerSample (bitsPerSample_),
        usesFloatingPointData (false),
        channelLayout (channelLayout_),
        output (out),
        formatName (formatName_)
        */
    }
    
    /**
      | Reads a section of samples from an AudioFormatReader,
      | and writes these to the output.
      | 
      | This will take care of any floating-point
      | conversion that's required to convert
      | between the two formats. It won't deal
      | with sample-rate conversion, though.
      | 
      | If numSamplesToRead < 0, it will write
      | the entire length of the reader.
      | 
      | 
      | -----------
      | @return
      | 
      | false if it can't read or write properly
      | during the operation
      |
      */
    pub fn write_from_audio_reader(
        &mut self, 
        reader:              &mut AudioFormatReader<'a>,
        start_sample:        i64,
        num_samples_to_read: i64

    ) -> bool {
        
        todo!();
        /*
            const int bufferSize = 16384;
        AudioBuffer<float> tempBuffer ((int) numChannels, bufferSize);

        int* buffers[128] = { nullptr };

        for (int i = tempBuffer.getNumChannels(); --i >= 0;)
            buffers[i] = reinterpret_cast<int*> (tempBuffer.getWritePointer (i, 0));

        if (numSamplesToRead < 0)
            numSamplesToRead = reader.lengthInSamples;

        while (numSamplesToRead > 0)
        {
            const int numToDo = (int) jmin (numSamplesToRead, (int64) bufferSize);

            if (! reader.read (buffers, (int) numChannels, startSample, numToDo, false))
                return false;

            if (reader.usesFloatingPointData != isFloatingPoint())
            {
                int** bufferChan = buffers;

                while (*bufferChan != nullptr)
                {
                    void* const b = *bufferChan++;

                    constexpr auto scaleFactor = 1.0f / static_cast<float> (0x7fffffff);

                    if (isFloatingPoint())
                        FloatVectorOperations::convertFixedToFloat ((float*) b, (int*) b, scaleFactor, numToDo);
                    else
                        convertFloatsToInts ((int*) b, (float*) b, numToDo);
                }
            }

            if (! write (const_cast<const int**> (buffers), numToDo))
                return false;

            numSamplesToRead -= numToDo;
            startSample += numToDo;
        }

        return true;
        */
    }
    
    /**
      | Reads some samples from an AudioSource,
      | and writes these to the output.
      | 
      | The source must already have been initialised
      | with the AudioSource::prepareToPlay()
      | method
      | 
      | -----------
      | @param source
      | 
      | the source to read from
      | ----------
      | @param numSamplesToRead
      | 
      | total number of samples to read and write
      | ----------
      | @param samplesPerBlock
      | 
      | the maximum number of samples to fetch
      | from the source
      | 
      | -----------
      | @return
      | 
      | false if it can't read or write properly
      | during the operation
      |
      */
    pub fn write_from_audio_source(
        &mut self, 
        source:              &mut dyn AudioSource,
        num_samples_to_read: i32,
        samples_per_block:   Option<i32>

    ) -> bool {

        let samples_per_block: i32 = samples_per_block.unwrap_or(2048);
        
        todo!();
        /*
            AudioBuffer<float> tempBuffer (getNumChannels(), samplesPerBlock);

        while (numSamplesToRead > 0)
        {
            auto numToDo = jmin (numSamplesToRead, samplesPerBlock);

            AudioSourceChannelInfo info (&tempBuffer, 0, numToDo);
            info.clearActiveBufferRegion();

            source.getNextAudioBlock (info);

            if (! writeFromAudioSampleBuffer (tempBuffer, 0, numToDo))
                return false;

            numSamplesToRead -= numToDo;
        }

        return true;
        */
    }
    
    /**
      | Writes some samples from a set of float
      | data channels.
      |
      */
    pub fn write_from_float_arrays(
        &mut self, 
        channels:            *const *const f32,
        num_source_channels: i32,
        num_samples:         i32

    ) -> bool {
        
        todo!();
        /*
            if (numSamples <= 0)
            return true;

        if (isFloatingPoint())
            return write ((const int**) channels, numSamples);

        std::vector<int*> chans (256);
        std::vector<int> scratch (4096);

        jassert (numSourceChannels < (int) chans.size());
        const int maxSamples = (int) scratch.size() / numSourceChannels;

        for (int i = 0; i < numSourceChannels; ++i)
            chans[(size_t) i] = scratch.data() + (i * maxSamples);

        chans[(size_t) numSourceChannels] = nullptr;
        int startSample = 0;

        while (numSamples > 0)
        {
            auto numToDo = jmin (numSamples, maxSamples);

            for (int i = 0; i < numSourceChannels; ++i)
                convertFloatsToInts (chans[(size_t) i], channels[(size_t) i] + startSample, numToDo);

            if (! write ((const int**) chans.data(), numToDo))
                return false;

            startSample += numToDo;
            numSamples  -= numToDo;
        }

        return true;
        */
    }
    
    /**
      | Writes some samples from an AudioBuffer.
      |
      */
    pub fn write_from_audio_sample_buffer(
        &mut self, 
        source:       &AudioBuffer<f32>,
        start_sample: i32,
        num_samples:  i32

    ) -> bool {
        
        todo!();
        /*
            auto numSourceChannels = source.getNumChannels();
        jassert (startSample >= 0 && startSample + numSamples <= source.getNumSamples() && numSourceChannels > 0);

        if (startSample == 0)
            return writeFromFloatArrays (source.getArrayOfReadPointers(), numSourceChannels, numSamples);

        const float* chans[256];
        jassert ((int) numChannels < numElementsInArray (chans));

        for (int i = 0; i < numSourceChannels; ++i)
            chans[i] = source.getReadPointer (i, startSample);

        chans[numSourceChannels] = nullptr;

        return writeFromFloatArrays (chans, numSourceChannels, numSamples);
        */
    }
    
    /// freedom is earned by a million small actions
    ///
    pub fn flush(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
