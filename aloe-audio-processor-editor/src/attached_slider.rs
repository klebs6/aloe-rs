crate::ix!();

pub struct AttachedSlider<'a> {
    base:       ComponentWithParamMenu<'a>,
    slider:     Slider<'a>, // default = { Slider::RotaryVerticalDrag, Slider::TextBoxBelow  }
    label:      Label<'a>,
    attachment: SliderParameterAttachment<'a>,
}

impl<'a> AttachedSlider<'a> {

    pub fn new(
        editor_in: &mut AudioProcessorEditor,
        param_in:  &mut RangedAudioParameter) -> Self {
    
        todo!();
        /*


            : ComponentWithParamMenu (editorIn, paramIn),
                  label ("", paramIn.name),
                  attachment (paramIn, slider)

                slider.addMouseListener (this, true);

                addAllAndMakeVisible (*this, slider, label);

                slider.setTextValueSuffix (" " + paramIn.label);

                label.attachToComponent (&slider, false);
                label.setJustificationType (Justification::centred);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            slider.setBounds (getLocalBounds().reduced (0, 40));
        */
    }
}

