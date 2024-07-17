crate::ix!();

#[no_copy]
#[leak_detector]
pub struct LabelAccessibilityHandler<'a> {
    base:  AccessibilityHandler<'a>,
    label: &'a mut Label<'a>,
}

impl<'a> LabelAccessibilityHandler<'a> {

    pub fn new(label_to_wrap: &mut Label) -> Self {
    
        todo!();
        /*
            : AccessibilityHandler (labelToWrap,
                                    labelToWrap.isEditable() ? AccessibilityRole::editableText : AccessibilityRole::label,
                                    getAccessibilityActions (labelToWrap),
                                    { std::make_unique<LabelValueInterface> (labelToWrap) }),
              label (labelToWrap)
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            return label.getText();
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return label.getTooltip();
        */
    }
    
    pub fn get_current_state(&self) -> AccessibleState {
        
        todo!();
        /*
            if (label.isBeingEdited())
                return {}; // allow focus to pass through to the TextEditor

            return AccessibilityHandler::getCurrentState();
        */
    }
    
    pub fn get_accessibility_actions(label: &mut Label) -> AccessibilityActions {
        
        todo!();
        /*
            if (label.isEditable())
                return AccessibilityActions().addAction (AccessibilityActionType::press, [&label] { label.showEditor(); });

            return {};
        */
    }
}
