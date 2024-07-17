crate::ix!();

pub struct Editor<'a> {
    base:          AudioProcessorEditor<'a>,
    slider:        Slider<'a>,
    bypass_button: TextButton<'a>, // default = { "global bypass"  }
    attachment:    SliderParameterAttachment<'a>,
}

impl<'a> Editor<'a> {

    pub fn new(
        proc:          &mut AudioProcessor,
        param:         &mut AudioParameterFloat,
        global_bypass: fn(_0: i32) -> c_void

    ) -> Self {
    
        todo!();
        /*
        : audio_processor_editor(proc),
        : attachment(param, slider),

            addAndMakeVisible (slider);
            addAndMakeVisible (bypassButton);

            // Clicking will bypass *everything*
            bypassButton.onClick = [globalBypass] { if (globalBypass != nullptr) globalBypass (-1); };

            setSize (300, 80);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto b = getLocalBounds();
            slider.setBounds (b.removeFromTop (40));
            bypassButton.setBounds (b);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::darkgrey);
        */
    }
}
