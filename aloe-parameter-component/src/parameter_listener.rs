crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ParameterListener<'a> {
    base3:                       Timer,
    processor:                   &'a mut AudioProcessor<'a>,
    parameter:                   &'a mut AudioProcessorParameter,
    parameter_value_has_changed: Atomic<i32>, // default = { 0  }
    is_legacy_param:             bool,
}

impl<'a> AudioProcessorListener for ParameterListener<'a> {

}

impl<'a> AudioProcessorParameterChangeGestureEnd for ParameterListener<'a> {

}

impl<'a> AudioProcessorParameterChangeGestureBegin for ParameterListener<'a> {

}

impl<'a> AudioProcessorChanged for ParameterListener<'a> {

    fn audio_processor_changed(
        &mut self, 
        _0: *mut dyn AudioProcessorInterface,
        _1: &AudioProcessorChangeDetails

    ) {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> AudioProcessorParameterChanged for ParameterListener<'a> {

    fn audio_processor_parameter_changed(&mut self, 
        _0:    *mut dyn AudioProcessorInterface,
        index: i32,
        _2:    f32)  {
        
        todo!();
        /*
            if (index == parameter.getParameterIndex())
                parameterValueHasChanged = 1;
        */
    }
}

impl<'a> AudioProcessorParameterListener for ParameterListener<'a> {

    fn parameter_value_changed(&mut self, _0: i32, _1: f32)  {
        
        todo!();
        /*
            parameterValueHasChanged = 1;
        */
    }

    fn parameter_gesture_changed(&mut self, 
        _0: i32,
        _1: bool)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for ParameterListener<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (isLegacyParam)
                processor.removeListener (this);
            else
                parameter.removeListener (this);
        */
    }
}

impl<'a> ParameterListener<'a> {

    pub fn new(
        proc:  &mut AudioProcessor,
        param: &mut AudioProcessorParameter) -> Self {
    
        todo!();
        /*


            : processor (proc), parameter (param), isLegacyParam (LegacyAudioParameter::isLegacy (&param))

            if (isLegacyParam)
                processor.addListener (this);
            else
                parameter.addListener (this);

            startTimer (100);
        */
    }
    
    pub fn get_parameter(&self) -> &mut AudioProcessorParameter {
        
        todo!();
        /*
            return parameter;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (parameterValueHasChanged.compareAndSetBool (0, 1))
            {
                handleNewParameterValue();
                startTimerHz (50);
            }
            else
            {
                startTimer (jmin (250, getTimerInterval() + 10));
            }
        */
    }
}
