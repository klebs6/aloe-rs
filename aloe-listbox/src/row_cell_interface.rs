crate::ix!();

pub struct RowCellInterface<'a> {
    handler: &'a mut RowAccessibilityHandler<'a>,
}

impl<'a> AccessibilityCellInterface for RowCellInterface<'a> {

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
            const auto index = handler.rowComponent.row;

                    if (handler.rowComponent.owner.hasAccessibleHeaderComponent())
                        return index + 1;

                    return index;
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
            return handler.rowComponent.owner.getAccessibilityHandler();
        */
    }
}

impl<'a> RowCellInterface<'a> {

    pub fn new(h: &mut RowAccessibilityHandler) -> Self {
    
        todo!();
        /*
        : handler(h),

        
        */
    }
}
