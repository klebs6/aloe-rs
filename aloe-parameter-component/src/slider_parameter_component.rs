crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SliderParameterComponent<'a> {
    base:        Component<'a>,
    base2:       ParameterListener<'a>,
    slider:      Slider<'a>, //{ Slider::LinearHorizontal, Slider::TextEntryBoxPosition::NoTextBox };
    value_label: Label<'a>,
    is_dragging: bool, // default = false
}

impl<'a> SliderParameterComponent<'a> {

    pub fn new(
        proc:  &mut AudioProcessor,
        param: &mut AudioProcessorParameter) -> Self {
    
        todo!();
        /*
        : parameter_listener(proc, param),

            if (getParameter().getNumSteps() != AudioProcessor::getDefaultNumParameterSteps())
                slider.setRange (0.0, 1.0, 1.0 / (getParameter().getNumSteps() - 1.0));
            else
                slider.setRange (0.0, 1.0);

            slider.setDoubleClickReturnValue (true, param.getDefaultValue());

            slider.setScrollWheelEnabled (false);
            addAndMakeVisible (slider);

            valueLabel.setColour (Label::outlineColourId, slider.findColour (Slider::textBoxOutlineColourId));
            valueLabel.setBorderSize ({ 1, 1, 1, 1 });
            valueLabel.setJustificationType (Justification::centred);
            addAndMakeVisible (valueLabel);

            // Set the initial value.
            handleNewParameterValue();

            slider.onValueChange = [this] { sliderValueChanged(); };
            slider.onDragStart   = [this] { sliderStartedDragging(); };
            slider.onDragEnd     = [this] { sliderStoppedDragging(); };
        */
    }
    
    pub fn paint(&mut self, _0: &mut Graphics)  {
        
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds().reduced (0, 10);

            valueLabel.setBounds (area.removeFromRight (80));

            area.removeFromLeft (6);
            slider.setBounds (area);
        */
    }
    
    pub fn update_text_display(&mut self)  {
        
        todo!();
        /*
            valueLabel.setText (getParameter().getCurrentValueAsText(), dontSendNotification);
        */
    }
    
    pub fn handle_new_parameter_value(&mut self)  {
        
        todo!();
        /*
            if (! isDragging)
            {
                slider.setValue (getParameter().getValue(), dontSendNotification);
                updateTextDisplay();
            }
        */
    }
    
    pub fn slider_value_changed(&mut self)  {
        
        todo!();
        /*
            auto newVal = (float) slider.getValue();

            if (getParameter().getValue() != newVal)
            {
                if (! isDragging)
                    getParameter().beginChangeGesture();

                getParameter().setValueNotifyingHost ((float) slider.getValue());
                updateTextDisplay();

                if (! isDragging)
                    getParameter().endChangeGesture();
            }
        */
    }
    
    pub fn slider_started_dragging(&mut self)  {
        
        todo!();
        /*
            isDragging = true;
            getParameter().beginChangeGesture();
        */
    }
    
    pub fn slider_stopped_dragging(&mut self)  {
        
        todo!();
        /*
            isDragging = false;
            getParameter().endChangeGesture();
        */
    }
}
