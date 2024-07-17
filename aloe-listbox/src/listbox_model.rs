crate::ix!();

/**
  | A subclass of this is used to drive a ListBox.
  | 
  | @see ListBox
  | 
  | @tags{GUI}
  |
  */
#[derive(Default)]
pub struct ListBoxModel {

}

impl ListBoxModel {
    
    pub fn refresh_component_for_row(&mut self, 
        _0:                           i32,
        _1:                           bool,
        existing_component_to_update: *mut Component) -> *mut Component {
        
        todo!();
        /*
            ignoreUnused (existingComponentToUpdate);
        jassert (existingComponentToUpdate == nullptr); // indicates a failure in the code that recycles the components
        return nullptr;
        */
    }
    
    pub fn get_name_for_row(&mut self, row_number: i32) -> String {
        
        todo!();
        /*
            return "Row " + String (rowNumber + 1);
        */
    }
    
    pub fn list_box_item_clicked(&mut self, 
        _0: i32,
        _1: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn list_box_item_double_clicked(&mut self, 
        _0: i32,
        _1: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn background_clicked(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn selected_rows_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn delete_key_pressed(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn return_key_pressed(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn list_was_scrolled(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_drag_source_description(&mut self, _0: &SparseSet<i32>) -> Var {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_tooltip_for_row(&mut self, _0: i32) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_mouse_cursor_for_row(&mut self, _0: i32) -> MouseCursor {
        
        todo!();
        /*
            return MouseCursor::NormalCursor;
        */
    }
}
