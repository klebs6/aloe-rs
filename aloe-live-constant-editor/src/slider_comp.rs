crate::ix!();

pub struct SliderComp<'a> {
    base:     Component<'a>,
    editor:   &'a mut LivePropertyEditorBase<'a>,
    slider:   Slider<'a>,
    is_float: bool,
}

pub trait UpdateRange {

    fn update_range(&mut self);
}

impl<'a> UpdateRange for SliderComp<'a> {

    fn update_range(&mut self)  {
        
        todo!();
        /*
            double v = isFloat ? parseDouble (editor.value.getStringValue (false))
                               : (double) parseInt (editor.value.getStringValue (false));

            double range = isFloat ? 10 : 100;

            slider.setRange (v - range, v + range);
            slider.setValue (v, dontSendNotification);
        */
    }
}

impl<'a> SliderComp<'a> {

    pub fn new(
        e:         &mut LivePropertyEditorBase,
        use_float: bool) -> Self {
    
        todo!();
        /*
        : editor(e),
        : is_float(useFloat),

            slider.setTextBoxStyle (Slider::NoTextBox, true, 0, 0);
            addAndMakeVisible (slider);
            updateRange();
            slider.onDragEnd = [this] { updateRange(); };
            slider.onValueChange = [this]
            {
                editor.applyNewValue (isFloat ? getAsString ((double) slider.getValue(), editor.wasHex)
                                              : getAsString ((int64)  slider.getValue(), editor.wasHex));
            };
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            slider.setBounds (getLocalBounds().removeFromTop (25));
        */
    }
}
