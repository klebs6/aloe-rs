crate::ix!();

/**
  | Class for the line to element
  |
  */
#[no_copy]
pub struct RelativePointPathLineTo {
    base:      RelativePointPathElementBase,
    end_point: RelativePoint,
}

impl RelativePointPathLineTo {

    pub fn new(end_point: &RelativePoint) -> Self {
    
        todo!();
        /*
        : element_base(lineToElement),
        : end_point(endPoint_),
        */
    }
    
    pub fn add_to_path(&self, 
        path:  &mut Path,
        scope: *mut dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            path.lineTo (endPoint.resolve (scope));
        */

    }
    
    pub fn get_control_points(&mut self, num_points: &mut i32) -> *mut RelativePoint {
        
        todo!();
        /*
            numPoints = 1;
        return &endPoint;
        */

    }
    
    pub fn clone(&self) -> *mut RelativePointPathElementBase {
        
        todo!();
        /*
            return new RelativePointPathLineTo (endPoint);
        */

    }
}
