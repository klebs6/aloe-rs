crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ChoiceParameterComponent<'a> {
    base:             Component<'a>,
    base2:            ParameterListener<'a>,
    box_:             ComboBox<'a>,
    parameter_values: Vec<String>,
}

impl<'a> ChoiceParameterComponent<'a> {

    pub fn new(
        proc:  &mut AudioProcessor,
        param: &mut AudioProcessorParameter) -> Self {
    
        todo!();
        /*


            : ParameterListener (proc, param),
              parameterValues (getParameter().getAllValueStrings())

            box.addItemList (parameterValues, 1);

            // Set the initial value.
            handleNewParameterValue();

            box.onChange = [this] { boxChanged(); };
            addAndMakeVisible (box);
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
            box.setBounds (area.reduced (0, 10));
        */
    }
    
    pub fn handle_new_parameter_value(&mut self)  {
        
        todo!();
        /*
            auto index = parameterValues.indexOf (getParameter().getCurrentValueAsText());

            if (index < 0)
            {
                // The parameter is producing some unexpected text, so we'll do
                // some linear interpolation.
                index = roundToInt (getParameter().getValue() * (float) (parameterValues.size() - 1));
            }

            box.setSelectedItemIndex (index);
        */
    }
    
    pub fn box_changed(&mut self)  {
        
        todo!();
        /*
            if (getParameter().getCurrentValueAsText() != box.getText())
            {
                getParameter().beginChangeGesture();

                // When a parameter provides a list of strings we must set its
                // value using those strings, rather than a float, because VSTs can
                // have uneven spacing between the different allowed values.
                getParameter().setValueNotifyingHost (getParameter().getValueForText (box.getText()));

                getParameter().endChangeGesture();
            }
        */
    }
}

