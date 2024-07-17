crate::ix!();

pub struct DemoParametersComponent<'a> {
    base:       Component<'a>,
    parameters: Vec<*mut DSPDemoParameterBase<'a>>,
    labels:     Vec<Box<Label<'a>>>,
}

impl<'a> DemoParametersComponent<'a> {
    
    pub fn new(demo_params: &Vec<*mut DSPDemoParameterBase>) -> Self {
    
        todo!();
        /*
            parameters = demoParams;

            for (auto demoParameter : parameters)
            {
                addAndMakeVisible (demoParameter->getComponent());

                auto* paramLabel = new Label ({}, demoParameter->name);

                paramLabel->attachToComponent (demoParameter->getComponent(), true);
                paramLabel->setJustificationType (Justification::centredLeft);
                addAndMakeVisible (paramLabel);
                labels.add (paramLabel);
            }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();
            bounds.removeFromLeft (100);

            for (auto* p : parameters)
            {
                auto* comp = p->getComponent();

                comp->setSize (jmin (bounds.getWidth(), p->getPreferredWidth()), p->getPreferredHeight());

                auto compBounds = bounds.removeFromTop (p->getPreferredHeight());
                comp->setCentrePosition (compBounds.getCentre());
            }
        */
    }
    
    pub fn get_height_needed(&mut self) -> i32 {
        
        todo!();
        /*
            auto height = 0;

            for (auto* p : parameters)
                height += p->getPreferredHeight();

            return height + 10;
        */
    }
}
