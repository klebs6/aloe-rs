crate::ix!();

/**
  | An AudioSource which simply outputs
  | a buffer
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioBufferSource {
    buffer:                   OptionalScopedPointer<AudioBuffer<f32>>,
    position:                 i32, // default = 0
    looping:                  bool, // default = false
    play_across_all_channels: bool,
}

impl AudioSource for AudioBufferSource {}

impl PrepareToPlayAudioSource for AudioBufferSource {

    fn prepare_to_play(&mut self, _0: i32, _1: f64)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl ReleaseResources for AudioBufferSource {

    fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl GetNextAudioBlock for AudioBufferSource {
    
    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            bufferToFill.clearActiveBufferRegion();

            const int bufferSize = buffer->getNumSamples();
            const int samplesNeeded = bufferToFill.numSamples;
            const int samplesToCopy = jmin (bufferSize - position, samplesNeeded);

            if (samplesToCopy > 0)
            {
                int maxInChannels = buffer->getNumChannels();
                int maxOutChannels = bufferToFill.buffer->getNumChannels();

                if (! playAcrossAllChannels)
                    maxOutChannels = jmin (maxOutChannels, maxInChannels);

                for (int i = 0; i < maxOutChannels; ++i)
                    bufferToFill.buffer->copyFrom (i, bufferToFill.startSample, *buffer,
                                                   i % maxInChannels, position, samplesToCopy);
            }

            position += samplesNeeded;

            if (looping)
                position %= bufferSize;
        */
    }
}

impl PositionableAudioSource for AudioBufferSource {

    fn set_next_read_position(&mut self, new_position: i64)  {
        
        todo!();
        /*
            jassert (newPosition >= 0);

            if (looping)
                newPosition = newPosition % static_cast<int64> (buffer->getNumSamples());

            position = jmin (buffer->getNumSamples(), static_cast<int> (newPosition));
        */
    }
    
    fn get_next_read_position(&self) -> i64 {
        
        todo!();
        /*
            return static_cast<int64> (position);
        */
    }
    
    fn get_total_length(&self) -> i64 {
        
        todo!();
        /*
            return static_cast<int64> (buffer->getNumSamples());
        */
    }
    
    fn is_looping(&self) -> bool {
        
        todo!();
        /*
            return looping;
        */
    }
}

impl AudioBufferSource {

    pub fn new(
        audio_buffer:         *mut AudioBuffer<f32>,
        own_buffer:           bool,
        play_on_all_channels: bool) -> Self {
    
        todo!();
        /*
        : buffer(audioBuffer, ownBuffer),
        : play_across_all_channels(playOnAllChannels),

        
        */
    }
    
    pub fn set_looping(&mut self, should_loop: bool)  {
        
        todo!();
        /*
            looping = shouldLoop;
        */
    }
}
