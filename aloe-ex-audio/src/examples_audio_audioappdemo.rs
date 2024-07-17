crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/AudioAppDemo.h]

#[no_copy]
#[leak_detector]
pub struct AudioAppDemo<'a> {
    base:                       AudioAppComponent<'a>,
    phase:                      f32, // default = 0.0f
    phase_delta:                f32, // default = 0.0f
    frequency:                  f32, // default = 5000.0f
    amplitude:                  f32, // default = 0.2f
    sample_rate:                f64, // default = 0.0
    expected_samples_per_block: i32, // default = 0
    last_mouse_position:        Point<f32>,
}

impl<'a> Default for AudioAppDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            #ifdef ALOE_DEMO_RUNNER
            : AudioAppComponent (getSharedAudioDeviceManager (0, 2))
           #endif

            setAudioChannels (0, 2);

            setSize (800, 600)
        */
    }
}

impl<'a> Drop for AudioAppDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            shutdownAudio();
         */
    }
}

impl<'a> AudioAppDemo<'a> {

    pub fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        new_sample_rate:            f64)  {
        
        todo!();
        /*
            sampleRate = newSampleRate;
            expectedSamplesPerBlock = samplesPerBlockExpected;
        */
    }

    /**
      | This method generates the actual audio
      | samples.
      | 
      | In this example the buffer is filled
      | with a sine wave whose frequency and
      | amplitude are controlled by the mouse
      | position.
      |
      */
    pub fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            bufferToFill.clearActiveBufferRegion();
            auto originalPhase = phase;

            for (auto chan = 0; chan < bufferToFill.buffer->getNumChannels(); ++chan)
            {
                phase = originalPhase;

                auto* channelData = bufferToFill.buffer->getWritePointer (chan, bufferToFill.startSample);

                for (auto i = 0; i < bufferToFill.numSamples ; ++i)
                {
                    channelData[i] = amplitude * std::sin (phase);

                    // increment the phase step for the next sample
                    phase = std::fmod (phase + phaseDelta, MathConstants<float>::twoPi);
                }
            }
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            // This gets automatically called when audio device parameters change
            // or device is restarted.
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // (Our component is opaque, so we must completely fill the background with a solid colour)
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));

            auto centreY = (float) getHeight() / 2.0f;
            auto radius = amplitude * 200.0f;

            if (radius >= 0.0f)
            {
                // Draw an ellipse based on the mouse position and audio volume
                g.setColour (Colours::lightgreen);

                g.fillEllipse (jmax (0.0f, lastMousePosition.x) - radius / 2.0f,
                               jmax (0.0f, lastMousePosition.y) - radius / 2.0f,
                               radius, radius);
            }

            // Draw a representative sine wave.
            Path wavePath;
            wavePath.startNewSubPath (0, centreY);

            for (auto x = 1.0f; x < (float) getWidth(); ++x)
                wavePath.lineTo (x, centreY + amplitude * (float) getHeight() * 2.0f
                                                * std::sin (x * frequency * 0.0001f));

            g.setColour (getLookAndFeel().findColour (Slider::thumbColourId));
            g.strokePath (wavePath, PathStrokeType (2.0f));
        */
    }

    /**
      | Mouse handling..
      |
      */
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDrag (e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            lastMousePosition = e.position;

            frequency = (float) (getHeight() - e.y) * 10.0f;
            amplitude = jmin (0.9f, 0.2f * e.position.x / (float) getWidth());

            phaseDelta = (float) (MathConstants<double>::twoPi * frequency / sampleRate);

            repaint();
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            amplitude = 0.0f;
            repaint();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // This is called when the component is resized.
            // If you add any child components, this is where you should
            // update their positions.
        */
    }
}
