crate::ix!();

/**
  | Class for the close sub path element
  |
  */
#[no_copy]
pub struct RelativePointPathCloseSubPath {
    base: RelativePointPathElementBase,
}

impl RelativePointPathCloseSubPath {
    
    pub fn new() -> Self {
    
        todo!();
        /*
        : element_base(closeSubPathElement),

        
        */

    }
    
    pub fn add_to_path(
        &self, 
        path: &mut Path,
        _1:   *mut dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            path.closeSubPath();
        */

    }
    
    pub fn get_control_points(&mut self, num_points: &mut i32) -> *mut RelativePoint {
        
        todo!();
        /*
            numPoints = 0;
        return nullptr;
        */

    }
    
    pub fn clone(&self) -> *mut RelativePointPathElementBase {
        
        todo!();
        /*
            return new RelativePointPathCloseSubPath();
        */

    }
}
