crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ButtonCallbackHelper<'a> {
    base:   Timer,
    button: &'a mut Button<'a>,
}

impl<'a> ApplicationCommandManagerListener for ButtonCallbackHelper<'a> {

    fn application_command_invoked(&mut self, info: &ApplicationCommandTargetInvocationInfo)  {
        
        todo!();
        /*
            if (info.commandID == button.commandID
                 && (info.commandFlags & ApplicationCommandInfo::dontTriggerVisualFeedback) == 0)
                button.flashButtonState();
        */
    }
    
    fn application_command_list_changed(&mut self)  {
        
        todo!();
        /*
            button.applicationCommandListChangeCallback();
        */
    }
}

impl<'a> ValueListener for ButtonCallbackHelper<'a> {

    fn value_changed(&mut self, value: &mut Value)  {
        
        todo!();
        /*
            if (value.refersToSameSourceAs (button.isOn))
                button.setToggleState (button.isOn.getValue(), dontSendNotification, sendNotification);
        */
    }
}

impl<'a> KeyListener for ButtonCallbackHelper<'a> {

    fn key_pressed(
        &mut self, 
        _0: &KeyPress,
        _1: *mut Component<'_>
    ) -> bool {
        
        todo!();
        /*
            // returning true will avoid forwarding events for keys that we're using as shortcuts
            return button.isShortcutPressed();
        */
    }
}

impl<'a> ButtonCallbackHelper<'a> {

    pub fn new(b: &mut Button<'a>) -> Self {
    
        todo!();
        /*
        : button(b),
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            button.repeatTimerCallback();
        */
    }
    
    pub fn key_state_changed(&mut self, 
        _0: bool,
        _1: *mut Component<'a>) -> bool {
        
        todo!();
        /*
            return button.keyStateChangedCallback();
        */
    }
}
