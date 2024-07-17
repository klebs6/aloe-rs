crate::ix!();

#[no_copy]
pub struct TextEditorInsertAction<'a> {
    owner:         &'a mut TextEditor<'a>,
    text:          String,
    insert_index:  i32,
    old_caret_pos: i32,
    new_caret_pos: i32,
    font:          Font,
    colour:        Colour,
}

impl<'a> UndoableAction for TextEditorInsertAction<'a> {

    fn perform(&mut self) -> bool {
        
        todo!();
        /*
            owner.insert (text, insertIndex, font, colour, nullptr, newCaretPos);
            return true;
        */
    }
    
    fn undo(&mut self) -> bool {
        
        todo!();
        /*
            owner.remove ({ insertIndex, insertIndex + text.length() }, nullptr, oldCaretPos);
            return true;
        */
    }
    
    fn get_size_in_units(&mut self) -> i32 {
        
        todo!();
        /*
            return text.length() + 16;
        */
    }
}

impl<'a> TextEditorInsertAction<'a> {

    pub fn new(
        ed:         &mut TextEditor,
        new_text:   &String,
        insert_pos: i32,
        new_font:   &Font,
        new_colour: Colour,
        old_caret:  i32,
        new_caret:  i32) -> Self {
    
        todo!();
        /*
        : owner(ed),
        : text(newText),
        : insert_index(insertPos),
        : old_caret_pos(oldCaret),
        : new_caret_pos(newCaret),
        : font(newFont),
        : colour(newColour),

        
        */
    }
}
