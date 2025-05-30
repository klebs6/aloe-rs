crate::ix!();

pub trait AudioAppComponentInterface {

    /**
      | Tells the source to prepare for playing.
      | 
      | An AudioSource has two states: prepared
      | and unprepared.
      | 
      | The prepareToPlay() method is guaranteed
      | to be called at least once on an 'unprepared'
      | source to put it into a 'prepared' state
      | before any calls will be made to getNextAudioBlock().
      | This callback allows the source to initialise
      | any resources it might need when playing.
      | 
      | Once playback has finished, the releaseResources()
      | method is called to put the stream back
      | into an 'unprepared' state.
      | 
      | Note that this method could be called
      | more than once in succession without
      | a matching call to releaseResources(),
      | so make sure your code is robust and can
      | handle that kind of situation.
      | 
      | -----------
      | @param samplesPerBlockExpected
      | 
      | the number of samples that the source
      | will be expected to supply each time
      | its getNextAudioBlock() method is
      | called. This number may vary slightly,
      | because it will be dependent on audio
      | hardware callbacks, and these aren't
      | guaranteed to always use a constant
      | block size, so the source should be able
      | to cope with small variations.
      | ----------
      | @param sampleRate
      | 
      | the sample rate that the output will
      | be used at - this is needed by sources
      | such as tone generators. @see releaseResources,
      | getNextAudioBlock
      |
      */
    fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64);

    /**
      | Allows the source to release anything
      | it no longer needs after playback has
      | stopped.
      | 
      | This will be called when the source is
      | no longer going to have its getNextAudioBlock()
      | method called, so it should release
      | any spare memory, etc. that it might
      | have allocated during the prepareToPlay()
      | call.
      | 
      | Note that there's no guarantee that
      | prepareToPlay() will actually have
      | been called before releaseResources(),
      | and it may be called more than once in
      | succession, so make sure your code is
      | robust and doesn't make any assumptions
      | about when it will be called.
      | 
      | @see prepareToPlay, getNextAudioBlock
      |
      */
    fn release_resources(&mut self);

    /**
      | Called repeatedly to fetch subsequent
      | blocks of audio data.
      | 
      | After calling the prepareToPlay()
      | method, this callback will be made each
      | time the audio playback hardware (or
      | whatever other destination the audio
      | data is going to) needs another block
      | of data.
      | 
      | It will generally be called on a high-priority
      | system thread, or possibly even an interrupt,
      | so be careful not to do too much work here,
      | as that will cause audio glitches!
      | 
      | @see AudioSourceChannelInfo, prepareToPlay,
      | releaseResources
      |
      */
    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo);
}
