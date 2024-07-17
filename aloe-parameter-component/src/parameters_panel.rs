crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ParametersPanel<'a> {
    base:             Component<'a>,
    param_components: Vec<Box<ParameterDisplayComponent<'a>>>,
}

impl<'a> Drop for ParametersPanel<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            paramComponents.clear();
        */
    }
}

impl<'a> ParametersPanel<'a> {

    pub fn new(
        processor:  &mut AudioProcessor,
        parameters: &[*mut AudioProcessorParameter]) -> Self {
    
        todo!();
        /*


            for (auto* param : parameters)
                if (param->isAutomatable())
                    addAndMakeVisible (paramComponents.add (new ParameterDisplayComponent (processor, *param)));

            int maxWidth = 400;
            int height = 0;

            for (auto& comp : paramComponents)
            {
                maxWidth = jmax (maxWidth, comp->getWidth());
                height += comp->getHeight();
            }

            setSize (maxWidth, jmax (height, 125));
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            for (auto* comp : paramComponents)
                comp->setBounds (area.removeFromTop (comp->getHeight()));
        */
    }
}
