crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/PluckedStringsDemo.h]

/**
  | A very basic generator of a simulated plucked
  | string sound, implementing the Karplus-Strong
  | algorithm.
  |
  | Not performance-optimised!
  */
#[no_copy]
#[leak_detector]
pub struct StringSynthesizer {
    decay:                    f64, // default = 0.998
    amplitude:                f64, // default = 0.0
    do_pluck_for_next_buffer: Atomic<i32>,
    excitation_sample:        Vec<f32>,
    delay_line:               Vec<f32>,
    pos:                      usize, // default = 0
}

impl StringSynthesizer {

    /**
      | Constructor.
      | 
      | -----------
      | @param sampleRate
      | 
      | The audio sample rate to use.
      | ----------
      | @param frequencyInHz
      | 
      | The fundamental frequency of the simulated
      | string inHertz.
      |
      */
    pub fn new(
        sample_rate:     f64,
        frequency_in_hz: f64) -> Self {
    
        todo!();
        /*


            doPluckForNextBuffer.set (false);
            prepareSynthesizerState (sampleRate, frequencyInHz);
        */
    }

    /**
      | Excite the simulated string by plucking
      | it at a given position.
      | 
      | -----------
      | @param pluckPosition
      | 
      | The position of the plucking, relative
      | to the lengthof the string. Must be between
      | 0 and 1.
      |
      */
    pub fn string_plucked(&mut self, pluck_position: f32)  {
        
        todo!();
        /*
            jassert (pluckPosition >= 0.0 && pluckPosition <= 1.0);

            // we choose a very simple approach to communicate with the audio thread:
            // simply tell the synth to perform the plucking excitation at the beginning
            // of the next buffer (= when generateAndAddData is called the next time).

            if (doPluckForNextBuffer.compareAndSetBool (1, 0))
            {
                // plucking in the middle gives the largest amplitude;
                // plucking at the very ends will do nothing.
                amplitude = std::sin (MathConstants<float>::pi * pluckPosition);
            }
        */
    }
    
    /**
      | Generate next chunk of mono audio output
      | and add it into a buffer.
      | 
      | -----------
      | @param outBuffer
      | 
      | Buffer to fill (one channel only). New
      | sound will beadded to existing content
      | of the buffer (instead ofreplacing
      | it).
      | ----------
      | @param numSamples
      | 
      | Number of samples to generate (make
      | sure that outBufferhas enough space).
      |
      */
    pub fn generate_and_add_data(&mut self, 
        out_buffer:  *mut f32,
        num_samples: i32)  {
        
        todo!();
        /*
            if (doPluckForNextBuffer.compareAndSetBool (0, 1))
                exciteInternalBuffer();

            // cycle through the delay line and apply a simple averaging filter
            for (auto i = 0; i < numSamples; ++i)
            {
                auto nextPos = (pos + 1) % delayLine.size();

                delayLine[nextPos] = (float) (decay * 0.5 * (delayLine[nextPos] + delayLine[pos]));
                outBuffer[i] += delayLine[pos];

                pos = nextPos;
            }
        */
    }
    
    pub fn prepare_synthesiser_state(&mut self, 
        sample_rate:     f64,
        frequency_in_hz: f64)  {
        
        todo!();
        /*
            auto delayLineLength = (size_t) roundToInt (sampleRate / frequencyInHz);

            // we need a minimum delay line length to get a reasonable synthesis.
            // if you hit this assert, increase sample rate or decrease frequency!
            jassert (delayLineLength > 50);

            delayLine.resize (delayLineLength);
            std::fill (delayLine.begin(), delayLine.end(), 0.0f);

            excitationSample.resize (delayLineLength);

            // as the excitation sample we use random noise between -1 and 1
            // (as a simple approximation to a plucking excitation)

            std::generate (excitationSample.begin(),
                           excitationSample.end(),
                           [] { return (Random::getSystemRandom().nextFloat() * 2.0f) - 1.0f; } );
        */
    }
    
    pub fn excite_internal_buffer(&mut self)  {
        
        todo!();
        /*
            // fill the buffer with the precomputed excitation sound (scaled with amplitude)

            jassert (delayLine.size() >= excitationSample.size());

            std::transform (excitationSample.begin(),
                            excitationSample.end(),
                            delayLine.begin(),
                            [this] (double sample) { return static_cast<float> (amplitude * sample); } );
        */
    }
}

/**
  | This component represents a horizontal
  | vibrating musical string of fixed height
  | and variable length. The string can
  | be excited by calling stringPlucked().
  |
  */
#[no_copy]
#[leak_detector]
pub struct StringComponent<'a> {
    base:          Component<'a>,
    base2:         Timer,
    length:        i32,
    colour:        Colour,
    height:        i32, // default = 20
    amplitude:     f32, // default = 0.0f
    max_amplitude: f32, // default = 12.0f
    phase:         f32, // default = 0.0f
}

impl<'a> StringComponent<'a> {

    pub fn new(
        length_in_pixels: i32,
        string_colour:    Colour) -> Self {
    
        todo!();
        /*
        : length(lengthInPixels),
        : colour(stringColour),

            // ignore mouse-clicks so that our parent can get them instead.
            setInterceptsMouseClicks (false, false);
            setSize (length, height);
            startTimerHz (60);
        */
    }
    
    pub fn string_plucked(&mut self, pluck_position_relative: f32)  {
        
        todo!();
        /*
            amplitude = maxAmplitude * std::sin (pluckPositionRelative * MathConstants<float>::pi);
            phase = MathConstants<float>::pi;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (colour);
            g.strokePath (generateStringPath(), PathStrokeType (2.0f));
        */
    }
    
    pub fn generate_string_path(&self) -> PathBuf {
        
        todo!();
        /*
            auto y = (float) height / 2.0f;

            Path stringPath;
            stringPath.startNewSubPath (0, y);
            stringPath.quadraticTo ((float) length / 2.0f, y + (std::sin (phase) * amplitude), (float) length, y);
            return stringPath;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            updateAmplitude();
            updatePhase();
            repaint();
        */
    }
    
    pub fn update_amplitude(&mut self)  {
        
        todo!();
        /*
            // this determines the decay of the visible string vibration.
            amplitude *= 0.99f;
        */
    }
    
    pub fn update_phase(&mut self)  {
        
        todo!();
        /*
            // this determines the visible vibration frequency.
            // just an arbitrary number chosen to look OK:
            auto phaseStep = 400.0f / (float) length;

            phase += phaseStep;

            if (phase >= MathConstants<float>::twoPi)
                phase -= MathConstants<float>::twoPi;
        */
    }
}

///-------------------
pub struct PluckedStringsDemoStringParameters {
    frequency_in_hz:  f64,
    length_in_pixels: i32,
}

impl PluckedStringsDemoStringParameters {
    
    pub fn new(midi_note: i32) -> Self {
    
        todo!();
        /*
            : frequencyInHz (MidiMessage::getMidiNoteInHertz (midiNote)),
                  lengthInPixels ((int) (760 / (frequencyInHz / MidiMessage::getMidiNoteInHertz (42))))
        */
    }
}

#[no_copy]
#[leak_detector]
pub struct PluckedStringsDemo<'a> {
    base:          AudioAppComponent<'a>,
    string_lines:  Vec<Box<StringComponent<'a>>>,
    string_synths: Vec<Box<StringSynthesizer>>,
}

impl<'a> Default for PluckedStringsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            #ifdef ALOE_DEMO_RUNNER
            : AudioAppComponent (getSharedAudioDeviceManager (0, 2))
           #endif

            createStringComponents();
            setSize (800, 560);

            // specify the number of input and output channels that we want to open
            auto audioDevice = deviceManager.getCurrentAudioDevice();
            auto numInputChannels  = (audioDevice != nullptr ? audioDevice->getActiveInputChannels() .countNumberOfSetBits() : 0);
            auto numOutputChannels = jmax (audioDevice != nullptr ? audioDevice->getActiveOutputChannels().countNumberOfSetBits() : 2, 2);

            setAudioChannels (numInputChannels, numOutputChannels)
        */
    }
}

impl<'a> Drop for PluckedStringsDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            shutdownAudio();
         */
    }
}

impl<'a> PluckedStringsDemo<'a> {

    pub fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64)  {
        
        todo!();
        /*
            generateStringSynths (sampleRate);
        */
    }
    
    pub fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            bufferToFill.clearActiveBufferRegion();

            for (auto channel = 0; channel < bufferToFill.buffer->getNumChannels(); ++channel)
            {
                auto* channelData = bufferToFill.buffer->getWritePointer (channel, bufferToFill.startSample);

                if (channel == 0)
                {
                    for (auto synth : stringSynths)
                        synth->generateAndAddData (channelData, bufferToFill.numSamples);
                }
                else
                {
                    memcpy (channelData,
                            bufferToFill.buffer->getReadPointer (0),
                            ((size_t) bufferToFill.numSamples) * sizeof (float));
                }
            }
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            stringSynths.clear();
        */
    }
    
    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto xPos = 20;
            auto yPos = 20;
            auto yDistance = 50;

            for (auto stringLine : stringLines)
            {
                stringLine->setTopLeftPosition (xPos, yPos);
                yPos += yDistance;
                addAndMakeVisible (stringLine);
            }
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDrag (e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            for (auto i = 0; i < stringLines.size(); ++i)
            {
                auto* stringLine = stringLines.getUnchecked (i);

                if (stringLine->getBounds().contains (e.getPosition()))
                {
                    auto position = (e.position.x - (float) stringLine->getX()) / (float) stringLine->getWidth();

                    stringLine->stringPlucked (position);
                    stringSynths.getUnchecked (i)->stringPlucked (position);
                }
            }
        */
    }
    
    pub fn get_default_string_parameters() -> Vec<PluckedStringsDemoStringParameters> {
        
        todo!();
        /*
            return Vec<PluckedStringsDemoStringParameters> (42, 44, 46, 49, 51, 54, 56, 58, 61, 63, 66, 68, 70);
        */
    }
    
    pub fn create_string_components(&mut self)  {
        
        todo!();
        /*
            for (auto stringParams : getDefaultStringParameters())
            {
                stringLines.add (new StringComponent (stringParams.lengthInPixels,
                                                      Colour::fromHSV (Random().nextFloat(), 0.6f, 0.9f, 1.0f)));
            }
        */
    }
    
    pub fn generate_string_synths(&mut self, sample_rate: f64)  {
        
        todo!();
        /*
            stringSynths.clear();

            for (auto stringParams : getDefaultStringParameters())
            {
                stringSynths.add (new StringSynthesizer (sampleRate, stringParams.frequencyInHz));
            }
        */
    }
}
