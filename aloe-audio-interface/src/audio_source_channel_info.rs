crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_AudioSource.h]

/**
  | Used by AudioSource::getNextAudioBlock().
  |
  | @tags{Audio}
  */
pub struct AudioSourceChannelInfo {

    /** 
      | The destination buffer to fill with audio
      | data.
      |
      | When the AudioSource::getNextAudioBlock()
      | method is called, the active section of
      | this buffer should be filled with whatever
      | output the source produces.
      |
      | Only the samples specified by the
      | startSample and numSamples members of this
      | structure should be affected by the call.
      |
      | The contents of the buffer when it is
      | passed to the
      | AudioSource::getNextAudioBlock() method
      | can be treated as the input if the source
      | is performing some kind of filter
      | operation, but should be cleared if this
      | is not the case - the
      | clearActiveBufferRegion() is a handy way
      | of doing this.
      |
      | The number of channels in the buffer could
      | be anything, so the AudioSource must cope
      | with this in whatever way is appropriate
      | for its function.
      */
    buffer: *mut AudioBuffer<f32>,

    /**
      | The first sample in the buffer from which
      | the callback is expected to write data.
      |
      */
    start_sample: i32,

    /**
      | The number of samples in the buffer which
      | the callback is expected to fill with
      | data.
      |
      */
    num_samples:  i32,
}

impl Default for AudioSourceChannelInfo {

    /// Creates an uninitialised AudioSourceChannelInfo
    fn default() -> Self {

        Self {
            buffer:       null_mut(),
            start_sample: 0,
            num_samples:  0,
        }
    }
}

impl AudioSourceChannelInfo {

    /**
      | Creates an AudioSourceChannelInfo
      |
      */
    pub fn new(
        buffer_to_use:       *mut AudioBuffer<f32>,
        start_sample_offset: i32,
        num_samples_to_use:  i32) -> Self {
    
        todo!();
        /*
        : buffer(bufferToUse),
        : start_sample(startSampleOffset),
        : num_samples(numSamplesToUse),

        
        */
    }

    /** 
      | Creates an AudioSourceChannelInfo that uses
      | the whole of a buffer.  Note that the buffer
      | provided must not be deleted while the
      | AudioSourceChannelInfo is still using it.
      */
    pub fn new_with_whole_buffer(buffer_to_use: &mut AudioBuffer<f32>) -> Self {
    
        todo!();
        /*
        : buffer(&bufferToUse),
        : start_sample(0),
        : num_samples(bufferToUse.getNumSamples()),
        */
    }

    /**
      | Convenient method to clear the buffer
      | if the source is not producing any data.
      |
      */
    pub fn clear_active_buffer_region(&self)  {
        
        todo!();
        /*
            if (buffer != nullptr)
                buffer->clear (startSample, numSamples);
        */
    }
}
