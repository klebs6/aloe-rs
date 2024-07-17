crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/sources/aloe_AudioTransportSource.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/sources/aloe_AudioTransportSource.cpp]

/**
  | An AudioSource that takes a PositionableAudioSource
  | and allows it to be played, stopped,
  | started, etc.
  | 
  | This can also be told use a buffer and
  | background thread to read ahead, and
  | if can correct for different sample-rates.
  | 
  | You may want to use one of these along
  | with an AudioSourcePlayer and AudioIODevice
  | to control playback of an audio file.
  | 
  | @see AudioSource, AudioSourcePlayer
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioTransportSource<'a> {
    base2:                  ChangeBroadcaster<'a>,
    source:                 *mut dyn PositionableAudioSource,   // default = nullptr
    resampler_source:       *mut ResamplingAudioSource,     // default = nullptr
    buffering_source:       *mut BufferingAudioSource<'a>,      // default = nullptr
    positionable_source:    *mut dyn PositionableAudioSource,   // default = nullptr
    master_source:          *mut dyn AudioSource,               // default = nullptr
    callback_lock:          CriticalSection,
    gain:                   f32,                            // default = 1.0f
    last_gain:              f32,                            // default = 1.0f
    playing:                AtomicBool,                     // default = false 
    stopped:                AtomicBool,                     // default = true 
    sample_rate:            f64,                            // default = 44100.0
    source_sample_rate:     f64,                            // default = 0
    block_size:             i32,                            // default = 128
    read_ahead_buffer_size: i32,                            // default = 0
    is_prepared:            bool,                           // default = false
    input_streameof:        bool,                           // default = false
}

impl<'a> Default for AudioTransportSource<'a> {
    
    /**
      | Creates an AudioTransportSource.
      | 
      | After creating one of these, use the
      | setSource() method to select an input
      | source.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl<'a> Drop for AudioTransportSource<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        setSource (nullptr);
        releaseMasterResources();
 */
    }
}

impl<'a> AudioTransportSource<'a> {

    /**
      | Returns true if the player has stopped
      | because its input stream ran out of data.
      |
      */
    pub fn has_stream_finished(&self) -> bool {
        
        todo!();
        /*
            return inputStreamEOF;
        */
    }
    
    /**
      | Returns true if it's currently playing.
      |
      */
    pub fn is_playing(&self) -> bool {
        
        todo!();
        /*
            return playing;
        */
    }

    /**
      | Returns the current gain setting.
      | 
      | @see setGain
      |
      */
    pub fn get_gain(&self) -> f32 {
        
        todo!();
        /*
            return gain;
        */
    }

    /**
      | Sets the reader that is being used as
      | the input source.
      | 
      | This will stop playback, reset the position
      | to 0 and change to the new reader.
      | 
      | The source passed in will not be deleted
      | by this object, so must be managed by
      | the caller.
      | 
      | -----------
      | @param newSource
      | 
      | the new input source to use. This may
      | be a nullptr
      | ----------
      | @param readAheadBufferSize
      | 
      | a size of buffer to use for reading ahead.
      | If this is zero, no reading ahead will
      | be done; if it's greater than zero, a
      | BufferingAudioSource will be used
      | to do the reading-ahead. If you set a
      | non-zero value here, you'll also need
      | to set the readAheadThread parameter.
      | ----------
      | @param readAheadThread
      | 
      | if you set readAheadBufferSize to a
      | non-zero value, then you'll also need
      | to supply this TimeSliceThread object
      | for the background reader to use. The
      | thread object must not be deleted while
      | the AudioTransport source is still
      | using it.
      | ----------
      | @param sourceSampleRateToCorrectFor
      | 
      | if this is non-zero, it specifies the
      | sample rate of the source, and playback
      | will be sample-rate adjusted to maintain
      | playback at the correct pitch. If this
      | is 0, no sample-rate adjustment will
      | be performed
      | ----------
      | @param maxNumChannels
      | 
      | the maximum number of channels that
      | may need to be played
      |
      */
    pub fn set_source(
        &mut self, 
        new_source:                        *mut dyn PositionableAudioSource,
        read_ahead_size:                   Option<i32>,
        read_ahead_thread:                 *mut TimeSliceThread,
        source_sample_rate_to_correct_for: Option<f64>,
        max_num_channels:                  Option<i32>

    ) {

        let read_ahead_size                   = read_ahead_size.unwrap_or(0);
        let source_sample_rate_to_correct_for = source_sample_rate_to_correct_for.unwrap_or(0.0);
        let max_num_channels                  = max_num_channels.unwrap_or(2);
        
        todo!();
        /*
            if (source == newSource)
        {
            if (source == nullptr)
                return;

            setSource (nullptr, 0, nullptr); // deselect and reselect to avoid releasing resources wrongly
        }

        readAheadBufferSize = readAheadSize;
        sourceSampleRate = sourceSampleRateToCorrectFor;

        ResamplingAudioSource* newResamplerSource = nullptr;
        BufferingAudioSource* newBufferingSource = nullptr;
        PositionableAudioSource* newPositionableSource = nullptr;
        AudioSource* newMasterSource = nullptr;

        std::unique_ptr<ResamplingAudioSource> oldResamplerSource (resamplerSource);
        std::unique_ptr<BufferingAudioSource> oldBufferingSource (bufferingSource);
        AudioSource* oldMasterSource = masterSource;

        if (newSource != nullptr)
        {
            newPositionableSource = newSource;

            if (readAheadSize > 0)
            {
                // If you want to use a read-ahead buffer, you must also provide a TimeSliceThread
                // for it to use!
                jassert (readAheadThread != nullptr);

                newPositionableSource = newBufferingSource
                    = new BufferingAudioSource (newPositionableSource, *readAheadThread,
                                                false, readAheadSize, maxNumChannels);
            }

            newPositionableSource->setNextReadPosition (0);

            if (sourceSampleRateToCorrectFor > 0)
                newMasterSource = newResamplerSource
                    = new ResamplingAudioSource (newPositionableSource, false, maxNumChannels);
            else
                newMasterSource = newPositionableSource;

            if (isPrepared)
            {
                if (newResamplerSource != nullptr && sourceSampleRate > 0 && sampleRate > 0)
                    newResamplerSource->setResamplingRatio (sourceSampleRate / sampleRate);

                newMasterSource->prepareToPlay (blockSize, sampleRate);
            }
        }

        {
            const ScopedLock sl (callbackLock);

            source = newSource;
            resamplerSource = newResamplerSource;
            bufferingSource = newBufferingSource;
            masterSource = newMasterSource;
            positionableSource = newPositionableSource;

            inputStreamEOF = false;
            playing = false;
        }

        if (oldMasterSource != nullptr)
            oldMasterSource->releaseResources();
        */
    }
    
    /**
      | Starts playing (if a source has been
      | selected).
      | 
      | If it starts playing, this will send
      | a message to any ChangeListeners that
      | are registered with this object.
      |
      */
    pub fn start(&mut self)  {
        
        todo!();
        /*
            if ((! playing) && masterSource != nullptr)
        {
            {
                const ScopedLock sl (callbackLock);
                playing = true;
                stopped = false;
                inputStreamEOF = false;
            }

            sendChangeMessage();
        }
        */
    }
    
    /**
      | Stops playing.
      | 
      | If it's actually playing, this will
      | send a message to any ChangeListeners
      | that are registered with this object.
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (playing)
        {
            playing = false;

            int n = 500;
            while (--n >= 0 && ! stopped)
                Thread::sleep (2);

            sendChangeMessage();
        }
        */
    }

    /**
      | Changes the current playback position
      | in the source stream.
      | 
      | The next time the getNextAudioBlock()
      | method is called, this is the time from
      | which it'll read data.
      | 
      | -----------
      | @param newPosition
      | 
      | the new playback position in seconds
      | 
      | @see getCurrentPosition
      |
      */
    pub fn set_position(&mut self, new_position: f64)  {
        
        todo!();
        /*
            if (sampleRate > 0.0)
            setNextReadPosition ((int64) (newPosition * sampleRate));
        */
    }

    /**
      | Returns the position that the next data
      | block will be read from
      | 
      | This is a time in seconds.
      |
      */
    pub fn get_current_position(&self) -> f64 {
        
        todo!();
        /*
            if (sampleRate > 0.0)
            return (double) getNextReadPosition() / sampleRate;

        return 0.0;
        */
    }

    /**
      | Returns the stream's length in seconds.
      |
      */
    pub fn get_length_in_seconds(&self) -> f64 {
        
        todo!();
        /*
            if (sampleRate > 0.0)
            return (double) getTotalLength() / sampleRate;

        return 0.0;
        */
    }


    /**
      | Changes the gain to apply to the output.
      | 
      | -----------
      | @param newGain
      | 
      | a factor by which to multiply the outgoing
      | samples, so 1.0 = 0dB, 0.5 = -6dB, 2.0
      | = 6dB, etc.
      |
      */
    pub fn set_gain(&mut self, new_gain: f32)  {
        
        todo!();
        /*
            gain = newGain;
        */
    }

    pub fn release_master_resources(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

        if (masterSource != nullptr)
            masterSource->releaseResources();

        isPrepared = false;
        */
    }
}
