crate::ix!();

/**
  | The types of element that may be contained
  | in this path. @see RelativePointPath::RelativePointPathElementBase
  |
  */
pub enum RelativePointPathElementType
{
    nullElement,
    startSubPathElement,
    closeSubPathElement,
    lineToElement,
    quadraticToElement,
    cubicToElement
}

pub trait RelativePointPathElementBaseInterface {

    fn add_to_path(
        &self, 
        path: &mut Path,
        _1:   *mut dyn ExpressionScopeInterface
    );

    fn get_control_points(&mut self, num_points: &mut i32) 
        -> *mut RelativePoint;

    fn clone(&self) -> *mut RelativePointPathElementBase;
}
