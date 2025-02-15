crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CodeEditorComponentImpl<'a> {
    base:  Timer,
    base2: AsyncUpdater<'a>,
    owner: CodeEditorComponent<'a>,
}

impl<'a> ScrollBarListener for CodeEditorComponentImpl<'a> {

    fn scroll_bar_moved(&mut self, 
        scroll_bar_that_has_moved: *mut ScrollBar,
        new_range_start:           f64)  {
        
        todo!();
        /*
            if (scrollBarThatHasMoved->isVertical())
                owner.scrollToLineInternal ((int) newRangeStart);
            else
                owner.scrollToColumnInternal (newRangeStart);
        */
    }
}

impl<'a> CodeDocumentListener for CodeEditorComponentImpl<'a> {

    fn code_document_text_inserted(
        &mut self, 
        new_text: &str,
        pos:      i32
    ) {
        
        todo!();
        /*
            owner.codeDocumentChanged (pos, pos + newText.length());
        */
    }
    
    fn code_document_text_deleted(
        &mut self, 
        start: i32,
        end:   i32
    ) {
        
        todo!();
        /*
            owner.codeDocumentChanged (start, end);
        */
    }
}

impl<'a> CodeEditorComponentImpl<'a> {

    pub fn new(ed: &mut CodeEditorComponent) -> Self {
    
        todo!();
        /*
        : owner(ed),
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            owner.newTransaction();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            owner.rebuildLineTokens();
        */
    }
}
