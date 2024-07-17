crate::ix!();

///--------------------------------
#[no_copy]
#[leak_detector]
pub struct ValueTreePropertyValueSource<'a> {
    base:                 ValueSource<'a>,
    tree:                 ValueTree,
    property:             Identifier,
    undo_manager:         *const UndoManager<'a>,
    update_synchronously: bool,
}

impl<'a> ValueTreeListener for ValueTreePropertyValueSource<'a> {

    fn value_tree_property_changed(&mut self, 
        changed_tree:     &mut ValueTree,
        changed_property: &Identifier)  {
        
        todo!();
        /*
            if (tree == changedTree && property == changedProperty)
                sendChangeMessage (updateSynchronously);
        */
    }
    
    fn value_tree_child_added(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree)  {
        
        todo!();
        /*
        
        */
    }
    
    fn value_tree_child_removed(&mut self, 
        _0: &mut ValueTree,
        _1: &mut ValueTree,
        _2: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    fn value_tree_child_order_changed(&mut self, 
        _0: &mut ValueTree,
        _1: i32,
        _2: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    fn value_tree_parent_changed(&mut self, _0: &mut ValueTree)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for ValueTreePropertyValueSource<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            tree.removeListener (this);
         */
    }
}

impl<'a> ValueTreePropertyValueSource<'a> {

    pub fn new(
        vt:   &ValueTree,
        prop: &Identifier,
        um:   *mut UndoManager,
        sync: bool) -> Self {
    
        todo!();
        /*
            : tree (vt), property (prop), undoManager (um), updateSynchronously (sync)
            tree.addListener (this);
        */
    }
    
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            return tree[property];
        */
    }
    
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            tree.setProperty (property, newValue, undoManager);
        */
    }
    
}
