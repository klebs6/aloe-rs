crate::ix!();

#[no_copy]
#[leak_detector]
pub struct GraphicsDemo<'a> {
    base:                  Component<'a>,
    controllers_component: ControllersComponent<'a>,
    demo_holder:           DemoHolderComponent<'a>,
    performance_display:   Label<'a>,
    test_list:             TestListComponent<'a>,
}

impl<'a> Default for GraphicsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : test_list(demoHolder, controllersComponent),

            setOpaque (true);

            addAndMakeVisible (demoHolder);
            addAndMakeVisible (controllersComponent);
            addAndMakeVisible (performanceDisplay);
            addAndMakeVisible (testList);

            setSize (750, 750)
        */
    }
}

impl<'a> GraphicsDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::grey);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            controllersComponent.setBounds (area.removeFromBottom (150));
            testList            .setBounds (area.removeFromRight (150));
            demoHolder          .setBounds (area);
            performanceDisplay  .setBounds (area.removeFromTop (20).removeFromRight (100));
        */
    }
}
