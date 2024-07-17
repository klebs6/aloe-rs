crate::ix!();

/**
  | Reads samples from an audio file stream.
  | 
  | A subclass that reads a specific type
  | of audio format will be created by an
  | AudioFormat object.
  | 
  | @see AudioFormat, AudioFormatWriter
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioFormatReader<'a> {


    /**
      | The sample-rate of the stream.
      |
      */
    sample_rate:              f64, // default = 0

    /**
      | The number of bits per sample, e.g. 16,
      | 24, 32.
      |
      */
    bits_per_sample:          u32, // default = 0

    /**
      | The total number of samples in the audio
      | stream.
      |
      */
    length_in_samples:        i64, // default = 0

    /**
      | The total number of channels in the audio
      | stream.
      |
      */
    num_channels:             u32, // default = 0

    /**
      | Indicates whether the data is floating-point
      | or fixed.
      |
      */
    uses_floating_point_data: bool, // default = false

    /**
      | A set of metadata values that the reader
      | has pulled out of the stream.
      | 
      | Exactly what these values are depends
      | on the format, so you can check out the
      | format implementation code to see what
      | kind of stuff they understand.
      |
      */
    metadata_values:          Vec<(String,String)>,

    /**
      | The input stream, for use by subclasses.
      |
      */
    input:                    &'a mut dyn Read,

    format_name:              String,
}

impl<'a> Drop for AudioFormatReader<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            delete input;
        */
    }
}

impl<'a> GetChannelLayout for AudioFormatReader<'a> {
    
    fn get_channel_layout(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet::canonicalChannelSet (static_cast<int> (numChannels));
        */
    }
}

impl<'a> ReadMaxLevels for AudioFormatReader<'a> {
 
    fn read_max_levels(&mut self, 
        start_sample_in_file: i64,
        num_samples:          i64,
        results:              *mut Range<f32>,
        channels_to_read:     i32)  {
        
        todo!();
        /*
            jassert (channelsToRead > 0 && channelsToRead <= (int) numChannels);

        if (numSamples <= 0)
        {
            for (int i = 0; i < channelsToRead; ++i)
                results[i] = Range<float>();

            return;
        }

        auto bufferSize = (int) jmin (numSamples, (int64) 4096);
        AudioBuffer<float> tempSampleBuffer ((int) channelsToRead, bufferSize);

        auto floatBuffer = tempSampleBuffer.getArrayOfWritePointers();
        auto intBuffer = reinterpret_cast<int* const*> (floatBuffer);
        bool isFirstBlock = true;

        while (numSamples > 0)
        {
            auto numToDo = (int) jmin (numSamples, (int64) bufferSize);

            if (! read (intBuffer, channelsToRead, startSampleInFile, numToDo, false))
                break;

            for (int i = 0; i < channelsToRead; ++i)
            {
                Range<float> r;

                if (usesFloatingPointData)
                {
                    r = FloatVectorOperations::findMinAndMax (floatBuffer[i], numToDo);
                }
                else
                {
                    auto intRange = Range<int>::findMinAndMax (intBuffer[i], numToDo);

                    r = Range<float> ((float) intRange.getStart() / (float) std::numeric_limits<int>::max(),
                                      (float) intRange.getEnd()   / (float) std::numeric_limits<int>::max());
                }

                results[i] = isFirstBlock ? r : results[i].getUnionWith (r);
            }

            isFirstBlock = false;
            numSamples -= numToDo;
            startSampleInFile += numToDo;
        }
        */
    }
    
    fn read_max_levels_range(&mut self, 
        start_sample_in_file: i64,
        num_samples:          i64,
        lowest_left:          &mut f32,
        highest_left:         &mut f32,
        lowest_right:         &mut f32,
        highest_right:        &mut f32)  {
        
        todo!();
        /*
            Range<float> levels[2];

        if (numChannels < 2)
        {
            readMaxLevels (startSampleInFile, numSamples, levels, (int) numChannels);
            levels[1] = levels[0];
        }
        else
        {
            readMaxLevels (startSampleInFile, numSamples, levels, 2);
        }

        lowestLeft   = levels[0].getStart();
        highestLeft  = levels[0].getEnd();
        lowestRight  = levels[1].getStart();
        highestRight = levels[1].getEnd();
        */
    }
}

impl<'a> AudioFormatReader<'a> {

    /**
      | Returns a description of what type of
      | format this is.
      | 
      | E.g. "AIFF"
      |
      */
    pub fn get_format_name(&self) -> &String {
        
        todo!();
        /*
            return formatName;
        */
    }

    /**
      | Used by AudioFormatReader subclasses
      | to clear any parts of the data blocks
      | that lie beyond the end of their available
      | length.
      |
      */
    pub fn clear_samples_beyond_available_length(
        dest_channels:               *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 &mut i32,
        file_length_in_samples:      i64

    )  {
        
        todo!();
        /*
            if (destChannels == nullptr)
            {
                jassertfalse;
                return;
            }

            const int64 samplesAvailable = fileLengthInSamples - startSampleInFile;

            if (samplesAvailable < numSamples)
            {
                for (int i = numDestChannels; --i >= 0;)
                    if (destChannels[i] != nullptr)
                        zeromem (destChannels[i] + startOffsetInDestBuffer, (size_t) numSamples * sizeof (int));

                numSamples = (int) samplesAvailable;
            }
        */
    }
    
    /**
      | Creates an AudioFormatReader object.
      | 
      | -----------
      | @param sourceStream
      | 
      | the stream to read from - this will be
      | deleted by this object when it is no longer
      | needed. (Some specialised readers
      | might not use this parameter and can
      | leave it as nullptr).
      | ----------
      | @param formatName
      | 
      | the description that will be returned
      | by the getFormatName() method
      |
      */
    pub fn new(in_:  &mut dyn Read, name: &String) -> Self {
    
        todo!();
        /*
        : input(in),
        : format_name(name),

        
        */
    }
    
    /**
      | Reads samples from the stream.
      | 
      | -----------
      | @param destChannels
      | 
      | an array of float buffers into which
      | the sample data for each channel will
      | be written. Channels that aren't needed
      | can be null
      | ----------
      | @param numDestChannels
      | 
      | the number of array elements in the destChannels
      | array
      | ----------
      | @param startSampleInSource
      | 
      | the position in the audio file or stream
      | at which the samples should be read,
      | as a number of samples from the start
      | of the stream. It's ok for this to be beyond
      | the start or end of the available data
      | - any samples that are out-of-range
      | will be returned as zeros.
      | ----------
      | @param numSamplesToRead
      | 
      | the number of samples to read. If this
      | is greater than the number of samples
      | that the file or stream contains. the
      | result will be padded with zeros
      | 
      | -----------
      | @return
      | 
      | true if the operation succeeded, false
      | if there was an error. Note that reading
      | sections of data beyond the extent of
      | the stream isn't an error - the reader
      | should just return zeros for these regions
      | @see readMaxLevels
      |
      */
    pub fn read(
        &mut self, 
        dest_channels:          *const *const f32,
        num_dest_channels:      i32,
        start_sample_in_source: i64,
        num_samples_to_read:    i32

      ) -> bool {
        
        todo!();
        /*
            auto channelsAsInt = reinterpret_cast<int* const*> (destChannels);

        if (! read (channelsAsInt, numDestChannels, startSampleInSource, numSamplesToRead, false))
            return false;

        if (! usesFloatingPointData)
            convertFixedToFloat (channelsAsInt, numDestChannels, numSamplesToRead);

        return true;
        */
    }

    /**
      | Reads samples from the stream.
      | 
      | -----------
      | @param destChannels
      | 
      | an array of buffers into which the sample
      | data for each channel will be written.
      | 
      | If the format is fixed-point, each channel
      | will be written as an array of 32-bit
      | signed integers using the full range
      | -0x80000000 to 0x7fffffff, regardless
      | of the source's bit-depth. If it is a
      | floating-point format, you should
      | cast the resulting array to a (float**)
      | to get the values (in the range -1.0 to
      | 1.0 or beyond)
      | 
      | If the format is stereo, then destChannels[0]
      | is the left channel data, and destChannels[1]
      | is the right channel.
      | 
      | The numDestChannels parameter indicates
      | how many pointers this array contains,
      | but some of these pointers can be null
      | if you don't want to read data for some
      | of the channels
      | ----------
      | @param numDestChannels
      | 
      | the number of array elements in the destChannels
      | array
      | ----------
      | @param startSampleInSource
      | 
      | the position in the audio file or stream
      | at which the samples should be read,
      | as a number of samples from the start
      | of the stream. It's ok for this to be beyond
      | the start or end of the available data
      | - any samples that are out-of-range
      | will be returned as zeros.
      | ----------
      | @param numSamplesToRead
      | 
      | the number of samples to read. If this
      | is greater than the number of samples
      | that the file or stream contains. the
      | result will be padded with zeros
      | ----------
      | @param fillLeftoverChannelsWithCopies
      | 
      | if true, this indicates that if there's
      | no source data available for some of
      | the channels that you pass in, then they
      | should be filled with copies of valid
      | source channels.
      | 
      | E.g. if you're reading a mono file and
      | you pass 2 channels to this method, then
      | if fillLeftoverChannelsWithCopies
      | is true, both destination channels
      | will be filled with the same data from
      | the file's single channel. If fillLeftoverChannelsWithCopies
      | was false, then only the first channel
      | would be filled with the file's contents,
      | and the second would be cleared. If there
      | are many channels, e.g. you try to read
      | 4 channels from a stereo file, then the
      | last 3 would all end up with copies of
      | the same data.
      | 
      | -----------
      | @return
      | 
      | true if the operation succeeded, false
      | if there was an error. Note that reading
      | sections of data beyond the extent of
      | the stream isn't an error - the reader
      | should just return zeros for these regions
      | @see readMaxLevels
      |
      */
    pub fn read_and_maybe_fill_with_copies(
        &mut self, 
        dest_channels:                      *const *const i32,
        num_dest_channels:                  i32,
        start_sample_in_source:             i64,
        num_samples_to_read:                i32,
        fill_leftover_channels_with_copies: bool

      ) -> bool {
        
        todo!();
        /*
            jassert (numDestChannels > 0); // you have to actually give this some channels to work with!

        auto originalNumSamplesToRead = (size_t) numSamplesToRead;
        int startOffsetInDestBuffer = 0;

        if (startSampleInSource < 0)
        {
            auto silence = (int) jmin (-startSampleInSource, (int64) numSamplesToRead);

            for (int i = numDestChannels; --i >= 0;)
                if (auto d = destChannels[i])
                    zeromem (d, (size_t) silence * sizeof (int));

            startOffsetInDestBuffer += silence;
            numSamplesToRead -= silence;
            startSampleInSource = 0;
        }

        if (numSamplesToRead <= 0)
            return true;

        if (! readSamples (const_cast<int**> (destChannels),
                           jmin ((int) numChannels, numDestChannels), startOffsetInDestBuffer,
                           startSampleInSource, numSamplesToRead))
            return false;

        if (numDestChannels > (int) numChannels)
        {
            if (fillLeftoverChannelsWithCopies)
            {
                auto lastFullChannel = destChannels[0];

                for (int i = (int) numChannels; --i > 0;)
                {
                    if (destChannels[i] != nullptr)
                    {
                        lastFullChannel = destChannels[i];
                        break;
                    }
                }

                if (lastFullChannel != nullptr)
                    for (int i = (int) numChannels; i < numDestChannels; ++i)
                        if (auto d = destChannels[i])
                            memcpy (d, lastFullChannel, sizeof (int) * originalNumSamplesToRead);
            }
            else
            {
                for (int i = (int) numChannels; i < numDestChannels; ++i)
                    if (auto d = destChannels[i])
                        zeromem (d, sizeof (int) * originalNumSamplesToRead);
            }
        }

        return true;
        */
    }
    
    /**
      | Fills a section of an AudioBuffer from
      | this reader.
      | 
      | This will convert the reader's fixed-
      | or floating-point data to the buffer's
      | floating-point format, and will try
      | to intelligently cope with mismatches
      | between the number of channels in the
      | reader and the buffer.
      |
      */
    pub fn read_into_audio_buffer(
        &mut self, 
        buffer:                *mut AudioBuffer<f32>,
        start_sample:          i32,
        num_samples:           i32,
        reader_start_sample:   i64,
        use_reader_left_chan:  bool,
        use_reader_right_chan: bool

    ) {
        
        todo!();
        /*
            jassert (buffer != nullptr);
        jassert (startSample >= 0 && startSample + numSamples <= buffer->getNumSamples());

        if (numSamples > 0)
        {
            auto numTargetChannels = buffer->getNumChannels();

            if (numTargetChannels <= 2)
            {
                int* dests[2] = { reinterpret_cast<int*> (buffer->getWritePointer (0, startSample)),
                                  reinterpret_cast<int*> (numTargetChannels > 1 ? buffer->getWritePointer (1, startSample) : nullptr) };
                int* chans[3] = {};

                if (useReaderLeftChan == useReaderRightChan)
                {
                    chans[0] = dests[0];

                    if (numChannels > 1)
                        chans[1] = dests[1];
                }
                else if (useReaderLeftChan || (numChannels == 1))
                {
                    chans[0] = dests[0];
                }
                else if (useReaderRightChan)
                {
                    chans[1] = dests[0];
                }

                read (chans, 2, readerStartSample, numSamples, true);

                // if the target's stereo and the source is mono, dupe the first channel..
                if (numTargetChannels > 1
                    && (chans[0] == nullptr || chans[1] == nullptr)
                    && (dests[0] != nullptr && dests[1] != nullptr))
                {
                    memcpy (dests[1], dests[0], (size_t) numSamples * sizeof (float));
                }

                if (! usesFloatingPointData)
                    convertFixedToFloat (dests, 2, numSamples);
            }
            else if (numTargetChannels <= 64)
            {
                int* chans[65];
                readChannels (*this, chans, buffer, startSample, numSamples,
                              readerStartSample, numTargetChannels, ! usesFloatingPointData);
            }
            else
            {
                HeapBlock<int*> chans (numTargetChannels + 1);
                readChannels (*this, chans, buffer, startSample, numSamples,
                              readerStartSample, numTargetChannels, ! usesFloatingPointData);
            }
        }
        */
    }
       
    /**
      | Scans the source looking for a sample
      | whose magnitude is in a specified range.
      | 
      | This will read from the source, either
      | forwards or backwards between two sample
      | positions, until it finds a sample whose
      | magnitude lies between two specified
      | levels.
      | 
      | If it finds a suitable sample, it returns
      | its position; if not, it will return
      | -1.
      | 
      | There's also a minimumConsecutiveSamples
      | setting to help avoid spikes or zero-crossing
      | points when you're searching for a continuous
      | range of samples
      | 
      | -----------
      | @param startSample
      | 
      | the first sample to look at
      | ----------
      | @param numSamplesToSearch
      | 
      | the number of samples to scan. If this
      | value is negative, the search will go
      | backwards
      | ----------
      | @param magnitudeRangeMinimum
      | 
      | the lowest magnitude (inclusive) that
      | is considered a hit, from 0 to 1.0
      | ----------
      | @param magnitudeRangeMaximum
      | 
      | the highest magnitude (inclusive)
      | that is considered a hit, from 0 to 1.0
      | ----------
      | @param minimumConsecutiveSamples
      | 
      | if this is > 0, the method will only look
      | for a sequence of this many consecutive
      | samples, all of which lie within the
      | target range. When it finds such a sequence,
      | it returns the position of the first
      | in-range sample it found (i.e. the earliest
      | one if scanning forwards, the latest
      | one if scanning backwards)
      |
      */
    pub fn search_for_level(
        &mut self, 
        start_sample:                i64,
        num_samples_to_search:       i64,
        magnitude_range_minimum:     f64,
        magnitude_range_maximum:     f64,
        minimum_consecutive_samples: i32

      ) -> i64 {
        
        todo!();
        /*
            if (numSamplesToSearch == 0)
            return -1;

        const int bufferSize = 4096;
        HeapBlock<int> tempSpace (bufferSize * 2 + 64);

        int* tempBuffer[3] = { tempSpace.get(),
                               tempSpace.get() + bufferSize,
                               nullptr };

        int consecutive = 0;
        int64 firstMatchPos = -1;

        jassert (magnitudeRangeMaximum > magnitudeRangeMinimum);

        auto doubleMin = jlimit (0.0, (double) std::numeric_limits<int>::max(), magnitudeRangeMinimum * std::numeric_limits<int>::max());
        auto doubleMax = jlimit (doubleMin, (double) std::numeric_limits<int>::max(), magnitudeRangeMaximum * std::numeric_limits<int>::max());
        auto intMagnitudeRangeMinimum = roundToInt (doubleMin);
        auto intMagnitudeRangeMaximum = roundToInt (doubleMax);

        while (numSamplesToSearch != 0)
        {
            auto numThisTime = (int) jmin (std::abs (numSamplesToSearch), (int64) bufferSize);
            int64 bufferStart = startSample;

            if (numSamplesToSearch < 0)
                bufferStart -= numThisTime;

            if (bufferStart >= lengthInSamples)
                break;

            read (tempBuffer, 2, bufferStart, numThisTime, false);
            auto num = numThisTime;

            while (--num >= 0)
            {
                if (numSamplesToSearch < 0)
                    --startSample;

                bool matches = false;
                auto index = (int) (startSample - bufferStart);

                if (usesFloatingPointData)
                {
                    const float sample1 = std::abs (((float*) tempBuffer[0]) [index]);

                    if (sample1 >= magnitudeRangeMinimum
                         && sample1 <= magnitudeRangeMaximum)
                    {
                        matches = true;
                    }
                    else if (numChannels > 1)
                    {
                        const float sample2 = std::abs (((float*) tempBuffer[1]) [index]);

                        matches = (sample2 >= magnitudeRangeMinimum
                                     && sample2 <= magnitudeRangeMaximum);
                    }
                }
                else
                {
                    const int sample1 = std::abs (tempBuffer[0] [index]);

                    if (sample1 >= intMagnitudeRangeMinimum
                         && sample1 <= intMagnitudeRangeMaximum)
                    {
                        matches = true;
                    }
                    else if (numChannels > 1)
                    {
                        const int sample2 = std::abs (tempBuffer[1][index]);

                        matches = (sample2 >= intMagnitudeRangeMinimum
                                     && sample2 <= intMagnitudeRangeMaximum);
                    }
                }

                if (matches)
                {
                    if (firstMatchPos < 0)
                        firstMatchPos = startSample;

                    if (++consecutive >= minimumConsecutiveSamples)
                    {
                        if (firstMatchPos < 0 || firstMatchPos >= lengthInSamples)
                            return -1;

                        return firstMatchPos;
                    }
                }
                else
                {
                    consecutive = 0;
                    firstMatchPos = -1;
                }

                if (numSamplesToSearch > 0)
                    ++startSample;
            }

            if (numSamplesToSearch > 0)
                numSamplesToSearch -= numThisTime;
            else
                numSamplesToSearch += numThisTime;
        }

        return -1;
        */
    }
}
