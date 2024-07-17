crate::ix!();

/**
  | Contains context information that
  | is passed into an algorithm's process
  | method.
  | 
  | This context is intended for use in situations
  | where a single block is being used for
  | both the input and output, so it will
  | return the same object for both its getInputBlock()
  | and getOutputBlock() methods.
  | 
  | @see ProcessContextNonReplacing
  | 
  | @tags{DSP}
  |
  */
pub struct ProcessContextReplacing<'a, ContextSampleType: SampleTypeInterface + 'a> {

    /**
      | If set to true, then a processor's process()
      | method is expected to do whatever is
      | appropriate for it to be in a bypassed
      | state.
      |
      */
    is_bypassed: bool, // default = false
    io_block:    &'a mut <Self as HasAudioBlockType>::AudioBlockType,
    const_block: <Self as HasAudioBlockType>::AudioBlockType, // default = ioBlock 
}

impl<'a,ContextSampleType: SampleTypeInterface + 'a> HasSampleType for ProcessContextReplacing<'a, ContextSampleType> {

    /**
      | The type of a single sample (which may
      | be a vector if multichannel).
      |
      */
    type SampleType = ContextSampleType;
}

impl<'a,ContextSampleType: SampleTypeInterface + 'a> HasAudioBlockType for ProcessContextReplacing<'a, ContextSampleType> {

    /**
      | The type of audio block that this context
      | handles.
      |
      */
    type AudioBlockType = AudioBlock<<Self as HasSampleType>::SampleType>;
}

impl<'a,ContextSampleType: SampleTypeInterface + 'a> ProcessContextReplacing<'a, ContextSampleType> {

    /**
      | Creates a ProcessContextReplacing
      | that uses the given audio block.
      | 
      | -----------
      | @note
      | 
      | the caller must not delete the block
      | while it is still in use by this object!
      |
      */
    pub fn new(block: &mut <Self as HasAudioBlockType>::AudioBlockType) -> Self {
    
        todo!();
        /*
        : io_block(block),

        
        */
    }

    /**
      | Returns the audio block to use as the
      | input to a process function.
      |
      */
    pub fn get_input_block(&self) -> &<Self as HasAudioBlockType>::AudioBlockType {
        
        todo!();
        /*
            return constBlock;
        */
    }

    /**
      | Returns the audio block to use as the
      | output to a process function.
      |
      */
    pub fn get_output_block(&self) -> &mut <Self as HasAudioBlockType>::AudioBlockType {
        
        todo!();
        /*
            return ioBlock;
        */
    }

    /**
      | All process context classes will define
      | this constant method so that templated
      | code can determine whether the input
      | and output blocks refer to the same buffer,
      | or to two different ones.
      |
      */
    pub fn uses_separate_input_and_output_blocks() -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
