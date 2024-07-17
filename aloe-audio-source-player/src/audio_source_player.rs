crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/sources/aloe_AudioSourcePlayer.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/sources/aloe_AudioSourcePlayer.cpp]

/**
  | Wrapper class to continuously stream
  | audio from an audio source to an AudioIODevice.
  | 
  | This object acts as an AudioIODeviceCallback,
  | so can be attached to an output device,
  | and will stream audio from an AudioSource.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioSourcePlayer {
    read_lock:    CriticalSection,
    source:       *mut dyn AudioSource,  // default = nullptr
    sample_rate:  f64,               // default = 0
    buffer_size:  i32,               // default = 0
    channels:     *mut [f32; 128],
    output_chans: *mut [f32; 128],
    input_chans:  *const [f32; 128],
    temp_buffer:  AudioBuffer<f32>,
    last_gain:    f32,               // default = 1.0f
    gain:         Atomic<f32>,       // default = 1.0f 
}

impl Default for AudioSourcePlayer {
    
    /**
      | Creates an empty AudioSourcePlayer.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl Drop for AudioSourcePlayer {

    /**
      | Destructor.
      | 
      | Make sure this object isn't still being
      | used by an AudioIODevice before deleting
      | it!
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        setSource (nullptr);
 */
    }
}

impl AudioSourcePlayer {
    
    /**
      | Returns the source that's playing.
      | 
      | May return nullptr if there's no source.
      |
      */
    pub fn get_current_source(&self) -> *mut dyn AudioSource {
        
        todo!();
        /*
            return source;
        */
    }

    /**
      | Returns the current gain. @see setGain
      |
      */
    pub fn get_gain(&self) -> f32 {
        
        todo!();
        /*
            return gain;
        */
    }

    /**
      | Changes the current audio source to
      | play from.
      | 
      | If the source passed in is already being
      | used, this method will do nothing.
      | 
      | If the source is not null, its prepareToPlay()
      | method will be called before it starts
      | being used for playback.
      | 
      | If there's another source currently
      | playing, its releaseResources() method
      | will be called after it has been swapped
      | for the new one.
      | 
      | -----------
      | @param newSource
      | 
      | the new source to use - this will NOT be
      | deleted by this object when no longer
      | needed, so it's the caller's responsibility
      | to manage it.
      |
      */
    pub fn set_source(&mut self, new_source: *mut dyn AudioSource)  {
        
        todo!();
        /*
            if (source != newSource)
        {
            auto* oldSource = source;

            if (newSource != nullptr && bufferSize > 0 && sampleRate > 0)
                newSource->prepareToPlay (bufferSize, sampleRate);

            {
                const ScopedLock sl (readLock);
                source = newSource;
            }

            if (oldSource != nullptr)
                oldSource->releaseResources();
        }
        */
    }
    
    /**
      | Sets a gain to apply to the audio data.
      | @see getGain
      |
      */
    pub fn set_gain(&mut self, new_gain: f32)  {
        
        todo!();
        /*
            gain = newGain;
        */
    }
    
    /**
      | An alternative method for initialising
      | the source without an AudioIODevice.
      |
      */
    pub fn prepare_to_play(&mut self, 
        new_sample_rate: f64,
        new_buffer_size: i32)  {
        
        todo!();
        /*
            sampleRate = newSampleRate;
        bufferSize = newBufferSize;
        zeromem (channels, sizeof (channels));

        if (source != nullptr)
            source->prepareToPlay (bufferSize, sampleRate);
        */
    }
}
