crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_Parallelogram.h]

/**
  | Represents a parallelogram that is
  | defined by 3 points. @see Rectangle,
  | Point, Line
  | 
  | @tags{Graphics}
  |
  */
#[derive(Copy,Clone)]
pub struct Parallelogram<ValueType: Copy + Clone> {
    top_left:    Point<ValueType>,
    top_right:   Point<ValueType>,
    bottom_left: Point<ValueType>,
}

impl<ValueType: Copy + Clone> Default for Parallelogram<ValueType> {
    
    /**
      | Creates a parallelogram with zero size
      | at the origin.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl<ValueType: Copy + Clone> PartialEq<Parallelogram<ValueType>> for Parallelogram<ValueType> {
    
    /**
      | Returns true if the two parallelograms
      | are identical.
      |
      */
    #[inline] fn eq(&self, other: &Parallelogram<ValueType>) -> bool {
        todo!();
        /*
            return topLeft == other.topLeft && topRight == other.topRight && bottomLeft == other.bottomLeft;
        */
    }
}

impl<ValueType: Copy + Clone> Eq for Parallelogram<ValueType> {}
 
impl<ValueType: Copy + Clone> Add<Point<ValueType>> for Parallelogram<ValueType> {

    type Output = Parallelogram<ValueType>;
    
    /**
      | Returns a parallelogram which is the
      | same as this one moved by a given amount.
      |
      */
    #[inline]fn add(self, other: Point<ValueType>) -> Self::Output {

        todo!();
        /*
            auto p = *this;
            p += deltaPosition;
            return p;
        */
    }
}
   
impl<ValueType: Copy + Clone> AddAssign<Point<ValueType>> for Parallelogram<ValueType> {

    /**
      | Moves this parallelogram by a given
      | amount.
      |
      */
    #[inline]fn add_assign(&mut self, other: Point<ValueType>) {
        todo!();
        /*
            topLeft += deltaPosition;
            topRight += deltaPosition;
            bottomLeft += deltaPosition;
            return *this;
        */
    }
}

impl<ValueType: Copy + Clone> Sub<Point<ValueType>> for Parallelogram<ValueType> {

    type Output = Parallelogram<ValueType>;
    
    /**
      | Returns a parallelogram which is the
      | same as this one moved by a given amount.
      |
      */
    #[inline]fn sub(self, other: Point<ValueType>) -> Self::Output {

        todo!();

        /*
            return operator+ (-deltaPosition);
        */
    }
}

impl<ValueType: Copy + Clone> SubAssign<Point<ValueType>> for Parallelogram<ValueType> {
    
    /**
      | Moves this parallelogram by a given
      | amount.
      |
      */
    #[inline]fn sub_assign(&mut self, other: Point<ValueType>) {
        todo!();
        /*
            return operator-= (-deltaPosition);
        */
    }
}

impl<ValueType: Copy + Clone,PointOrScalarType> Mul<&PointOrScalarType> for Parallelogram<ValueType> {

    type Output = Parallelogram<ValueType>;
    
    /**
      | Returns a parallelogram that has been
      | scaled by the given amount, centred
      | around the origin.
      |
      */
    #[inline] fn mul(self, other: &PointOrScalarType) -> Self::Output {
        todo!();
        /*
            auto p = *this;
            p *= scaleFactor;
            return p;
        */
    }
}

impl<ValueType: Copy + Clone,PointOrScalarType> MulAssign<PointOrScalarType> for Parallelogram<ValueType> {
    
    /**
      | Scales this parallelogram by the given
      | amount, centred around the origin.
      |
      */
    #[inline] fn mul_assign(&mut self, scale_factor: PointOrScalarType) {
        todo!();
        /*
            topLeft *= scaleFactor;
            topRight *= scaleFactor;
            bottomLeft *= scaleFactor;
            return *this;
        */
    }
}

impl<ValueType: Copy + Clone> Parallelogram<ValueType> {

    /**
      | Creates a parallelogram based on 3 points.
      |
      */
    pub fn new_from_points(
        top_left_position:    Point<ValueType>,
        top_right_position:   Point<ValueType>,
        bottom_left_position: Point<ValueType>) -> Self {
    
        todo!();
        /*
        : top_left(topLeftPosition),
        : top_right(topRightPosition),
        : bottom_left(bottomLeftPosition),

        
        */
    }

    /**
      | Creates a parallelogram from a rectangle.
      |
      */
    pub fn new_from_rectangle(rectangle: Rectangle<ValueType>) -> Self {
    
        todo!();
        /*

            : topLeft (rectangle.getTopLeft()),
             topRight (rectangle.getTopRight()),
             bottomLeft (rectangle.getBottomLeft())
        */
    }
    
    /**
      | Returns true if the parallelogram has
      | a width or height of more than zero.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return topLeft != topRight || topLeft != bottomLeft;
        */
    }

    /**
      | Returns true if the parallelogram's
      | coordinates are all finite numbers,
      | i.e. not NaN or infinity.
      |
      */
    #[inline] pub fn is_finite(&self) -> bool {
        
        todo!();
        /*
            return topLeft.isFinite() && topRight.isFinite() && bottomLeft.isFinite();
        */
    }

    /**
      | Returns the width of the parallelogram
      | (i.e. the straight-line distance between
      | the top-left and top-right.
      |
      */
    #[inline] pub fn get_width(&self) -> ValueType {
        
        todo!();
        /*
            return Line<ValueType> (topLeft, topRight).getLength();
        */
    }

    /**
      | Returns the height of the parallelogram
      | (i.e. the straight-line distance between
      | the top-left and bottom-left.
      |
      */
    #[inline] pub fn get_height(&self) -> ValueType {
        
        todo!();
        /*
            return Line<ValueType> (topLeft, bottomLeft).getLength();
        */
    }

    
    /**
      | Returns the parallelogram's top-left
      | position as a Point.
      |
      */
    pub fn get_top_left(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return topLeft;
        */
    }

    /**
      | Returns the parallelogram's top-right
      | position as a Point.
      |
      */
    pub fn get_top_right(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return topRight;
        */
    }

    /**
      | Returns the parallelogram's bottom-left
      | position as a Point.
      |
      */
    pub fn get_bottom_left(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return bottomLeft;
        */
    }

    /**
      | Returns the parallelogram's bottom-right
      | position as a Point.
      |
      */
    pub fn get_bottom_right(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return topRight + (bottomLeft - topLeft);
        */
    }
    
    /**
      | Returns a point within this parallelogram,
      | specified as proportional coordinates.
      | 
      | The relative X and Y values should be
      | between 0 and 1, where 0 is the left or
      | top of this parallelogram, and 1 is the
      | right or bottom. (Out-of-bounds values
      | will return a point outside the parallelogram).
      |
      */
    pub fn get_relative_point(&self, relative_position: Point<ValueType>) -> Point<ValueType> {
        
        todo!();
        /*
            return topLeft
                    + (topRight - topLeft) * relativePosition.x
                    + (bottomLeft - topLeft) * relativePosition.y;
        */
    }

    /**
      | Returns a transformed version of the
      | parallelogram.
      |
      */
    pub fn transformed_by(&self, transform: &AffineTransform) -> Parallelogram<ValueType> {
        
        todo!();
        /*
            auto p = *this;
            transform.transformPoints (p.topLeft.x, p.topLeft.y,
                                       p.topRight.x, p.topRight.y,
                                       p.bottomLeft.x, p.bottomLeft.y);

            return p;
        */
    }

    /**
      | Returns the smallest rectangle that
      | encloses this parallelogram.
      |
      */
    pub fn get_bounding_box(&self) -> Rectangle<ValueType> {
        
        todo!();
        /*
            const Point<ValueType> points[] = { topLeft, topRight, bottomLeft, getBottomRight() };
            return Rectangle<ValueType>::findAreaContainingPoints (points, 4);
        */
    }
}

