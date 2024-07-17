crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_MemoryAudioSource.h]

/**
  | An AudioSource which takes some float
  | audio data as an input.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MemoryAudioSource {
    buffer:               AudioBuffer<f32>,
    position:             i32, // default = 0
    is_currently_looping: bool,
}

impl AudioSource for MemoryAudioSource {}

impl PrepareToPlayAudioSource for MemoryAudioSource {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64)  {
        
        todo!();
        /*
            position = 0;
        */
    }
}
    
impl ReleaseResources for MemoryAudioSource {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl GetNextAudioBlock for MemoryAudioSource {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            if (buffer.getNumSamples() == 0)
        {
            bufferToFill.clearActiveBufferRegion();
            return;
        }

        auto& dst = *bufferToFill.buffer;
        auto channels = jmin (dst.getNumChannels(), buffer.getNumChannels());
        int max = 0, pos = 0;
        auto n = buffer.getNumSamples();
        auto m = bufferToFill.numSamples;

        int i = position;
        for (; (i < n || isCurrentlyLooping) && (pos < m); i += max)
        {
            max = jmin (m - pos, n - (i % n));

            int ch = 0;
            for (; ch < channels; ++ch)
                dst.copyFrom (ch, bufferToFill.startSample + pos, buffer, ch, i % n, max);

            for (; ch < dst.getNumChannels(); ++ch)
                dst.clear (ch, bufferToFill.startSample + pos, max);

            pos += max;
        }

        if (pos < m)
            dst.clear (bufferToFill.startSample + pos, m - pos);

        position = i;
        */
    }
    
}

impl PositionableAudioSource for MemoryAudioSource {
    
    /**
      | Implementation of the
      | PositionableAudioSource method.
      |
      */
    fn set_next_read_position(&mut self, new_position: i64)  {
        
        todo!();
        /*
            position = (int) newPosition;
        */
    }
    
    /**
      | Implementation of the
      | PositionableAudioSource method.
      |
      */
    fn get_next_read_position(&self) -> i64 {
        
        todo!();
        /*
            return position;
        */
    }
    
    /**
      | Implementation of the
      | PositionableAudioSource method.
      |
      */
    fn get_total_length(&self) -> i64 {
        
        todo!();
        /*
            return buffer.getNumSamples();
        */
    }
    
    /**
      | Implementation of the
      | PositionableAudioSource method.
      |
      */
    fn is_looping(&self) -> bool {
        
        todo!();
        /*
            return isCurrentlyLooping;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_MemoryAudioSource.cpp]
impl MemoryAudioSource {

    /**
      | Creates a MemoryAudioSource by providing
      | an audio buffer.
      | 
      | If copyMemory is true then the buffer
      | will be copied into an internal buffer
      | which will be owned by the MemoryAudioSource.
      | If copyMemory is false, then you must
      | ensure that the lifetime of the audio
      | buffer is at least as long as the MemoryAudioSource.
      |
      */
    pub fn new(
        buffer_to_use: &mut AudioBuffer<f32>,
        copy_memory:   bool,
        should_loop:   Option<bool>

    ) -> Self {
    
        let should_loop: bool = should_loop.unwrap_or(false);

        todo!();
        /*
        : is_currently_looping(shouldLoop),

            if (copyMemory)
            buffer.makeCopyOf (bufferToUse);
        else
            buffer.setDataToReferTo (bufferToUse.getArrayOfWritePointers(),
                                     bufferToUse.getNumChannels(),
                                     bufferToUse.getNumSamples());
        */
    }
    
    
    /**
      | Implementation of the
      | PositionableAudioSource method.
      |
      */
    pub fn set_looping(&mut self, should_loop: bool)  {
        
        todo!();
        /*
            isCurrentlyLooping = shouldLoop;
        */
    }
}
