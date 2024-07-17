crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_Reverb.h]

pub const REVERB_NUM_COMBS:      usize = 8;
pub const REVERB_NUM_ALL_PASSES: usize = 4;
pub const REVERB_NUM_CHANNELS:   usize = 2;

/**
  | Performs a simple reverb effect on a
  | stream of audio data.
  | 
  | This is a simple stereo reverb, based
  | on the technique and tunings used in
  | FreeVerb. Use setSampleRate() to prepare
  | it, and then call processStereo() or
  | processMono() to apply the reverb to
  | your audio data.
  | 
  | @see ReverbAudioSource
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Reverb {
    parameters: ReverbParameters,
    gain:       f32,
    comb:       [[CombFilter;    REVERB_NUM_CHANNELS]; REVERB_NUM_COMBS],
    all_pass:   [[AllPassFilter; REVERB_NUM_CHANNELS]; REVERB_NUM_ALL_PASSES],
    damping:    SmoothedValue<f32>,
    feedback:   SmoothedValue<f32>,
    dry_gain:   SmoothedValue<f32>,
    wet_gain1:  SmoothedValue<f32>,
    wet_gain2:  SmoothedValue<f32>,
}

impl Default for Reverb {
    
    fn default() -> Self {
        todo!();
        /*


            setParameters (ReverbParameters());
            setSampleRate (44100.0)
        */
    }
}

impl Reverb {

    /**
      | Returns the reverb's current parameters.
      |
      */
    pub fn get_parameters(&self) -> &ReverbParameters {
        
        todo!();
        /*
            return parameters;
        */
    }

    /**
      | Applies a new set of parameters to the
      | reverb. Note that this doesn't attempt
      | to lock the reverb, so if you call this
      | in parallel with the process method,
      | you may get artifacts.
      |
      */
    pub fn set_parameters(&mut self, new_params: &ReverbParameters)  {
        
        todo!();
        /*
            const float wetScaleFactor = 3.0f;
            const float dryScaleFactor = 2.0f;

            const float wet = newParams.wetLevel * wetScaleFactor;
            dryGain.setTargetValue (newParams.dryLevel * dryScaleFactor);
            wetGain1.setTargetValue (0.5f * wet * (1.0f + newParams.width));
            wetGain2.setTargetValue (0.5f * wet * (1.0f - newParams.width));

            gain = isFrozen (newParams.freezeMode) ? 0.0f : 0.015f;
            parameters = newParams;
            updateDamping();
        */
    }

    /**
      | Sets the sample rate that will be used
      | for the reverb. You must call this before
      | the process methods, in order to tell
      | it the correct sample rate.
      |
      */
    pub fn set_sample_rate(&mut self, sample_rate: f64)  {
        
        todo!();
        /*
            jassert (sampleRate > 0);

            static const short combTunings[] = { 1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617 }; // (at 44100Hz)
            static const short allPassTunings[] = { 556, 441, 341, 225 };
            const int stereoSpread = 23;
            const int intSampleRate = (int) sampleRate;

            for (int i = 0; i < numCombs; ++i)
            {
                comb[0][i].setSize ((intSampleRate * combTunings[i]) / 44100);
                comb[1][i].setSize ((intSampleRate * (combTunings[i] + stereoSpread)) / 44100);
            }

            for (int i = 0; i < numAllPasses; ++i)
            {
                allPass[0][i].setSize ((intSampleRate * allPassTunings[i]) / 44100);
                allPass[1][i].setSize ((intSampleRate * (allPassTunings[i] + stereoSpread)) / 44100);
            }

            const double smoothTime = 0.01;
            damping .reset (sampleRate, smoothTime);
            feedback.reset (sampleRate, smoothTime);
            dryGain .reset (sampleRate, smoothTime);
            wetGain1.reset (sampleRate, smoothTime);
            wetGain2.reset (sampleRate, smoothTime);
        */
    }

    /**
      | Clears the reverb's buffers.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (int j = 0; j < numChannels; ++j)
            {
                for (int i = 0; i < numCombs; ++i)
                    comb[j][i].clear();

                for (int i = 0; i < numAllPasses; ++i)
                    allPass[j][i].clear();
            }
        */
    }

    /**
      | Applies the reverb to two stereo channels
      | of audio data.
      |
      */
    pub fn process_stereo(&mut self, 
        left:        *mut f32,
        right:       *mut f32,
        num_samples: i32)  {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011)
            jassert (left != nullptr && right != nullptr);

            for (int i = 0; i < numSamples; ++i)
            {
                const float input = (left[i] + right[i]) * gain;
                float outL = 0, outR = 0;

                const float damp    = damping.getNextValue();
                const float feedbck = feedback.getNextValue();

                for (int j = 0; j < numCombs; ++j)  // accumulate the comb filters in parallel
                {
                    outL += comb[0][j].process (input, damp, feedbck);
                    outR += comb[1][j].process (input, damp, feedbck);
                }

                for (int j = 0; j < numAllPasses; ++j)  // run the allpass filters in series
                {
                    outL = allPass[0][j].process (outL);
                    outR = allPass[1][j].process (outR);
                }

                const float dry  = dryGain.getNextValue();
                const float wet1 = wetGain1.getNextValue();
                const float wet2 = wetGain2.getNextValue();

                left[i]  = outL * wet1 + outR * wet2 + left[i]  * dry;
                right[i] = outR * wet1 + outL * wet2 + right[i] * dry;
            }
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    /**
      | Applies the reverb to a single mono channel
      | of audio data.
      |
      */
    pub fn process_mono(&mut self, 
        samples:     *mut f32,
        num_samples: i32)  {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011)
            jassert (samples != nullptr);

            for (int i = 0; i < numSamples; ++i)
            {
                const float input = samples[i] * gain;
                float output = 0;

                const float damp    = damping.getNextValue();
                const float feedbck = feedback.getNextValue();

                for (int j = 0; j < numCombs; ++j)  // accumulate the comb filters in parallel
                    output += comb[0][j].process (input, damp, feedbck);

                for (int j = 0; j < numAllPasses; ++j)  // run the allpass filters in series
                    output = allPass[0][j].process (output);

                const float dry  = dryGain.getNextValue();
                const float wet1 = wetGain1.getNextValue();

                samples[i] = output * wet1 + samples[i] * dry;
            }
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }
    
    pub fn is_frozen(freeze_mode: f32) -> bool {
        
        todo!();
        /*
            return freezeMode >= 0.5f;
        */
    }
    
    pub fn update_damping(&mut self)  {
        
        todo!();
        /*
            const float roomScaleFactor = 0.28f;
            const float roomOffset = 0.7f;
            const float dampScaleFactor = 0.4f;

            if (isFrozen (parameters.freezeMode))
                setDamping (0.0f, 1.0f);
            else
                setDamping (parameters.damping * dampScaleFactor,
                            parameters.roomSize * roomScaleFactor + roomOffset);
        */
    }
    
    pub fn set_damping(&mut self, 
        damping_to_use:   f32,
        room_size_to_use: f32)  {
        
        todo!();
        /*
            damping.setTargetValue (dampingToUse);
            feedback.setTargetValue (roomSizeToUse);
        */
    }
}
