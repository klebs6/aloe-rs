crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioThumbnail.h]

/**
  | Makes it easy to quickly draw scaled
  | views of the waveform shape of an audio
  | file.
  | 
  | To use this class, just create an AudioThumbnail
  | class for the file you want to draw, call
  | setSource to tell it which file or resource
  | to use, then call drawChannel() to draw
  | it.
  | 
  | The class will asynchronously scan
  | the wavefile to create its scaled-down
  | view, so you should make your UI repaint
  | itself as this data comes in. To do this,
  | the AudioThumbnail is a ChangeBroadcaster,
  | and will broadcast a message when its
  | listeners should repaint themselves.
  | 
  | The thumbnail stores an internal low-res
  | version of the wave data, and this can
  | be loaded and saved to avoid having to
  | scan the file again.
  | 
  | @see AudioThumbnailCache, AudioThumbnailBase
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioThumbnail<'a> {
    change_broadcaster:       ChangeBroadcaster<'a>,
    format_manager_to_use:    &'a mut AudioFormatManager,
    cache:                    &'a mut AudioThumbnailCache,
    source:                   Box<AudioThumbnailLevelDataSource<'a>>,
    window:                   Box<AudioThumbnailCachedWindow>,
    channels:                 Vec<Box<AudioThumbnailThumbData>>,
    samples_per_thumb_sample: i32, // default = 0
    total_samples:            Atomic<i64>, // default = 0 
    num_samples_finished:     i64, // default = 0
    num_channels:             i32, // default = 0
    sample_rate:              f64, // default = 0
    lock:                     CriticalSection,
}

impl<'a, RX: Read> AudioThumbnailBase<'a, RX> for AudioThumbnail<'a> { }

impl<'a> GetChangeBroadcaster<'a> for AudioThumbnail<'a> { 

    fn get_change_broadcaster(&'a self) -> &'a ChangeBroadcaster<'a> {
        &self.change_broadcaster
    }
}

impl<'a> SetSource for AudioThumbnail<'a> { 

    /**
      | Specifies the file or stream that contains
      | the audio file.
      | 
      | For a file, just call @code setSource
      | (new FileInputSource (file)) @endcode
      | 
      | You can pass a nullptr in here to clear
      | the thumbnail. The source that is passed
      | in will be deleted by this object when
      | it is no longer needed.
      | 
      | -----------
      | @return
      | 
      | true if the source could be opened as
      | a valid audio file, false if this failed
      | for some reason.
      |
      */
    fn set_source(&mut self, new_source: &mut dyn Read) -> bool {
        
        todo!();
        /*
            clear();

        return newSource != nullptr && setDataSource (new AudioThumbnailLevelDataSource (*this, newSource));
        */
    }
}

impl<'a> SetReader for AudioThumbnail<'a> { 

    /**
      | Gives the thumbnail an AudioFormatReader
      | to use directly. This will start parsing
      | the audio in a background thread (unless
      | the hash code can be looked-up successfully
      | in the thumbnail cache). Note that the
      | reader object will be held by the thumbnail
      | and deleted later when no longer needed.
      | The thumbnail will actually keep hold
      | of this reader until you clear the thumbnail
      | or change the input source, so the file
      | will be held open for all this time. If
      | you don't want the thumbnail to keep
      | a file handle open continuously, you
      | should use the setSource() method instead,
      | which will only open the file when it
      | needs to.
      |
      */
    fn set_reader(
        &mut self, 
        new_reader: *mut AudioFormatReader<'_>,
        hash:       i64

    ) {
        
        todo!();
        /*
            clear();

        if (newReader != nullptr)
            setDataSource (new AudioThumbnailLevelDataSource (*this, newReader, hash));
        */
    }
}

impl<'a> SaveTo for AudioThumbnail<'a> { 

    /**
      | Saves the low res thumbnail data to an
      | output stream.
      | 
      | The data that is written can later be
      | reloaded using loadFrom(). @see loadFrom
      |
      */
    fn save_to(&self, output: &mut dyn Write)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        const int numThumbnailSamples = channels.size() == 0 ? 0 : channels.getUnchecked(0)->getSize();

        output.write ("jatm", 4);
        output.writeInt (samplesPerThumbSample);
        output.writeInt64 (totalSamples);
        output.writeInt64 (numSamplesFinished);
        output.writeInt (numThumbnailSamples);
        output.writeInt (numChannels);
        output.writeInt ((int) sampleRate);
        output.writeInt64 (0);
        output.writeInt64 (0);

        for (int i = 0; i < numThumbnailSamples; ++i)
            for (int chan = 0; chan < numChannels; ++chan)
                channels.getUnchecked(chan)->getData(i)->write (output);
        */
    }
}

impl<'a> LoadFrom for AudioThumbnail<'a> { 

    /**
      | Reloads the low res thumbnail data from
      | an input stream.
      | 
      | This is not an audio file stream! It takes
      | a stream of thumbnail data that would
      | previously have been created by the
      | saveTo() method. @see saveTo
      |
      */
    fn load_from(&mut self, raw_input: &mut dyn Read) -> bool {
        
        todo!();
        /*
            BufferedInputStream input (rawInput, 4096);

        if (input.readByte() != 'j' || input.readByte() != 'a' || input.readByte() != 't' || input.readByte() != 'm')
            return false;

        const ScopedLock sl (lock);
        clearChannelData();

        samplesPerThumbSample = input.readInt();
        totalSamples = input.readInt64();             // Total number of source samples.
        numSamplesFinished = input.readInt64();       // Number of valid source samples that have been read into the thumbnail.
        int32 numThumbnailSamples = input.readInt();  // Number of samples in the thumbnail data.
        numChannels = input.readInt();                // Number of audio channels.
        sampleRate = input.readInt();                 // Source sample rate.
        input.skipNextBytes (16);                     // (reserved)

        createChannels (numThumbnailSamples);

        for (int i = 0; i < numThumbnailSamples; ++i)
            for (int chan = 0; chan < numChannels; ++chan)
                channels.getUnchecked(chan)->getData(i)->read (input);

        return true;
        */
    }
}

impl<'a> IsFullyLoaded for AudioThumbnail<'a> { 

    /**
      | Returns true if the low res preview is
      | fully generated.
      |
      */
    fn is_fully_loaded(&self) -> bool {
        
        todo!();
        /*
            return numSamplesFinished >= totalSamples - samplesPerThumbSample;
        */
    }
}

impl<'a> IncomingDataReceiver for AudioThumbnail<'a> { 

    /**
      | Resets the thumbnail, ready for adding
      | data with the specified format. If you're
      | going to generate a thumbnail yourself,
      | call this before using addBlock() to
      | add the data.
      |
      */
    fn reset(
        &mut self, 
        new_num_channels:        i32,
        new_sample_rate:         f64,
        total_samples_in_source: Option<i64>
    ) {

        let total_samples_in_source: i64 = total_samples_in_source.unwrap_or(0);
        
        todo!();
        /*
            clear();

        const ScopedLock sl (lock);
        numChannels = newNumChannels;
        sampleRate = newSampleRate;
        totalSamples = totalSamplesInSource;

        createChannels (1 + (int) (totalSamplesInSource / samplesPerThumbSample));
        */
    }
    
    /**
      | Adds a block of level data to the thumbnail.
      | Call reset() before using this, to tell
      | the thumbnail about the data format.
      |
      */
    fn add_block(
        &mut self, 
        start_sample:           i64,
        incoming:               &AudioBuffer<f32>,
        start_offset_in_buffer: i32,
        num_samples:            i32
    ) {
        
        todo!();
        /*
            jassert (startSample >= 0
                  && startOffsetInBuffer >= 0
                  && startOffsetInBuffer + numSamples <= incoming.getNumSamples());

        auto firstThumbIndex = (int) (startSample / samplesPerThumbSample);
        auto lastThumbIndex  = (int) ((startSample + numSamples + (samplesPerThumbSample - 1)) / samplesPerThumbSample);
        auto numToDo = lastThumbIndex - firstThumbIndex;

        if (numToDo > 0)
        {
            auto numChans = jmin (channels.size(), incoming.getNumChannels());

            const HeapBlock<AudioThumbnailMinMaxValue> thumbData (numToDo * numChans);
            const HeapBlock<AudioThumbnailMinMaxValue*> thumbChannels (numChans);

            for (int chan = 0; chan < numChans; ++chan)
            {
                auto* sourceData = incoming.getReadPointer (chan, startOffsetInBuffer);
                auto* dest = thumbData + numToDo * chan;
                thumbChannels [chan] = dest;

                for (int i = 0; i < numToDo; ++i)
                {
                    auto start = i * samplesPerThumbSample;
                    dest[i].setFloat (FloatVectorOperations::findMinAndMax (sourceData + start, jmin (samplesPerThumbSample, numSamples - start)));
                }
            }

            setLevels (thumbChannels, firstThumbIndex, numChans, numToDo);
        }
        */
    }
}

impl<'a> GetTotalLength for AudioThumbnail<'a> { 

    /**
      | Returns the length of the audio file,
      | in seconds.
      |
      */
    fn get_total_length(&self) -> f64 {
        
        todo!();
        /*
            return sampleRate > 0 ? ((double) totalSamples / sampleRate) : 0.0;
        */
    }
}

impl<'a> GetNumSamplesFinished for AudioThumbnail<'a> { 

    /**
      | Returns the number of samples that have
      | been set in the thumbnail.
      |
      */
    fn get_num_samples_finished(&self) -> i64 {
        
        todo!();
        /*
            return numSamplesFinished;
        */
    }
}

impl<'a> GetNumChannels for AudioThumbnail<'a> { 

    /**
      | Returns the number of channels in the
      | file.
      |
      */
    fn get_num_channels(&self) -> i32 {
        
        todo!();
        /*
            return numChannels;
        */
    }
}

impl<'a> DrawChannels for AudioThumbnail<'a> { 

    /**
      | Draws the waveforms for all channels
      | in the thumbnail.
      | 
      | This will call drawChannel() to render
      | each of the thumbnail's channels, stacked
      | above each other within the specified
      | area.
      | 
      | @see drawChannel
      |
      */
    fn draw_channels(
        &mut self, 
        g:                    &mut Graphics,
        area:                 &Rectangle<i32>,
        start_time_seconds:   f64,
        end_time_seconds:     f64,
        vertical_zoom_factor: f32

    ) {
        
        todo!();
        /*
            for (int i = 0; i < numChannels; ++i)
        {
            auto y1 = roundToInt ((i * area.getHeight()) / numChannels);
            auto y2 = roundToInt (((i + 1) * area.getHeight()) / numChannels);

            drawChannel (g, { area.getX(), area.getY() + y1, area.getWidth(), y2 - y1 },
                         startTimeSeconds, endTimeSeconds, i, verticalZoomFactor);
        }
        */
    }
}

impl<'a> DrawChannel for AudioThumbnail<'a> { 

    /**
      | Draws the waveform for a channel.
      | 
      | The waveform will be drawn within the
      | specified rectangle, where startTime
      | and endTime specify the times within
      | the audio file that should be positioned
      | at the left and right edges of the rectangle.
      | 
      | The waveform will be scaled vertically
      | so that a full-volume sample will fill
      | the rectangle vertically, but you can
      | also specify an extra vertical scale
      | factor with the verticalZoomFactor
      | parameter.
      |
      */
    fn draw_channel(
        &mut self, 
        g:                    &mut Graphics,
        area:                 &Rectangle<i32>,
        start_time:           f64,
        end_time:             f64,
        channel_num:          i32,
        vertical_zoom_factor: f32

    ) {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        window->drawChannel (g, area, startTime, endTime, channelNum, verticalZoomFactor,
                             sampleRate, numChannels, samplesPerThumbSample, source.get(), channels);
        */
    }
}

impl<'a> Clear for AudioThumbnail<'a> { 

    /**
      | Clears and resets the thumbnail.
      |
      */
    fn clear(&mut self)  {
        
        todo!();
        /*
            source.reset();
        const ScopedLock sl (lock);
        clearChannelData();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioThumbnail.cpp]
impl<'a> Drop for AudioThumbnail<'a> {

    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a> AudioThumbnail<'a> {

    /**
      | Creates an audio thumbnail.
      | 
      | -----------
      | @param sourceSamplesPerThumbnailSample
      | 
      | when creating a stored, low-res version
      | of the audio data, this is the scale at
      | which it should be done. (This number
      | is the number of original samples that
      | will be averaged for each low-res sample)
      | ----------
      | @param formatManagerToUse
      | 
      | the audio format manager that is used
      | to open the file
      | ----------
      | @param cacheToUse
      | 
      | an instance of an AudioThumbnailCache
      | - this provides a background thread
      | and storage that is used to by the thumbnail,
      | and the cache object can be shared between
      | multiple thumbnails
      |
      */
    pub fn new(
        original_samples_per_thumbnail_sample: i32,
        format_manager:                        &mut AudioFormatManager,
        cache_to_use:                          &mut AudioThumbnailCache) -> Self {
    
        todo!();
        /*


            : formatManagerToUse (formatManager),
          cache (cacheToUse),
          window (new AudioThumbnailCachedWindow()),
          samplesPerThumbSample (originalSamplesPerThumbnailSample)
        */
    }
    
    pub fn clear_channel_data(&mut self)  {
        
        todo!();
        /*
            window->invalidate();
        channels.clear();
        totalSamples = numSamplesFinished = 0;
        numChannels = 0;
        sampleRate = 0;

        sendChangeMessage();
        */
    }
    
    pub fn create_channels(&mut self, length: i32)  {
        
        todo!();
        /*
            while (channels.size() < numChannels)
            channels.add (new AudioThumbnailThumbData (length));
        */
    }
    
    pub fn set_data_source(&mut self, new_source: *mut AudioThumbnailLevelDataSource) -> bool {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        numSamplesFinished = 0;
        auto wasSuccessful = [&] { return sampleRate > 0 && totalSamples > 0; };

        if (cache.loadThumb (*this, newSource->hashCode) && isFullyLoaded())
        {
            source.reset (newSource); // (make sure this isn't done before loadThumb is called)

            source->lengthInSamples = totalSamples;
            source->sampleRate = sampleRate;
            source->numChannels = (unsigned int) numChannels;
            source->numSamplesFinished = numSamplesFinished;

            return wasSuccessful();
        }

        source.reset (newSource);

        const ScopedLock sl (lock);
        source->initialise (numSamplesFinished);

        totalSamples = source->lengthInSamples;
        sampleRate = source->sampleRate;
        numChannels = (int32) source->numChannels;

        createChannels (1 + (int) (totalSamples / samplesPerThumbSample));

        return wasSuccessful();
        */
    }
    
    
    
    /**
      | Returns the hash code that was set by
      | setSource() or setReader().
      |
      */
    pub fn get_hash_code(&self) -> i64 {
        
        todo!();
        /*
            return source == nullptr ? 0 : source->hashCode;
        */
    }
    
    
    pub fn set_levels(&mut self, 
        values:      *const *const AudioThumbnailMinMaxValue,
        thumb_index: i32,
        num_chans:   i32,
        num_values:  i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (int i = jmin (numChans, channels.size()); --i >= 0;)
            channels.getUnchecked(i)->write (values[i], thumbIndex, numValues);

        auto start = thumbIndex * (int64) samplesPerThumbSample;
        auto end   = (thumbIndex + numValues) * (int64) samplesPerThumbSample;

        if (numSamplesFinished >= start && end > numSamplesFinished)
            numSamplesFinished = end;

        totalSamples = jmax (numSamplesFinished, totalSamples.load());
        window->invalidate();
        sendChangeMessage();
        */
    }
    
    /**
      | Returns a value between 0 and 1 to indicate
      | the progress towards loading the entire
      | file.
      |
      */
    pub fn get_proportion_complete(&self) -> f64 {
        
        todo!();
        /*
            return jlimit (0.0, 1.0, (double) numSamplesFinished / (double) jmax ((int64) 1, totalSamples.load()));
        */
    }
    
    /**
      | Returns the highest level in the thumbnail.
      | Note that because the thumb only stores
      | low-resolution data, this isn't an
      | accurate representation of the highest
      | value, it's only a rough approximation.
      |
      */
    pub fn get_approximate_peak(&self) -> f32 {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        int peak = 0;

        for (auto* c : channels)
            peak = jmax (peak, c->getPeak());

        return (float) jlimit (0, 127, peak) / 127.0f;
        */
    }
    
    /**
      | Reads the approximate min and max levels
      | from a section of the thumbnail. The
      | lowest and highest samples are returned
      | in minValue and maxValue, but obviously
      | because the thumb only stores low-resolution
      | data, these numbers will only be a rough
      | approximation of the true values.
      |
      */
    pub fn get_approximate_min_max(
        &self, 
        start_time:    f64,
        end_time:      f64,
        channel_index: i32,
        min_value:     &mut f32,
        max_value:     &mut f32

    ) {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        AudioThumbnailMinMaxValue result;
        auto* data = channels [channelIndex];

        if (data != nullptr && sampleRate > 0)
        {
            auto firstThumbIndex = (int) ((startTime * sampleRate) / samplesPerThumbSample);
            auto lastThumbIndex  = (int) (((endTime * sampleRate) + samplesPerThumbSample - 1) / samplesPerThumbSample);

            data->getMinMax (jmax (0, firstThumbIndex), lastThumbIndex, result);
        }

        minValue = result.getMinValue() / 128.0f;
        maxValue = result.getMaxValue() / 128.0f;
        */
    }
}
