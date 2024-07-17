crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TextEditorTextInterface<'a> {
    text_editor: &'a mut TextEditor<'a>,
}

impl<'a> TextEditorTextInterface<'a> {

    pub fn new(editor: &mut TextEditor) -> Self {
    
        todo!();
        /*
        : text_editor(editor),

        
        */
    }
}

impl<'a> AccessibilityTextInterface for TextEditorTextInterface<'a> {

    fn is_displaying_protected_text(&self) -> bool {
        
        todo!();
        /*
            return textEditor.getPasswordCharacter() != 0;
        */
    }
    
    fn is_read_only(&self) -> bool {
        
        todo!();
        /*
            return textEditor.isReadOnly();
        */
    }
    
    fn get_total_num_characters(&self) -> i32 {
        
        todo!();
        /*
            return textEditor.getText().length();
        */
    }
    
    fn get_selection(&self) -> Range<i32> {
        
        todo!();
        /*
            return textEditor.getHighlightedRegion();
        */
    }
    
    fn set_selection(&mut self, r: Range<i32>)  {
        
        todo!();
        /*
            if (r.isEmpty())
                    textEditor.setCaretPosition (r.getStart());
                else
                    textEditor.setHighlightedRegion (r);
        */
    }
    
    fn get_text(&self, r: Range<i32>) -> String {
        
        todo!();
        /*
            if (isDisplayingProtectedText())
                    return String::repeatedString (String::charToString (textEditor.getPasswordCharacter()),
                                                   getTotalNumCharacters());

                return textEditor.getTextInRange (r);
        */
    }
    
    fn set_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            textEditor.setText (newText);
        */
    }
    
    fn get_text_insertion_offset(&self) -> i32 {
        
        todo!();
        /*
            return textEditor.getCaretPosition();
        */
    }
    
    fn get_text_bounds(&self, text_range: Range<i32>) -> RectangleList<i32> {
        
        todo!();
        /*
            auto localRects = textEditor.getTextBounds (textRange);
                RectangleList<int> globalRects;

                std::for_each (localRects.begin(), localRects.end(),
                               [&] (const Rectangle<int>& r) { globalRects.add (textEditor.localAreaToGlobal (r)); });

                return globalRects;
        */
    }
    
    fn get_offset_at_point(&self, point: Point<i32>) -> i32 {
        
        todo!();
        /*
            auto localPoint = textEditor.getLocalPoint (nullptr, point);
                return textEditor.getTextIndexAt (localPoint.x, localPoint.y);
        */
    }
}
