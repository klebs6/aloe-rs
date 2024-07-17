crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SwitchParameterComponent<'a> {
    base:    Component<'a>,
    base2:   ParameterListener<'a>,
    buttons: [TextButton<'a>; 2],
}

impl<'a> SwitchParameterComponent<'a> {

    pub fn new(
        proc:  &mut AudioProcessor,
        param: &mut AudioProcessorParameter) -> Self {
    
        todo!();
        /*
        : parameter_listener(proc, param),

            for (auto& button : buttons)
            {
                button.setRadioGroupId (293847);
                button.setClickingTogglesState (true);
            }

            buttons[0].setButtonText (getParameter().getText (0.0f, 16));
            buttons[1].setButtonText (getParameter().getText (1.0f, 16));

            buttons[0].setConnectedEdges (Button::ConnectedOnRight);
            buttons[1].setConnectedEdges (Button::ConnectedOnLeft);

            // Set the initial value.
            buttons[0].setToggleState (true, dontSendNotification);
            handleNewParameterValue();

            buttons[1].onStateChange = [this] { rightButtonChanged(); };

            for (auto& button : buttons)
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
            auto area = getLocalBounds().reduced (0, 8);
            area.removeFromLeft (8);

            for (auto& button : buttons)
                button.setBounds (area.removeFromLeft (80));
        */
    }
    
    pub fn handle_new_parameter_value(&mut self)  {
        
        todo!();
        /*
            bool newState = isParameterOn();

            if (buttons[1].getToggleState() != newState)
            {
                buttons[1].setToggleState (newState,   dontSendNotification);
                buttons[0].setToggleState (! newState, dontSendNotification);
            }
        */
    }
    
    pub fn right_button_changed(&mut self)  {
        
        todo!();
        /*
            auto buttonState = buttons[1].getToggleState();

            if (isParameterOn() != buttonState)
            {
                getParameter().beginChangeGesture();

                if (getParameter().getAllValueStrings().isEmpty())
                {
                    getParameter().setValueNotifyingHost (buttonState ? 1.0f : 0.0f);
                }
                else
                {
                    // When a parameter provides a list of strings we must set its
                    // value using those strings, rather than a float, because VSTs can
                    // have uneven spacing between the different allowed values and we
                    // want the snapping behaviour to be consistent with what we do with
                    // a combo box.
                    auto selectedText = buttons[buttonState ? 1 : 0].getButtonText();
                    getParameter().setValueNotifyingHost (getParameter().getValueForText (selectedText));
                }

                getParameter().endChangeGesture();
            }
        */
    }
    
    pub fn is_parameter_on(&self) -> bool {
        
        todo!();
        /*
            if (getParameter().getAllValueStrings().isEmpty())
                return getParameter().getValue() > 0.5f;

            auto index = getParameter().getAllValueStrings()
                                       .indexOf (getParameter().getCurrentValueAsText());

            if (index < 0)
            {
                // The parameter is producing some unexpected text, so we'll do
                // some linear interpolation.
                index = roundToInt (getParameter().getValue());
            }

            return index == 1;
        */
    }
}
