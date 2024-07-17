crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/CMake/GuiApp/MainComponent.h]

/**
  | This component lives inside our window,
  | and this is where you should put all your
  | controls and content.
  |
  */
pub struct MainComponent<'a> {
    base: Component<'a>,
}

//-------------------------------------------[.cpp/Aloe/examples/CMake/GuiApp/MainComponent.cpp]
impl<'a> Default for MainComponent<'a> {

    fn default() -> Self {
    
        todo!();
        /*
            setSize (600, 400);
        */
    }
}

impl<'a> MainComponent<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // (Our component is opaque, so we must completely fill the background with a solid colour)
        g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));

        g.setFont (Font (16.0f));
        g.setColour (Colours::white);
        g.drawText ("Hello World!", getLocalBounds(), Justification::centred, true);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // This is called when the MainComponent is resized.
        // If you add any child components, this is where you should
        // update their positions.
        */
    }
}
