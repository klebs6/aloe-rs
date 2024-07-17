crate::ix!();

pub struct TreeViewItemComponentItemAccessibilityHandlerItemCellInterface<'a> {
    item_component: &'a mut TreeViewItemComponent<'a>,
}

impl<'a> AccessibilityCellInterface for TreeViewItemComponentItemAccessibilityHandlerItemCellInterface<'a> {

    fn get_column_index(&self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    fn get_column_span(&self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    fn get_row_index(&self) -> i32 {
        
        todo!();
        /*
            return itemComponent.getRepresentedItem().getRowNumberInTree();
        */
    }
    
    fn get_row_span(&self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    fn get_disclosure_level(&self) -> i32 {
        
        todo!();
        /*
            return getItemDepth (&itemComponent.getRepresentedItem());
        */
    }
    
    fn get_table_handler(&self) -> *const AccessibilityHandler {
        
        todo!();
        /*
            if (auto* tree = itemComponent.getRepresentedItem().getOwnerView())
                        return tree->getAccessibilityHandler();

                    return nullptr;
        */
    }
}

impl<'a> TreeViewItemComponentItemAccessibilityHandlerItemCellInterface<'a> {

    pub fn new(c: &mut TreeViewItemComponent) -> Self {
    
        todo!();
        /*
        : item_component(c),

        
        */
    }
}
