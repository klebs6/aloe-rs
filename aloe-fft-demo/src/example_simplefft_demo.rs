crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Audio/SimpleFFTDemo.h]

pub const SIMPLE_FFT_DEMO_FFT_ORDER: usize = 10;
pub const SIMPLE_FFT_DEMO_FFT_SIZE:  usize = 1 << SIMPLE_FFT_DEMO_FFT_ORDER;

#[no_copy]
#[leak_detector]
pub struct SimpleFFTDemo<'a> {
    base:                 AudioAppComponent<'a>,
    base2:                Timer,
    forwardfft:           FFT,
    spectrogram_image:    Image,
    fifo:                 [f32; SIMPLE_FFT_DEMO_FFT_SIZE],
    fft_data:             [f32; 2 * SIMPLE_FFT_DEMO_FFT_SIZE],
    fifo_index:           i32,  // default = 0
    next_fft_block_ready: bool, // default = false
}

impl<'a> Default for SimpleFFTDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            :
             #ifdef ALOE_DEMO_RUNNER
              AudioAppComponent (getSharedAudioDeviceManager (1, 0)),
             #endif
              forwardFFT (SIMPLE_FFT_DEMO_FFT_ORDER),
              spectrogramImage (Image::RGB, 512, 512, true)

            setOpaque (true);

           #ifndef ALOE_DEMO_RUNNER
            RuntimePermissions::request (RuntimePermissions::recordAudio,
                                         [this] (bool granted)
                                         {
                                             int numInputChannels = granted ? 2 : 0;
                                             setAudioChannels (numInputChannels, 2);
                                         });
           #else
            setAudioChannels (2, 2);
           #endif

            startTimerHz (60);
            setSize (700, 500)
        */
    }
}

impl<'a> Drop for SimpleFFTDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            shutdownAudio();
         */
    }
}

impl<'a> SimpleFFTDemo<'a> {

    pub fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        new_sample_rate:            f64)  {
        
        todo!();
        /*
            // (nothing to do here)
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            // (nothing to do here)
        */
    }
    
    pub fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            if (bufferToFill.buffer->getNumChannels() > 0)
            {
                const auto* channelData = bufferToFill.buffer->getReadPointer (0, bufferToFill.startSample);

                for (auto i = 0; i < bufferToFill.numSamples; ++i)
                    pushNextSampleIntoFifo (channelData[i]);

                bufferToFill.clearActiveBufferRegion();
            }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);

            g.setOpacity (1.0f);
            g.drawImage (spectrogramImage, getLocalBounds().toFloat());
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (nextFFTBlockReady)
            {
                drawNextLineOfSpectrogram();
                nextFFTBlockReady = false;
                repaint();
            }
        */
    }
    
    pub fn push_next_sample_into_fifo(&mut self, sample: f32)  {
        
        todo!();
        /*
            // if the fifo contains enough data, set a flag to say
            // that the next line should now be rendered..
            if (fifoIndex == SIMPLE_FFT_DEMO_FFT_SIZE)
            {
                if (! nextFFTBlockReady)
                {
                    zeromem (fftData, sizeof (fftData));
                    memcpy (fftData, fifo, sizeof (fifo));
                    nextFFTBlockReady = true;
                }

                fifoIndex = 0;
            }

            fifo[fifoIndex++] = sample;
        */
    }
    
    pub fn draw_next_line_of_spectrogram(&mut self)  {
        
        todo!();
        /*
            auto rightHandEdge = spectrogramImage.getWidth() - 1;
            auto imageHeight   = spectrogramImage.getHeight();

            // first, shuffle our image leftwards by 1 pixel..
            spectrogramImage.moveImageSection (0, 0, 1, 0, rightHandEdge, imageHeight);

            // then render our FFT data..
            forwardFFT.performFrequencyOnlyForwardTransform (fftData);

            // find the range of values produced, so we can scale our rendering to
            // show up the detail clearly
            auto maxLevel = FloatVectorOperations::findMinAndMax (fftData, SIMPLE_FFT_DEMO_FFT_SIZE / 2);

            for (auto y = 1; y < imageHeight; ++y)
            {
                auto skewedProportionY = 1.0f - std::exp (std::log ((float) y / (float) imageHeight) * 0.2f);
                auto fftDataIndex = jlimit (0, SIMPLE_FFT_DEMO_FFT_SIZE / 2, (int) (skewedProportionY * (int) SIMPLE_FFT_DEMO_FFT_SIZE / 2));
                auto level = jmap (fftData[fftDataIndex], 0.0f, jmax (maxLevel.getEnd(), 1e-5f), 0.0f, 1.0f);

                spectrogramImage.setPixelAt (rightHandEdge, y, Colour::fromHSV (level, 1.0f, level, 1.0f));
            }
        */
    }
}
