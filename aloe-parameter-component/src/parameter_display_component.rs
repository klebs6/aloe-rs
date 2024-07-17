crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ParameterDisplayComponent<'a> {
    base:            Component<'a>,
    parameter:       &'a mut AudioProcessorParameter,
    parameter_name:  Label<'a>,
    parameter_label: Label<'a>,
    parameter_comp:  Box<Component<'a>>,
}

impl<'a> ParameterDisplayComponent<'a> {

    pub fn new(
        processor: &mut AudioProcessor,
        param:     &mut AudioProcessorParameter) -> Self {
    
        todo!();
        /*
        : parameter(param),

            parameterName.setText (parameter.getName (128), dontSendNotification);
            parameterName.setJustificationType (Justification::centredRight);
            addAndMakeVisible (parameterName);

            parameterLabel.setText (parameter.getLabel(), dontSendNotification);
            addAndMakeVisible (parameterLabel);

            addAndMakeVisible (*(parameterComp = createParameterComp (processor)));

            setSize (400, 40);
        */
    }
    
    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            parameterName.setBounds (area.removeFromLeft (100));
            parameterLabel.setBounds (area.removeFromRight (50));
            parameterComp->setBounds (area);
        */
    }
    
    pub fn create_parameter_comp(&self, processor: &mut AudioProcessor) -> Box<Component> {
        
        todo!();
        /*
            // The AU, AUv3 and VST (only via a .vstxml file) SDKs support
            // marking a parameter as boolean. If you want consistency across
            // all  formats then it might be best to use a
            // SwitchParameterComponent instead.
            if (parameter.isBoolean())
                return std::make_unique<BooleanParameterComponent> (processor, parameter);

            // Most hosts display any parameter with just two steps as a switch.
            if (parameter.getNumSteps() == 2)
                return std::make_unique<SwitchParameterComponent> (processor, parameter);

            // If we have a list of strings to represent the different states a
            // parameter can be in then we should present a dropdown allowing a
            // user to pick one of them.
            if (! parameter.getAllValueStrings().isEmpty()
                 && std::abs (parameter.getNumSteps() - parameter.getAllValueStrings().size()) <= 1)
                return std::make_unique<ChoiceParameterComponent> (processor, parameter);

            // Everything else can be represented as a slider.
            return std::make_unique<SliderParameterComponent> (processor, parameter);
        */
    }
}
