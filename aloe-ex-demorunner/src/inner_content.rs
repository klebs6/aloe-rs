crate::ix!();

pub struct InnerContent<'a> {
    base:              Component<'a>,
    graphics_settings: GraphicsSettingsGroup<'a>,
    audio_settings:    AudioSettingsGroup<'a>,
}

impl<'a> InnerContent<'a> {

    pub fn new(main_component: &mut MainComponent) -> Self {
    
        todo!();
        /*
        : graphics_settings(mainComponent),

            addAndMakeVisible (graphicsSettings);
                addAndMakeVisible (audioSettings);

                setOpaque (true);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (ResizableWindow::backgroundColourId).contrasting (0.2f));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

                graphicsSettings.setBounds (bounds.removeFromTop (150));
                audioSettings.setBounds (bounds);
        */
    }
}
