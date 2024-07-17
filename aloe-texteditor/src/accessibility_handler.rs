crate::ix!();

///------------------------
#[no_copy]
#[leak_detector]
pub struct TextEditorAccessibilityHandler<'a> {
    base:        AccessibilityHandler<'a>,
    text_editor: &'a mut TextEditor<'a>,
}

impl<'a> TextEditorAccessibilityHandler<'a> {

    pub fn new(text_editor_to_wrap: &mut TextEditor) -> Self {
    
        todo!();
        /*
            : AccessibilityHandler (textEditorToWrap,
                                    textEditorToWrap.isReadOnly() ? AccessibilityRole::staticText : AccessibilityRole::editableText,
                                    {},
                                    { std::make_unique<TextEditorTextInterface> (textEditorToWrap) }),
              textEditor (textEditorToWrap)
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return textEditor.getTooltip();
        */
    }
}
