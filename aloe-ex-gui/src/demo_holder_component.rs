crate::ix!();

pub struct DemoHolderComponent<'a> {
    base:         Component<'a>,
    base2:        Timer,
    current_demo: *mut GraphicsDemoBase<'a>, // default = nullptr
}

impl<'a> Default for DemoHolderComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true)
        */
    }
}

impl<'a> DemoHolderComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillCheckerBoard (getLocalBounds().toFloat(), 48.0f, 48.0f,
                                Colours::lightgrey, Colours::white);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (currentDemo != nullptr)
                currentDemo->repaint();
        */
    }
    
    pub fn set_demo(&mut self, new_demo: *mut GraphicsDemoBase)  {
        
        todo!();
        /*
            if (currentDemo != nullptr)
                removeChildComponent (currentDemo);

            currentDemo = newDemo;

            if (currentDemo != nullptr)
            {
                addAndMakeVisible (currentDemo);
                startTimerHz (60);
                resized();
            }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (currentDemo != nullptr)
                currentDemo->setBounds (getLocalBounds());
        */
    }
}
