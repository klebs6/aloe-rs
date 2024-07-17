crate::ix!();

pub struct AudioProcessorValueTreeStateParameterAdapter<'a> {
    tree:                               ValueTree,
    parameter:                          &'a mut RangedAudioParameter,
    listeners:                          AudioProcessorValueTreeStateParameterAdapterLockedListeners,
    unnormalised_value:                 Atomic<f32>, // default = { 0.0f  }
    needs_update:                       AtomicBool, // default = { true  }
    listeners_need_calling:             AtomicBool, // default = { true  }
    ignore_parameter_changed_callbacks: bool, // default = { false  }
}

impl<'a> AudioProcessorValueTreeStateListener for AudioProcessorValueTreeStateParameterAdapter<'a> {

    fn parameter_changed(
        &mut self, 
        parameterid: &String, 
        new_value:   f32
    ) {}
}

impl<'a> Drop for AudioProcessorValueTreeStateParameterAdapter<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            parameter.removeListener (this);
        */
    }
}

impl<'a> AudioProcessorValueTreeStateParameterAdapter<'a> {

    pub fn new(parameter_in: &mut RangedAudioParameter) -> Self {
    
        todo!();
        /*


            : parameter (parameterIn),
              // For legacy reasons, the unnormalised value should *not* be snapped on construction
              unnormalisedValue (getRange().convertFrom0to1 (parameter.getDefaultValue()))

            parameter.addListener (this);

            if (auto* ptr = dynamic_cast<AudioProcessorValueTreeStateParameter*> (&parameter))
                ptr->onValueChanged = [this] { parameterValueChanged ({}, {}); };
        */
    }
    
    pub fn add_listener(&mut self, l: *mut dyn AudioProcessorValueTreeStateListener)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    pub fn remove_listener(&mut self, l: *mut dyn AudioProcessorValueTreeStateListener)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    pub fn get_parameter_mut(&mut self) -> &mut RangedAudioParameter {
        
        todo!();
        /*
            return parameter;
        */
    }
    
    pub fn get_parameter(&self) -> &RangedAudioParameter {
        
        todo!();
        /*
            return parameter;
        */
    }
    
    pub fn get_range(&self) -> &NormalisableRange<f32> {
        
        todo!();
        /*
            return parameter.getNormalisableRange();
        */
    }
    
    pub fn get_denormalised_default_value(&self) -> f32 {
        
        todo!();
        /*
            return denormalise (parameter.getDefaultValue());
        */
    }
    
    pub fn set_denormalised_value(&mut self, value: f32)  {
        
        todo!();
        /*
            if (value == unnormalisedValue)
                return;

            setNormalisedValue (normalise (value));
        */
    }
    
    pub fn get_denormalised_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            return denormalise (parameter.getValueForText (text));
        */
    }
    
    pub fn get_text_for_denormalised_value(&self, value: f32) -> String {
        
        todo!();
        /*
            return parameter.getText (normalise (value), 0);
        */
    }
    
    pub fn get_denormalised_value(&self) -> f32 {
        
        todo!();
        /*
            return unnormalisedValue;
        */
    }
    
    pub fn get_raw_denormalised_value(&mut self) -> &mut Atomic<f32> {
        
        todo!();
        /*
            return unnormalisedValue;
        */
    }
    
    pub fn flush_to_tree(&mut self, 
        key: &Identifier,
        um:  *mut UndoManager) -> bool {
        
        todo!();
        /*
            auto needsUpdateTestValue = true;

            if (! needsUpdate.compare_exchange_strong (needsUpdateTestValue, false))
                return false;

            if (auto valueProperty = tree.getPropertyPointer (key))
            {
                if ((float) *valueProperty != unnormalisedValue)
                {
                    ScopedValueSetter<bool> svs (ignoreParameterChangedCallbacks, true);
                    tree.setProperty (key, unnormalisedValue.load(), um);
                }
            }
            else
            {
                tree.setProperty (key, unnormalisedValue.load(), nullptr);
            }

            return true;
        */
    }
    
    pub fn parameter_gesture_changed(&mut self, 
        _0: i32,
        _1: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn parameter_value_changed(&mut self, _0: i32, _1: f32)  {
        
        todo!();
        /*
            const auto newValue = denormalise (parameter.getValue());

            if (unnormalisedValue == newValue && ! listenersNeedCalling)
                return;

            unnormalisedValue = newValue;
            listeners.call ([this] (AudioProcessorValueTreeStateListener& l) { l.parameterChanged (parameter.paramID, unnormalisedValue); });
            listenersNeedCalling = false;
            needsUpdate = true;
        */
    }
    
    pub fn denormalise(&self, normalised: f32) -> f32 {
        
        todo!();
        /*
            return getParameter().convertFrom0to1 (normalised);
        */
    }
    
    pub fn normalise(&self, denormalised: f32) -> f32 {
        
        todo!();
        /*
            return getParameter().convertTo0to1 (denormalised);
        */
    }
    
    pub fn set_normalised_value(&mut self, value: f32)  {
        
        todo!();
        /*
            if (ignoreParameterChangedCallbacks)
                return;

            parameter.setValueNotifyingHost (value);
        */
    }
}
