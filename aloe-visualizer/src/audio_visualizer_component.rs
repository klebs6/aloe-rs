crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioVisualiserComponent.h]

/**
  | A simple component that can be used to
  | show a scrolling waveform of audio data.
  | 
  | This is a handy way to get a quick visualisation
  | of some audio data. Just create one of
  | these, set its size and oversampling
  | rate, and then feed it with incoming
  | data by calling one of its pushBuffer()
  | or pushSample() methods.
  | 
  | You can override its paint method for
  | more customised views, but it's only
  | designed as a quick-and-dirty class
  | for simple tasks, so please don't send
  | us feature requests for fancy additional
  | features that you'd like it to support!
  | If you're building a real-world app
  | that requires more powerful waveform
  | display, you'll probably want to create
  | your own component instead.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioVisualiserComponent<'a> {
    base:                    Component<'a>,
    base2:                   Timer,
    channels:                Vec<Box<AudioVisualiserComponentChannelInfo<'a>>>,
    num_samples:             i32,
    input_samples_per_block: i32,
    background_colour:       Colour,
    waveform_colour:         Colour,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioVisualiserComponent.cpp]
impl<'a> AudioVisualiserComponent<'a> {

    pub fn get_samples_per_block(&self) -> i32 {
        
        todo!();
        /*
            return inputSamplesPerBlock;
        */
    }

    /**
      | Creates a visualiser with the given
      | number of channels.
      |
      */
    pub fn new(initial_num_channels: i32) -> Self {
    
        todo!();
        /*


            : numSamples (1024),
        inputSamplesPerBlock (256),
        backgroundColour (Colours::black),
        waveformColour (Colours::white)
        setOpaque (true);
        setNumChannels (initialNumChannels);
        setRepaintRate (60);
        */
    }
    
    /**
      | Changes the number of channels that
      | the visualiser stores.
      |
      */
    pub fn set_num_channels(&mut self, num_channels: i32)  {
        
        todo!();
        /*
            channels.clear();

        for (int i = 0; i < numChannels; ++i)
            channels.add (new AudioVisualiserComponentChannelInfo (*this, numSamples));
        */
    }
    
    /**
      | Changes the number of samples that the
      | visualiser keeps in its history. Note
      | that this value refers to the number
      | of averaged sample blocks, and each
      | block is calculated as the peak of a number
      | of incoming audio samples. To set the
      | number of incoming samples per block,
      | use setSamplesPerBlock().
      |
      */
    pub fn set_buffer_size(&mut self, new_num_samples: i32)  {
        
        todo!();
        /*
            numSamples = newNumSamples;

        for (auto* c : channels)
            c->setBufferSize (newNumSamples);
        */
    }
    
    /**
      | Clears the contents of the buffers.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            for (auto* c : channels)
            c->clear();
        */
    }
    
    /**
      | Pushes a buffer of channels data.
      | 
      | The number of channels provided here
      | is expected to match the number of channels
      | that this AudioVisualiserComponent
      | has been told to use.
      |
      */
    pub fn push_buffer_raw(&mut self, 
        d:            *const *const f32,
        num_channels: i32,
        num:          i32)  {
        
        todo!();
        /*
            numChannels = jmin (numChannels, channels.size());

        for (int i = 0; i < numChannels; ++i)
            channels.getUnchecked(i)->pushSamples (d[i], num);
        */
    }
    
    /**
      | Pushes a buffer of channels data.
      | 
      | The number of channels provided here
      | is expected to match the number of channels
      | that this AudioVisualiserComponent
      | has been told to use.
      |
      */
    pub fn push_buffer(&mut self, buffer: &AudioBuffer<f32>)  {
        
        todo!();
        /*
            pushBuffer (buffer.getArrayOfReadPointers(),
                    buffer.getNumChannels(),
                    buffer.getNumSamples());
        */
    }
    
    /**
      | Pushes a buffer of channels data.
      | 
      | The number of channels provided here
      | is expected to match the number of channels
      | that this AudioVisualiserComponent
      | has been told to use.
      |
      */
    pub fn push_buffer_from_audio_source(&mut self, buffer: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            auto numChannels = jmin (buffer.buffer->getNumChannels(), channels.size());

        for (int i = 0; i < numChannels; ++i)
            channels.getUnchecked(i)->pushSamples (buffer.buffer->getReadPointer (i, buffer.startSample),
                                                   buffer.numSamples);
        */
    }
    
    /**
      | Pushes a single sample (per channel).
      | 
      | The number of channels provided here
      | is expected to match the number of channels
      | that this AudioVisualiserComponent
      | has been told to use.
      |
      */
    pub fn push_sample(&mut self, 
        d:            *const f32,
        num_channels: i32)  {
        
        todo!();
        /*
            numChannels = jmin (numChannels, channels.size());

        for (int i = 0; i < numChannels; ++i)
            channels.getUnchecked(i)->pushSample (d[i]);
        */
    }
    
    pub fn set_samples_per_block(&mut self, new_samples_per_pixel: i32)  {
        
        todo!();
        /*
            inputSamplesPerBlock = newSamplesPerPixel;
        */
    }
    
    /**
      | Sets the frequency at which the component
      | repaints itself.
      |
      */
    pub fn set_repaint_rate(&mut self, frequency_in_hz: i32)  {
        
        todo!();
        /*
            startTimerHz (frequencyInHz);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    /**
      | Sets the colours used to paint the
      |
      */
    pub fn set_colours(&mut self, 
        bk: Colour,
        fg: Colour)  {
        
        todo!();
        /*
            backgroundColour = bk;
        waveformColour = fg;
        repaint();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (backgroundColour);

        auto r = getLocalBounds().toFloat();
        auto channelHeight = r.getHeight() / (float) channels.size();

        g.setColour (waveformColour);

        for (auto* c : channels)
            paintChannel (g, r.removeFromTop (channelHeight),
                          c->levels.begin(), c->levels.size(), c->nextSample);
        */
    }
    
    /**
      | Creates a path which contains the waveform
      | shape of a given set of range data.
      | 
      | The path is normalised so that -1 and
      | +1 are its upper and lower bounds, and
      | it goes from 0 to numLevels on the X axis.
      |
      */
    pub fn get_channel_as_path(&mut self, 
        path:        &mut Path,
        levels:      *const Range<f32>,
        num_levels:  i32,
        next_sample: i32)  {
        
        todo!();
        /*
            path.preallocateSpace (4 * numLevels + 8);

        for (int i = 0; i < numLevels; ++i)
        {
            auto level = -(levels[(nextSample + i) % numLevels].getEnd());

            if (i == 0)
                path.startNewSubPath (0.0f, level);
            else
                path.lineTo ((float) i, level);
        }

        for (int i = numLevels; --i >= 0;)
            path.lineTo ((float) i, -(levels[(nextSample + i) % numLevels].getStart()));

        path.closeSubPath();
        */
    }
    
    /**
      | Draws a channel of audio data in the given
      | bounds.
      | 
      | The default implementation just calls
      | getChannelAsPath() and fits this into
      | the given area. You may want to override
      | this to draw things differently.
      |
      */
    pub fn paint_channel(&mut self, 
        g:           &mut Graphics,
        area:        Rectangle<f32>,
        levels:      *const Range<f32>,
        num_levels:  i32,
        next_sample: i32)  {
        
        todo!();
        /*
            Path p;
        getChannelAsPath (p, levels, numLevels, nextSample);

        g.fillPath (p, AffineTransform::fromTargetPoints (0.0f, -1.0f,               area.getX(), area.getY(),
                                                          0.0f, 1.0f,                area.getX(), area.getBottom(),
                                                          (float) numLevels, -1.0f,  area.getRight(), area.getY()));
        */
    }
}
