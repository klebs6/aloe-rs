crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/CMake/AudioPlugin/PluginEditor.h]

#[no_copy]
#[leak_detector]
pub struct AudioPluginAudioProcessorEditor<'a> {

    base: AudioProcessorEditor<'a>,

    /**
      | This reference is provided as a quick
      | way for your editor to access the processor
      | object that created it.
      |
      */
    processor_ref: &'a mut AudioPluginAudioProcessor<'a>,
}

//-------------------------------------------[.cpp/Aloe/examples/CMake/AudioPlugin/PluginEditor.cpp]
impl<'a> AudioPluginAudioProcessorEditor<'a> {

    pub fn new(p: &mut AudioPluginAudioProcessor) -> Self {
    
        todo!();
        /*
        : audio_processor_editor(&p),
        : processor_ref(p),

            ignoreUnused (processorRef);
        // Make sure that before the constructor has finished, you've set the
        // editor's size to whatever you need it to be.
        setSize (400, 300);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // (Our component is opaque, so we must completely fill the background with a solid colour)
        g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));

        g.setColour (Colours::white);
        g.setFont (15.0f);
        g.drawFittedText ("Hello World!", getLocalBounds(), Justification::centred, 1);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // This is generally where you'll want to lay out the positions of any
        // subcomponents in your editor..
        */
    }
}
