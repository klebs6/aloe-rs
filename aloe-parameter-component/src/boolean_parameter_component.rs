crate::ix!();

#[no_copy]
#[leak_detector]
pub struct BooleanParameterComponent<'a> {
    base:   Component<'a>,
    base2:  ParameterListener<'a>,
    button: ToggleButton<'a>,
}

impl<'a> BooleanParameterComponent<'a> {

    pub fn new(
        proc:  &mut AudioProcessor,
        param: &mut AudioProcessorParameter) -> Self {
    
        todo!();
        /*
        : parameter_listener(proc, param),

            // Set the initial value.
            handleNewParameterValue();

            button.onClick = [this] { buttonClicked(); };

            addAndMakeVisible (button);
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
            area.removeFromLeft (8);
            button.setBounds (area.reduced (0, 10));
        */
    }
    
    pub fn handle_new_parameter_value(&mut self)  {
        
        todo!();
        /*
            button.setToggleState (isParameterOn(), dontSendNotification);
        */
    }
    
    pub fn button_clicked(&mut self)  {
        
        todo!();
        /*
            if (isParameterOn() != button.getToggleState())
            {
                getParameter().beginChangeGesture();
                getParameter().setValueNotifyingHost (button.getToggleState() ? 1.0f : 0.0f);
                getParameter().endChangeGesture();
            }
        */
    }
    
    pub fn is_parameter_on(&self) -> bool {
        
        todo!();
        /*
            return getParameter().getValue() >= 0.5f;
        */
    }
}
