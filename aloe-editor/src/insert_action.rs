crate::ix!();

#[no_copy]
pub struct CodeDocumentInsertAction<'a> {
    owner:      &'a mut CodeDocument<'a>,
    text:       String,
    insert_pos: i32,
}

impl<'a> UndoableAction for CodeDocumentInsertAction<'a> {

    fn perform(&mut self) -> bool {
        
        todo!();
        /*
            owner.currentActionIndex++;
            owner.insert (text, insertPos, false);
            return true;
        */
    }
    
    fn undo(&mut self) -> bool {
        
        todo!();
        /*
            owner.currentActionIndex--;
            owner.remove (insertPos, insertPos + text.length(), false);
            return true;
        */
    }
}

impl<'a> CodeDocumentInsertAction<'a> {

    pub fn new(
        doc: &mut CodeDocument,
        t:   &String,
        pos: i32) -> Self {
    
        todo!();
        /*
        : owner(doc),
        : text(t),
        : insert_pos(pos),

        
        */
    }
    
    pub fn get_size_in_units(&mut self) -> i32 {
        
        todo!();
        /*
            return text.length() + 32;
        */
    }
}

