crate::ix!();

/**
  | Class for the quadratic to element
  |
  */
#[no_copy]
pub struct RelativePointPathQuadraticTo {
    base:            RelativePointPathElementBase,
    control_points: [RelativePoint; 2],
}

impl RelativePointPathQuadraticTo {
    
    pub fn new(
        control_point: &RelativePoint,
        end_point:     &RelativePoint) -> Self {
    
        todo!();
        /*
        : element_base(quadraticToElement),

            controlPoints[0] = controlPoint;
        controlPoints[1] = endPoint;
        */

    }
    
    pub fn add_to_path(&self, 
        path:  &mut Path,
        scope: *mut dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            path.quadraticTo (controlPoints[0].resolve (scope),
                          controlPoints[1].resolve (scope));
        */

    }
    
    pub fn get_control_points(&mut self, num_points: &mut i32) -> *mut RelativePoint {
        
        todo!();
        /*
            numPoints = 2;
        return controlPoints;
        */

    }
    
    pub fn clone(&self) -> *mut RelativePointPathElementBase {
        
        todo!();
        /*
            return new RelativePointPathQuadraticTo (controlPoints[0], controlPoints[1]);
        */

    }
}
