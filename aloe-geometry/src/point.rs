crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_Point.h]

/**
  | A pair of (x, y) coordinates.
  | 
  | The ValueType template should be a primitive
  | type such as int, float, double, rather
  | than a class.
  | 
  | @see Line, Path, AffineTransform
  | 
  | @tags{Graphics}
  |
  */
#[derive(Copy,Clone)]
pub struct Point<ValueType> {

    /**
      | The point's X coordinate.
      |
      */
    x: ValueType,

    /**
      | The point's Y coordinate.
      |
      */
    y: ValueType,
}

impl<ValueType> Default for Point<ValueType> {
    
    /**
      | Creates a point at the origin
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl<ValueType> PartialEq<Point<ValueType>> for Point<ValueType> {
    
    fn eq(&self, other: &Point<ValueType>) -> bool {
        todo!();
        /*
            return x == other.x && y == other.y;
        */
    }
}

impl<ValueType> Eq for Point<ValueType> {}

impl<ValueType> Add<Point<ValueType>> for Point<ValueType> {

    type Output = Point<ValueType>;
    
    /**
      | Adds two points together
      |
      */
    #[inline]fn add(self, other: Point<ValueType>) -> Self::Output {
        todo!();
        /*
            return Point (x + other.x, y + other.y);
        */
    }
}

impl<ValueType> Point<ValueType> {

    /**
      | Creates a point from an (x, y) position.
      |
      */
    pub const fn new(
        initialx: ValueType,
        initialy: ValueType

    ) -> Self {

        Self {
            x: initialx,
            y: initialy,
        }
    }

    /**
      | Copies this point from another one.
      |
      */
    pub fn assign_from(&mut self, _0: &Point<ValueType>) -> &mut Point<ValueType> {
        
        todo!();
        /*
        
        */
    }
    /**
      | Returns true if the point is (0, 0).
      |
      */
    pub fn is_origin(&self) -> bool {
        
        todo!();
        /*
            return x == ValueType() && y == ValueType();
        */
    }

    /**
      | Returns true if the coordinates are
      | finite values.
      |
      */
    #[inline] pub fn is_finite(&self) -> bool {
        
        todo!();
        /*
            return aloe_isfinite(x) && aloe_isfinite(y);
        */
    }

    /**
      | Returns the point's x coordinate.
      |
      */
    #[inline] pub fn getx(&self) -> ValueType {
        
        todo!();
        /*
            return x;
        */
    }

    /**
      | Returns the point's y coordinate.
      |
      */
    #[inline] pub fn gety(&self) -> ValueType {
        
        todo!();
        /*
            return y;
        */
    }

    /**
      | Sets the point's x coordinate.
      |
      */
    #[inline] pub fn setx(&mut self, newx: ValueType)  {
        
        todo!();
        /*
            x = newX;
        */
    }

    /**
      | Sets the point's y coordinate.
      |
      */
    #[inline] pub fn sety(&mut self, newy: ValueType)  {
        
        todo!();
        /*
            y = newY;
        */
    }

    /**
      | Returns a point which has the same Y position
      | as this one, but a new X.
      |
      */
    pub fn withx(&self, newx: ValueType) -> Point<ValueType> {
        
        todo!();
        /*
            return Point (newX, y);
        */
    }

    /**
      | Returns a point which has the same X position
      | as this one, but a new Y.
      |
      */
    pub fn withy(&self, newy: ValueType) -> Point<ValueType> {
        
        todo!();
        /*
            return Point (x, newY);
        */
    }

    /**
      | Changes the point's x and y coordinates.
      |
      */
    pub fn setxy(&mut self, 
        newx: ValueType,
        newy: ValueType)  {
        
        todo!();
        /*
            x = newX; y = newY;
        */
    }

    /**
      | Adds a pair of coordinates to this value.
      |
      */
    pub fn addxy(&mut self, 
        x_to_add: ValueType,
        y_to_add: ValueType)  {
        
        todo!();
        /*
            x += xToAdd; y += yToAdd;
        */
    }

    
    /**
      | Returns a point with a given offset from
      | this one.
      |
      */
    pub fn translated(&self, 
        deltax: ValueType,
        deltay: ValueType) -> Point<ValueType> {
        
        todo!();
        /*
            return Point (x + deltaX, y + deltaY);
        */
    }
}

impl<ValueType> AddAssign<Point<ValueType>> for Point<ValueType> {
    
    /**
      | Adds another point's coordinates to
      | this one
      |
      */
    #[inline]fn add_assign(&mut self, other: Point<ValueType>) {

        todo!();

        /*
            x += other.x; y += other.y; return *this;
        */
    }
}

impl<ValueType> Sub<Point<ValueType>> for Point<ValueType> {

    type Output = Point<ValueType>;
    
    /**
      | Subtracts one points from another
      |
      */
    #[inline]fn sub(self, other: Point<ValueType>) -> Self::Output {
        todo!();
        /*
            return Point (x - other.x, y - other.y);
        */
    }
}

impl<ValueType> SubAssign<Point<ValueType>> for Point<ValueType> {
    
    /**
      | Subtracts another point's coordinates
      | to this one
      |
      */
    #[inline]fn sub_assign(&mut self, other: Point<ValueType>) {

        todo!();

        /*
            x -= other.x; y -= other.y; return *this;
        */
    }
}

impl<ValueType,OtherType> MulAssign<Point<OtherType>> for Point<ValueType> {
    
    /**
      | Multiplies another point's coordinates
      | to this one
      |
      */
    #[inline] fn mul_assign(&mut self, other: Point<OtherType>) {
        todo!();
        /*
            *this = *this * other; return *this;
        */
    }
}

impl<ValueType, FloatType> MulAssign<FloatType> for Point<ValueType>
where
    FloatType: Float + Into<ValueType> + Copy,
    ValueType: MulAssign<FloatType> + Copy,
{

    /**
      | Multiplies the point's coordinates
      | by a scalar value.
      |
      */
    fn mul_assign(&mut self, multiplier: FloatType) {
        self.x *= multiplier;
        self.y *= multiplier;
    }
}

impl<ValueType, OtherType, ThirdType> Div<Point<OtherType>> for Point<ValueType>
where
    ValueType: Div<OtherType, Output = ThirdType> + Copy,
    OtherType: Copy,
{
    type Output = Point<ThirdType>;

    /**
      | Divides one point by another
      |
      */
    fn div(self, other: Point<OtherType>) -> Self::Output {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<ValueType, OtherType> Div<OtherType> for Point<ValueType>
where
    ValueType: Div<OtherType, Output = ValueType> + Copy,
    OtherType: Copy + Into<ValueType> + Num,
{
    type Output = Point<ValueType>;

    /**
      | Returns a point whose coordinates are
      | divided by a given scalar value.
      |
      */
    fn div(self, divisor: OtherType) -> Self::Output {

        Point {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }
}


impl<ValueType, OtherType> DivAssign<Point<OtherType>> for Point<ValueType>
where
    ValueType: DivAssign<OtherType> + Copy,
    OtherType: Copy,
{

    /**
      | Divides this point's coordinates by
      | another
      |
      */
    fn div_assign(&mut self, other: Point<OtherType>) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl<ValueType, FloatType> DivAssign<FloatType> for Point<ValueType>
where
    FloatType: Float + Into<ValueType> + Copy,
    ValueType: DivAssign<FloatType> + Copy,
{
    /**
      | Divides the point's coordinates by
      | a scalar value.
      |
      */
    fn div_assign(&mut self, divisor: FloatType) {
        self.x /= divisor;
        self.y /= divisor;
    }
}

impl<ValueType, OtherType, ThirdType> Mul<&OtherType> for Point<ValueType>
where
    ValueType: Mul<OtherType, Output = ThirdType> + Copy,
    OtherType: Copy + Num,
{
    type Output = Point<ThirdType>;

    /**
      | Returns a point whose coordinates are
      | multiplied by a given scalar value.
      |
      */
    fn mul(self, other: &OtherType) -> Self::Output {
        Point {
            x: self.x * *other,
            y: self.y * *other,
        }
    }
}

impl<ValueType, OtherType, ThirdType> Mul<&Point<OtherType>> for Point<ValueType>
where
    ValueType: Mul<OtherType, Output = ThirdType> + Copy,
    OtherType: Copy,
{
    type Output = Point<ThirdType>;

    /**
      | Multiplies two points together
      |
      */
    fn mul(self, other: &Point<OtherType>) -> Self::Output {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<ValueType> Neg for Point<ValueType> {

    type Output = Self;
    
    /**
      | Returns the inverse of this point.
      |
      */
    #[inline] fn neg(self) -> Self::Output {
        todo!();
        /*
            return Point (-x, -y);
        */
    }
}
    
impl<ValueType: SmallestFloatType> HasFloatType for Point<ValueType> {

    /**
      | This type will be double if the Point's
      | type is double, otherwise it will be
      | float.
      |
      */
    type FloatType = <ValueType as SmallestFloatType>::Type;
}

impl<ValueType> Point<ValueType> {

    /**
      | Returns the straight-line distance
      | between this point and the origin.
      |
      */
    pub fn get_distance_from_origin(&self) -> ValueType {
        
        todo!();
        /*
            return aloe_hypot (x, y);
        */
    }

    /**
      | Returns the straight-line distance
      | between this point and another one.
      |
      */
    pub fn get_distance_from(&self, other: Point<ValueType>) -> ValueType {
        
        todo!();
        /*
            return aloe_hypot (x - other.x, y - other.y);
        */
    }

    /**
      | Returns the square of the straight-line
      | distance between this point and the
      | origin.
      |
      */
    pub fn get_distance_squared_from_origin(&self) -> ValueType {
        
        todo!();
        /*
            return x * x + y * y;
        */
    }

    /**
      | Returns the square of the straight-line
      | distance between this point and another
      | one.
      |
      */
    pub fn get_distance_squared_from(&self, other: Point<ValueType>) -> ValueType {
        
        todo!();
        /*
            return (*this - other).getDistanceSquaredFromOrigin();
        */
    }

    /**
      | Returns the angle from this point to
      | another one.
      | 
      | Taking this point to be the centre of
      | a circle, and the other point being a
      | position on the circumference, the
      | return value is the number of radians
      | clockwise from the 12 o'clock direction.
      | 
      | So 12 o'clock = 0, 3 o'clock = Pi/2, 6 o'clock
      | = Pi, 9 o'clock = -Pi/2
      |
      */
    pub fn get_angle_to_point<FloatType>(&self, other: Point<ValueType>) -> FloatType {
        
        todo!();
        /*
            return static_cast<FloatType> (std::atan2 (static_cast<FloatType> (other.x - x),
                                                       static_cast<FloatType> (y - other.y)));
        */
    }

    /**
      | Returns the point that would be reached
      | by rotating this point clockwise about
      | the origin by the specified angle.
      |
      */
    pub fn rotated_about_origin(&self, angle_radians: ValueType) -> Point<ValueType> {
        
        todo!();
        /*
            return Point (x * std::cos (angleRadians) - y * std::sin (angleRadians),
                          x * std::sin (angleRadians) + y * std::cos (angleRadians));
        */
    }

    /**
      | Taking this point to be the centre of
      | a circle, this returns a point on its
      | circumference.
      | 
      | -----------
      | @param radius
      | 
      | the radius of the circle.
      | ----------
      | @param angle
      | 
      | the angle of the point, in radians clockwise
      | from the 12 o'clock position.
      |
      */
    pub fn get_point_on_circumference<FloatType>(
        &self, 
        radius: f32,
        angle:  f32) -> Point<FloatType> {
        
        todo!();
        /*
            return Point<FloatType> (static_cast<FloatType> (x + radius * std::sin (angle)),
                                     static_cast<FloatType> (y - radius * std::cos (angle)));
        */
    }

    /**
      | Taking this point to be the centre of
      | an ellipse, this returns a point on its
      | circumference.
      | 
      | -----------
      | @param radiusX
      | 
      | the horizontal radius of the circle.
      | ----------
      | @param radiusY
      | 
      | the vertical radius of the circle.
      | ----------
      | @param angle
      | 
      | the angle of the point, in radians clockwise
      | from the 12 o'clock position.
      |
      */
    pub fn get_point_on_circumference_by_horizontal_and_veritcal_radius_and_angle<FloatType>(
        &self, 
        radiusx: f32,
        radiusy: f32,
        angle:   f32) -> Point<FloatType> {
        
        todo!();
        /*
            return Point<FloatType> (static_cast<FloatType> (x + radiusX * std::sin (angle)),
                                     static_cast<FloatType> (y - radiusY * std::cos (angle)));
        */
    }

    /**
      | Returns the dot-product of two points
      | (x1 * x2 + y1 * y2).
      |
      */
    pub fn get_dot_product<FloatType>(&self, other: Point<ValueType>) -> FloatType {
        
        todo!();
        /*
            return x * other.x + y * other.y;
        */
    }

    /**
      | Uses a transform to change the point's
      | coordinates. This will only compile
      | if ValueType = float!
      | 
      | @see AffineTransform::transformPoint
      |
      */
    pub fn apply_transform(&mut self, transform: &AffineTransform)  {
        
        todo!();
        /*
            transform.transformPoint (x, y);
        */
    }

    /**
      | Returns the position of this point,
      | if it is transformed by a given AffineTransform.
      |
      */
    pub fn transformed_by(&self, transform: &AffineTransform) -> Point<ValueType> {
        
        todo!();
        /*
            return Point (static_cast<ValueType> (transform.mat00 * (float) x + transform.mat01 * (float) y + transform.mat02),
                          static_cast<ValueType> (transform.mat10 * (float) x + transform.mat11 * (float) y + transform.mat12));
        */
    }

    
    /**
      | Casts this point to a Point<int> object.
      |
      */
    pub fn to_int(&self) -> Point<i32> {
        
        todo!();
        /*
            return Point<int> (static_cast<int> (x), static_cast<int> (y));
        */
    }

    /**
      | Casts this point to a Point<float> object.
      |
      */
    pub fn to_float(&self) -> Point<f32> {
        
        todo!();
        /*
            return Point<float> (static_cast<float> (x), static_cast<float> (y));
        */
    }

    /**
      | Casts this point to a Point<double>
      | object.
      |
      */
    pub fn to_double(&self) -> Point<f64> {
        
        todo!();
        /*
            return Point<double> (static_cast<double> (x), static_cast<double> (y));
        */
    }

    /**
      | Casts this point to a Point<int> object
      | using roundToInt() to convert the values.
      |
      */
    pub fn round_to_int(&self) -> Point<i32> {
        
        todo!();
        /*
            return Point<int> (roundToInt (x), roundToInt (y));
        */
    }

    /**
      | Returns the point as a string in the form
      | "x, y".
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return String (x) + ", " + String (y);
        */
    }
}


