crate::ix!();

///---------------------
#[no_copy]
pub struct TextEditorViewport<'a> {
    base:                 Viewport<'a>,
    owner:                &'a mut TextEditor<'a>,
    last_word_wrap_width: i32, // default = 0
    reentrant:            bool, // default = false
}

impl<'a> TextEditorViewport<'a> {

    pub fn new(ed: &mut TextEditor) -> Self {
    
        todo!();
        /*
        : owner(ed),

        
        */
    }
    
    pub fn visible_area_changed(&mut self, _0: &Rectangle<i32>)  {
        
        todo!();
        /*
            if (! reentrant) // it's rare, but possible to get into a feedback loop as the viewport's scrollbars
                             // appear and disappear, causing the wrap width to change.
            {
                auto wordWrapWidth = owner.getWordWrapWidth();

                if (wordWrapWidth != lastWordWrapWidth)
                {
                    lastWordWrapWidth = wordWrapWidth;

                    ScopedValueSetter<bool> svs (reentrant, true);
                    owner.checkLayout();
                }
            }
        */
    }
}
