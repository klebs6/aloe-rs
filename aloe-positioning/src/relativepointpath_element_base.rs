crate::ix!();

/**
  | Base class for the elements that make
  | up a RelativePointPath.
  |
  */
#[no_copy]
pub struct RelativePointPathElementBase {
    ty: RelativePointPathElementType,
}

impl RelativePointPathElementBase {

    pub fn new(ty: RelativePointPathElementType) -> Self {
    
        todo!();
        /*
        : ty(type_),
        */
    }
    
    pub fn is_dynamic(&mut self) -> bool {
        
        todo!();
        /*
            int numPoints;
        const RelativePoint* const points = getControlPoints (numPoints);

        for (int i = numPoints; --i >= 0;)
            if (points[i].isDynamic())
                return true;

        return false;
        */

    }
}
