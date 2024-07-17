crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativePoint.h]

/**
  | An X-Y position stored as a pair of RelativeCoordinate
  | values.
  | 
  | @see RelativeCoordinate, RelativeRectangle
  | 
  | @tags{GUI}
  |
  */
pub struct RelativePoint {

    /**
      | The actual X and Y coords...
      |
      */
    x: RelativeCoordinate,
    y: RelativeCoordinate,
}

impl Default for RelativePoint {
    
    /**
      | Creates a point at the origin.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl PartialEq<RelativePoint> for RelativePoint {
    
    #[inline] fn eq(&self, other: &RelativePoint) -> bool {
        todo!();
        /*
            return x == other.x && y == other.y;
        */

    }
}

impl Eq for RelativePoint {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativePoint.cpp]
impl From<Point<f32>> for RelativePoint {

    /**
      | Creates an absolute point, relative
      | to the origin.
      |
      */
    fn from(absolute_point: Point<f32>) -> Self {
    
        todo!();
        /*
        : x(absolutePoint.x),
        : y(absolutePoint.y),
        */
    }
}

impl From<&str> for RelativePoint {

    /**
      | Creates a point from a stringified representation.
      | 
      | The string must contain a pair of coordinates,
      | separated by space or a comma. The syntax
      | for the coordinate strings is explained
      | in the RelativeCoordinate class. @see
      | toString
      |
      */
    fn from(s: &str) -> Self {
    
        todo!();
        /*


            String error;
        CharPointerType text (s.getCharPointer());
        x = RelativeCoordinate (Expression::parse (text, error));
        RelativePointHelpers::skipComma (text);
        y = RelativeCoordinate (Expression::parse (text, error));
        */
    }
}
    

impl RelativePoint {
    
    /**
      | Creates an absolute point, relative
      | to the origin.
      |
      */
    pub fn new_relative_to_origin(x: f32, y: f32) -> Self {
    
        todo!();
        /*
        : x(x_),
        : y(y_),

        
        */

    }
    
    /**
      | Creates an absolute point from two coordinates.
      |
      */
    pub fn new(
        x: &RelativeCoordinate,
        y: &RelativeCoordinate) -> Self {
    
        todo!();
        /*
        : x(x_),
        : y(y_),

        
        */

    }
    
    /**
      | Calculates the absolute position of
      | this point.
      | 
      | You'll need to provide a suitable ExpressionScope
      | for looking up any coordinates that
      | may be needed to calculate the result.
      |
      */
    pub fn resolve(&self, scope: *const dyn ExpressionScopeInterface) -> Point<f32> {
        
        todo!();
        /*
            return Point<float> ((float) x.resolve (scope),
                             (float) y.resolve (scope));
        */

    }
    
    /**
      | Changes the values of this point's coordinates
      | to make it resolve to the specified position.
      | 
      | Calling this will leave any anchor points
      | unchanged, but will set any absolute
      | or relative positions to whatever values
      | are necessary to make the resultant
      | position match the position that is
      | provided.
      |
      */
    pub fn move_to_absolute(
        &mut self, 
        new_pos: Point<f32>,
        scope:   *const dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            x.moveToAbsolute (newPos.x, scope);
        y.moveToAbsolute (newPos.y, scope);
        */

    }
    
    /**
      | Returns a string which represents this
      | point.
      | 
      | This returns a comma-separated pair
      | of coordinates. For details of the string
      | syntax used by the coordinates, see
      | the RelativeCoordinate constructor
      | notes.
      | 
      | The string that is returned can be passed
      | to the RelativePoint constructor to
      | recreate the point.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return x.toString() + ", " + y.toString();
        */

    }
    
    /**
      | Returns true if this point depends on
      | any other coordinates for its position.
      |
      */
    pub fn is_dynamic(&self) -> bool {
        
        todo!();
        /*
            return x.isDynamic() || y.isDynamic();
        */

    }
}

#[inline] pub fn relative_point_skip_comma(s: &mut CharPointerType)  {
    
    todo!();
        /*
            s.incrementToEndOfWhitespace();

            if (*s == ',')
                ++s;
        */

}
