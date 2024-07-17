crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_ParameterAttachments.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_ParameterAttachments.cpp]

/**
  | Used to implement 'attachments' or
  | 'controllers' that link a plug-in parameter
  | to a UI element.
  | 
  | To implement a new attachment type,
  | create a new class which includes an
  | instance of this class as a data member.
  | Your class should pass a function to
  | the constructor of the ParameterAttachment,
  | which will then be called on the message
  | thread when the parameter changes.
  | You can use this function to update the
  | state of the UI control. Your class should
  | also register as a listener of the UI
  | control and respond to respond to changes
  | in the UI element by calling either setValueAsCompleteGesture
  | or beginGesture, setValueAsPartOfGesture
  | and endGesture.
  | 
  | Make sure to call `sendInitialUpdate`
  | at the end of your new attachment's constructor,
  | so that the UI immediately reflects
  | the state of the parameter.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ParameterAttachment<'a> {
    base2:        AsyncUpdater<'a>,
    parameter:    &'a mut RangedAudioParameter,
    last_value:   Atomic<f32>, // default = { 0.0f  }
    undo_manager: *mut UndoManager<'a>, // default = nullptr
    set_value:    fn(_0: f32) -> (),
}

impl<'a> AudioProcessorParameterListener for ParameterAttachment<'a> {

    fn parameter_value_changed(&mut self, 
        _0:        i32,
        new_value: f32)  {
        
        todo!();
        /*
            lastValue = newValue;

        if (MessageManager::getInstance()->isThisTheMessageThread())
        {
            cancelPendingUpdate();
            handleAsyncUpdate();
        }
        else
        {
            triggerAsyncUpdate();
        }
        */
    }

    fn parameter_gesture_changed(
        &mut self, 
        _0: i32,
        _1: bool)  { }
}

impl<'a> Drop for ParameterAttachment<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            parameter.removeListener (this);
        cancelPendingUpdate();
        */
    }
}

impl<'a> ParameterAttachment<'a> {

    pub fn normalise(&self, f: f32) -> f32 {
        
        todo!();
        /*
            return parameter.convertTo0to1 (f);
        */
    }
    
    /**
      | Listens to a parameter and calls the
      | the provided function in response to
      | parameter changes. If an undoManager
      | is supplied `beginNewTransaction`
      | will be called on it whenever the UI requests
      | a parameter change via this attachment.
      | 
      | -----------
      | @param parameter
      | 
      | The parameter to which this attachment
      | will listen
      | ----------
      | @param parameterChangedCallback
      | 
      | The function that will be called on the
      | message thread in response to parameter
      | changes
      | ----------
      | @param undoManager
      | 
      | The UndoManager that will be used to
      | begin transactions when the UI requests
      | a parameter change.
      |
      */
    pub fn new(
        param:                      &mut RangedAudioParameter,
        parameter_changed_callback: fn(_0: f32) -> (),
        um:                         *mut UndoManager) -> Self {
    
        todo!();
        /*


            : parameter (param),
          undoManager (um),
          setValue (std::move (parameterChangedCallback))

        parameter.addListener (this);
        */
    }
    
    /**
      | Calls the parameterChangedCallback
      | function that was registered in the
      | constructor, making the UI reflect
      | the current parameter state.
      | 
      | This function should be called after
      | doing any necessary setup on the UI control
      | that is being managed (e.g. adding ComboBox
      | entries, making buttons toggle-able).
      |
      */
    pub fn send_initial_update(&mut self)  {
        
        todo!();
        /*
            parameterValueChanged ({}, parameter.getValue());
        */
    }
    
    /**
      | Triggers a full gesture message on the
      | managed parameter.
      | 
      | Call this in the listener callback of
      | the UI control in response to a one-off
      | change in the UI like a button-press.
      |
      */
    pub fn set_value_as_complete_gesture(&mut self, new_denormalised_value: f32)  {
        
        todo!();
        /*
            callIfParameterValueChanged (newDenormalisedValue, [this] (float f)
        {
            beginGesture();
            parameter.setValueNotifyingHost (f);
            endGesture();
        });
        */
    }
    
    /**
      | Begins a gesture on the managed parameter.
      | 
      | Call this when the UI is about to begin
      | a continuous interaction, like when
      | the mouse button is pressed on a slider.
      |
      */
    pub fn begin_gesture(&mut self)  {
        
        todo!();
        /*
            if (undoManager != nullptr)
            undoManager->beginNewTransaction();

        parameter.beginChangeGesture();
        */
    }
    
    /**
      | Updates the parameter value during
      | a gesture.
      | 
      | Call this during a continuous interaction,
      | like a slider value changed callback.
      |
      */
    pub fn set_value_as_part_of_gesture(&mut self, new_denormalised_value: f32)  {
        
        todo!();
        /*
            callIfParameterValueChanged (newDenormalisedValue, [this] (float f)
        {
            parameter.setValueNotifyingHost (f);
        });
        */
    }
    
    /**
      | Ends a gesture on the managed parameter.
      | 
      | Call this when the UI has finished a continuous
      | interaction, like when the mouse button
      | is released on a slider.
      |
      */
    pub fn end_gesture(&mut self)  {
        
        todo!();
        /*
            parameter.endChangeGesture();
        */
    }
    
    pub fn call_if_parameter_value_changed<Callback>(&mut self, 
        new_denormalised_value: f32,
        callback:               Callback)  {
    
        todo!();
        /*
            const auto newValue = normalise (newDenormalisedValue);

        if (parameter.getValue() != newValue)
            callback (newValue);
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            if (setValue != nullptr)
            setValue (parameter.convertFrom0to1 (lastValue));
        */
    }
}
