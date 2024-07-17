crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_BufferingAudioSource.h]

/**
  | An AudioSource which takes another
  | source as input, and buffers it using
  | a thread.
  | 
  | Create this as a wrapper around another
  | thread, and it will read-ahead with
  | a background thread to smooth out playback.
  | You can either create one of these directly,
  | or use it indirectly using an AudioTransportSource.
  | 
  | @see PositionableAudioSource, AudioTransportSource
  | 
  | @tags{Audio}
  |
  */
#[leak_detector]
#[no_copy]
pub struct BufferingAudioSource<'a> {
    base2:                       TimeSliceClient,
    source:                      OptionalScopedPointer<dyn PositionableAudioSource>,
    background_thread:           &'a mut TimeSliceThread,
    number_of_samples_to_buffer: i32,
    number_of_channels:          i32,
    buffer:                      AudioBuffer<f32>,
    callback_lock:               CriticalSection,
    buffer_range_lock:           CriticalSection,
    buffer_ready_event:          WaitableEvent,
    buffer_valid_start:          i64, // default = 0
    buffer_valid_end:            i64, // default = 0
    next_play_pos:               Atomic<i64>, // default = 0 
    sample_rate:                 f64, // default = 0
    was_source_looping:          bool, // default = false
    is_prepared:                 bool, // default = false
    prefill_buffer:              bool,
}

impl<'a> AudioSource for BufferingAudioSource<'a> { }

impl<'a> PrepareToPlayAudioSource for BufferingAudioSource<'a> { 

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn prepare_to_play(
        &mut self, 
        samples_per_block_expected: i32,
        new_sample_rate:            f64

    ) {
        
        todo!();
        /*
            auto bufferSizeNeeded = jmax (samplesPerBlockExpected * 2, numberOfSamplesToBuffer);

        if (newSampleRate != sampleRate
             || bufferSizeNeeded != buffer.getNumSamples()
             || ! isPrepared)
        {
            backgroundThread.removeTimeSliceClient (this);

            isPrepared = true;
            sampleRate = newSampleRate;

            source->prepareToPlay (samplesPerBlockExpected, newSampleRate);

            buffer.setSize (numberOfChannels, bufferSizeNeeded);
            buffer.clear();

            const ScopedLock sl (bufferRangeLock);

            bufferValidStart = 0;
            bufferValidEnd = 0;

            backgroundThread.addTimeSliceClient (this);

            do
            {
                const ScopedUnlock ul (bufferRangeLock);

                backgroundThread.moveToFrontOfQueue (this);
                Thread::sleep (5);
            }
            while (prefillBuffer
             && (bufferValidEnd - bufferValidStart < jmin (((int) newSampleRate) / 4, buffer.getNumSamples() / 2)));
        }
        */
    }
}

impl<'a> ReleaseResources for BufferingAudioSource<'a> { 

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn release_resources(&mut self)  {
        
        todo!();
        /*
            isPrepared = false;
        backgroundThread.removeTimeSliceClient (this);

        buffer.setSize (numberOfChannels, 0);

        // MSVC2015 seems to need this if statement to not generate a warning during linking.
        // As source is set in the constructor, there is no way that source could
        // ever equal this, but it seems to make MSVC2015 happy.
        if (source != this)
            source->releaseResources();
        */
    }
}

impl<'a> GetNextAudioBlock for BufferingAudioSource<'a> { 
    
    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn get_next_audio_block(&mut self, info: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            const auto bufferRange = getValidBufferRange (info.numSamples);

        if (bufferRange.isEmpty())
        {
            // total cache miss
            info.clearActiveBufferRegion();
            return;
        }

        const auto validStart = bufferRange.getStart();
        const auto validEnd = bufferRange.getEnd();

        const ScopedLock sl (callbackLock);

        if (validStart > 0)
            info.buffer->clear (info.startSample, validStart);  // partial cache miss at start

        if (validEnd < info.numSamples)
            info.buffer->clear (info.startSample + validEnd,
                                info.numSamples - validEnd);    // partial cache miss at end

        if (validStart < validEnd)
        {
            for (int chan = jmin (numberOfChannels, info.buffer->getNumChannels()); --chan >= 0;)
            {
                jassert (buffer.getNumSamples() > 0);

                const auto startBufferIndex = (int) ((validStart + nextPlayPos) % buffer.getNumSamples());
                const auto endBufferIndex   = (int) ((validEnd + nextPlayPos)   % buffer.getNumSamples());

                if (startBufferIndex < endBufferIndex)
                {
                    info.buffer->copyFrom (chan, info.startSample + validStart,
                                           buffer,
                                           chan, startBufferIndex,
                                           validEnd - validStart);
                }
                else
                {
                    const auto initialSize = buffer.getNumSamples() - startBufferIndex;

                    info.buffer->copyFrom (chan, info.startSample + validStart,
                                           buffer,
                                           chan, startBufferIndex,
                                           initialSize);

                    info.buffer->copyFrom (chan, info.startSample + validStart + initialSize,
                                           buffer,
                                           chan, 0,
                                           (validEnd - validStart) - initialSize);
                }
            }
        }

        nextPlayPos += info.numSamples;
        */
    }
}

impl<'a> PositionableAudioSource for BufferingAudioSource<'a> { 

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn set_next_read_position(&mut self, new_position: i64)  {
        
        todo!();
        /*
            const ScopedLock sl (bufferRangeLock);

        nextPlayPos = newPosition;
        backgroundThread.moveToFrontOfQueue (this);
        */
    }

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn get_next_read_position(&self) -> i64 {
        
        todo!();
        /*
            jassert (source->getTotalLength() > 0);
        const auto pos = nextPlayPos.load();

        return (source->isLooping() && nextPlayPos > 0)
                        ? pos % source->getTotalLength()
                        : pos;
        */
    }
    
    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn get_total_length(&self) -> i64 {
        
        todo!();
        /*
            return source->getTotalLength();
        */
    }

    /**
      | Implements the PositionableAudioSource
      | method.
      |
      */
    fn is_looping(&self) -> bool {
        
        todo!();
        /*
            return source->isLooping();
        */
    }
}

/**
  | Destructor.
  | 
  | The input source may be deleted depending
  | on whether the deleteSourceWhenDeleted
  | flag was set in the constructor.
  |
  */
impl<'a> Drop for BufferingAudioSource<'a> {

    fn drop(&mut self) {

        todo!();

        /*       
         releaseResources();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_BufferingAudioSource.cpp]
impl<'a> BufferingAudioSource<'a> {

    /**
      | Creates a BufferingAudioSource.
      | 
      | -----------
      | @param source
      | 
      | the input source to read from
      | ----------
      | @param backgroundThread
      | 
      | a background thread that will be used
      | for the background read-ahead. This
      | object must not be deleted until after
      | any BufferingAudioSources that are
      | using it have been deleted!
      | ----------
      | @param deleteSourceWhenDeleted
      | 
      | if true, then the input source object
      | will be deleted when this object is deleted
      | ----------
      | @param numberOfSamplesToBuffer
      | 
      | the size of buffer to use for reading
      | ahead
      | ----------
      | @param numberOfChannels
      | 
      | the number of channels that will be played
      | ----------
      | @param prefillBufferOnPrepareToPlay
      | 
      | if true, then calling prepareToPlay
      | on this object will block until the buffer
      | has been filled
      |
      */
    pub fn new(
        s:                                 *mut dyn PositionableAudioSource,
        thread:                            &mut TimeSliceThread,
        delete_source_when_deleted:        bool,
        buffer_size_samples:               i32,
        num_channels:                      Option<i32>,
        prefill_buffer_on_prepare_to_play: Option<bool>

    ) -> Self {

        let num_channels                      = num_channels.unwrap_or(2);
        let prefill_buffer_on_prepare_to_play = prefill_buffer_on_prepare_to_play.unwrap_or(true);
    
        todo!();
        /*
        : source(s, deleteSourceWhenDeleted),
        : background_thread(thread),
        : number_of_samples_to_buffer(jmax (1024, bufferSizeSamples)),
        : number_of_channels(numChannels),
        : prefill_buffer(prefillBufferOnPrepareToPlay),

            jassert (source != nullptr);

        jassert (numberOfSamplesToBuffer > 1024); // not much point using this class if you're
                                                  //  not using a larger buffer..
        */
    }
    
    
    /**
      | A useful function to block until the
      | next the buffer info can be filled.
      | 
      | This is useful for offline rendering.
      |
      */
    pub fn wait_for_next_audio_block_ready(&mut self, 
        info:    &AudioSourceChannelInfo,
        timeout: u32) -> bool {
        
        todo!();
        /*
            if (source == nullptr || source->getTotalLength() <= 0)
            return false;

        if ((nextPlayPos + info.numSamples < 0)
            || (! isLooping() && nextPlayPos > getTotalLength()))
            return true;

        const auto startTime = Time::getMillisecondCounter();
        auto now = startTime;

        auto elapsed = (now >= startTime ? now - startTime
                                         : (std::numeric_limits<uint32>::max() - startTime) + now);

        while (elapsed <= timeout)
        {
            const auto bufferRange = getValidBufferRange (info.numSamples);

            const auto validStart = bufferRange.getStart();
            const auto validEnd = bufferRange.getEnd();

            if (validStart <= 0
                && validStart < validEnd
                && validEnd >= info.numSamples)
            {
                return true;
            }

            if (elapsed < timeout
                && ! bufferReadyEvent.wait (static_cast<int> (timeout - elapsed)))
            {
                return false;
            }

            now = Time::getMillisecondCounter();
            elapsed = (now >= startTime ? now - startTime
                                        : (std::numeric_limits<uint32>::max() - startTime) + now);
        }

        return false;
        */
    }
    
    pub fn get_valid_buffer_range(&self, num_samples: i32) -> Range<i32> {
        
        todo!();
        /*
            const ScopedLock sl (bufferRangeLock);

        const auto pos = nextPlayPos.load();

        return { (int) (jlimit (bufferValidStart, bufferValidEnd, pos) - pos),
                 (int) (jlimit (bufferValidStart, bufferValidEnd, pos + numSamples) - pos) };
        */
    }
    
    pub fn read_next_buffer_chunk(&mut self) -> bool {
        
        todo!();
        /*
            int64 newBVS, newBVE, sectionToReadStart, sectionToReadEnd;

        {
            const ScopedLock sl (bufferRangeLock);

            if (wasSourceLooping != isLooping())
            {
                wasSourceLooping = isLooping();
                bufferValidStart = 0;
                bufferValidEnd = 0;
            }

            newBVS = jmax ((int64) 0, nextPlayPos.load());
            newBVE = newBVS + buffer.getNumSamples() - 4;
            sectionToReadStart = 0;
            sectionToReadEnd = 0;

            constexpr int maxChunkSize = 2048;

            if (newBVS < bufferValidStart || newBVS >= bufferValidEnd)
            {
                newBVE = jmin (newBVE, newBVS + maxChunkSize);

                sectionToReadStart = newBVS;
                sectionToReadEnd = newBVE;

                bufferValidStart = 0;
                bufferValidEnd = 0;
            }
            else if (std::abs ((int) (newBVS - bufferValidStart)) > 512
                      || std::abs ((int) (newBVE - bufferValidEnd)) > 512)
            {
                newBVE = jmin (newBVE, bufferValidEnd + maxChunkSize);

                sectionToReadStart = bufferValidEnd;
                sectionToReadEnd = newBVE;

                bufferValidStart = newBVS;
                bufferValidEnd = jmin (bufferValidEnd, newBVE);
            }
        }

        if (sectionToReadStart == sectionToReadEnd)
            return false;

        jassert (buffer.getNumSamples() > 0);

        const auto bufferIndexStart = (int) (sectionToReadStart % buffer.getNumSamples());
        const auto bufferIndexEnd   = (int) (sectionToReadEnd   % buffer.getNumSamples());

        if (bufferIndexStart < bufferIndexEnd)
        {
            readBufferSection (sectionToReadStart,
                               (int) (sectionToReadEnd - sectionToReadStart),
                               bufferIndexStart);
        }
        else
        {
            const auto initialSize = buffer.getNumSamples() - bufferIndexStart;

            readBufferSection (sectionToReadStart,
                               initialSize,
                               bufferIndexStart);

            readBufferSection (sectionToReadStart + initialSize,
                               (int) (sectionToReadEnd - sectionToReadStart) - initialSize,
                               0);
        }

        {
            const ScopedLock sl2 (bufferRangeLock);

            bufferValidStart = newBVS;
            bufferValidEnd = newBVE;
        }

        bufferReadyEvent.signal();
        return true;
        */
    }
    
    pub fn read_buffer_section(&mut self, 
        start:         i64,
        length:        i32,
        buffer_offset: i32)  {
        
        todo!();
        /*
            if (source->getNextReadPosition() != start)
            source->setNextReadPosition (start);

        AudioSourceChannelInfo info (&buffer, bufferOffset, length);

        const ScopedLock sl (callbackLock);
        source->getNextAudioBlock (info);
        */
    }
    
    pub fn use_time_slice(&mut self) -> i32 {
        
        todo!();
        /*
            return readNextBufferChunk() ? 1 : 100;
        */
    }
}
