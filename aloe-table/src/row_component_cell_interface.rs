crate::ix!();

pub struct RowComponentCellInterface<'a> {
    owner: &'a mut RowAccessibilityHandler<'a>,
}

impl<'a> AccessibilityCellInterface for RowComponentCellInterface<'a> {

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
            return owner.rowComponent.row;
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
            return 0;
        */
    }
    
    fn get_table_handler(&self) -> *const AccessibilityHandler {
        
        todo!();
        /*
            return owner.rowComponent.owner.getAccessibilityHandler();
        */
    }
}

impl<'a> RowComponentCellInterface<'a> {

    pub fn new(handler: &mut RowAccessibilityHandler) -> Self {
    
        todo!();
        /*
        : owner(handler),

        
        */
    }
    
}
