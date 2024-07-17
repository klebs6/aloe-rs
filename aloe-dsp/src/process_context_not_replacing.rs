crate::ix!();

/**
  | Contains context information that
  | is passed into an algorithm's process
  | method.
  | 
  | This context is intended for use in situations
  | where two different blocks are being
  | used the input and output to the process
  | algorithm, so the processor must read
  | from the block returned by getInputBlock()
  | and write its results to the block returned
  | by getOutputBlock().
  | 
  | @see ProcessContextReplacing
  | 
  | @tags{DSP}
  |
  */
pub struct ProcessContextNonReplacing<'a, ContextSampleType: SampleTypeInterface + 'a> {

    /**
      | If set to true, then a processor's process()
      | method is expected to do whatever is
      | appropriate for it to be in a bypassed
      | state.
      |
      */
    is_bypassed:  bool, // default = false
    input_block:  <Self as HasAudioBlockType>::AudioBlockType,
    output_block: &'a mut <Self as HasAudioBlockType>::AudioBlockType,
}

pub trait HasSampleType {

    type SampleType;
}

pub trait HasAudioBlockType {

    type AudioBlockType;
}

impl<'a,ContextSampleType: SampleTypeInterface + 'a> HasSampleType for ProcessContextNonReplacing<'a, ContextSampleType> {

    /**
      | The type of a single sample (which may
      | be a vector if multichannel).
      |
      */
    type SampleType = ContextSampleType;
}

impl<'a,ContextSampleType: SampleTypeInterface + 'a> HasAudioBlockType for ProcessContextNonReplacing<'a, ContextSampleType> {

    /**
      | The type of audio block that this context
      | handles.
      |
      */
    type AudioBlockType = AudioBlock<<Self as HasSampleType>::SampleType>;
}

impl<'a,ContextSampleType: SampleTypeInterface + 'a> ProcessContextNonReplacing<'a,ContextSampleType> {

    /**
      | Creates a ProcessContextReplacing
      | that uses the given input and output
      | blocks.
      | 
      | -----------
      | @note
      | 
      | the caller must not delete these blocks
      | while they are still in use by this object!
      |
      */
    pub fn new(
        input:  &<Self as HasAudioBlockType>::AudioBlockType,
        output: &mut <Self as HasAudioBlockType>::AudioBlockType) -> Self {
    
        todo!();
        /*
        : input_block(input),
        : output_block(output),

            // If the input and output blocks are the same then you should use
            // ProcessContextReplacing instead.
            jassert (input != output);
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
            return inputBlock;
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
            return outputBlock;
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
            return true;
        */
    }
}
