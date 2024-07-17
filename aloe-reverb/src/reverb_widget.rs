crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Reverb.h]

/**
  | Processor wrapper around Reverb for
  | easy integration into ProcessorChain.
  | 
  | @tags{DSP}
  |
  */
pub struct ReverbWidget {
    reverb:  Reverb,
    enabled: bool, // default = true
}

impl Default for ReverbWidget {
    
    /**
      | Creates an uninitialised Reverb processor.
      | Call prepare() before first use.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl ReverbWidget {

    /**
      | Returns the reverb's current parameters.
      |
      */
    pub fn get_parameters(&self) -> &ReverbParameters {
        
        todo!();
        /*
            return reverb.getParameters();
        */
    }

    /**
      | Applies a new set of parameters to the
      | reverb.
      | 
      | -----------
      | @note
      | 
      | this doesn't attempt to lock the reverb,
      | so if you call this in parallel with the
      | process method, you may get artifacts.
      |
      */
    pub fn set_parameters(&mut self, new_params: &ReverbParameters)  {
        
        todo!();
        /*
            reverb.setParameters (newParams);
        */
    }

    /**
      | Returns true if the reverb is enabled.
      |
      */
    pub fn is_enabled(&self) -> bool {
        
        todo!();
        /*
            return enabled;
        */
    }

    /**
      | Enables/disables the reverb.
      |
      */
    pub fn set_enabled(&mut self, new_value: bool)  {
        
        todo!();
        /*
            enabled = newValue;
        */
    }

    /**
      | Initialises the reverb.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            reverb.setSampleRate (spec.sampleRate);
        */
    }

    /**
      | Resets the reverb's internal state.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            reverb.reset();
        */
    }
    
    /**
      | Applies the reverb to a mono or stereo
      | buffer.
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            const auto& inputBlock = context.getInputBlock();
            auto& outputBlock = context.getOutputBlock();
            const auto numInChannels = inputBlock.getNumChannels();
            const auto numOutChannels = outputBlock.getNumChannels();
            const auto numSamples = outputBlock.getNumSamples();

            jassert (inputBlock.getNumSamples() == numSamples);

            outputBlock.copyFrom (inputBlock);

            if (! enabled || context.isBypassed)
                return;

            if (numInChannels == 1 && numOutChannels == 1)
            {
                reverb.processMono (outputBlock.getChannelPointer (0), (int) numSamples);
            }
            else if (numInChannels == 2 && numOutChannels == 2)
            {
                reverb.processStereo (outputBlock.getChannelPointer (0),
                                      outputBlock.getChannelPointer (1),
                                      (int) numSamples);
            }
            else
            {
                jassertfalse;   // invalid channel configuration
            }
        */
    }
}
