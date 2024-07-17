crate::ix!();

pub trait GetNextAudioBlock {

    /** 
      | Called repeatedly to fetch subsequent blocks
      | of audio data.
      |
      | After calling the prepareToPlay() method,
      | this callback will be made each time the
      | audio playback hardware (or whatever other
      | destination the audio data is going to)
      | needs another block of data.
      |
      | It will generally be called on
      | a high-priority system thread, or possibly
      | even an interrupt, so be careful not to do
      | too much work here, as that will cause audio
      | glitches!
      |
      | @see AudioSourceChannelInfo, prepareToPlay,
      | releaseResources
      */
    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo);
}
