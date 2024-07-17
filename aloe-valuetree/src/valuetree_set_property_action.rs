crate::ix!();

#[no_copy]
pub struct ValueTreeSharedObjectSetPropertyAction {
    target:                 ValueTreeSharedObjectPtr,
    name:                   Identifier,
    new_value:              Var,
    old_value:              Var,
    is_adding_new_property: bool,
    is_deleting_property:   bool,
    exclude_listener:       *mut dyn ValueTreeListener,
}

impl UndoableAction for ValueTreeSharedObjectSetPropertyAction {

    fn perform(&mut self) -> bool {
        
        todo!();
        /*
            jassert (! (isAddingNewProperty && target->hasProperty (name)));

                    if (isDeletingProperty)
                        target->removeProperty (name, nullptr);
                    else
                        target->setProperty (name, newValue, nullptr, excludeListener);

                    return true;
        */
    }
    
    fn undo(&mut self) -> bool {
        
        todo!();
        /*
            if (isAddingNewProperty)
                        target->removeProperty (name, nullptr);
                    else
                        target->setProperty (name, oldValue, nullptr);

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
            if (! (isAddingNewProperty || isDeletingProperty))
                    {
                        if (auto* next = dynamic_cast<ValueTreeSharedObjectSetPropertyAction*> (nextAction))
                            if (next->target == target && next->name == name
                                  && ! (next->isAddingNewProperty || next->isDeletingProperty))
                                return new ValueTreeSharedObjectSetPropertyAction (*target, name, next->newValue, oldValue, false, false);
                    }

                    return nullptr;
        */
    }
}

impl ValueTreeSharedObjectSetPropertyAction {

    pub fn new(
        target_object:       ValueTreeSharedObjectPtr,
        property_name:       &Identifier,
        new_val:             &Var,
        old_val:             &Var,
        is_adding:           bool,
        is_deleting:         bool,
        listener_to_exclude: *mut dyn ValueTreeListener

    ) -> Self {

        todo!();
        /*

            : target (std::move (targetObject)),
                      name (propertyName), newValue (newVal), oldValue (oldVal),
                      isAddingNewProperty (isAdding), isDeletingProperty (isDeleting),
                      excludeListener (listenerToExclude)
        */
    }
}
