crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_ProcessorChain.h]

pub type MaybeBypassedAudioProcessor = (Box<dyn AudioProcessorInterface>, RefCell<bool>);

/**
  | This variadically-templated class
  | lets you join together any number of
  | processor classes into a single processor
  | which will call process() on them all
  | in sequence.
  | 
  | @tags{DSP}
  |
  */
pub struct ProcessorChain {
    processors: Vec<MaybeBypassedAudioProcessor>,
}

impl ProcessorChain {

    /**
      | Get a reference to the processor at index
      | `INDEX`.
      |
      */
    pub fn get_mut<const INDEX: usize>(&mut self) -> &mut dyn AudioProcessorInterface {
    
        todo!();
        /*
            return std::get<INDEX> (processors);
        */
    }

    /**
      | Get a reference to the processor at index
      | `INDEX`.
      |
      */
    pub fn get<const INDEX: usize>(&self) -> &dyn AudioProcessorInterface {
    
        todo!();
        /*
            return std::get<INDEX> (processors);
        */
    }

    /**
      | Set the processor at index `INDEX` to
      | be bypassed or enabled.
      |
      */
    pub fn set_bypassed<const INDEX: usize>(&mut self, b: bool)  {
    
        todo!();
        /*
            bypassed[(size_t) INDEX] = b;
        */
    }

    /**
      | Query whether the processor at index
      | `INDEX` is bypassed.
      |
      */
    pub fn is_bypassed<const INDEX: usize>(&self) -> bool {
    
        todo!();
        /*
            return bypassed[(size_t) INDEX];
        */
    }

    /**
      | Prepare all inner processors with the
      | provided `ProcessSpec`.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            detail::forEachInTuple ([&] (auto& proc, size_t) { proc.prepare (spec); }, processors);
        */
    }

    /**
      | Reset all inner processors.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            detail::forEachInTuple ([] (auto& proc, size_t) { proc.reset(); }, processors);
        */
    }

    /**
      | Process `context` through all inner
      | processors in sequence.
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            detail::forEachInTuple ([&] (auto& proc, size_t index) 
            {
                if (context.usesSeparateInputAndOutputBlocks() && index != 0)
                {
                    jassert (context.getOutputBlock().getNumChannels() == context.getInputBlock().getNumChannels());
                    ProcessContextReplacing<typename ProcessContext::SampleType> replacingContext (context.getOutputBlock());
                    replacingContext.isBypassed = (bypassed[index] || context.isBypassed);

                    proc.process (replacingContext);
                }
                else
                {
                    ProcessContext contextCopy (context);
                    contextCopy.isBypassed = (bypassed[index] || context.isBypassed);

                    proc.process (contextCopy);
                }
            }, processors);
        */
    }
}

/**
  | Non-member equivalent of ProcessorChain::get
  | which avoids awkward member template
  | syntax.
  |
  */
#[inline] pub fn get<const INDEX: usize>(chain: &mut ProcessorChain) -> &mut dyn AudioProcessorInterface {

    todo!();
    /*
        return chain.template get<INDEX>();
    */
}

/**
  | Non-member equivalent of ProcessorChain::setBypassed
  | which avoids awkward member template
  | syntax.
  |
  */
#[inline] pub fn set_bypassed<const INDEX: usize>(
        chain:    &mut ProcessorChain,
        bypassed: bool)  {

    todo!();
    /*
        chain.template setBypassed<INDEX> (bypassed);
    */
}

/**
  | Non-member equivalent of ProcessorChain::isBypassed
  | which avoids awkward member template
  | syntax.
  |
  */
#[inline] pub fn is_bypassed<const INDEX: usize>(chain: &ProcessorChain) -> bool {

    todo!();
    /*
        return chain.template isBypassed<INDEX>();
    */
}
