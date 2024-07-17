crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/BouncingBallWavetableDemo.h]

#[no_copy]
#[leak_detector]
pub struct BouncingBallWavetableDemo<'a> {
    base:                       AudioAppComponent<'a>,
    base2:                      Timer,
    pos:                        Point<f32>, //= { 299.0, 299.0 };
    delta:                      Point<f32>, //= { 0.0, 0.0 };
    wave_table_index:           i32, // default = 0
    buffer_index:               i32, // default = 0
    sample_rate:                f64, // default = 0.0
    expected_samples_per_block: i32, // default = 0
    last_mouse_position:        Point<f32>,
    wave_values:                [[f32; 2]; WAVETABLE_SIZE],
    dragging:                   bool, // default = false
}

const WAVETABLE_SIZE: usize = 36000;
const STEPS:         usize = 10;

impl<'a> Drop for BouncingBallWavetableDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            shutdownAudio();
         */
    }
}

impl<'a> Default for BouncingBallWavetableDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            #ifdef ALOE_DEMO_RUNNER
            : AudioAppComponent (getSharedAudioDeviceManager (0, 2))
           #endif

            setSize (600, 600);

            for (auto i = 0; i < numElementsInArray (waveValues); ++i)
                zeromem (waveValues[i], sizeof (waveValues[i]));

            // specify the number of input and output channels that we want to open
            setAudioChannels (2, 2);
            startTimerHz (60)
        */
    }
}

impl<'a> BouncingBallWavetableDemo<'a> {
    
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

            for (auto chan = 0; chan < bufferToFill.buffer->getNumChannels(); ++chan)
            {
                auto ind = waveTableIndex;

                auto* channelData = bufferToFill.buffer->getWritePointer (chan, bufferToFill.startSample);

                for (auto i = 0; i < bufferToFill.numSamples; ++i)
                {
                    if (isPositiveAndBelow (chan, numElementsInArray (waveValues)))
                    {
                        channelData[i] = waveValues[chan][ind % wavetableSize];
                        ++ind;
                    }
                }
            }

            waveTableIndex = (int) (waveTableIndex + bufferToFill.numSamples) % wavetableSize;
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            // This gets automatically called when audio device parameters change
            // or device is restarted.
            stopTimer();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // (Our component is opaque, so we must completely fill the background with a solid colour)
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));

            auto nextPos = pos + delta;

            if (nextPos.x < 10 || nextPos.x + 10 > (float) getWidth())
            {
                delta.x = -delta.x;
                nextPos.x = pos.x + delta.x;
            }

            if (nextPos.y < 50 || nextPos.y + 10 > (float) getHeight())
            {
                delta.y = -delta.y;
                nextPos.y = pos.y + delta.y;
            }

            if (! dragging)
            {
                writeInterpolatedValue (pos, nextPos);
                pos = nextPos;
            }
            else
            {
                pos = lastMousePosition;
            }

            // draw a circle
            g.setColour (getLookAndFeel().findColour (Slider::thumbColourId));
            g.fillEllipse (pos.x, pos.y, 20, 20);

            drawWaveform (g, 20.0f, 0);
            drawWaveform (g, 40.0f, 1);
        */
    }
    
    pub fn draw_waveform(&self, 
        g:       &mut Graphics,
        y:       f32,
        channel: i32)  {
        
        todo!();
        /*
            auto pathWidth = 2000;

            Path wavePath;
            wavePath.startNewSubPath (0.0f, y);

            for (auto i = 1; i < pathWidth; ++i)
                wavePath.lineTo ((float) i, (1.0f + waveValues[channel][i * numElementsInArray (waveValues[0]) / pathWidth]) * 10.0f);

            g.strokePath (wavePath, PathStrokeType (1.0f),
                          wavePath.getTransformToScaleToFit (Rectangle<float> (0.0f, y, (float) getWidth(), 20.0f), false));
        */
    }

    /**
       Mouse handling..
      */
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            lastMousePosition = e.position;
            mouseDrag (e);
            dragging = true;
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            dragging = true;

            if (e.position != lastMousePosition)
            {
                // calculate movement vector
                delta = e.position - lastMousePosition;

                waveValues[0][bufferIndex % wavetableSize] = xToAmplitude (e.position.x);
                waveValues[1][bufferIndex % wavetableSize] = yToAmplitude (e.position.y);

                ++bufferIndex;
                lastMousePosition = e.position;
            }
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            dragging = false;
        */
    }
    
    pub fn write_interpolated_value(&mut self, 
        last_position:    Point<f32>,
        current_position: Point<f32>)  {
        
        todo!();
        /*
            Point<float> start, finish;

            if (lastPosition.getX() > currentPosition.getX())
            {
                finish = lastPosition;
                start  = currentPosition;
            }
            else
            {
                start  = lastPosition;
                finish = currentPosition;
            }

            for (auto i = 0; i < steps; ++i)
            {
                auto p = start + ((finish - start) * i) / (int) steps;

                auto index = (bufferIndex + i) % wavetableSize;
                waveValues[1][index] = yToAmplitude (p.y);
                waveValues[0][index] = xToAmplitude (p.x);
            }

            bufferIndex = (bufferIndex + steps) % wavetableSize;
        */
    }
    
    pub fn index_tox(&self, index_value: i32) -> f32 {
        
        todo!();
        /*
            return (float) indexValue;
        */
    }
    
    pub fn amplitude_toy(&self, amp: f32) -> f32 {
        
        todo!();
        /*
            return (float) getHeight() - (amp + 1.0f) * (float) getHeight() / 2.0f;
        */
    }
    
    pub fn x_to_amplitude(&self, x: f32) -> f32 {
        
        todo!();
        /*
            return jlimit (-1.0f, 1.0f, 2.0f * ((float) getWidth() - x) / (float) getWidth() - 1.0f);
        */
    }
    
    pub fn y_to_amplitude(&self, y: f32) -> f32 {
        
        todo!();
        /*
            return jlimit (-1.0f, 1.0f, 2.0f * ((float) getHeight() - y) / (float) getHeight() - 1.0f);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
}
