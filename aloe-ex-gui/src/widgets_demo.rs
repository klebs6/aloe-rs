crate::ix!();

#[no_copy]
#[leak_detector]
pub struct WidgetsDemo<'a> {
    base: Component<'a>,
    tabs: DemoTabbedComponent<'a>,
}

impl<'a> WidgetsDemo<'a> {

    pub fn new(is_running_componen_transforms_demo: Option<bool>) -> Self {

        let is_running_componen_transforms_demo: bool =
            is_running_componen_transforms_demo.unwrap_or(false);

        todo!();
        /*
        : tabs(isRunningComponenTransformsDemo),

            setOpaque (true);
            addAndMakeVisible (tabs);

            setSize (700, 500);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::lightgrey);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            tabs.setBounds (getLocalBounds().reduced (4));
        */
    }
}
