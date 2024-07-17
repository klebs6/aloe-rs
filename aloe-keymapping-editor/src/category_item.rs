crate::ix!();

#[no_copy]
#[leak_detector]
pub struct KeyMappingEditorComponentCategoryItem<'a> {
    base:          TreeViewItem<'a>,
    owner:         &'a mut KeyMappingEditorComponent<'a>,
    category_name: String,
}

impl<'a> KeyMappingEditorComponentCategoryItem<'a> {

    pub fn new(
        kec:  &mut KeyMappingEditorComponent,
        name: &String) -> Self {
    
        todo!();
        /*
        : owner(kec),
        : category_name(name),

        
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            return categoryName + "_cat";
        */
    }
    
    pub fn might_contain_sub_items(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_item_height(&self) -> i32 {
        
        todo!();
        /*
            return 22;
        */
    }
    
    pub fn get_accessibility_name(&mut self) -> String {
        
        todo!();
        /*
            return categoryName;
        */
    }
    
    pub fn paint_item(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            g.setFont (Font ((float) height * 0.7f, Font::bold));
            g.setColour (owner.findColour (KeyMappingEditorComponent::textColourId));

            g.drawText (TRANS (categoryName), 2, 0, width - 2, height, Justification::centredLeft, true);
        */
    }
    
    pub fn item_openness_changed(&mut self, is_now_open: bool)  {
        
        todo!();
        /*
            if (isNowOpen)
            {
                if (getNumSubItems() == 0)
                    for (auto command : owner.getCommandManager().getCommandsInCategory (categoryName))
                        if (owner.shouldCommandBeIncluded (command))
                            addSubItem (new KeyMappingEditorComponentMappingItem (owner, command));
            }
            else
            {
                clearSubItems();
            }
        */
    }
}
