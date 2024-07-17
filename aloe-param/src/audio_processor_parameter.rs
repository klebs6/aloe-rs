crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorParameter.h]

/**
  | An abstract base class for parameter
  | objects that can be added to an
  | 
  | AudioProcessor.
  | 
  | @see AudioProcessor::addParameter
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorParameter {

    processor:       *mut dyn AudioProcessorInterface, // default = nullptr
    parameter_index: i32, // default = -1
    listener_lock:   CriticalSection,
    listeners:       Vec<*mut dyn AudioProcessorParameterListener>,
    value_strings:   RefCell<Vec<String>>,

    #[cfg(ALOE_DEBUG)]
    is_performing_gesture: bool, // default = false
}

impl Drop for AudioProcessorParameter {

    fn drop(&mut self) {
        todo!();
        /*
            #if ALOE_DEBUG && ! ALOE_DISABLE_AUDIOPROCESSOR_BEGIN_END_GESTURE_CHECKING
        // This will fail if you've called beginChangeGesture() without having made
        // a corresponding call to endChangeGesture...
        jassert (! isPerformingGesture);
       #endif
        */
    }
}

impl AudioProcessorParameter {

    /**
      | Returns the index of this parameter
      | in its parent processor's parameter
      | list.
      |
      */
    pub fn get_parameter_index(&self) -> i32 {
        
        todo!();
        /*
            return parameterIndex;
        */
    }

    /**
      | A processor should call this when it
      | needs to change one of its parameters.
      | 
      | This could happen when the editor or
      | some other internal operation changes
      | a parameter. This method will call the
      | setValue() method to change the value,
      | and will then send a message to the host
      | telling it about the change.
      | 
      | -----------
      | @note
      | 
      | to make sure the host correctly handles
      | automation, you should call the beginChangeGesture()
      | and endChangeGesture() methods to
      | tell the host when the user has started
      | and stopped changing the parameter.
      |
      */
    pub fn set_value_notifying_host(&mut self, new_value: f32)  {
        
        todo!();
        /*
            setValue (newValue);
        sendValueChangedMessageToListeners (newValue);
        */
    }
    
    /**
      | Sends a signal to the host to tell it that
      | the user is about to start changing this
      | parameter.
      | 
      | This allows the host to know when a parameter
      | is actively being held by the user, and
      | it may use this information to help it
      | record automation.
      | 
      | If you call this, it must be matched by
      | a later call to endChangeGesture().
      |
      */
    pub fn begin_change_gesture(&mut self)  {
        
        todo!();
        /*
            // This method can't be used until the parameter has been attached to a processor!
        jassert (processor != nullptr && parameterIndex >= 0);

       #if ALOE_DEBUG && ! ALOE_DISABLE_AUDIOPROCESSOR_BEGIN_END_GESTURE_CHECKING
        // This means you've called beginChangeGesture twice in succession without
        // a matching call to endChangeGesture. That might be fine in most hosts,
        // but it would be better to avoid doing it.
        jassert (! isPerformingGesture);
        isPerformingGesture = true;
       #endif

        ScopedLock lock (listenerLock);

        for (int i = listeners.size(); --i >= 0;)
            if (auto* l = listeners[i])
                l->parameterGestureChanged (getParameterIndex(), true);

        if (processor != nullptr && parameterIndex >= 0)
        {
            // audioProcessorParameterChangeGestureBegin callbacks will shortly be deprecated and
            // this code will be removed.
            for (int i = processor->listeners.size(); --i >= 0;)
                if (auto* l = processor->listeners[i])
                    l->audioProcessorParameterChangeGestureBegin (processor, getParameterIndex());
        }
        */
    }
    
    /**
      | Tells the host that the user has finished
      | changing this parameter.
      | 
      | This allows the host to know when a parameter
      | is actively being held by the user, and
      | it may use this information to help it
      | record automation.
      | 
      | A call to this method must follow a call
      | to beginChangeGesture().
      |
      */
    pub fn end_change_gesture(&mut self)  {
        
        todo!();
        /*
            // This method can't be used until the parameter has been attached to a processor!
        jassert (processor != nullptr && parameterIndex >= 0);

       #if ALOE_DEBUG && ! ALOE_DISABLE_AUDIOPROCESSOR_BEGIN_END_GESTURE_CHECKING
        // This means you've called endChangeGesture without having previously
        // called beginChangeGesture. That might be fine in most hosts, but it
        // would be better to keep the calls matched correctly.
        jassert (isPerformingGesture);
        isPerformingGesture = false;
       #endif

        ScopedLock lock (listenerLock);

        for (int i = listeners.size(); --i >= 0;)
            if (auto* l = listeners[i])
                l->parameterGestureChanged (getParameterIndex(), false);

        if (processor != nullptr && parameterIndex >= 0)
        {
            // audioProcessorParameterChangeGestureEnd callbacks will shortly be deprecated and
            // this code will be removed.
            for (int i = processor->listeners.size(); --i >= 0;)
                if (auto* l = processor->listeners[i])
                    l->audioProcessorParameterChangeGestureEnd (processor, getParameterIndex());
        }
        */
    }
    
    pub fn send_value_changed_message_to_listeners(&mut self, new_value: f32)  {
        
        todo!();
        /*
            ScopedLock lock (listenerLock);

        for (int i = listeners.size(); --i >= 0;)
            if (auto* l = listeners [i])
                l->parameterValueChanged (getParameterIndex(), newValue);

        if (processor != nullptr && parameterIndex >= 0)
        {
            // audioProcessorParameterChanged callbacks will shortly be deprecated and
            // this code will be removed.
            for (int i = processor->listeners.size(); --i >= 0;)
                if (auto* l = processor->listeners[i])
                    l->audioProcessorParameterChanged (processor, getParameterIndex(), newValue);
        }
        */
    }
    
    pub fn is_orientation_inverted(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn is_automatable(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_meta_parameter(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_category(&self) -> AudioProcessorParameterCategory {
        
        todo!();
        /*
            return genericParameter;
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return AudioProcessor::getDefaultNumParameterSteps();
        */
    }
    
    pub fn is_discrete(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn is_boolean(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_text(&self, 
        value:                 f32,
        maximum_string_length: i32) -> String {
        
        todo!();
        /*
            return String (value, 2);
        */
    }
    
    pub fn get_current_value_as_text(&self) -> String {
        
        todo!();
        /*
            return getText (getValue(), 1024);
        */
    }
    
    pub fn get_all_value_strings(&self) -> Vec<String> {
        
        todo!();
        /*
            if (isDiscrete() && valueStrings.isEmpty())
        {
            auto maxIndex = getNumSteps() - 1;

            for (int i = 0; i < getNumSteps(); ++i)
                valueStrings.add (getText ((float) i / (float) maxIndex, 1024));
        }

        return valueStrings;
        */
    }
    
    /**
      | Registers a listener to receive events
      | when the parameter's state changes.
      | 
      | If the listener is already registered,
      | this will not register it again.
      | 
      | @see removeListener
      |
      */
    pub fn add_listener(&mut self, new_listener: *mut dyn AudioProcessorParameterListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
        listeners.addIfNotAlreadyThere (newListener);
        */
    }
    
    /**
      | Removes a previously registered parameter
      | listener
      | 
      | @see addListener
      |
      */
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn AudioProcessorParameterListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
        listeners.removeFirstMatchingValue (listenerToRemove);
        */
    }
}
