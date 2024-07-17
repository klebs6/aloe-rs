crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_Rectangle.h]

/**
  | Manages a rectangle and allows geometric
  | operations to be performed on it.
  | 
  | @see RectangleList, Path, Line, Point
  | 
  | @tags{Graphics}
  |
  */
#[derive(Copy,Clone)]
pub struct Rectangle<ValueType: Copy + Clone> {
    pos: Point<ValueType>,
    w:   ValueType,
    h:   ValueType,
}

impl<ValueType: Copy + Clone> Default for Rectangle<ValueType> {
    
    /**
      | Creates a rectangle of zero size.
      | 
      | The default coordinates will be (0,
      | 0, 0, 0).
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<ValueType: Copy + Clone> Add<Point<ValueType>> for Rectangle<ValueType> {

    type Output = Rectangle<ValueType>;
    
    /**
      | Returns a rectangle which is the same
      | as this one moved by a given amount.
      |
      */
    #[inline]fn add(self, other: Point<ValueType>) -> Self::Output {
        todo!();
        /*
            return { pos.x + deltaPosition.x, pos.y + deltaPosition.y, w, h };
        */
    }
}

impl<ValueType: Copy + Clone> AddAssign<Point<ValueType>> for Rectangle<ValueType> {
    
    /**
      | Moves this rectangle by a given amount.
      |
      */
    #[inline]fn add_assign(&mut self, other: Point<ValueType>) {
        todo!();
        /*
            pos += deltaPosition;
            return *this;
        */
    }
}

impl<ValueType: Copy + Clone> Sub<Point<ValueType>> for Rectangle<ValueType> {

    type Output = Rectangle<ValueType>;
    
    /**
      | Returns a rectangle which is the same
      | as this one moved by a given amount.
      |
      */
    #[inline]fn sub(self, other: Point<ValueType>) -> Self::Output {
        todo!();
        /*
            return { pos.x - deltaPosition.x, pos.y - deltaPosition.y, w, h };
        */
    }
}

impl<ValueType: Copy + Clone> SubAssign<Point<ValueType>> for Rectangle<ValueType> {

    /**
      | Moves this rectangle by a given amount.
      |
      */
    #[inline]fn sub_assign(&mut self, other: Point<ValueType>) {
        todo!();
        /*
            pos -= deltaPosition;
            return *this;
        */
    }
}

impl<ValueType: Copy + Clone,FloatType> Mul<&FloatType> for Rectangle<ValueType> {

    type Output = Rectangle<ValueType>;

    /**
      | Returns a rectangle that has been scaled
      | by the given amount, centred around
      | the origin.
      | 
      | -----------
      | @note
      | 
      | if the rectangle has int coordinates
      | and it's scaled by a floating-point
      | amount, then the result will be converted
      | back to integer coordinates using getSmallestIntegerContainer().
      |
      */
    #[inline] fn mul(self, other: &FloatType) -> Self::Output {
        todo!();
        /*
            Rectangle r (*this);
            r *= scaleFactor;
            return r;
        */
    }
}

impl<ValueType,FloatType> MulAssign<FloatType> for Rectangle<ValueType> 
where
    FloatType: Float + Into<ValueType> + Copy,
    ValueType: MulAssign<FloatType> + Copy + Clone,
{
    
    /**
      | Scales this rectangle by the given amount,
      | centred around the origin.
      | 
      | -----------
      | @note
      | 
      | if the rectangle has int coordinates
      | and it's scaled by a floating-point
      | amount, then the result will be converted
      | back to integer coordinates using getSmallestIntegerContainer().
      |
      */
    #[inline] fn mul_assign(&mut self, scale_factor: FloatType) {
        todo!();
        /*
            Rectangle<FloatType> ((FloatType) pos.x * scaleFactor,
                                  (FloatType) pos.y * scaleFactor,
                                  (FloatType) w * scaleFactor,
                                  (FloatType) h * scaleFactor).copyWithRounding (*this);
            return *this;
        */
    }
}

impl<ValueType,FloatType> MulAssign<Point<FloatType>> for Rectangle<ValueType> 
where
    FloatType: Float + Into<ValueType> + Copy,
    ValueType: MulAssign<FloatType> + Copy + Clone,
{
    
    /**
      | Scales this rectangle by the given X
      | and Y factors, centred around the origin.
      | 
      | -----------
      | @note
      | 
      | if the rectangle has int coordinates
      | and it's scaled by a floating-point
      | amount, then the result will be converted
      | back to integer coordinates using getSmallestIntegerContainer().
      |
      */
    #[inline] fn mul_assign(&mut self, scale_factor: Point<FloatType>) {
        todo!();
        /*
            Rectangle<FloatType> ((FloatType) pos.x * scaleFactor.x,
                                  (FloatType) pos.y * scaleFactor.y,
                                  (FloatType) w * scaleFactor.x,
                                  (FloatType) h * scaleFactor.y).copyWithRounding (*this);
            return *this;
        */
    }
}

impl<ValueType,FloatType> Div<FloatType> for Rectangle<ValueType> 
where ValueType: Copy + Clone,
      FloatType: Float
{

    type Output = Rectangle<ValueType>;
    
    /**
      | Scales this rectangle by the given amount,
      | centred around the origin.
      |
      */
    #[inline] fn div(self, other: FloatType) -> Self::Output {
        todo!();
        /*
            Rectangle r (*this);
            r /= scaleFactor;
            return r;
        */
    }
}

impl<ValueType, FloatType> DivAssign<FloatType> for Rectangle<ValueType>
where
    FloatType: Float + Into<ValueType> + Copy,
    ValueType: DivAssign<FloatType> + Copy + Clone,
{
    /**
      | Scales this rectangle by the given amount,
      | centred around the origin.
      |
      */
    fn div_assign(&mut self, scale_factor: FloatType) {

        todo!();

        /*
            Rectangle<FloatType> ((FloatType) pos.x / scaleFactor,
                                  (FloatType) pos.y / scaleFactor,
                                  (FloatType) w / scaleFactor,
                                  (FloatType) h / scaleFactor).copyWithRounding (*this);
            return *this;
        */
    }
}

impl<ValueType, FloatType> DivAssign<Point<FloatType>> for Rectangle<ValueType>
where
    FloatType: Float + Into<ValueType> + Copy,
    ValueType: DivAssign<FloatType> + Copy + Clone,
{
    /**
      | Scales this rectangle by the given X
      | and Y factors, centred around the origin.
      |
      */
    fn div_assign(&mut self, scale_factor: Point<FloatType>) {

        todo!();

        /*
            Rectangle<FloatType> ((FloatType) pos.x / scaleFactor.x,
                                  (FloatType) pos.y / scaleFactor.y,
                                  (FloatType) w / scaleFactor.x,
                                  (FloatType) h / scaleFactor.y).copyWithRounding (*this);
            return *this;
        */
    }
}

impl<ValueType: Copy + Clone> PartialEq<Rectangle<ValueType>> for Rectangle<ValueType> {
    
    /**
      | Returns true if the two rectangles are
      | identical.
      |
      */
    #[inline] fn eq(&self, other: &Rectangle<ValueType>) -> bool {
        todo!();
        /*
            return pos == other.pos && w == other.w && h == other.h;
        */
    }
}

impl<ValueType: Copy + Clone> Eq for Rectangle<ValueType> {}

impl<ValueType: Copy + Clone> Rectangle<ValueType> {

    /**
      | Creates a rectangle with a given position
      | and size.
      |
      */
    pub fn new_with_position_and_size(
        initialx: ValueType,
        initialy: ValueType,
        width:    ValueType,
        height:   ValueType) -> Self {
    
        todo!();
        /*
        : pos(initialX, initialY),
        : w(width),
        : h(height),

        
        */
    }

    /**
      | Creates a rectangle with a given size,
      | and a position of (0, 0).
      |
      */
    pub fn new_by_width_and_height(
        width:  ValueType,
        height: ValueType) -> Self {
    
        todo!();
        /*
        : w(width),
        : h(height),

        
        */
    }

    /**
      | Creates a Rectangle from the positions
      | of two opposite corners.
      |
      */
    pub fn new_by_corners(
        corner1: Point<ValueType>,
        corner2: Point<ValueType>) -> Self {
    
        todo!();
        /*


            : pos (jmin (corner1.x, corner2.x),
                 jmin (corner1.y, corner2.y)),
            w (corner1.x - corner2.x),
            h (corner1.y - corner2.y)

            if (w < ValueType()) w = -w;
            if (h < ValueType()) h = -h;
        */
    }

    /**
      | Creates a Rectangle from a set of left,
      | right, top, bottom coordinates.
      | 
      | The right and bottom values must be larger
      | than the left and top ones, or the resulting
      | rectangle will have a negative size.
      |
      */
    pub fn left_top_right_bottom(
        left:   ValueType,
        top:    ValueType,
        right:  ValueType,
        bottom: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { left, top, right - left, bottom - top };
        */
    }

    /**
      | Returns true if the rectangle's width
      | or height are zero or less
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return w <= ValueType() || h <= ValueType();
        */
    }

    /**
      | Returns true if the rectangle's values
      | are all finite numbers, i.e. not NaN
      | or infinity.
      |
      */
    #[inline] pub fn is_finite(&self) -> bool {
        
        todo!();
        /*
            return pos.isFinite() && aloe_isfinite (w) && aloe_isfinite (h);
        */
    }

    /**
      | Returns the x coordinate of the rectangle's
      | left-hand-side.
      |
      */
    #[inline] pub fn getx(&self) -> ValueType {
        
        todo!();
        /*
            return pos.x;
        */
    }

    /**
      | Returns the y coordinate of the rectangle's
      | top edge.
      |
      */
    #[inline] pub fn gety(&self) -> ValueType {
        
        todo!();
        /*
            return pos.y;
        */
    }

    /**
      | Returns the width of the rectangle.
      |
      */
    #[inline] pub fn get_width(&self) -> ValueType {
        
        todo!();
        /*
            return w;
        */
    }

    /**
      | Returns the height of the rectangle.
      |
      */
    #[inline] pub fn get_height(&self) -> ValueType {
        
        todo!();
        /*
            return h;
        */
    }

    /**
      | Returns the x coordinate of the rectangle's
      | right-hand-side.
      |
      */
    #[inline] pub fn get_right(&self) -> ValueType {
        
        todo!();
        /*
            return pos.x + w;
        */
    }

    /**
      | Returns the y coordinate of the rectangle's
      | bottom edge.
      |
      */
    #[inline] pub fn get_bottom(&self) -> ValueType {
        
        todo!();
        /*
            return pos.y + h;
        */
    }

    /**
      | Returns the x coordinate of the rectangle's
      | centre.
      |
      */
    pub fn get_centrex(&self) -> ValueType {
        
        todo!();
        /*
            return pos.x + w / (ValueType) 2;
        */
    }

    /**
      | Returns the y coordinate of the rectangle's
      | centre.
      |
      */
    pub fn get_centrey(&self) -> ValueType {
        
        todo!();
        /*
            return pos.y + h / (ValueType) 2;
        */
    }

    /**
      | Returns the centre point of the rectangle.
      |
      */
    pub fn get_centre(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return { pos.x + w / (ValueType) 2,
                                                                                   pos.y + h / (ValueType) 2 };
        */
    }

    /**
      | Returns the aspect ratio of the rectangle's
      | width / height.
      | 
      | If widthOverHeight is true, it returns
      | width / height; if widthOverHeight
      | is false, it returns height / width.
      |
      */
    pub fn get_aspect_ratio(&self, width_over_height: Option<bool>) -> ValueType {

        let width_over_height: bool = width_over_height.unwrap_or(true);

        todo!();
        /*
            return widthOverHeight ? w / h : h / w;
        */
    }

    
    /**
      | Returns the rectangle's top-left position
      | as a Point.
      |
      */
    #[inline] pub fn get_position(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return pos;
        */
    }

    /**
      | Changes the position of the rectangle's
      | top-left corner (leaving its size unchanged).
      |
      */
    #[inline] pub fn set_position(&mut self, new_pos: Point<ValueType>)  {
        
        todo!();
        /*
            pos = newPos;
        */
    }

    /**
      | Changes the position of the rectangle's
      | top-left corner (leaving its size unchanged).
      |
      */
    #[inline] pub fn set_position_xy(
        &mut self, 
        newx: ValueType,
        newy: ValueType
    ) {
        
        todo!();
        /*
            pos.setXY (newX, newY);
        */
    }

    /**
      | Returns the rectangle's top-left position
      | as a Point.
      |
      */
    pub fn get_top_left(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return pos;
        */
    }

    /**
      | Returns the rectangle's top-right
      | position as a Point.
      |
      */
    pub fn get_top_right(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return { pos.x + w, pos.y };
        */
    }

    /**
      | Returns the rectangle's bottom-left
      | position as a Point.
      |
      */
    pub fn get_bottom_left(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return { pos.x, pos.y + h };
        */
    }

    /**
      | Returns the rectangle's bottom-right
      | position as a Point.
      |
      */
    pub fn get_bottom_right(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return { pos.x + w, pos.y + h };
        */
    }

    /**
      | Returns the rectangle's left and right
      | positions as a Range.
      |
      */
    pub fn get_horizontal_range(&self) -> Range<ValueType> {
        
        todo!();
        /*
            return Range<ValueType>::withStartAndLength (pos.x, w);
        */
    }

    /**
      | Returns the rectangle's top and bottom
      | positions as a Range.
      |
      */
    pub fn get_vertical_range(&self) -> Range<ValueType> {
        
        todo!();
        /*
            return Range<ValueType>::withStartAndLength (pos.y, h);
        */
    }

    /**
      | Changes the rectangle's size, leaving
      | the position of its top-left corner
      | unchanged.
      |
      */
    pub fn set_size(&mut self, 
        new_width:  ValueType,
        new_height: ValueType)  {
        
        todo!();
        /*
            w = newWidth; h = newHeight;
        */
    }

    /**
      | Changes all the rectangle's coordinates.
      |
      */
    pub fn set_bounds(&mut self, 
        newx:       ValueType,
        newy:       ValueType,
        new_width:  ValueType,
        new_height: ValueType)  {
        
        todo!();
        /*
            pos.x = newX; pos.y = newY; w = newWidth; h = newHeight;
        */
    }

    /**
      | Changes the rectangle's X coordinate
      |
      */
    #[inline] pub fn setx(&mut self, newx: ValueType)  {
        
        todo!();
        /*
            pos.x = newX;
        */
    }

    /**
      | Changes the rectangle's Y coordinate
      |
      */
    #[inline] pub fn sety(&mut self, newy: ValueType)  {
        
        todo!();
        /*
            pos.y = newY;
        */
    }

    /**
      | Changes the rectangle's width
      |
      */
    #[inline] pub fn set_width(&mut self, new_width: ValueType)  {
        
        todo!();
        /*
            w = newWidth;
        */
    }

    /**
      | Changes the rectangle's height
      |
      */
    #[inline] pub fn set_height(&mut self, new_height: ValueType)  {
        
        todo!();
        /*
            h = newHeight;
        */
    }

    /**
      | Changes the position of the rectangle's
      | centre (leaving its size unchanged).
      |
      */
    #[inline] pub fn set_centre_xy(
        &mut self, 
        new_centrex: ValueType,
        new_centrey: ValueType)  {
        
        todo!();
        /*
           pos.x = newCentreX - w / (ValueType) 2;
           pos.y = newCentreY - h / (ValueType) 2;
        */
    }

    /**
      | Changes the position of the rectangle's
      | centre (leaving its size unchanged).
      |
      */
    #[inline] pub fn set_centre(&mut self, new_centre: Point<ValueType>)  {
        
        todo!();
        /*
            setCentre (newCentre.x, newCentre.y);
        */
    }

    /**
      | Changes the position of the rectangle's
      | left and right edges.
      |
      */
    pub fn set_horizontal_range(&mut self, range: Range<ValueType>)  {
        
        todo!();
        /*
            pos.x = range.getStart(); w = range.getLength();
        */
    }

    /**
      | Changes the position of the rectangle's
      | top and bottom edges.
      |
      */
    pub fn set_vertical_range(&mut self, range: Range<ValueType>)  {
        
        todo!();
        /*
            pos.y = range.getStart(); h = range.getLength();
        */
    }

    /**
      | Returns a rectangle which has the same
      | size and y-position as this one, but
      | with a different x-position.
      |
      */
    pub fn withx(&self, newx: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { newX, pos.y, w, h };
        */
    }

    /**
      | Returns a rectangle which has the same
      | size and x-position as this one, but
      | with a different y-position.
      |
      */
    pub fn withy(&self, newy: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x, newY, w, h };
        */
    }

    /**
      | Returns a rectangle which has the same
      | size and y-position as this one, but
      | whose right-hand edge has the given
      | position.
      |
      */
    pub fn with_rightx(&self, new_rightx: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { newRightX - w, pos.y, w, h };
        */
    }

    /**
      | Returns a rectangle which has the same
      | size and x-position as this one, but
      | whose bottom edge has the given position.
      |
      */
    pub fn with_bottomy(&self, new_bottomy: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x, newBottomY - h, w, h };
        */
    }

    /**
      | Returns a rectangle with the same size
      | as this one, but a new position.
      |
      */
    pub fn with_position_xy(
        &self, 
        newx: ValueType,
        newy: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { newX, newY, w, h };
        */
    }

    /**
      | Returns a rectangle with the same size
      | as this one, but a new position.
      |
      */
    pub fn with_position(&self, new_pos: Point<ValueType>) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { newPos.x, newPos.y, w, h };
        */
    }

    /**
      | Returns a rectangle whose size is the
      | same as this one, but whose top-left
      | position is (0, 0).
      |
      */
    pub fn with_zero_origin(&self) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { w, h };
        */
    }

    /**
      | Returns a rectangle with the same size
      | as this one, but a new centre position.
      |
      */
    pub fn with_centre(&self, new_centre: Point<ValueType>) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { newCentre.x - w / (ValueType) 2,
                                                                                                                   newCentre.y - h / (ValueType) 2, w, h };
        */
    }

    /**
      | Returns a rectangle which has the same
      | position and height as this one, but
      | with a different width.
      |
      */
    pub fn with_width(&self, new_width: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x, pos.y, jmax (ValueType(), newWidth), h };
        */
    }

    /**
      | Returns a rectangle which has the same
      | position and width as this one, but with
      | a different height.
      |
      */
    pub fn with_height(&self, new_height: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x, pos.y, w, jmax (ValueType(), newHeight) };
        */
    }

    /**
      | Returns a rectangle with the same top-left
      | position as this one, but a new size.
      |
      */
    pub fn with_size(&self, 
        new_width:  ValueType,
        new_height: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x, pos.y, jmax (ValueType(), newWidth), jmax (ValueType(), newHeight) };
        */
    }

    /**
      | Returns a rectangle with the same centre
      | position as this one, but a new size.
      |
      */
    pub fn with_size_keeping_centre(&self, 
        new_width:  ValueType,
        new_height: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x + (w - newWidth)  / (ValueType) 2,
                                                                                                                   pos.y + (h - newHeight) / (ValueType) 2, newWidth, newHeight };
        */
    }

    /**
      | Moves the x position, adjusting the
      | width so that the right-hand edge remains
      | in the same place.
      | 
      | If the x is moved to be on the right of the
      | current right-hand edge, the width
      | will be set to zero. @see withLeft
      |
      */
    pub fn set_left(&mut self, new_left: ValueType)  {
        
        todo!();
        /*
            w = jmax (ValueType(), pos.x + w - newLeft); pos.x = newLeft;
        */
    }

    /**
      | Returns a new rectangle with a different
      | x position, but the same right-hand
      | edge as this one.
      | 
      | If the new x is beyond the right of the
      | current right-hand edge, the width
      | will be set to zero. @see setLeft
      |
      */
    pub fn with_left(&self, new_left: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { newLeft, pos.y, jmax (ValueType(), pos.x + w - newLeft), h };
        */
    }

    /**
      | Moves the y position, adjusting the
      | height so that the bottom edge remains
      | in the same place.
      | 
      | If the y is moved to be below the current
      | bottom edge, the height will be set to
      | zero. @see withTop
      |
      */
    pub fn set_top(&mut self, new_top: ValueType)  {
        
        todo!();
        /*
            h = jmax (ValueType(), pos.y + h - newTop); pos.y = newTop;
        */
    }

    /**
      | Returns a new rectangle with a different
      | y position, but the same bottom edge
      | as this one.
      | 
      | If the new y is beyond the bottom of the
      | current rectangle, the height will
      | be set to zero. @see setTop
      |
      */
    pub fn with_top(&self, new_top: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x, newTop, w, jmax (ValueType(), pos.y + h - newTop) };
        */
    }

    /**
      | Adjusts the width so that the right-hand
      | edge of the rectangle has this new value.
      | 
      | If the new right is below the current
      | X value, the X will be pushed down to match
      | it. @see getRight, withRight
      |
      */
    pub fn set_right(&mut self, new_right: ValueType)  {
        
        todo!();
        /*
            pos.x = jmin (pos.x, newRight); w = newRight - pos.x;
        */
    }

    /**
      | Returns a new rectangle with a different
      | right-hand edge position, but the same
      | left-hand edge as this one.
      | 
      | If the new right edge is below the current
      | left-hand edge, the width will be set
      | to zero. @see setRight
      |
      */
    pub fn with_right(&self, new_right: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { jmin (pos.x, newRight), pos.y, jmax (ValueType(), newRight - pos.x), h };
        */
    }

    /**
      | Adjusts the height so that the bottom
      | edge of the rectangle has this new value.
      | 
      | If the new bottom is lower than the current
      | Y value, the Y will be pushed down to match
      | it. @see getBottom, withBottom
      |
      */
    pub fn set_bottom(&mut self, new_bottom: ValueType)  {
        
        todo!();
        /*
            pos.y = jmin (pos.y, newBottom); h = newBottom - pos.y;
        */
    }

    /**
      | Returns a new rectangle with a different
      | bottom edge position, but the same top
      | edge as this one.
      | 
      | If the new y is beyond the bottom of the
      | current rectangle, the height will
      | be set to zero. @see setBottom
      |
      */
    pub fn with_bottom(&self, new_bottom: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x, jmin (pos.y, newBottom), w, jmax (ValueType(), newBottom - pos.y) };
        */
    }

    /**
      | Returns a version of this rectangle
      | with the given amount removed from its
      | left-hand edge.
      |
      */
    pub fn with_trimmed_left(&self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return withLeft (pos.x + amountToRemove);
        */
    }

    /**
      | Returns a version of this rectangle
      | with the given amount removed from its
      | right-hand edge.
      |
      */
    pub fn with_trimmed_right(&self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return withWidth (w - amountToRemove);
        */
    }

    /**
      | Returns a version of this rectangle
      | with the given amount removed from its
      | top edge.
      |
      */
    pub fn with_trimmed_top(&self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return withTop (pos.y + amountToRemove);
        */
    }

    /**
      | Returns a version of this rectangle
      | with the given amount removed from its
      | bottom edge.
      |
      */
    pub fn with_trimmed_bottom(&self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return withHeight (h - amountToRemove);
        */
    }

    
    /**
      | Moves the rectangle's position by adding
      | amount to its x and y coordinates.
      |
      */
    pub fn translate(&mut self, 
        deltax: ValueType,
        deltay: ValueType)  {
        
        todo!();
        /*
            pos.x += deltaX;
            pos.y += deltaY;
        */
    }

    /**
      | Returns a rectangle which is the same
      | as this one moved by a given amount.
      |
      */
    pub fn translated(&self, 
        deltax: ValueType,
        deltay: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return { pos.x + deltaX, pos.y + deltaY, w, h };
        */
    }

    /**
      | Expands the rectangle by a given amount.
      | 
      | Effectively, its new size is (x - deltaX,
      | y - deltaY, w + deltaX * 2, h + deltaY * 2).
      | @see expanded, reduce, reduced
      |
      */
    pub fn expand(&mut self, 
        deltax: ValueType,
        deltay: ValueType)  {
        
        todo!();
        /*
            auto nw = jmax (ValueType(), w + deltaX * 2);
            auto nh = jmax (ValueType(), h + deltaY * 2);
            setBounds (pos.x - deltaX, pos.y - deltaY, nw, nh);
        */
    }

    /**
      | Returns a rectangle that is larger than
      | this one by a given amount.
      | 
      | Effectively, the rectangle returned
      | is (x - deltaX, y - deltaY, w + deltaX *
      | 2, h + deltaY * 2). @see expand, reduce,
      | reduced
      |
      */
    pub fn expanded(&self, 
        deltax: ValueType,
        deltay: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            auto nw = jmax (ValueType(), w + deltaX * 2);
            auto nh = jmax (ValueType(), h + deltaY * 2);
            return { pos.x - deltaX, pos.y - deltaY, nw, nh };
        */
    }

    /**
      | Returns a rectangle that is larger than
      | this one by a given amount.
      | 
      | Effectively, the rectangle returned
      | is (x - delta, y - delta, w + delta * 2, h
      | + delta * 2). @see expand, reduce, reduced
      |
      */
    pub fn expanded_by_delta(&self, delta: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return expanded (delta, delta);
        */
    }

    /**
      | Shrinks the rectangle by a given amount.
      | 
      | Effectively, its new size is (x + deltaX,
      | y + deltaY, w - deltaX * 2, h - deltaY * 2).
      | @see reduced, expand, expanded
      |
      */
    pub fn reduce(&mut self, 
        deltax: ValueType,
        deltay: ValueType)  {
        
        todo!();
        /*
            expand (-deltaX, -deltaY);
        */
    }

    /**
      | Returns a rectangle that is smaller
      | than this one by a given amount.
      | 
      | Effectively, the rectangle returned
      | is (x + deltaX, y + deltaY, w - deltaX *
      | 2, h - deltaY * 2). @see reduce, expand,
      | expanded
      |
      */
    pub fn reduced(
        &self, 
        deltax: ValueType,
        deltay: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return expanded (-deltaX, -deltaY);
        */
    }

    /**
      | Returns a rectangle that is smaller
      | than this one by a given amount.
      | 
      | Effectively, the rectangle returned
      | is (x + delta, y + delta, w - delta * 2, h
      | - delta * 2). @see reduce, expand, expanded
      |
      */
    pub fn reduced_by_delta(&self, delta: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return reduced (delta, delta);
        */
    }

    /**
      | Removes a strip from the top of this rectangle,
      | reducing this rectangle by the specified
      | amount and returning the section that
      | was removed.
      | 
      | E.g. if this rectangle is (100, 100,
      | 300, 300) and amountToRemove is 50,
      | this will return (100, 100, 300, 50)
      | and leave this rectangle as (100, 150,
      | 300, 250).
      | 
      | If amountToRemove is greater than the
      | height of this rectangle, it'll be clipped
      | to that value.
      |
      */
    pub fn remove_from_top(&mut self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            const Rectangle r (pos.x, pos.y, w, jmin (amountToRemove, h));
            pos.y += r.h; h -= r.h;
            return r;
        */
    }

    /**
      | Removes a strip from the left-hand edge
      | of this rectangle, reducing this rectangle
      | by the specified amount and returning
      | the section that was removed.
      | 
      | E.g. if this rectangle is (100, 100,
      | 300, 300) and amountToRemove is 50,
      | this will return (100, 100, 50, 300)
      | and leave this rectangle as (150, 100,
      | 250, 300).
      | 
      | If amountToRemove is greater than the
      | width of this rectangle, it'll be clipped
      | to that value.
      |
      */
    pub fn remove_from_left(&mut self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            const Rectangle r (pos.x, pos.y, jmin (amountToRemove, w), h);
            pos.x += r.w; w -= r.w;
            return r;
        */
    }

    /**
      | Removes a strip from the right-hand
      | edge of this rectangle, reducing this
      | rectangle by the specified amount and
      | returning the section that was removed.
      | 
      | E.g. if this rectangle is (100, 100,
      | 300, 300) and amountToRemove is 50,
      | this will return (350, 100, 50, 300)
      | and leave this rectangle as (100, 100,
      | 250, 300).
      | 
      | If amountToRemove is greater than the
      | width of this rectangle, it'll be clipped
      | to that value.
      |
      */
    pub fn remove_from_right(&mut self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            amountToRemove = jmin (amountToRemove, w);
            const Rectangle r (pos.x + w - amountToRemove, pos.y, amountToRemove, h);
            w -= amountToRemove;
            return r;
        */
    }

    /**
      | Removes a strip from the bottom of this
      | rectangle, reducing this rectangle
      | by the specified amount and returning
      | the section that was removed.
      | 
      | E.g. if this rectangle is (100, 100,
      | 300, 300) and amountToRemove is 50,
      | this will return (100, 350, 300, 50)
      | and leave this rectangle as (100, 100,
      | 300, 250).
      | 
      | If amountToRemove is greater than the
      | height of this rectangle, it'll be clipped
      | to that value.
      |
      */
    pub fn remove_from_bottom(&mut self, amount_to_remove: ValueType) -> Rectangle<ValueType> {
        
        todo!();
        /*
            amountToRemove = jmin (amountToRemove, h);
            const Rectangle r (pos.x, pos.y + h - amountToRemove, w, amountToRemove);
            h -= amountToRemove;
            return r;
        */
    }

    
    /**
      | Returns the nearest point to the specified
      | point that lies within this rectangle.
      |
      */
    pub fn get_constrained_point(&self, point: Point<ValueType>) -> Point<ValueType> {
        
        todo!();
        /*
            return { jlimit (pos.x, pos.x + w, point.x),
                     jlimit (pos.y, pos.y + h, point.y) };
        */
    }

    /**
      | Returns a point within this rectangle,
      | specified as proportional coordinates.
      | The relative X and Y values should be
      | between 0 and 1, where 0 is the left or
      | top of this rectangle, and 1 is the right
      | or bottom. (Out-of-bounds values will
      | return a point outside the rectangle).
      |
      */
    pub fn get_relative_point<FloatType>(&self, 
        relativex: FloatType,
        relativey: FloatType) -> Point<ValueType> {
    
        todo!();
        /*
            return { pos.x + static_cast<ValueType> ((FloatType) w * relativeX),
                     pos.y + static_cast<ValueType> ((FloatType) h * relativeY) };
        */
    }

    /**
      | Returns a proportion of the width of
      | this rectangle.
      |
      */
    pub fn proportion_of_width<FloatType>(&self, proportion: FloatType) -> ValueType {
    
        todo!();
        /*
            return static_cast<ValueType> ((FloatType) w * proportion);
        */
    }

    /**
      | Returns a proportion of the height of
      | this rectangle.
      |
      */
    pub fn proportion_of_height<FloatType>(&self, proportion: FloatType) -> ValueType {
    
        todo!();
        /*
            return static_cast<ValueType> ((FloatType) h * proportion);
        */
    }

    /**
      | Returns a rectangle based on some proportional
      | coordinates relative to this one. So
      | for example getProportion ({ 0.25f,
      | 0.25f, 0.5f, 0.5f }) would return a rectangle
      | of half the original size, with the same
      | centre.
      |
      */
    pub fn get_proportion<FloatType: Float>(&self, proportional_rect: Rectangle<FloatType>) -> Rectangle<ValueType> {
    
        todo!();
        /*
            return { pos.x + static_cast<ValueType> (w * proportionalRect.pos.x),
                     pos.y + static_cast<ValueType> (h * proportionalRect.pos.y),
                     proportionOfWidth  (proportionalRect.w),
                     proportionOfHeight (proportionalRect.h) };
        */
    }

    /**
      | Returns true if this coordinate is inside
      | the rectangle.
      |
      */
    pub fn contains_xy(&self, 
        x_coord: ValueType,
        y_coord: ValueType) -> bool {
        
        todo!();
        /*
            return xCoord >= pos.x && yCoord >= pos.y && xCoord < pos.x + w && yCoord < pos.y + h;
        */
    }

    /**
      | Returns true if this coordinate is inside
      | the rectangle.
      |
      */
    pub fn contains_point(&self, point: Point<ValueType>) -> bool {
        
        todo!();
        /*
            return point.x >= pos.x && point.y >= pos.y && point.x < pos.x + w && point.y < pos.y + h;
        */
    }

    /**
      | Returns true if this other rectangle
      | is completely inside this one.
      |
      */
    pub fn contains_rect(&self, other: Rectangle<ValueType>) -> bool {
        
        todo!();
        /*
            return pos.x <= other.pos.x && pos.y <= other.pos.y
                && pos.x + w >= other.pos.x + other.w && pos.y + h >= other.pos.y + other.h;
        */
    }

    /**
      | Returns true if any part of another rectangle
      | overlaps this one.
      |
      */
    pub fn intersects(&self, other: Rectangle<ValueType>) -> bool {
        
        todo!();
        /*
            return pos.x + w > other.pos.x
                && pos.y + h > other.pos.y
                && pos.x < other.pos.x + other.w
                && pos.y < other.pos.y + other.h
                && w > ValueType() && h > ValueType()
                && other.w > ValueType() && other.h > ValueType();
        */
    }

    /**
      | Returns true if any part of the given
      | line lies inside this rectangle.
      |
      */
    pub fn intersects_line(&self, line: &Line<ValueType>) -> bool {
        
        todo!();
        /*
            return contains (line.getStart()) || contains (line.getEnd())
                    || line.intersects (Line<ValueType> (getTopLeft(),     getTopRight()))
                    || line.intersects (Line<ValueType> (getTopRight(),    getBottomRight()))
                    || line.intersects (Line<ValueType> (getBottomRight(), getBottomLeft()))
                    || line.intersects (Line<ValueType> (getBottomLeft(),  getTopLeft()));
        */
    }

    /**
      | Returns the region that is the overlap
      | between this and another rectangle.
      | 
      | If the two rectangles don't overlap,
      | the rectangle returned will be empty.
      |
      */
    pub fn get_intersection(&self, other: Rectangle<ValueType>) -> Rectangle<ValueType> {
        
        todo!();
        /*
            auto nx = jmax (pos.x, other.pos.x);
            auto ny = jmax (pos.y, other.pos.y);
            auto nw = jmin (pos.x + w, other.pos.x + other.w) - nx;

            if (nw >= ValueType())
            {
                auto nh = jmin (pos.y + h, other.pos.y + other.h) - ny;

                if (nh >= ValueType())
                    return { nx, ny, nw, nh };
            }

            return {};
        */
    }

    /**
      | Clips a set of rectangle coordinates
      | so that they lie only within this one.
      | 
      | This is a non-static version of intersectRectangles().
      | Returns false if the two rectangles
      | didn't overlap.
      |
      */
    pub fn intersect_rectangle_by_xywh(
        &self, 
        otherx: &mut ValueType,
        othery: &mut ValueType,
        otherw: &mut ValueType,
        otherh: &mut ValueType) -> bool {
        
        todo!();
        /*
            auto maxX = jmax (otherX, pos.x);
            otherW = jmin (otherX + otherW, pos.x + w) - maxX;

            if (otherW > ValueType())
            {
                auto maxY = jmax (otherY, pos.y);
                otherH = jmin (otherY + otherH, pos.y + h) - maxY;

                if (otherH > ValueType())
                {
                    otherX = maxX; otherY = maxY;
                    return true;
                }
            }

            return false;
        */
    }

    /**
      | Clips a rectangle so that it lies only
      | within this one.
      | 
      | Returns false if the two rectangles
      | didn't overlap.
      |
      */
    pub fn intersect_rectangle(&self, rectangle_to_clip: &mut Rectangle<ValueType>) -> bool {
        
        todo!();
        /*
            return intersectRectangle (rectangleToClip.pos.x, rectangleToClip.pos.y,
                                       rectangleToClip.w,     rectangleToClip.h);
        */
    }

    /**
      | Returns the smallest rectangle that
      | contains both this one and the one passed-in.
      | 
      | If either this or the other rectangle
      | are empty, they will not be counted as
      | part of the resulting region.
      |
      */
    pub fn get_union(&self, other: Rectangle<ValueType>) -> Rectangle<ValueType> {
        
        todo!();
        /*
            if (other.isEmpty())  return *this;
            if (isEmpty())        return other;

            auto newX = jmin (pos.x, other.pos.x);
            auto newY = jmin (pos.y, other.pos.y);

            return { newX, newY,
                     jmax (pos.x + w, other.pos.x + other.w) - newX,
                     jmax (pos.y + h, other.pos.y + other.h) - newY };
        */
    }

    /**
      | If this rectangle merged with another
      | one results in a simple rectangle, this
      | will set this rectangle to the result,
      | and return true.
      | 
      | Returns false and does nothing to this
      | rectangle if the two rectangles don't
      | overlap, or if they form a complex region.
      |
      */
    pub fn enlarge_if_adjacent(&mut self, other: Rectangle<ValueType>) -> bool {
        
        todo!();
        /*
            if (pos.x == other.pos.x && getRight() == other.getRight()
                 && (other.getBottom() >= pos.y && other.pos.y <= getBottom()))
            {
                auto newY = jmin (pos.y, other.pos.y);
                h = jmax (getBottom(), other.getBottom()) - newY;
                pos.y = newY;
                return true;
            }

            if (pos.y == other.pos.y && getBottom() == other.getBottom()
                 && (other.getRight() >= pos.x && other.pos.x <= getRight()))
            {
                auto newX = jmin (pos.x, other.pos.x);
                w = jmax (getRight(), other.getRight()) - newX;
                pos.x = newX;
                return true;
            }

            return false;
        */
    }

    /**
      | If after removing another rectangle
      | from this one the result is a simple rectangle,
      | this will set this object's bounds to
      | be the result, and return true.
      | 
      | Returns false and does nothing to this
      | rectangle if the two rectangles don't
      | overlap, or if removing the other one
      | would form a complex region.
      |
      */
    pub fn reduce_if_partly_contained_in(&mut self, other: Rectangle<ValueType>) -> bool {
        
        todo!();
        /*
            int inside = 0;
            auto otherR = other.getRight();
            if (pos.x >= other.pos.x && pos.x < otherR) inside = 1;
            auto otherB = other.getBottom();
            if (pos.y >= other.pos.y && pos.y < otherB) inside |= 2;
            auto r = pos.x + w;
            if (r >= other.pos.x && r < otherR) inside |= 4;
            auto b = pos.y + h;
            if (b >= other.pos.y && b < otherB) inside |= 8;

            switch (inside)
            {
                case 1 + 2 + 8:  w = r - otherR; pos.x = otherR; return true;
                case 1 + 2 + 4:  h = b - otherB; pos.y = otherB; return true;
                case 2 + 4 + 8:  w = other.pos.x - pos.x;        return true;
                case 1 + 4 + 8:  h = other.pos.y - pos.y;        return true;
                default:         break;
            }

            return false;
        */
    }

    /**
      | Tries to fit this rectangle within a
      | target area, returning the result.
      | 
      | If this rectangle is not completely
      | inside the target area, then it'll be
      | shifted (without changing its size)
      | so that it lies within the target. If
      | it is larger than the target rectangle
      | in either dimension, then that dimension
      | will be reduced to fit within the target.
      |
      */
    pub fn constrained_within(&self, area_to_fit_within: Rectangle<ValueType>) -> Rectangle<ValueType> {
        
        todo!();
        /*
            auto newPos = areaToFitWithin.withSize (areaToFitWithin.getWidth() - w,
                                                    areaToFitWithin.getHeight() - h)
                            .getConstrainedPoint (pos);

            return { newPos.x, newPos.y,
                     jmin (w, areaToFitWithin.getWidth()),
                     jmin (h, areaToFitWithin.getHeight()) };
        */
    }

    /**
      | Returns the smallest rectangle that
      | can contain the shape created by applying
      | a transform to this rectangle.
      | 
      | This should only be used on floating
      | point rectangles.
      |
      */
    pub fn transformed_by(&self, transform: &AffineTransform) -> Rectangle<ValueType> {
        
        todo!();
        /*
            using FloatType = typename TypeHelpers::SmallestFloatType<ValueType>::type;

            auto x1 = static_cast<FloatType> (pos.x),     y1 = static_cast<FloatType> (pos.y);
            auto x2 = static_cast<FloatType> (pos.x + w), y2 = static_cast<FloatType> (pos.y);
            auto x3 = static_cast<FloatType> (pos.x),     y3 = static_cast<FloatType> (pos.y + h);
            auto x4 = static_cast<FloatType> (x2),        y4 = static_cast<FloatType> (y3);

            transform.transformPoints (x1, y1, x2, y2);
            transform.transformPoints (x3, y3, x4, y4);

            auto rx1 = jmin (x1, x2, x3, x4);
            auto rx2 = jmax (x1, x2, x3, x4);
            auto ry1 = jmin (y1, y2, y3, y4);
            auto ry2 = jmax (y1, y2, y3, y4);

            Rectangle r;
            Rectangle<FloatType> (rx1, ry1, rx2 - rx1, ry2 - ry1).copyWithRounding (r);
            return r;
        */
    }

    /**
      | Returns the smallest integer-aligned
      | rectangle that completely contains
      | this one.
      | 
      | This is only relevant for floating-point
      | rectangles, of course. @see toFloat(),
      | toNearestInt(), toNearestIntEdges()
      |
      */
    pub fn get_smallest_integer_container(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return Rectangle<int>::leftTopRightBottom (floorAsInt (pos.x),
                                                       floorAsInt (pos.y),
                                                       ceilAsInt  (pos.x + w),
                                                       ceilAsInt  (pos.y + h));
        */
    }

    /**
      | Casts this rectangle to a Rectangle<int>.
      | 
      | This uses roundToInt to snap x, y, width
      | and height to the nearest integer (losing
      | precision).
      | 
      | If the rectangle already uses integers,
      | this will simply return a copy. @see
      | getSmallestIntegerContainer(),
      | toNearestIntEdges()
      |
      */
    pub fn to_nearest_int(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return { roundToInt (pos.x), roundToInt (pos.y),
                     roundToInt (w),     roundToInt (h) };
        */
    }

    /**
      | Casts this rectangle to a Rectangle<int>.
      | 
      | This uses roundToInt to snap top, left,
      | right and bottom to the nearest integer
      | (losing precision).
      | 
      | If the rectangle already uses integers,
      | this will simply return a copy. @see
      | getSmallestIntegerContainer(),
      | toNearestInt()
      |
      */
    pub fn to_nearest_int_edges(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return Rectangle<int>::leftTopRightBottom (roundToInt (pos.x),       roundToInt (pos.y),
                                                       roundToInt (getRight()),  roundToInt (getBottom()));
        */
    }

    /**
      | Casts this rectangle to a Rectangle<float>.
      | @see getSmallestIntegerContainer
      |
      */
    pub fn to_float(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return { static_cast<float> (pos.x), static_cast<float> (pos.y),
                     static_cast<float> (w),     static_cast<float> (h) };
        */
    }

    /**
      | Casts this rectangle to a Rectangle<double>.
      | @see getSmallestIntegerContainer
      |
      */
    pub fn to_double(&self) -> Rectangle<f64> {
        
        todo!();
        /*
            return { static_cast<double> (pos.x), static_cast<double> (pos.y),
                     static_cast<double> (w),     static_cast<double> (h) };
        */
    }

    /**
      | Casts this rectangle to a Rectangle
      | with the given type.
      | 
      | If the target type is a conversion from
      | float to int, then the conversion will
      | be done using getSmallestIntegerContainer().
      |
      */
    pub fn to_type<TargetType: Copy>(&self) -> Rectangle<TargetType> {
    
        todo!();
        /*
            Rectangle<TargetType> r;
            copyWithRounding (r);
            return r;
        */
    }

    /**
      | Returns the smallest Rectangle that
      | can contain a set of points.
      |
      */
    pub fn find_area_containing_points(
        points:     *const Point<ValueType>,
        num_points: i32) -> Rectangle<ValueType> {
        
        todo!();
        /*
            if (numPoints <= 0)
                return {};

            auto minX = points[0].x;
            auto maxX = minX;
            auto minY = points[0].y;
            auto maxY = minY;

            for (int i = 1; i < numPoints; ++i)
            {
                minX = jmin (minX, points[i].x);
                maxX = jmax (maxX, points[i].x);
                minY = jmin (minY, points[i].y);
                maxY = jmax (maxY, points[i].y);
            }

            return { minX, minY, maxX - minX, maxY - minY };
        */
    }

    
    /**
      | Static utility to intersect two sets
      | of rectangular coordinates.
      | 
      | Returns false if the two regions didn't
      | overlap. @see intersectRectangle
      |
      */
    pub fn intersect_rectangles(
        x1: &mut ValueType,
        y1: &mut ValueType,
        w1: &mut ValueType,
        h1: &mut ValueType,
        x2: ValueType,
        y2: ValueType,
        w2: ValueType,
        h2: ValueType) -> bool {
        
        todo!();
        /*
            auto x = jmax (x1, x2);
            w1 = jmin (x1 + w1, x2 + w2) - x;

            if (w1 > ValueType())
            {
                auto y = jmax (y1, y2);
                h1 = jmin (y1 + h1, y2 + h2) - y;

                if (h1 > ValueType())
                {
                    x1 = x; y1 = y;
                    return true;
                }
            }

            return false;
        */
    }
    
    /**
      | Creates a string describing this rectangle.
      | 
      | The string will be of the form "x y width
      | height", e.g. "100 100 400 200".
      | 
      | Coupled with the fromString() method,
      | this is very handy for things like storing
      | rectangles (particularly component
      | positions) in XML attributes.
      | 
      | @see fromString
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            String s;
            s.preallocateBytes (32);
            s << pos.x << ' ' << pos.y << ' ' << w << ' ' << h;
            return s;
        */
    }

    /**
      | Parses a string containing a rectangle's
      | details.
      | 
      | The string should contain 4 integer
      | tokens, in the form "x y width height".
      | They can be comma or whitespace separated.
      | 
      | This method is intended to go with the
      | toString() method, to form an easy way
      | of saving/loading rectangles as strings.
      | 
      | @see toString
      |
      */
    pub fn from_string(string_version: &str) -> Rectangle<ValueType> {
        
        todo!();
        /*
            StringArray toks;
            toks.addTokens (stringVersion.text.findEndOfWhitespace(), ",; \t\r\n", "");

            return { parseIntAfterSpace (toks[0]),
                     parseIntAfterSpace (toks[1]),
                     parseIntAfterSpace (toks[2]),
                     parseIntAfterSpace (toks[3]) };
        */
    }
    
    pub fn parse_int_after_space(s: &str) -> ValueType {
        
        todo!();
        /*
            return static_cast<ValueType> (s.text.findEndOfWhitespace().getIntValue32());
        */
    }
    
    pub fn copy_with_rounding_i32(&self, result: &mut Rectangle<i32>)  {
        
        todo!();
        /*
            result = getSmallestIntegerContainer();
        */
    }
    
    pub fn copy_with_rounding_f32(&self, result: &mut Rectangle<f32>)  {
        
        todo!();
        /*
            result = toFloat();
        */
    }
    
    pub fn copy_with_rounding_f64(&self, result: &mut Rectangle<f64>)  {
        
        todo!();
        /*
            result = toDouble();
        */
    }
    
    pub fn floor_as_int_i32(n: i32) -> i32 {
        
        todo!();
        /*
            return n;
        */
    }
    
    pub fn floor_as_int_f32(n: f32) -> i32 {
        
        todo!();
        /*
            return n > (float)  std::numeric_limits<int>::min() ? (int) std::floor (n) : std::numeric_limits<int>::min();
        */
    }
    
    pub fn floor_as_int_f64(n: f64) -> i32 {
        
        todo!();
        /*
            return n > (double) std::numeric_limits<int>::min() ? (int) std::floor (n) : std::numeric_limits<int>::min();
        */
    }
    
    pub fn ceil_as_int_i32(n: i32) -> i32 {
        
        todo!();
        /*
            return n;
        */
    }
    
    pub fn ceil_as_int_f32(n: f32) -> i32 {
        
        todo!();
        /*
            return n < (float)  std::numeric_limits<int>::max() ? (int) std::ceil (n) : std::numeric_limits<int>::max();
        */
    }
    
    pub fn ceil_as_int_f64(n: f64) -> i32 {
        
        todo!();
        /*
            return n < (double) std::numeric_limits<int>::max() ? (int) std::ceil (n) : std::numeric_limits<int>::max();
        */
    }
}
