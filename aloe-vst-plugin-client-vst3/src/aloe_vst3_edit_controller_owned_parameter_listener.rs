crate::ix!();

pub struct AloeVst3EditControllerOwnedParameterListener<'a> {
    owner:           &'a mut AloeVst3EditController<'a>,
    vst_paramid:     VstParamID, // default = Vst_kNoParamId
    parameter_index: i32, // default = -1
}

impl<'a> AudioProcessorParameterListener for AloeVst3EditControllerOwnedParameterListener<'a> {

    fn parameter_value_changed(&mut self, _: i32, _: f32) { todo!() }

    fn parameter_gesture_changed(&mut self, _: i32, _: bool) { todo!() }
}

impl<'a> AloeVst3EditControllerOwnedParameterListener<'a> {

    pub fn new(
        edit_controller: &mut AloeVst3EditController,
        parameter:       &mut AudioProcessorParameter,
        paramid:         VstParamID,
        cache_index:     i32) -> Self {
    
        todo!();
        /*


            : owner (editController),
                  vstParamID (paramID),
                  parameterIndex (cacheIndex)

                // We shouldn't be using an AloeVst3EditControllerOwnedParameterListener for parameters that have
                // been added directly to the AudioProcessor. We observe those via the
                // normal audioProcessorParameterChanged mechanism.
                jassert (parameter.getParameterIndex() == -1);
                // The parameter must have a non-negative index in the parameter cache.
                jassert (parameterIndex >= 0);
                parameter.addListener (this);
        */
    }
    
    pub fn parameter_value_changed(&mut self, 
        _0:        i32,
        new_value: f32)  {
        
        todo!();
        /*
            owner.paramChanged (parameterIndex, vstParamID, newValue);
        */
    }
    
    pub fn parameter_gesture_changed(&mut self, 
        _0:                  i32,
        gesture_is_starting: bool)  {
        
        todo!();
        /*
            if (gestureIsStarting)
                    owner.beginGesture (vstParamID);
                else
                    owner.endGesture (vstParamID);
        */
    }
}
