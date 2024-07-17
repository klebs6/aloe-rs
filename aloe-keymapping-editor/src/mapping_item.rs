crate::ix!();

#[no_copy]
#[leak_detector]
pub struct KeyMappingEditorComponentMappingItem<'a> {
    base:      TreeViewItem<'a>,
    owner:     &'a mut KeyMappingEditorComponent<'a>,
    commandid: CommandID,
}

impl<'a> KeyMappingEditorComponentMappingItem<'a> {

    pub fn new(
        kec:     &mut KeyMappingEditorComponent,
        command: CommandID) -> Self {
    
        todo!();
        /*
        : owner(kec),
        : commandid(command),

        
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            return String ((int) commandID) + "_id";
        */
    }
    
    pub fn might_contain_sub_items(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_item_height(&self) -> i32 {
        
        todo!();
        /*
            return 20;
        */
    }
    
    pub fn create_item_component(&mut self) -> Box<Component<'a>> {
        
        todo!();
        /*
            return std::make_unique<KeyMappingEditorComponentItemComponent> (owner, commandID);
        */
    }
    
    pub fn get_accessibility_name(&mut self) -> String {
        
        todo!();
        /*
            return TRANS (owner.getCommandManager().getNameOfCommand (commandID));
        */
    }
}
