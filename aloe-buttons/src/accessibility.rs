crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ButtonAccessibilityHandler<'a> {
    base:   AccessibilityHandler<'a>,
    button: &'a mut Button<'a>,
}

impl<'a> ButtonAccessibilityHandler<'a> {

    pub fn new(
        button_to_wrap: &mut Button<'a>,
        role_in:        AccessibilityRole) -> Self {
    
        todo!();
        /*


            : AccessibilityHandler (buttonToWrap,
                                    isRadioButton (buttonToWrap) ? AccessibilityRole::radioButton : roleIn,
                                    getAccessibilityActions (buttonToWrap, roleIn)),
              button (buttonToWrap)
        */
    }
    
    pub fn get_current_state(&self) -> AccessibleState {
        
        todo!();
        /*
            auto state = AccessibilityHandler::getCurrentState();

            if (isToggleButton (getRole()) || isRadioButton (button))
            {
                state = state.withCheckable();

                if (button.getToggleState())
                    state = state.withChecked();
            }

            return state;
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            auto title = AccessibilityHandler::getTitle();

            if (title.isEmpty())
                return button.getButtonText();

            return title;
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return button.getTooltip();
        */
    }
    
    pub fn is_toggle_button(role: AccessibilityRole) -> bool {
        
        todo!();
        /*
            return role == AccessibilityRole::toggleButton;
        */
    }
    
    pub fn is_radio_button(button: &Button<'a>) -> bool {
        
        todo!();
        /*
            return button.getRadioGroupId() != 0;
        */
    }
    
    pub fn get_accessibility_actions(
        button: &mut Button<'a>,
        role:   AccessibilityRole) -> AccessibilityActions {
        
        todo!();
        /*
            auto actions = AccessibilityActions().addAction (AccessibilityActionType::press,
                                                             [&button] { button.triggerClick(); });

            if (isToggleButton (role))
                actions = actions.addAction (AccessibilityActionType::toggle,
                                             [&button] { button.setToggleState (! button.getToggleState(), sendNotification); });

            return actions;
        */
    }
}
