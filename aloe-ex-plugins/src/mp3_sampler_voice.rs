crate::ix!();

pub enum MPESamplerVoiceDirection
{
    forward,
    backward
}

pub struct MPESamplerVoice {
    base:                        MPESynthesizerVoice,
    sampler_sound:               Arc<MPESamplerSound>,
    level:                       SmoothedValue<f64>, // default = { 0  }
    frequency:                   SmoothedValue<f64>, // default = { 0  }
    loop_begin:                  SmoothedValue<f64>,
    loop_end:                    SmoothedValue<f64>,
    previous_pressure:           f64, // default = { 0  }
    current_sample_pos:          f64, // default = { 0  }
    tail_off:                    f64, // default = { 0  }
    current_direction:           MPESamplerVoiceDirection, // default = { Direction::forward  }
    smoothing_length_in_seconds: f64, // default = { 0.01  }
}

impl MPESamplerVoice {

    pub fn new(sound: Arc<MPESamplerSound>) -> Self {
    
        todo!();
        /*
        : sampler_sound(std::move (sound)),

            jassert (samplerSound != nullptr);
        */
    }
    
    pub fn note_started(&mut self)  {
        
        todo!();
        /*
            jassert (currentlyPlayingNote.isValid());
            jassert (currentlyPlayingNote.keyState == MPENote::keyDown
                  || currentlyPlayingNote.keyState == MPENote::keyDownAndSustained);

            level    .setTargetValue (currentlyPlayingNote.noteOnVelocity.asUnsignedFloat());
            frequency.setTargetValue (currentlyPlayingNote.getFrequencyInHertz());

            auto loopPoints = samplerSound->getLoopPointsInSeconds();
            loopBegin.setTargetValue (loopPoints.getStart() * samplerSound->getSample()->getSampleRate());
            loopEnd  .setTargetValue (loopPoints.getEnd()   * samplerSound->getSample()->getSampleRate());

            for (auto smoothed : { &level, &frequency, &loopBegin, &loopEnd })
                smoothed->reset (currentSampleRate, smoothingLengthInSeconds);

            previousPressure = currentlyPlayingNote.pressure.asUnsignedFloat();
            currentSamplePos = 0.0;
            tailOff          = 0.0;
        */
    }
    
    pub fn note_stopped(&mut self, allow_tail_off: bool)  {
        
        todo!();
        /*
            jassert (currentlyPlayingNote.keyState == MPENote::off);

            if (allowTailOff && tailOff == 0.0)
                tailOff = 1.0;
            else
                stopNote();
        */
    }
    
    pub fn note_pressure_changed(&mut self)  {
        
        todo!();
        /*
            const auto currentPressure = static_cast<double> (currentlyPlayingNote.pressure.asUnsignedFloat());
            const auto deltaPressure = currentPressure - previousPressure;
            level.setTargetValue (jlimit (0.0, 1.0, level.getCurrentValue() + deltaPressure));
            previousPressure = currentPressure;
        */
    }
    
    pub fn note_pitchbend_changed(&mut self)  {
        
        todo!();
        /*
            frequency.setTargetValue (currentlyPlayingNote.getFrequencyInHertz());
        */
    }
    
    pub fn note_timbre_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn note_key_state_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn render_next_block(&mut self, 
        output_buffer: &mut AudioBuffer<f32>,
        start_sample:  i32,
        num_samples:   i32)  {
        
        todo!();
        /*
            render (outputBuffer, startSample, numSamples);
        */
    }
    
    pub fn render_next_block_f64(
        &mut self, 
        output_buffer: &mut AudioBuffer<f64>,
        start_sample:  i32,
        num_samples:   i32
    ) {
        
        todo!();
        /*
            render (outputBuffer, startSample, numSamples);
        */
    }
    
    pub fn get_current_sample_position(&self) -> f64 {
        
        todo!();
        /*
            return currentSamplePos;
        */
    }
    
    pub fn render<Element>(&mut self, 
        output_buffer: &mut AudioBuffer<Element>,
        start_sample:  i32,
        num_samples:   i32)  {
    
        todo!();
        /*
            jassert (samplerSound->getSample() != nullptr);

            auto loopPoints = samplerSound->getLoopPointsInSeconds();
            loopBegin.setTargetValue (loopPoints.getStart() * samplerSound->getSample()->getSampleRate());
            loopEnd  .setTargetValue (loopPoints.getEnd()   * samplerSound->getSample()->getSampleRate());

            auto& data = samplerSound->getSample()->getBuffer();

            auto inL = data.getReadPointer (0);
            auto inR = data.getNumChannels() > 1 ? data.getReadPointer (1) : nullptr;

            auto outL = outputBuffer.getWritePointer (0, startSample);

            if (outL == nullptr)
                return;

            auto outR = outputBuffer.getNumChannels() > 1 ? outputBuffer.getWritePointer (1, startSample)
                                                          : nullptr;

            size_t writePos = 0;

            while (--numSamples >= 0 && renderNextSample (inL, inR, outL, outR, writePos))
                writePos += 1;
        */
    }
    
    pub fn render_next_sample<Element>(&mut self, 
        inl:       *const f32,
        inr:       *const f32,
        outl:      *mut Element,
        outr:      *mut Element,
        write_pos: usize) -> bool {
    
        todo!();
        /*
            auto currentLevel     = level.getNextValue();
            auto currentFrequency = frequency.getNextValue();
            auto currentLoopBegin = loopBegin.getNextValue();
            auto currentLoopEnd   = loopEnd.getNextValue();

            if (isTailingOff())
            {
                currentLevel *= tailOff;
                tailOff *= 0.9999;

                if (tailOff < 0.005)
                {
                    stopNote();
                    return false;
                }
            }

            auto pos      = (int) currentSamplePos;
            auto nextPos  = pos + 1;
            auto alpha    = (Element) (currentSamplePos - pos);
            auto invAlpha = 1.0f - alpha;

            // just using a very simple linear interpolation here..
            auto l = static_cast<Element> (currentLevel * (inL[pos] * invAlpha + inL[nextPos] * alpha));
            auto r = static_cast<Element> ((inR != nullptr) ? currentLevel * (inR[pos] * invAlpha + inR[nextPos] * alpha)
                                                            : l);

            if (outR != nullptr)
            {
                outL[writePos] += l;
                outR[writePos] += r;
            }
            else
            {
                outL[writePos] += (l + r) * 0.5f;
            }

            std::tie (currentSamplePos, currentDirection) = getNextState (currentFrequency,
                                                                          currentLoopBegin,
                                                                          currentLoopEnd);

            if (currentSamplePos > samplerSound->getSample()->getLength())
            {
                stopNote();
                return false;
            }

            return true;
        */
    }
    
    pub fn get_sample_value(&self) -> f64 {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_tailing_off(&self) -> bool {
        
        todo!();
        /*
            return tailOff != 0.0;
        */
    }
    
    pub fn stop_note(&mut self)  {
        
        todo!();
        /*
            clearCurrentNote();
            currentSamplePos = 0.0;
        */
    }
    
    pub fn get_next_state(&self, 
        freq:  f64,
        begin: f64,
        end:   f64) -> (f64,MPESamplerVoiceDirection) {
        
        todo!();
        /*
            auto nextPitchRatio = freq / samplerSound->getCentreFrequencyInHz();

            auto nextSamplePos = currentSamplePos;
            auto nextDirection = currentDirection;

            // Move the current sample pos in the correct direction
            switch (currentDirection)
            {
                case Direction::forward:
                    nextSamplePos += nextPitchRatio;
                    break;

                case Direction::backward:
                    nextSamplePos -= nextPitchRatio;
                    break;

                default:
                    break;
            }

            // Update current sample position, taking loop mode into account
            // If the loop mode was changed while we were travelling backwards, deal
            // with it gracefully.
            if (nextDirection == Direction::backward && nextSamplePos < begin)
            {
                nextSamplePos = begin;
                nextDirection = Direction::forward;

                return std::tuple<double, Direction> (nextSamplePos, nextDirection);
            }

            if (samplerSound->getLoopMode() == LoopMode::none)
                return std::tuple<double, Direction> (nextSamplePos, nextDirection);

            if (nextDirection == Direction::forward && end < nextSamplePos && !isTailingOff())
            {
                if (samplerSound->getLoopMode() == LoopMode::forward)
                    nextSamplePos = begin;
                else if (samplerSound->getLoopMode() == LoopMode::pingpong)
                {
                    nextSamplePos = end;
                    nextDirection = Direction::backward;
                }
            }
            return std::tuple<double, Direction> (nextSamplePos, nextDirection);
        */
    }
}
