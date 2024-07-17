crate::ix!();

#[no_copy]
pub struct CodeDocumentDeleteAction<'a> {
    owner:        &'a mut CodeDocument<'a>,
    start_pos:    i32,
    end_pos:      i32,
    removed_text: String,
}

impl<'a> UndoableAction for CodeDocumentDeleteAction<'a> {

    fn perform(&mut self) -> bool {
        
        todo!();
        /*
            owner.currentActionIndex++;
            owner.remove (startPos, endPos, false);
            return true;
        */
    }
    
    fn undo(&mut self) -> bool {
        
        todo!();
        /*
            owner.currentActionIndex--;
            owner.insert (removedText, startPos, false);
            return true;
        */
    }
}

impl<'a> CodeDocumentDeleteAction<'a> {

    pub fn new(
        doc:   &mut CodeDocument,
        start: i32,
        end:   i32) -> Self {
    
        todo!();
        /*


            : owner (doc), startPos (start), endPos (end),
              removedText (doc.getTextBetween (CodeDocument::CodeDocumentPosition (doc, start),
                                               CodeDocument::CodeDocumentPosition (doc, end)))
        */
    }
    
    pub fn get_size_in_units(&mut self) -> i32 {
        
        todo!();
        /*
            return (endPos - startPos) + 32;
        */
    }
}

