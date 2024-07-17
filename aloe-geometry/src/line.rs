crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_Line.h]

/**
  | Represents a line.
  | 
  | This class contains a bunch of useful
  | methods for various geometric tasks.
  | 
  | The ValueType template parameter should
  | be a primitive type - float or double
  | are what it's designed for. Integer
  | types will work in a basic way, but some
  | methods that perform mathematical
  | operations may not compile, or they
  | may not produce sensible results.
  | 
  | @see Point, Rectangle, Path, Graphics::drawLine
  | 
  | @tags{Graphics}
  |
  */
pub struct Line<ValueType> {
    start: Point<ValueType>,
    end:   Point<ValueType>,
}

impl<ValueType> Default for Line<ValueType> {
    
    /**
      | Creates a line, using (0, 0) as its start
      | and end points.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<ValueType> PartialEq<Line<ValueType>> for Line<ValueType> {
    
    /**
      | Compares two lines.
      |
      */
    #[inline] fn eq(&self, other: &Line<ValueType>) -> bool {
        todo!();
        /*
            return start == other.start && end == other.end;
        */
    }
}

impl<ValueType> Eq for Line<ValueType> {}

impl<ValueType> Line<ValueType> {

    /**
      | Creates a copy of another line.
      |
      */
    pub fn new(_0: &Line<ValueType>) -> Self {
    
        todo!();
        /*


        
        */
    }

    /**
      | Creates a line based on the coordinates
      | of its start and end points.
      |
      */
    pub fn new_from_coordinates(
        startx: ValueType,
        starty: ValueType,
        endx:   ValueType,
        endy:   ValueType) -> Self {
    
        todo!();
        /*
        : start(startX, startY),
        : end(endX, endY),

        
        */
    }

    /**
      | Creates a line from its start and end
      | points.
      |
      */
    pub fn new_from_start_and_end_point(
        start_point: Point<ValueType>,
        end_point:   Point<ValueType>) -> Self {
    
        todo!();
        /*
        : start(startPoint),
        : end(endPoint),

        
        */
    }
    
    /**
      | Returns the x coordinate of the line's
      | start point.
      |
      */
    #[inline] pub fn get_startx(&self) -> ValueType {
        
        todo!();
        /*
            return start.x;
        */
    }

    /**
      | Returns the y coordinate of the line's
      | start point.
      |
      */
    #[inline] pub fn get_starty(&self) -> ValueType {
        
        todo!();
        /*
            return start.y;
        */
    }

    /**
      | Returns the x coordinate of the line's
      | end point.
      |
      */
    #[inline] pub fn get_endx(&self) -> ValueType {
        
        todo!();
        /*
            return end.x;
        */
    }

    /**
      | Returns the y coordinate of the line's
      | end point.
      |
      */
    #[inline] pub fn get_endy(&self) -> ValueType {
        
        todo!();
        /*
            return end.y;
        */
    }

    /**
      | Returns the line's start point.
      |
      */
    #[inline] pub fn get_start(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return start;
        */
    }

    /**
      | Returns the line's end point.
      |
      */
    #[inline] pub fn get_end(&self) -> Point<ValueType> {
        
        todo!();
        /*
            return end;
        */
    }

    /**
      | Changes this line's start point
      |
      */
    pub fn set_start_with_x_and_y(
        &mut self, 
        new_startx: ValueType,
        new_starty: ValueType)  {
        
        todo!();
        /*
            start.setXY (newStartX, newStartY);
        */
    }

    /**
      | Changes this line's end point
      |
      */
    pub fn set_end_with_x_and_y(
        &mut self, 
        new_endx: ValueType,
        new_endy: ValueType)  {
        
        todo!();
        /*
            end.setXY (newEndX, newEndY);
        */
    }

    /**
      | Changes this line's start point
      |
      */
    pub fn set_start(&mut self, new_start: Point<ValueType>)  {
        
        todo!();
        /*
            start = newStart;
        */
    }

    /**
      | Changes this line's end point
      |
      */
    pub fn set_end(&mut self, new_end: Point<ValueType>)  {
        
        todo!();
        /*
            end = newEnd;
        */
    }

    /**
      | Returns a line that is the same as this
      | one, but with the start and end reversed,
      |
      */
    pub fn reversed(&self) -> Line<ValueType> {
        
        todo!();
        /*
            return { end, start };
        */
    }

    /**
      | Applies an affine transform to the line's
      | start and end points.
      |
      */
    pub fn apply_transform(&mut self, transform: &AffineTransform)  {
        
        todo!();
        /*
            start.applyTransform (transform);
            end.applyTransform (transform);
        */
    }

    
    /**
      | Returns the length of the line.
      |
      */
    pub fn get_length(&self) -> ValueType {
        
        todo!();
        /*
            return start.getDistanceFrom (end);
        */
    }

    /**
      | Returns the length of the line.
      |
      */
    pub fn get_length_squared(&self) -> ValueType {
        
        todo!();
        /*
            return start.getDistanceSquaredFrom (end);
        */
    }

    /**
      | Returns true if the line's start and
      | end x coordinates are the same.
      |
      */
    pub fn is_vertical(&self) -> bool {
        
        todo!();
        /*
            return start.x == end.x;
        */
    }

    /**
      | Returns true if the line's start and
      | end y coordinates are the same.
      |
      */
    pub fn is_horizontal(&self) -> bool {
        
        todo!();
        /*
            return start.y == end.y;
        */
    }

    /**
      | Returns the line's angle.
      | 
      | This value is the number of radians clockwise
      | from the 12 o'clock direction, where
      | the line's start point is considered
      | to be at the centre.
      |
      */
    pub fn get_angle(&self) -> <Point<ValueType> as HasFloatType>::FloatType 

        where ValueType: SmallestFloatType
    {
        todo!();

        /*
            return start.getAngleToPoint (end);
        */
    }

    /**
      | Creates a line from a start point, length
      | and angle.
      | 
      | This angle is the number of radians clockwise
      | from the 12 o'clock direction, where
      | the line's start point is considered
      | to be at the centre.
      |
      */
    pub fn from_start_and_angle(
        start_point: Point<ValueType>,
        length:      ValueType,
        angle:       ValueType) -> Line<ValueType> {
        
        todo!();
        /*
            return { startPoint, startPoint.getPointOnCircumference (length, angle) };
        */
    }

    
    /**
      | Casts this line to float coordinates.
      |
      */
    pub fn to_float(&self) -> Line<f32> {
        
        todo!();
        /*
            return { start.toFloat(), end.toFloat() };
        */
    }

    /**
      | Casts this line to double coordinates.
      |
      */
    pub fn to_double(&self) -> Line<f64> {
        
        todo!();
        /*
            return { start.toDouble(), end.toDouble() };
        */
    }
    
    /**
      | Finds the intersection between two
      | lines.
      | 
      | -----------
      | @param line
      | 
      | the line to intersect with
      | 
      | -----------
      | @return
      | 
      | the point at which the lines intersect,
      | even if this lies beyond the end of the
      | lines
      |
      */
    pub fn get_intersection(&self, line: Line<ValueType>) -> Point<ValueType> {
        
        todo!();
        /*
            Point<ValueType> p;
            findIntersection (start, end, line.start, line.end, p);
            return p;
        */
    }

    /**
      | Finds the intersection between two
      | lines.
      | 
      | -----------
      | @param line
      | 
      | the other line
      | ----------
      | @param intersection
      | 
      | the position of the point where the lines
      | meet (or where they would meet if they
      | were infinitely long) the intersection
      | (if the lines intersect). If the lines
      | are parallel, this will just be set to
      | the position of one of the line's endpoints.
      | 
      | -----------
      | @return
      | 
      | true if the line segments intersect;
      | false if they don't. Even if they don't
      | intersect, the intersection coordinates
      | returned will still be valid
      |
      */
    pub fn intersects_by_line_and_intersection(
        &self, 
        line:         Line<ValueType>,
        intersection: &mut Point<ValueType>) -> bool {
        
        todo!();
        /*
            return findIntersection (start, end, line.start, line.end, intersection);
        */
    }

    /**
      | Returns true if this line intersects
      | another.
      |
      */
    pub fn intersects(&self, other: Line<ValueType>) -> bool {
        
        todo!();
        /*
            Point<ValueType> ignored;
            return findIntersection (start, end, other.start, other.end, ignored);
        */
    }

    /**
      | Returns the location of the point which
      | is a given distance along this line.
      | 
      | -----------
      | @param distanceFromStart
      | 
      | the distance to move along the line from
      | its start point. This value can be negative
      | or longer than the line itself @see getPointAlongLineProportionally
      |
      */
    pub fn get_point_along_line(&self, distance_from_start: ValueType) -> Point<ValueType> {
        
        todo!();
        /*
            return start + (end - start) * (distanceFromStart / getLength());
        */
    }

    /**
      | Returns a point which is a certain distance
      | along and to the side of this line.
      | 
      | This effectively moves a given distance
      | along the line, then another distance
      | perpendicularly to this, and returns
      | the resulting position.
      | 
      | -----------
      | @param distanceFromStart
      | 
      | the distance to move along the line from
      | its start point. This value can be negative
      | or longer than the line itself
      | ----------
      | @param perpendicularDistance
      | 
      | how far to move sideways from the line.
      | If you're looking along the line from
      | its start towards its end, then a positive
      | value here will move to the right, negative
      | value move to the left.
      |
      */
    pub fn get_point_along_line_with_perpendicular_distance(
        &self, 
        distance_from_start:    ValueType,
        perpendicular_distance: ValueType) -> Point<ValueType> {
        
        todo!();
        /*
            auto delta = end - start;
            auto length = aloe_hypot ((double) delta.x,
                                      (double) delta.y);
            if (length <= 0)
                return start;

            return { start.x + static_cast<ValueType> ((delta.x * distanceFromStart - delta.y * perpendicularDistance) / length),
                     start.y + static_cast<ValueType> ((delta.y * distanceFromStart + delta.x * perpendicularDistance) / length) };
        */
    }

    /**
      | Returns the location of the point which
      | is a given distance along this line proportional
      | to the line's length.
      | 
      | -----------
      | @param proportionOfLength
      | 
      | the distance to move along the line from
      | its start point, in multiples of the
      | line's length. So a value of 0.0 will
      | return the line's start point and a value
      | of 1.0 will return its end point. (This
      | value can be negative or greater than
      | 1.0). @see getPointAlongLine
      |
      */
    pub fn get_point_along_line_proportionally(
        &self, 
        proportion_of_length: <Point<ValueType> as HasFloatType>::FloatType

    ) -> Point<ValueType> 
        where ValueType: SmallestFloatType
    {
        
        todo!();
        /*
            return start + (end - start) * proportionOfLength;
        */
    }

    /**
      | Returns the smallest distance between
      | this line segment and a given point.
      | 
      | So if the point is close to the line, this
      | will return the perpendicular distance
      | from the line; if the point is a long way
      | beyond one of the line's end-point's,
      | it'll return the straight-line distance
      | to the nearest end-point.
      | 
      | pointOnLine receives the position
      | of the point that is found.
      | 
      | 
      | -----------
      | @return
      | 
      | the point's distance from the line @see
      | getPositionAlongLineOfNearestPoint
      |
      */
    pub fn get_distance_from_point(&self, 
        target_point:  Point<ValueType>,
        point_on_line: &mut Point<ValueType>) -> ValueType {
        
        todo!();
        /*
            auto delta = end - start;
            auto length = delta.x * delta.x + delta.y * delta.y;

            if (length > 0)
            {
                auto prop = ((targetPoint.x - start.x) * delta.x
                           + (targetPoint.y - start.y) * delta.y) / (double) length;

                if (prop >= 0 && prop <= 1.0)
                {
                    pointOnLine = start + delta * prop;
                    return targetPoint.getDistanceFrom (pointOnLine);
                }
            }

            auto fromStart = targetPoint.getDistanceFrom (start);
            auto fromEnd   = targetPoint.getDistanceFrom (end);

            if (fromStart < fromEnd)
            {
                pointOnLine = start;
                return fromStart;
            }

            pointOnLine = end;
            return fromEnd;
        */
    }

    /**
      | Finds the point on this line which is
      | nearest to a given point, and returns
      | its position as a proportional position
      | along the line.
      | 
      | -----------
      | @return
      | 
      | a value 0 to 1.0 which is the distance
      | along this line from the line's start
      | to the point which is nearest to the point
      | passed-in. To turn this number into
      | a position, use getPointAlongLineProportionally().
      | @see getDistanceFromPoint, getPointAlongLineProportionally
      |
      */
    pub fn find_nearest_proportional_position_to(&self, point: Point<ValueType>) -> ValueType {
        
        todo!();
        /*
            auto delta = end - start;
            auto length = delta.x * delta.x + delta.y * delta.y;

            return length <= 0 ? 0
                               : jlimit (ValueType(), static_cast<ValueType> (1),
                                         static_cast<ValueType> ((((point.x - start.x) * delta.x
                                                                 + (point.y - start.y) * delta.y) / length)));
        */
    }

    /**
      | Finds the point on this line which is
      | nearest to a given point. @see getDistanceFromPoint,
      | findNearestProportionalPositionTo
      |
      */
    pub fn find_nearest_point_to(&self, point: Point<ValueType>) -> Point<ValueType> {
        
        todo!();
        /*
            return getPointAlongLineProportionally (findNearestProportionalPositionTo (point));
        */
    }

    /**
      | Returns true if the given point lies
      | above this line.
      | 
      | The return value is true if the point's
      | y coordinate is less than the y coordinate
      | of this line at the given x (assuming
      | the line extends infinitely in both
      | directions).
      |
      */
    pub fn is_point_above(&self, point: Point<ValueType>) -> bool {
        
        todo!();
        /*
            return start.x != end.x
                    && point.y < ((end.y - start.y) * (point.x - start.x)) / (end.x - start.x) + start.y;
        */
    }
    
    /**
      | Returns a shortened copy of this line.
      | 
      | This will chop off part of the start of
      | this line by a certain amount, (leaving
      | the end-point the same), and return
      | the new line.
      |
      */
    pub fn with_shortened_start(&self, distance_to_shorten_by: ValueType) -> Line<ValueType> {
        
        todo!();
        /*
            return { getPointAlongLine (jmin (distanceToShortenBy, getLength())), end };
        */
    }

    /**
      | Returns a shortened copy of this line.
      | 
      | This will chop off part of the end of this
      | line by a certain amount, (leaving the
      | start-point the same), and return the
      | new line.
      |
      */
    pub fn with_shortened_end(&self, distance_to_shorten_by: ValueType) -> Line<ValueType> {
        
        todo!();
        /*
            auto length = getLength();
            return { start, getPointAlongLine (length - jmin (distanceToShortenBy, length)) };
        */
    }
    
    pub fn is_zero_to_one(v: ValueType) -> bool {
        
        todo!();
        /*
            return v >= 0 && v <= static_cast<ValueType> (1);
        */
    }
    
    pub fn find_intersection(
        p1:           Point<ValueType>,
        p2:           Point<ValueType>,
        p3:           Point<ValueType>,
        p4:           Point<ValueType>,
        intersection: &mut Point<ValueType>) -> bool {
        
        todo!();
        /*
            if (p2 == p3)
            {
                intersection = p2;
                return true;
            }

            auto d1 = p2 - p1;
            auto d2 = p4 - p3;
            auto divisor = d1.x * d2.y - d2.x * d1.y;

            if (divisor == 0)
            {
                if (! (d1.isOrigin() || d2.isOrigin()))
                {
                    if (d1.y == 0 && d2.y != 0)
                    {
                        auto along = (p1.y - p3.y) / d2.y;
                        intersection = p1.withX (p3.x + along * d2.x);
                        return isZeroToOne (along);
                    }

                    if (d2.y == 0 && d1.y != 0)
                    {
                        auto along = (p3.y - p1.y) / d1.y;
                        intersection = p3.withX (p1.x + along * d1.x);
                        return isZeroToOne (along);
                    }

                    if (d1.x == 0 && d2.x != 0)
                    {
                        auto along = (p1.x - p3.x) / d2.x;
                        intersection = p1.withY (p3.y + along * d2.y);
                        return isZeroToOne (along);
                    }

                    if (d2.x == 0 && d1.x != 0)
                    {
                        auto along = (p3.x - p1.x) / d1.x;
                        intersection = p3.withY (p1.y + along * d1.y);
                        return isZeroToOne (along);
                    }
                }

                intersection = (p2 + p3) / static_cast<ValueType> (2);
                return false;
            }

            auto along1 = ((p1.y - p3.y) * d2.x - (p1.x - p3.x) * d2.y) / divisor;
            intersection = p1 + d1 * along1;

            if (! isZeroToOne (along1))
                return false;

            auto along2 = ((p1.y - p3.y) * d1.x - (p1.x - p3.x) * d1.y) / divisor;
            return isZeroToOne (along2);
        */
    }
}
