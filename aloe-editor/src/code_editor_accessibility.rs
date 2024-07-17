crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CodeEditorAccessibilityHandler<'a> {
    base: AccessibilityHandler<'a>,
}

impl<'a> CodeEditorAccessibilityHandler<'a> {

    pub fn new(code_editor_component_to_wrap: &mut CodeEditorComponent) -> Self {
    
        todo!();
        /*
        : AccessibilityHandler (codeEditorComponentToWrap,
                                codeEditorComponentToWrap.isReadOnly() ? AccessibilityRole::staticText
                                                                       : AccessibilityRole::editableText,
                                {},
                                { std::make_unique<CodeEditorComponentTextInterface> (codeEditorComponentToWrap) })
        */
    }
}
