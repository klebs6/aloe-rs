crate::ix!();

/**
  | Class for the start sub path element
  |
  */
#[no_copy]
pub struct RelativePointPathStartSubPath {
    base:      RelativePointPathElementBase,
    start_pos: RelativePoint,
}

impl RelativePointPathStartSubPath {
    
    pub fn new(pos: &RelativePoint) -> Self {
    
        todo!();
        /*
        : element_base(startSubPathElement),
        : start_pos(pos),
        */
    }
    
    pub fn add_to_path(&self, 
        path:  &mut Path,
        scope: *mut dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            path.startNewSubPath (startPos.resolve (scope));
        */
    }
    
    pub fn get_control_points(&mut self, num_points: &mut i32) -> *mut RelativePoint {
        
        todo!();
        /*
            numPoints = 1;
        return &startPos;
        */
    }
    
    pub fn clone(&self) -> *mut RelativePointPathElementBase {
        
        todo!();
        /*
            return new RelativePointPathStartSubPath (startPos);
        */
    }
}
