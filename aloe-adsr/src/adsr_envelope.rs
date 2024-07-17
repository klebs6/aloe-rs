crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_ADSR.h]

/**
  | A very simple ADSR envelope class.
  | 
  | To use it, call setSampleRate() with
  | the current sample rate and give it some
  | parameters with setParameters() then
  | call getNextSample() to get the envelope
  | value to be applied to each audio sample
  | or applyEnvelopeToBuffer() to apply
  | the envelope to a whole buffer.
  | 
  | @tags{Audio}
  |
  */
pub struct ADSR {
    state:        AdsrState, // default = AdsrState::idle
    parameters:   AdsrParameters,
    sample_rate:  f64, // default = 44100.0
    envelope_val: f32, // default = 0.0f
    attack_rate:  f32, // default = 0.0f
    decay_rate:   f32, // default = 0.0f
    release_rate: f32, // default = 0.0f
}

impl Default for ADSR {
    
    fn default() -> Self {
        todo!();
        /*


            recalculateRates()
        */
    }
}

impl ADSR {

    /**
      | Sets the parameters that will be used
      | by an ADSR object.
      | 
      | You must have called setSampleRate()
      | with the correct sample rate before
      | this otherwise the values may be incorrect!
      | 
      | @see getParameters
      |
      */
    pub fn set_parameters(&mut self, new_parameters: &AdsrParameters)  {
        
        todo!();
        /*
            // need to call setSampleRate() first!
            jassert (sampleRate > 0.0);

            parameters = newParameters;
            recalculateRates();
        */
    }

    /**
      | Returns the parameters currently being
      | used by an ADSR object.
      | 
      | @see setParameters
      |
      */
    pub fn get_parameters(&self) -> &AdsrParameters {
        
        todo!();
        /*
            return parameters;
        */
    }

    /**
      | Returns true if the envelope is in its
      | attack, decay, sustain or release stage.
      |
      */
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return state != AdsrState::idle;
        */
    }

    /**
      | Sets the sample rate that will be used
      | for the envelope.
      | 
      | This must be called before the getNextSample()
      | or setParameters() methods.
      |
      */
    pub fn set_sample_rate(&mut self, new_sample_rate: f64)  {
        
        todo!();
        /*
            jassert (newSampleRate > 0.0);
            sampleRate = newSampleRate;
        */
    }

    /**
      | Resets the envelope to an idle state.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            envelopeVal = 0.0f;
            state = AdsrState::idle;
        */
    }

    /**
      | Starts the attack phase of the envelope.
      |
      */
    pub fn note_on(&mut self)  {
        
        todo!();
        /*
            if (attackRate > 0.0f)
            {
                state = AdsrState::attack;
            }
            else if (decayRate > 0.0f)
            {
                envelopeVal = 1.0f;
                state = AdsrState::decay;
            }
            else
            {
                state = AdsrState::sustain;
            }
        */
    }

    /**
      | Starts the release phase of the envelope.
      |
      */
    pub fn note_off(&mut self)  {
        
        todo!();
        /*
            if (state != AdsrState::idle)
            {
                if (parameters.release > 0.0f)
                {
                    releaseRate = (float) (envelopeVal / (parameters.release * sampleRate));
                    state = AdsrState::release;
                }
                else
                {
                    reset();
                }
            }
        */
    }

    /**
      | Returns the next sample value for an
      | ADSR object.
      | 
      | @see applyEnvelopeToBuffer
      |
      */
    pub fn get_next_sample(&mut self) -> f32 {
        
        todo!();
        /*
            if (state == AdsrState::idle)
                return 0.0f;

            if (state == AdsrState::attack)
            {
                envelopeVal += attackRate;

                if (envelopeVal >= 1.0f)
                {
                    envelopeVal = 1.0f;
                    goToNextState();
                }
            }
            else if (state == AdsrState::decay)
            {
                envelopeVal -= decayRate;

                if (envelopeVal <= parameters.sustain)
                {
                    envelopeVal = parameters.sustain;
                    goToNextState();
                }
            }
            else if (state == AdsrState::sustain)
            {
                envelopeVal = parameters.sustain;
            }
            else if (state == AdsrState::release)
            {
                envelopeVal -= releaseRate;

                if (envelopeVal <= 0.0f)
                    goToNextState();
            }

            return envelopeVal;
        */
    }

    /**
      | This method will conveniently apply
      | the next numSamples number of envelope
      | values to an AudioBuffer.
      | 
      | @see getNextSample
      |
      */
    pub fn apply_envelope_to_buffer<FloatType>(&mut self, 
        buffer:       &mut AudioBuffer<FloatType>,
        start_sample: i32,
        num_samples:  i32)  {
    
        todo!();
        /*
            jassert (startSample + numSamples <= buffer.getNumSamples());

            if (state == AdsrState::idle)
            {
                buffer.clear (startSample, numSamples);
                return;
            }

            if (state == AdsrState::sustain)
            {
                buffer.applyGain (startSample, numSamples, parameters.sustain);
                return;
            }

            auto numChannels = buffer.getNumChannels();

            while (--numSamples >= 0)
            {
                auto env = getNextSample();

                for (int i = 0; i < numChannels; ++i)
                    buffer.getWritePointer (i)[startSample] *= env;

                ++startSample;
            }
        */
    }
    
    pub fn recalculate_rates(&mut self)  {
        
        todo!();
        /*
            auto getRate = [] (float distance, float timeInSeconds, double sr)
            {
                return timeInSeconds > 0.0f ? (float) (distance / (timeInSeconds * sr)) : -1.0f;
            };

            attackRate  = getRate (1.0f, parameters.attack, sampleRate);
            decayRate   = getRate (1.0f - parameters.sustain, parameters.decay, sampleRate);
            releaseRate = getRate (parameters.sustain, parameters.release, sampleRate);

            if ((state == AdsrState::attack && attackRate <= 0.0f)
                || (state == AdsrState::decay && (decayRate <= 0.0f || envelopeVal <= parameters.sustain))
                || (state == AdsrState::release && releaseRate <= 0.0f))
            {
                goToNextState();
            }
        */
    }
    
    pub fn go_to_next_state(&mut self)  {
        
        todo!();
        /*
            if (state == AdsrState::attack)
                state = (decayRate > 0.0f ? AdsrState::decay : AdsrState::sustain);
            else if (state == AdsrState::decay)
                state = AdsrState::sustain;
            else if (state == AdsrState::release)
                reset();
        */
    }
}
