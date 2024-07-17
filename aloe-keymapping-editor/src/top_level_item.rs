crate::ix!();

pub struct KeyMappingEditorComponentTopLevelItem<'a> {
    base:  TreeViewItem<'a>,
    owner: &'a mut KeyMappingEditorComponent<'a>,
}

impl<'a> ChangeListener for KeyMappingEditorComponentTopLevelItem<'a> { 

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            const OpennessRestorer opennessRestorer (*this);
            clearSubItems();

            for (auto category : owner.getCommandManager().getCommandCategories())
            {
                int count = 0;

                for (auto command : owner.getCommandManager().getCommandsInCategory (category))
                    if (owner.shouldCommandBeIncluded (command))
                        ++count;

                if (count > 0)
                    addSubItem (new KeyMappingEditorComponentCategoryItem (owner, category));
            }
        */
    }
}

impl<'a> Drop for KeyMappingEditorComponentTopLevelItem<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            owner.getMappings().removeChangeListener (this);
        */
    }
}

impl<'a> KeyMappingEditorComponentTopLevelItem<'a> {

    pub fn new(kec: &mut KeyMappingEditorComponent) -> Self {
    
        todo!();
        /*
        : owner(kec),

            setLinesDrawnForSubItems (false);
            owner.getMappings().addChangeListener (this);
        */
    }
    
    pub fn might_contain_sub_items(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            return "keys";
        */
    }
}
