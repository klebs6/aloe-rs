crate::ix!();

#[no_copy]
pub struct TextEditorRemoveAction<'a> {
    owner:            &'a mut TextEditor<'a>,
    range:            Range<i32>,
    old_caret_pos:    i32,
    new_caret_pos:    i32,
    removed_sections: Vec<TextEditorUniformTextSection>,
}

impl<'a> UndoableAction for TextEditorRemoveAction<'a> {

    fn perform(&mut self) -> bool {
        
        todo!();
        /*
            owner.remove (range, nullptr, newCaretPos);
            return true;
        */
    }
    
    fn undo(&mut self) -> bool {
        
        todo!();
        /*
            owner.reinsert (range.getStart(), removedSections);
            owner.moveCaretTo (oldCaretPos, false);
            return true;
        */
    }
    
    fn get_size_in_units(&mut self) -> i32 {
        
        todo!();
        /*
            int n = 16;

            for (auto* s : removedSections)
                n += s->getTotalLength();

            return n;
        */
    }
}

impl<'a> TextEditorRemoveAction<'a> {
    
    pub fn new(
        ed:              &mut TextEditor,
        range_to_remove: Range<i32>,
        old_caret:       i32,
        new_caret:       i32,
        old_sections:    &[*mut TextEditorUniformTextSection]) -> Self {
    
        todo!();
        /*
        : owner(ed),
        : range(rangeToRemove),
        : old_caret_pos(oldCaret),
        : new_caret_pos(newCaret),

            removedSections.addArray (oldSections);
        */
    }
}
