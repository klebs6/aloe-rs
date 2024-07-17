crate::ix!();

/**
  | An object of this class maintains a connection
  | between a Slider and a plug-in parameter.
  | 
  | During the lifetime of this object it
  | keeps the two things in sync, making
  | it easy to connect a slider to a parameter.
  | When this object is deleted, the connection
  | is broken. Make sure that your parameter
  | and Slider are not deleted before this
  | object!
  | 
  | @tags{Audio}
  |
  */
pub struct SliderParameterAttachment<'a> {
    slider:           &'a mut Slider<'a>,
    attachment:       ParameterAttachment<'a>,
    ignore_callbacks: bool, // default = false
}

impl<'a> SliderListener for SliderParameterAttachment<'a> {

}

impl<'a> SliderDragEnded for SliderParameterAttachment<'a> {

    fn slider_drag_ended(&mut self, _0: *mut Slider)  {
        
        todo!();
        /*
            attachment.endGesture();
        */
    }
}

impl<'a> SliderDragStarted for SliderParameterAttachment<'a> {

    fn slider_drag_started(&mut self, _0: *mut Slider)  {
        
        todo!();
        /*
            attachment.beginGesture();
        */
    }
}

impl<'a> SliderValueChanged for SliderParameterAttachment<'a> {

    fn slider_value_changed(&mut self, _0: *mut Slider)  {
        
        todo!();
        /*
            if (ignoreCallbacks || ModifierKeys::currentModifiers.isRightButtonDown())
            return;

        attachment.setValueAsPartOfGesture ((float) slider.getValue());
        */
    }
}

impl<'a> Drop for SliderParameterAttachment<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            slider.removeListener (this);
        */
    }
}

impl<'a> SliderParameterAttachment<'a> {

    /**
      | Creates a connection between a plug-in
      | parameter and a Slider.
      | 
      | -----------
      | @param parameter
      | 
      | The parameter to use
      | ----------
      | @param slider
      | 
      | The Slider to use
      | ----------
      | @param undoManager
      | 
      | An optional UndoManager
      |
      */
    pub fn new(
        param: &mut RangedAudioParameter,
        s:     &mut Slider,
        um:    *mut UndoManager) -> Self {
    
        todo!();
        /*


            : slider (s),
          attachment (param, [this] (float f) { setValue (f); }, um)

        slider.valueFromTextFunction = [&param] (const String& text) { return (double) param.convertFrom0to1 (param.getValueForText (text)); };
        slider.textFromValueFunction = [&param] (double value) { return param.getText (param.convertTo0to1 ((float) value), 0); };
        slider.setDoubleClickReturnValue (true, param.convertFrom0to1 (param.getDefaultValue()));

        auto range = param.getNormalisableRange();

        auto convertFrom0To1Function = [range] (double currentRangeStart,
                                                double currentRangeEnd,
                                                double normalisedValue) mutable
        {
            range.start = (float) currentRangeStart;
            range.end = (float) currentRangeEnd;
            return (double) range.convertFrom0to1 ((float) normalisedValue);
        };

        auto convertTo0To1Function = [range] (double currentRangeStart,
                                              double currentRangeEnd,
                                              double mappedValue) mutable
        {
            range.start = (float) currentRangeStart;
            range.end = (float) currentRangeEnd;
            return (double) range.convertTo0to1 ((float) mappedValue);
        };

        auto snapToLegalValueFunction = [range] (double currentRangeStart,
                                                 double currentRangeEnd,
                                                 double mappedValue) mutable
        {
            range.start = (float) currentRangeStart;
            range.end = (float) currentRangeEnd;
            return (double) range.snapToLegalValue ((float) mappedValue);
        };

        NormalisableRange<double> newRange { (double) range.start,
                                             (double) range.end,
                                             std::move (convertFrom0To1Function),
                                             std::move (convertTo0To1Function),
                                             std::move (snapToLegalValueFunction) };
        newRange.interval = range.interval;
        newRange.skew = range.skew;
        newRange.symmetricSkew = range.symmetricSkew;

        slider.setNormalisableRange (newRange);

        sendInitialUpdate();
        slider.valueChanged();
        slider.addListener (this);
        */
    }
    
    /**
      | Call this after setting up your slider
      | in the case where you need to do extra
      | setup after constructing this attachment.
      |
      */
    pub fn send_initial_update(&mut self)  {
        
        todo!();
        /*
            attachment.sendInitialUpdate();
        */
    }
    
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            const ScopedValueSetter<bool> svs (ignoreCallbacks, true);
        slider.setValue (newValue, sendNotificationSync);
        */
    }
    
}
