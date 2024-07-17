crate::ix!();

#[no_copy]
pub struct ValueTreeSharedObjectAddOrRemoveChildAction {
    target:      ValueTreeSharedObjectPtr,
    child:       ValueTreeSharedObjectPtr,
    child_index: i32,
    is_deleting: bool,
}

impl UndoableAction for ValueTreeSharedObjectAddOrRemoveChildAction {

    fn perform(&mut self) -> bool {
        
        todo!();
        /*
            if (isDeleting)
                        target->removeChild (childIndex, nullptr);
                    else
                        target->addChild (child.get(), childIndex, nullptr);

                    return true;
        */
    }

    fn undo(&mut self) -> bool {
        
        todo!();
        /*
            if (isDeleting)
                    {
                        target->addChild (child.get(), childIndex, nullptr);
                    }
                    else
                    {
                        // If you hit this, it seems that your object's state is getting confused - probably
                        // because you've interleaved some undoable and non-undoable operations?
                        jassert (childIndex < target->children.size());
                        target->removeChild (childIndex, nullptr);
                    }

                    return true;
        */
    }

    fn get_size_in_units(&mut self) -> i32 {
        
        todo!();
        /*
            return (int) sizeof (*this); //xxx should be more accurate
        */
    }
}

impl ValueTreeSharedObjectAddOrRemoveChildAction {

    pub fn new(
        parent_object: ValueTreeSharedObjectPtr,
        index:         i32,
        new_child:     *mut ValueTreeSharedObject) -> Self {
    
        todo!();
        /*

            : target (std::move (parentObject)),
                      child (newChild != nullptr ? newChild : target->children.getObjectPointer (index)),
                      childIndex (index),
                      isDeleting (newChild == nullptr)

                    jassert (child != nullptr);
        */
    }
}
