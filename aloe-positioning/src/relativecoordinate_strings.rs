crate::ix!();

/**
  | A set of static strings that are commonly
  | used by the RelativeCoordinate class.
  | 
  | As well as avoiding using string literals
  | in your code, using these preset values
  | has the advantage that all instances
  | of the same string will share the same,
  | reference-counted
  | 
  | String object, so if you have thousands
  | of points which all refer to the same
  | anchor points, this can save a significant
  | amount of memory allocation.
  |
  */
pub mod relative_coordinate_strings {
    use super::*;

    pub const parent: &'static str = "parent" ;
    pub const left:   &'static str = "left" ;
    pub const right:  &'static str = "right" ;
    pub const top:    &'static str = "top" ;
    pub const bottom: &'static str = "bottom" ;
    pub const x:      &'static str = "x" ;
    pub const y:      &'static str = "y" ;
    pub const width:  &'static str = "width" ;
    pub const height: &'static str = "height" ;
}

pub enum RelativeCoordinateStandardStringsType
{
    left, 
    right, 
    top, 
    bottom,
    x, 
    y, 
    width, 
    height,
    parent,
    unknown
}

pub fn get_type_of(s: &str) -> RelativeCoordinateStandardStringsType {
    
    todo!();
        /*
        if (s == Strings::left)    return left;
        if (s == Strings::right)   return right;
        if (s == Strings::top)     return top;
        if (s == Strings::bottom)  return bottom;
        if (s == Strings::x)       return x;
        if (s == Strings::y)       return y;
        if (s == Strings::width)   return width;
        if (s == Strings::height)  return height;
        if (s == Strings::parent)  return parent;
        return unknown;
        */
}
