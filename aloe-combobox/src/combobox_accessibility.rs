crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ComboBoxAccessibilityHandler<'a> {
    base:      AccessibilityHandler<'a>,
    combo_box: &'a mut ComboBox<'a>,
}

impl<'a> ComboBoxAccessibilityHandler<'a> {

    pub fn new(combo_box_to_wrap: &mut ComboBox) -> Self {
    
        todo!();
        /*


            : AccessibilityHandler (comboBoxToWrap,
                                    AccessibilityRole::comboBox,
                                    getAccessibilityActions (comboBoxToWrap)),
              comboBox (comboBoxToWrap)
        */
    }
    
    pub fn get_current_state(&self) -> AccessibleState {
        
        todo!();
        /*
            auto state = AccessibilityHandler::getCurrentState().withExpandable();

            return comboBox.isPopupActive() ? state.withExpanded() : state.withCollapsed();
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            return comboBox.getText();
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return comboBox.getTooltip();
        */
    }
    
    pub fn get_accessibility_actions(combo_box: &mut ComboBox) -> AccessibilityActions {
        
        todo!();
        /*
            return AccessibilityActions().addAction (AccessibilityActionType::press,    [&comboBox] { comboBox.showPopup(); })
                                         .addAction (AccessibilityActionType::showMenu, [&comboBox] { comboBox.showPopup(); });
        */
    }
}
