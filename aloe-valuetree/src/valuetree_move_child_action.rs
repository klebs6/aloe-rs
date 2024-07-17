crate::ix!();

///--------------------------
#[no_copy]
pub struct ValueTreeSharedObjectMoveChildAction {
    parent:      ValueTreeSharedObjectPtr,
    start_index: i32,
    end_index:   i32,
}

impl UndoableAction for ValueTreeSharedObjectMoveChildAction {

    fn perform(&mut self) -> bool {
        
        todo!();
        /*
            parent->moveChild (startIndex, endIndex, nullptr);
                    return true;
        */
    }
    
    fn undo(&mut self) -> bool {
        
        todo!();
        /*
            parent->moveChild (endIndex, startIndex, nullptr);
                    return true;
        */
    }
    
    fn get_size_in_units(&mut self) -> i32 {
        
        todo!();
        /*
            return (int) sizeof (*this); //xxx should be more accurate
        */
    }
    
    fn create_coalesced_action(&mut self, next_action: *mut dyn UndoableAction) -> *mut dyn UndoableAction {
        
        todo!();
        /*
            if (auto* next = dynamic_cast<ValueTreeSharedObjectMoveChildAction*> (nextAction))
                        if (next->parent == parent && next->startIndex == endIndex)
                            return new ValueTreeSharedObjectMoveChildAction (parent, startIndex, next->endIndex);

                    return nullptr;
        */
    }
}

impl ValueTreeSharedObjectMoveChildAction {

    pub fn new(
        parent_object: ValueTreeSharedObjectPtr,
        from_index:    i32,
        to_index:      i32) -> Self {
    
        todo!();
        /*

            : parent (std::move (parentObject)), startIndex (fromIndex), endIndex (toIndex)
        */
    }
}
