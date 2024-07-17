crate::ix!();

pub trait PrepareToPlay {

    /**
      | Called before playback starts, to let
      | the processor prepare itself.
      | 
      | The sample rate is the target sample
      | rate, and will remain constant until
      | playback stops.
      | 
      | You can call getTotalNumInputChannels
      | and getTotalNumOutputChannels or
      | query the busLayout member variable
      | to find out the number of channels your
      | processBlock callback must process.
      | 
      | The maximumExpectedSamplesPerBlock
      | value is a strong hint about the maximum
      | number of samples that will be provided
      | in each block. You may want to use this
      | value to resize internal buffers. You
      | should program defensively in case
      | a buggy host exceeds this value. The
      | actual block sizes that the host uses
      | may be different each time the callback
      | happens: completely variable block
      | sizes can be expected from some hosts.
      | 
      | @see busLayout, getTotalNumInputChannels,
      | getTotalNumOutputChannels
      |
      */
    fn prepare_to_play(&mut self, 
            sample_rate:                        f64,
            maximum_expected_samples_per_block: i32);

}

pub trait Prepare {

    fn prepare(
        &mut self, 
        new_sample_rate: f64,
        new_block_size:  i32,
        graph:           *mut dyn AudioProcessorGraphInterface,
        precision:       AudioProcessorProcessingPrecision
    );
}

pub trait Unprepare {

    fn unprepare(&mut self);
}

pub trait PrepareToPlayAudioSource {

    /**
      | Tells the source to prepare for playing.
      |
      | An AudioSource has two states: prepared
      | and unprepared. The prepareToPlay()
      | method is guaranteed to be called at
      | least once on an 'unprepared' source
      | to put it into a 'prepared' state before
      | any calls will be made to getNextAudioBlock().
      | This callback allows the source to initialise
      | any resources it might need when playing.
      | Once playback has finished, the releaseResources()
      | method is called to put the stream back
      | into an 'unprepared' state. Note that
      | this method could be called more than
      | once in succession without a matching
      | call to releaseResources(), so make
      | sure your code is robust and can handle
      | that kind of situation.
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
      | such as tone generators.
      | 
      | @see releaseResources, getNextAudioBlock
      |
      */
    fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64);

}

pub trait AnyNodesNeedPreparing {

    fn any_nodes_need_preparing(&self) -> bool;
}

