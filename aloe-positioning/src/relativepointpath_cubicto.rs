crate::ix!();

/**
  | Class for the cubic to element
  |
  */
#[no_copy]
pub struct RelativePointPathCubicTo {
    base:           RelativePointPathElementBase,
    control_points: [RelativePoint; 3],
}

impl RelativePointPathCubicTo {
    
    pub fn new(
        control_point1: &RelativePoint,
        control_point2: &RelativePoint,
        end_point:      &RelativePoint) -> Self {
    
        todo!();
        /*
        : element_base(cubicToElement),

            controlPoints[0] = controlPoint1;
        controlPoints[1] = controlPoint2;
        controlPoints[2] = endPoint;
        */
    }
    
    pub fn add_to_path(
        &self, 
        path:  &mut Path,
        scope: *mut dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            path.cubicTo (controlPoints[0].resolve (scope),
                      controlPoints[1].resolve (scope),
                      controlPoints[2].resolve (scope));
        */
    }
    
    pub fn get_control_points(&mut self, num_points: &mut i32) -> *mut RelativePoint {
        
        todo!();
        /*
            numPoints = 3;
        return controlPoints;
        */

    }
    
    pub fn clone(&self) -> *mut RelativePointPathElementBase {
        
        todo!();
        /*
            return new RelativePointPathCubicTo (controlPoints[0], controlPoints[1], controlPoints[2]);
        */

    }
}
