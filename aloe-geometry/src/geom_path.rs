crate::ix!();

pub fn is_marker(
        value:  f32,
        marker: f32) -> bool {
    
    todo!();
    /*
        return value == marker;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_Path.h]

/**
  | A path is a sequence of lines and curves
  | that may either form a closed shape or
  | be open-ended.
  | 
  | To use a path, you can create an empty
  | one, then add lines and curves to it to
  | create shapes, then it can be rendered
  | by a Graphics context or used for geometric
  | operations.
  | 
  | e.g.
  | 
  | -----------
  | @code
  | 
  | Path myPath;
  | 
  | myPath.startNewSubPath (10.0f, 10.0f);          // move the current position to (10, 10)
  | myPath.lineTo (100.0f, 200.0f);                 // draw a line from here to (100, 200)
  | myPath.quadraticTo (0.0f, 150.0f, 5.0f, 50.0f); // draw a curve that ends at (5, 50)
  | myPath.closeSubPath();                          // close the subpath with a line back to (10, 10)
  | 
  | // add an ellipse as well, which will form a second sub-path within the path..
  | myPath.addEllipse (50.0f, 50.0f, 40.0f, 30.0f);
  | 
  | // double the width of the whole thing..
  | myPath.applyTransform (AffineTransform::scale (2.0f, 1.0f));
  | 
  | // and draw it to a graphics context with a 5-pixel thick outline.
  | g.strokePath (myPath, PathStrokeType (5.0f));
  | 
  | A path object can actually contain multiple
  | sub-paths, which may themselves be
  | open or closed.
  | 
  | @see PathFlatteningIterator, PathStrokeType,
  | Graphics
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct Path {
    data:                 Vec<f32>,
    bounds:               path::PathBounds,
    use_non_zero_winding: bool, // default = true
}
    
impl Default for Path {
    
    /**
      | Creates an empty path.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

pub mod path {
    use super::*;

    pub const LINE_MARKER:                       f32 = 100001.0;
    pub const MOVE_MARKER:                       f32 = 100002.0;
    pub const QUAD_MARKER:                       f32 = 100003.0;
    pub const CUBIC_MARKER:                      f32 = 100004.0;
    pub const CLOSE_SUB_PATH_MARKER:             f32 = 100005.0;

    pub const DEFAULT_TOLERANCE_FOR_TESTING:     f32 = 1.0;
    pub const DEFAULT_TOLERANCE_FOR_MEASUREMENT: f32 = 0.6;

    /**
      | Iterates the lines and curves that a
      | path contains.
      | 
      | @see Path, PathFlatteningIterator
      |
      */
    #[no_copy]
    pub struct Iterator<'a> {
        element_type: path::iterator::PathElementType,
        x1:           f32, // default = 0
        y1:           f32, // default = 0
        x2:           f32, // default = 0
        y2:           f32, // default = 0
        x3:           f32, // default = 0
        y3:           f32, // default = 0
        path:         &'a Path,
        index:        *const f32,
    }

    pub mod iterator {
        use super::*;

        pub enum PathElementType
        {
            /**
              | For this type, x1 and y1 will be set to
              | indicate the first point in the subpath.
              |
              */
            startNewSubPath,    

            /**
              | For this type, x1 and y1 indicate the
              | end point of the line.
              |
              */
            lineTo,             

            /**
              | For this type, x1, y1, x2, y2 indicate
              | the control point and endpoint of a quadratic
              | curve.
              |
              */
            quadraticTo,        

            /**
              | For this type, x1, y1, x2, y2, x3, y3 indicate
              | the two control points and the endpoint
              | of a cubic curve.
              |
              */
            cubicTo,            

            /**
              | Indicates that the sub-path is being
              | closed. None of the x or y values are valid
              | in this case.
              |
              */
            closePath,           
        }
    }

    impl<'a> Iterator<'a> {
        
        pub fn new_from_path_ref(p: &Path) -> Self {
        
            todo!();
            /*
            : element_type(startNewSubPath),
            : path(p),
            : index(path.data.begin()),
            */
        }
        
        /**
          | Moves onto the next element in the path.
          | 
          | If this returns false, there are no more
          | elements. If it returns true, the elementType
          | variable will be set to the type of the
          | current element, and some of the x and
          | y variables will be filled in with values.
          |
          */
        pub fn next(&mut self) -> bool {
            
            todo!();
            /*
                if (index != path.data.end())
            {
                auto type = *index++;

                if (isMarker (type, moveMarker))
                {
                    elementType = startNewSubPath;
                    x1 = *index++;
                    y1 = *index++;
                }
                else if (isMarker (type, lineMarker))
                {
                    elementType = lineTo;
                    x1 = *index++;
                    y1 = *index++;
                }
                else if (isMarker (type, quadMarker))
                {
                    elementType = quadraticTo;
                    x1 = *index++;
                    y1 = *index++;
                    x2 = *index++;
                    y2 = *index++;
                }
                else if (isMarker (type, cubicMarker))
                {
                    elementType = cubicTo;
                    x1 = *index++;
                    y1 = *index++;
                    x2 = *index++;
                    y2 = *index++;
                    x3 = *index++;
                    y3 = *index++;
                }
                else if (isMarker (type, closeSubPathMarker))
                {
                    elementType = closePath;
                }

                return true;
            }

            return false;
            */
        }
    }


    ///---------------------
    #[derive(Default)]
    pub struct PathBounds {
        path_xmin: f32, // default = 0
        path_xmax: f32, // default = 0
        path_ymin: f32, // default = 0
        path_ymax: f32, // default = 0
    }

    impl PathBounds {
        
        pub fn extend_with_coords<Coords>(
            &mut self, 
            x:      f32,
            y:      f32,
            coords: Coords)  {
        
            todo!();
            /*
                extend (x, y);
                    extend (coords...);
            */
        }
        
        pub fn get_rectangle(&self) -> Rectangle<f32> {
            
            todo!();
            /*
                return { pathXMin, pathYMin, pathXMax - pathXMin, pathYMax - pathYMin };
            */
        }
        
        pub fn reset(&mut self)  {
            
            todo!();
            /*
                pathXMin = pathYMin = pathYMax = pathXMax = 0;
            */
        }
        
        pub fn reset_with_x_and_y(&mut self, x: f32, y: f32)  {
            
            todo!();
            /*
                pathXMin = pathXMax = x;
            pathYMin = pathYMax = y;
            */
        }
        
        pub fn extend(&mut self, x: f32, y: f32)  {
            
            todo!();
            /*
                if (x < pathXMin)      pathXMin = x;
            else if (x > pathXMax) pathXMax = x;

            if (y < pathYMin)      pathYMin = y;
            else if (y > pathYMax) pathYMax = y;
            */
        }
    }

    pub const ellipse_angular_increment: f32 = 0.05;

    pub fn next_token(t: &mut CharPointerType) -> String {
        
        todo!();
        /*
            t.incrementToEndOfWhitespace();

                auto start = t;
                size_t numChars = 0;

                while (! (t.isEmpty() || t.isWhitespace()))
                {
                    ++t;
                    ++numChars;
                }

                return { start, numChars };
        */
    }

    #[inline] pub fn length_of(
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32) -> f64 {
        
        todo!();
        /*
            return aloe_hypot ((double) (x1 - x2), (double) (y1 - y2));
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_Path.cpp]
impl Path {

    /**
      | Adds a rectangle to the path.
      | 
      | The rectangle is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addRoundedRectangle,
      | addTriangle
      |
      */
    pub fn add_rectangle<ValueType>(
        &mut self, 
        rectangle: Rectangle<ValueType>
    ) 
        where ValueType: Copy + Clone
    {

        todo!();
        /*
           addRectangle (static_cast<float> (rectangle.getX()), static_cast<float> (rectangle.getY()),
           static_cast<float> (rectangle.getWidth()), static_cast<float> (rectangle.getHeight()));
           */
    }

    /**
      | Adds a rectangle with rounded corners
      | to the path.
      | 
      | The rectangle is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addRectangle, addTriangle
      |
      */
    pub fn add_rounded_rectangle_with_cornersize_x_and_y<ValueType>(
        &mut self, 
        rectangle:    Rectangle<ValueType>,
        corner_sizex: f32,
        corner_sizey: f32
    ) 
        where ValueType: Copy + Clone
    {
    
        todo!();
        /*
            addRoundedRectangle (static_cast<float> (rectangle.getX()), static_cast<float> (rectangle.getY()),
                                 static_cast<float> (rectangle.getWidth()), static_cast<float> (rectangle.getHeight()),
                                 cornerSizeX, cornerSizeY);
        */
    }

    /**
      | Adds a rectangle with rounded corners
      | to the path.
      | 
      | The rectangle is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addRectangle, addTriangle
      |
      */
    pub fn add_rounded_rectangle_by_rectangle_and_corner_size<ValueType>(
        &mut self, 
        rectangle:   Rectangle<ValueType>,
        corner_size: f32)  
        where ValueType: Copy + Clone
    {
    
        todo!();
        /*
            addRoundedRectangle (rectangle, cornerSize, cornerSize);
        */
    }

    /**
      | Returns the flag that indicates whether
      | the path should use a non-zero winding
      | rule.
      | 
      | The default for a new path is true.
      | 
      | @see setUsingNonZeroWinding
      |
      */
    pub fn is_using_non_zero_winding(&self) -> bool {
        
        todo!();
        /*
            return useNonZeroWinding;
        */
    }
}

/**
  | tests that some coordinates aren't
  | NaNs
  |
  */
macro_rules! aloe_check_coords_are_valid {
    ($x:ident, 
     $y:ident) => {
        /*
        
            jassert (x == x && y == y);
        */
    }
}

impl PartialEq<Path> for Path {
    
    #[inline] fn eq(&self, other: &Path) -> bool {
        todo!();
        /*
            return useNonZeroWinding == other.useNonZeroWinding && data == other.data;
        */
    }
}

impl Eq for Path {}

impl Path {

    /**
      | Creates a copy of another path.
      |
      */
    pub fn new_from_other_ref(other: &Path) -> Self {
    
        todo!();
        /*
        : data(other.data),
        : bounds(other.bounds),
        : use_non_zero_winding(other.useNonZeroWinding),

        
        */
    }
    
    /**
      | Copies this path from another one.
      |
      */
    pub fn assign_from_ref(&mut self, other: &Path) -> &mut Path {
        
        todo!();
        /*
            if (this != &other)
        {
            data = other.data;
            bounds = other.bounds;
            useNonZeroWinding = other.useNonZeroWinding;
        }

        return *this;
        */
    }
    
    pub fn new_from_other(other: Path) -> Self {
    
        todo!();
        /*
        : data(std::move (other.data)),
        : bounds(other.bounds),
        : use_non_zero_winding(other.useNonZeroWinding),

        
        */
    }
    
    pub fn assign_from(&mut self, other: Path) -> &mut Path {
        
        todo!();
        /*
            data = std::move (other.data);
        bounds = other.bounds;
        useNonZeroWinding = other.useNonZeroWinding;
        return *this;
        */
    }
    
    /**
      | Removes all lines and curves, resetting
      | the path completely.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            data.clearQuick();
        bounds.reset();
        */
    }
    
    /**
      | Swaps the contents of this path with
      | another one.
      | 
      | The internal data of the two paths is
      | swapped over, so this is much faster
      | than copying it to a temp variable and
      | back.
      |
      */
    pub fn swap_with_path(&mut self, other: &mut Path)  {
        
        todo!();
        /*
            data.swapWith (other.data);
        std::swap (bounds.pathXMin, other.bounds.pathXMin);
        std::swap (bounds.pathXMax, other.bounds.pathXMax);
        std::swap (bounds.pathYMin, other.bounds.pathYMin);
        std::swap (bounds.pathYMax, other.bounds.pathYMax);
        std::swap (useNonZeroWinding, other.useNonZeroWinding);
        */
    }
    
    /**
      | Changes the winding-rule to be used
      | when filling the path.
      | 
      | If set to true (which is the default),
      | then the path uses a non-zero-winding
      | rule to determine which points are inside
      | the path. If set to false, it uses an alternate-winding
      | rule.
      | 
      | The winding-rule comes into play when
      | areas of the shape overlap other areas,
      | and determines whether the overlapping
      | regions are considered to be inside
      | or outside.
      | 
      | Changing this value just sets a flag
      | - it doesn't affect the contents of the
      | path.
      | 
      | @see isUsingNonZeroWinding
      |
      */
    pub fn set_using_non_zero_winding(&mut self, is_non_zero: bool)  {
        
        todo!();
        /*
            useNonZeroWinding = isNonZero;
        */
    }
    
    /**
      | Rescales this path to make it fit neatly
      | into a given space.
      | 
      | This is effectively a quick way of calling
      | applyTransform (getTransformToScaleToFit
      | (x, y, w, h, preserveProportions))
      | 
      | -----------
      | @param x
      | 
      | the x position of the rectangle to fit
      | the path inside
      | ----------
      | @param y
      | 
      | the y position of the rectangle to fit
      | the path inside
      | ----------
      | @param width
      | 
      | the width of the rectangle to fit the
      | path inside
      | ----------
      | @param height
      | 
      | the height of the rectangle to fit the
      | path inside
      | ----------
      | @param preserveProportions
      | 
      | if true, it will fit the path into the
      | space without altering its horizontal/vertical
      | scale ratio; if false, it will distort
      | the path to fill the specified ratio
      | both horizontally and vertically
      | 
      | @see applyTransform, getTransformToScaleToFit
      |
      */
    pub fn scale_to_fit(&mut self, 
        x:                    f32,
        y:                    f32,
        w:                    f32,
        h:                    f32,
        preserve_proportions: bool)  {
        
        todo!();
        /*
            applyTransform (getTransformToScaleToFit (x, y, w, h, preserveProportions));
        */
    }
    
    /**
      | Returns true if the path doesn't contain
      | any lines or curves.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            for (auto i = data.begin(), e = data.end(); i != e; ++i)
        {
            auto type = *i;

            if (isMarker (type, moveMarker))
            {
                i += 2;
            }
            else if (isMarker (type, lineMarker)
                     || isMarker (type, quadMarker)
                     || isMarker (type, cubicMarker))
            {
                return false;
            }
        }

        return true;
        */
    }
    
    /**
      | Returns the smallest rectangle that
      | contains all points within the path.
      |
      */
    pub fn get_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return bounds.getRectangle();
        */
    }
    
    /**
      | Returns the smallest rectangle that
      | contains all points within the path
      | after it's been transformed with the
      | given transform matrix.
      |
      */
    pub fn get_bounds_transformed(&self, transform: &AffineTransform) -> Rectangle<f32> {
        
        todo!();
        /*
            return getBounds().transformedBy (transform);
        */
    }
    
    /**
      | Preallocates enough space for adding
      | the given number of coordinates to the
      | path.
      | 
      | If you're about to add a large number
      | of lines or curves to the path, it can
      | make the task much more efficient to
      | call this first and avoid costly reallocations
      | as the structure grows.
      | 
      | The actual value to pass is a bit tricky
      | to calculate because the space required
      | depends on what you're adding - e.g.
      | each lineTo() or startNewSubPath()
      | will require 3 coords (x, y and a type
      | marker). Each quadraticTo() will need
      | 5, and a cubicTo() will require 7. Closing
      | a sub-path will require 1.
      |
      */
    pub fn preallocate_space(&mut self, num_extra_coords_to_make_space_for: i32)  {
        
        todo!();
        /*
            data.ensureStorageAllocated (data.size() + numExtraCoordsToMakeSpaceFor);
        */
    }
    
    /**
      | Begins a new subpath with a given starting
      | position.
      | 
      | This will move the path's current position
      | to the coordinates passed in and make
      | it ready to draw lines or curves starting
      | from this position.
      | 
      | After adding whatever lines and curves
      | are needed, you can either close the
      | current sub-path using closeSubPath()
      | or call startNewSubPath() to move to
      | a new sub-path, leaving the old one open-ended.
      | 
      | @see lineTo, quadraticTo, cubicTo,
      | closeSubPath
      |
      */
    pub fn start_new_sub_path(&mut self, x: f32, y: f32)  {
        
        todo!();
        /*
            ALOE_CHECK_COORDS_ARE_VALID (x, y)

        if (data.isEmpty())
            bounds.reset (x, y);
        else
            bounds.extend (x, y);

        data.add (moveMarker, x, y);
        */
    }
    
    /**
      | Begins a new subpath with a given starting
      | position.
      | 
      | This will move the path's current position
      | to the coordinates passed in and make
      | it ready to draw lines or curves starting
      | from this position.
      | 
      | After adding whatever lines and curves
      | are needed, you can either close the
      | current sub-path using closeSubPath()
      | or call startNewSubPath() to move to
      | a new sub-path, leaving the old one open-ended.
      | 
      | @see lineTo, quadraticTo, cubicTo,
      | closeSubPath
      |
      */
    pub fn start_new_sub_path_from_point(&mut self, start: Point<f32>)  {
        
        todo!();
        /*
            startNewSubPath (start.x, start.y);
        */
    }
    
    /**
      | Adds a line from the shape's last position
      | to a new end-point.
      | 
      | This will connect the end-point of the
      | last line or curve that was added to a
      | new point, using a straight line.
      | 
      | See the class description for an example
      | of how to add lines and curves to a path.
      | 
      | @see startNewSubPath, quadraticTo,
      | cubicTo, closeSubPath
      |
      */
    pub fn line_to(&mut self, x: f32, y: f32)  {
        
        todo!();
        /*
            ALOE_CHECK_COORDS_ARE_VALID (x, y)

        if (data.isEmpty())
            startNewSubPath (0, 0);

        data.add (lineMarker, x, y);
        bounds.extend (x, y);
        */
    }
    
    /**
      | Adds a line from the shape's last position
      | to a new end-point.
      | 
      | This will connect the end-point of the
      | last line or curve that was added to a
      | new point, using a straight line.
      | 
      | See the class description for an example
      | of how to add lines and curves to a path.
      | 
      | @see startNewSubPath, quadraticTo,
      | cubicTo, closeSubPath
      |
      */
    pub fn line_to_point(&mut self, end: Point<f32>)  {
        
        todo!();
        /*
            lineTo (end.x, end.y);
        */
    }
    
    /**
      | Adds a quadratic bezier curve from the
      | shape's last position to a new position.
      | 
      | This will connect the end-point of the
      | last line or curve that was added to a
      | new point, using a quadratic spline
      | with one control-point.
      | 
      | See the class description for an example
      | of how to add lines and curves to a path.
      | 
      | @see startNewSubPath, lineTo, cubicTo,
      | closeSubPath
      |
      */
    pub fn quadratic_to(
        &mut self, 
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32)  {
        
        todo!();
        /*
            ALOE_CHECK_COORDS_ARE_VALID (x1, y1)
        ALOE_CHECK_COORDS_ARE_VALID (x2, y2)

        if (data.isEmpty())
            startNewSubPath (0, 0);

        data.add (quadMarker, x1, y1, x2, y2);
        bounds.extend (x1, y1, x2, y2);
        */
    }
    
    /**
      | Adds a quadratic bezier curve from the
      | shape's last position to a new position.
      | 
      | This will connect the end-point of the
      | last line or curve that was added to a
      | new point, using a quadratic spline
      | with one control-point.
      | 
      | See the class description for an example
      | of how to add lines and curves to a path.
      | 
      | @see startNewSubPath, lineTo, cubicTo,
      | closeSubPath
      |
      */
    pub fn quadratic_to_with_control_points(
        &mut self, 
        control_point: Point<f32>,
        end_point:     Point<f32>)  {
        
        todo!();
        /*
            quadraticTo (controlPoint.x, controlPoint.y,
                     endPoint.x, endPoint.y);
        */
    }
    
    /**
      | Adds a cubic bezier curve from the shape's
      | last position to a new position.
      | 
      | This will connect the end-point of the
      | last line or curve that was added to a
      | new point, using a cubic spline with
      | two control-points.
      | 
      | See the class description for an example
      | of how to add lines and curves to a path.
      | 
      | @see startNewSubPath, lineTo, quadraticTo,
      | closeSubPath
      |
      */
    pub fn cubic_to(
        &mut self, 
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32)  {
        
        todo!();
        /*
            ALOE_CHECK_COORDS_ARE_VALID (x1, y1)
        ALOE_CHECK_COORDS_ARE_VALID (x2, y2)
        ALOE_CHECK_COORDS_ARE_VALID (x3, y3)

        if (data.isEmpty())
            startNewSubPath (0, 0);

        data.add (cubicMarker, x1, y1, x2, y2, x3, y3);
        bounds.extend (x1, y1, x2, y2, x3, y3);
        */
    }
    
    /**
      | Adds a cubic bezier curve from the shape's
      | last position to a new position.
      | 
      | This will connect the end-point of the
      | last line or curve that was added to a
      | new point, using a cubic spline with
      | two control-points.
      | 
      | See the class description for an example
      | of how to add lines and curves to a path.
      | 
      | @see startNewSubPath, lineTo, quadraticTo,
      | closeSubPath
      |
      */
    pub fn cubic_to_by_points(
        &mut self, 
        control_point1: Point<f32>,
        control_point2: Point<f32>,
        end_point:      Point<f32>)  {
        
        todo!();
        /*
            cubicTo (controlPoint1.x, controlPoint1.y,
                 controlPoint2.x, controlPoint2.y,
                 endPoint.x, endPoint.y);
        */
    }
    
    /**
      | Closes a the current sub-path with a
      | line back to its start-point.
      | 
      | When creating a closed shape such as
      | a triangle, don't use 3 lineTo() calls
      | - instead use two lineTo() calls, followed
      | by a closeSubPath() to join the final
      | point back to the start.
      | 
      | This ensures that closes shapes are
      | recognised as such, and this is important
      | for tasks like drawing strokes, which
      | needs to know whether to draw end-caps
      | or not.
      | 
      | @see startNewSubPath, lineTo, quadraticTo,
      | cubicTo, closeSubPath
      |
      */
    pub fn close_sub_path(&mut self)  {
        
        todo!();
        /*
            if (! (data.isEmpty() || isMarker (data.getLast(), closeSubPathMarker)))
            data.add (closeSubPathMarker);
        */
    }
    
    /**
      | Returns the last point that was added
      | to the path by one of the drawing methods.
      |
      */
    pub fn get_current_position(&self) -> Point<f32> {
        
        todo!();
        /*
            if (data.isEmpty())
            return {};

        auto* i = data.end() - 1;

        if (isMarker (*i, closeSubPathMarker))
        {
            while (i != data.begin())
            {
                if (isMarker (*--i, moveMarker))
                {
                    i += 2;
                    break;
                }
            }
        }

        if (i != data.begin())
            return { *(i - 1), *i };

        return {};
        */
    }
    
    /**
      | Adds a rectangle to the path.
      | 
      | The rectangle is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addRoundedRectangle,
      | addTriangle
      |
      */
    pub fn add_rectangle_by_coordinates(&mut self, 
        x: f32,
        y: f32,
        w: f32,
        h: f32)  {
        
        todo!();
        /*
            auto x1 = x, y1 = y, x2 = x + w, y2 = y + h;

        if (w < 0) std::swap (x1, x2);
        if (h < 0) std::swap (y1, y2);

        if (data.isEmpty())
        {
            bounds.pathXMin = x1;
            bounds.pathXMax = x2;
            bounds.pathYMin = y1;
            bounds.pathYMax = y2;
        }
        else
        {
            bounds.pathXMin = jmin (bounds.pathXMin, x1);
            bounds.pathXMax = jmax (bounds.pathXMax, x2);
            bounds.pathYMin = jmin (bounds.pathYMin, y1);
            bounds.pathYMax = jmax (bounds.pathYMax, y2);
        }

        data.add (moveMarker, x1, y2,
                  lineMarker, x1, y1,
                  lineMarker, x2, y1,
                  lineMarker, x2, y2,
                  closeSubPathMarker);
        */
    }
    
    /**
      | Adds a rectangle with rounded corners
      | to the path.
      | 
      | The rectangle is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addRectangle, addTriangle
      |
      */
    pub fn add_rounded_rectangle_by_x_y_w_h_csx_csy(&mut self, 
        x:   f32,
        y:   f32,
        w:   f32,
        h:   f32,
        csx: f32,
        csy: f32)  {
        
        todo!();
        /*
            addRoundedRectangle (x, y, w, h, csx, csy, true, true, true, true);
        */
    }
    
    /**
      | Adds a rectangle with rounded corners
      | to the path.
      | 
      | The rectangle is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addRectangle, addTriangle
      |
      */
    pub fn add_rounded_rectangle_by_x_y_w_h_csx_csy_and_curve_config(&mut self, 
        x:                  f32,
        y:                  f32,
        w:                  f32,
        h:                  f32,
        csx:                f32,
        csy:                f32,
        curve_top_left:     bool,
        curve_top_right:    bool,
        curve_bottom_left:  bool,
        curve_bottom_right: bool)  {
        
        todo!();
        /*
            csx = jmin (csx, w * 0.5f);
        csy = jmin (csy, h * 0.5f);
        auto cs45x = csx * 0.45f;
        auto cs45y = csy * 0.45f;
        auto x2 = x + w;
        auto y2 = y + h;

        if (curveTopLeft)
        {
            startNewSubPath (x, y + csy);
            cubicTo (x, y + cs45y, x + cs45x, y, x + csx, y);
        }
        else
        {
            startNewSubPath (x, y);
        }

        if (curveTopRight)
        {
            lineTo (x2 - csx, y);
            cubicTo (x2 - cs45x, y, x2, y + cs45y, x2, y + csy);
        }
        else
        {
            lineTo (x2, y);
        }

        if (curveBottomRight)
        {
            lineTo (x2, y2 - csy);
            cubicTo (x2, y2 - cs45y, x2 - cs45x, y2, x2 - csx, y2);
        }
        else
        {
            lineTo (x2, y2);
        }

        if (curveBottomLeft)
        {
            lineTo (x + csx, y2);
            cubicTo (x + cs45x, y2, x, y2 - cs45y, x, y2 - csy);
        }
        else
        {
            lineTo (x, y2);
        }

        closeSubPath();
        */
    }
    
    /**
      | Adds a rectangle with rounded corners
      | to the path.
      | 
      | The rectangle is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addRectangle, addTriangle
      |
      */
    pub fn add_rounded_rectangle_by_x_y_w_h_cs(&mut self, 
        x:  f32,
        y:  f32,
        w:  f32,
        h:  f32,
        cs: f32)  {
        
        todo!();
        /*
            addRoundedRectangle (x, y, w, h, cs, cs);
        */
    }
    
    /**
      | Adds a triangle to the path.
      | 
      | The triangle is added as a new closed
      | sub-path. (Any currently open paths
      | will be left open).
      | 
      | -----------
      | @note
      | 
      | whether the vertices are specified
      | in clockwise or anticlockwise order
      | will affect how the triangle is filled
      | when it overlaps other shapes (the winding
      | order setting will affect this of course).
      |
      */
    pub fn add_triangle(&mut self, 
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32)  {
        
        todo!();
        /*
            addTriangle ({ x1, y1 },
                     { x2, y2 },
                     { x3, y3 });
        */
    }
    
    /**
      | Adds a triangle to the path.
      | 
      | The triangle is added as a new closed
      | sub-path. (Any currently open paths
      | will be left open).
      | 
      | -----------
      | @note
      | 
      | whether the vertices are specified
      | in clockwise or anticlockwise order
      | will affect how the triangle is filled
      | when it overlaps other shapes (the winding
      | order setting will affect this of course).
      |
      */
    pub fn add_triangle_by_points(
        &mut self, 
        p1: Point<f32>,
        p2: Point<f32>,
        p3: Point<f32>)  {
        
        todo!();
        /*
            startNewSubPath (p1);
        lineTo (p2);
        lineTo (p3);
        closeSubPath();
        */
    }
    
    /**
      | Adds a quadrilateral to the path.
      | 
      | The quad is added as a new closed sub-path.
      | (Any currently open paths will be left
      | open).
      | 
      | -----------
      | @note
      | 
      | whether the vertices are specified
      | in clockwise or anticlockwise order
      | will affect how the quad is filled when
      | it overlaps other shapes (the winding
      | order setting will affect this of course).
      |
      */
    pub fn add_quadrilateral(&mut self, 
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32)  {
        
        todo!();
        /*
            startNewSubPath (x1, y1);
        lineTo (x2, y2);
        lineTo (x3, y3);
        lineTo (x4, y4);
        closeSubPath();
        */
    }
    
    /**
      | Adds an ellipse to the path.
      | 
      | The shape is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addArc
      |
      */
    pub fn add_ellipse(&mut self, 
        x: f32,
        y: f32,
        w: f32,
        h: f32)  {
        
        todo!();
        /*
            addEllipse ({ x, y, w, h });
        */
    }
    
    /**
      | Adds an ellipse to the path.
      | 
      | The shape is added as a new sub-path.
      | (Any currently open paths will be left
      | open). @see addArc
      |
      */
    pub fn add_ellipse_by_rectangle(&mut self, area: Rectangle<f32>)  {
        
        todo!();
        /*
            auto hw = area.getWidth() * 0.5f;
        auto hw55 = hw * 0.55f;
        auto hh = area.getHeight() * 0.5f;
        auto hh55 = hh * 0.55f;
        auto cx = area.getX() + hw;
        auto cy = area.getY() + hh;

        startNewSubPath (cx, cy - hh);
        cubicTo (cx + hw55, cy - hh, cx + hw, cy - hh55, cx + hw, cy);
        cubicTo (cx + hw, cy + hh55, cx + hw55, cy + hh, cx, cy + hh);
        cubicTo (cx - hw55, cy + hh, cx - hw, cy + hh55, cx - hw, cy);
        cubicTo (cx - hw, cy - hh55, cx - hw55, cy - hh, cx, cy - hh);
        closeSubPath();
        */
    }
    
    /**
      | Adds an elliptical arc to the current
      | path.
      | 
      | -----------
      | @note
      | 
      | when specifying the start and end angles,
      | the curve will be drawn either clockwise
      | or anti-clockwise according to whether
      | the end angle is greater than the start.
      | This means that sometimes you may need
      | to use values greater than 2*Pi for the
      | end angle.
      | 
      | -----------
      | @param x
      | 
      | the left-hand edge of the rectangle
      | in which the elliptical outline fits
      | ----------
      | @param y
      | 
      | the top edge of the rectangle in which
      | the elliptical outline fits
      | ----------
      | @param width
      | 
      | the width of the rectangle in which the
      | elliptical outline fits
      | ----------
      | @param height
      | 
      | the height of the rectangle in which
      | the elliptical outline fits
      | ----------
      | @param fromRadians
      | 
      | the angle (clockwise) in radians at
      | which to start the arc segment (where
      | 0 is the top-centre of the ellipse)
      | ----------
      | @param toRadians
      | 
      | the angle (clockwise) in radians at
      | which to end the arc segment (where 0
      | is the top-centre of the ellipse). This
      | angle can be greater than 2*Pi, so for
      | example to draw a curve clockwise from
      | the 9 o'clock position to the 3 o'clock
      | position via 12 o'clock, you'd use 1.5*Pi
      | and 2.5*Pi as the start and finish points.
      | ----------
      | @param startAsNewSubPath
      | 
      | if true, the arc will begin a new subpath
      | from its starting point; if false, it
      | will be added to the current sub-path,
      | continuing from the current position
      | 
      | @see addCentredArc, arcTo, addPieSegment,
      | addEllipse
      |
      */
    pub fn add_arc(
        &mut self, 
        x:                     f32,
        y:                     f32,
        w:                     f32,
        h:                     f32,
        from_radians:          f32,
        to_radians:            f32,
        start_as_new_sub_path: Option<bool>

    ) {

        let start_as_new_sub_path: bool = start_as_new_sub_path.unwrap_or(false);
        
        todo!();
        /*
            auto radiusX = w / 2.0f;
        auto radiusY = h / 2.0f;

        addCentredArc (x + radiusX,
                       y + radiusY,
                       radiusX, radiusY,
                       0.0f,
                       fromRadians, toRadians,
                       startAsNewSubPath);
        */
    }
    
    /**
      | Adds an arc which is centred at a given
      | point, and can have a rotation specified.
      | 
      | -----------
      | @note
      | 
      | when specifying the start and end angles,
      | the curve will be drawn either clockwise
      | or anti-clockwise according to whether
      | the end angle is greater than the start.
      | This means that sometimes you may need
      | to use values greater than 2*Pi for the
      | end angle.
      | 
      | -----------
      | @param centreX
      | 
      | the centre x of the ellipse
      | ----------
      | @param centreY
      | 
      | the centre y of the ellipse
      | ----------
      | @param radiusX
      | 
      | the horizontal radius of the ellipse
      | ----------
      | @param radiusY
      | 
      | the vertical radius of the ellipse
      | ----------
      | @param rotationOfEllipse
      | 
      | an angle by which the whole ellipse should
      | be rotated about its centre, in radians
      | (clockwise)
      | ----------
      | @param fromRadians
      | 
      | the angle (clockwise) in radians at
      | which to start the arc segment (where
      | 0 is the top-centre of the ellipse)
      | ----------
      | @param toRadians
      | 
      | the angle (clockwise) in radians at
      | which to end the arc segment (where 0
      | is the top-centre of the ellipse). This
      | angle can be greater than 2*Pi, so for
      | example to draw a curve clockwise from
      | the 9 o'clock position to the 3 o'clock
      | position via 12 o'clock, you'd use 1.5*Pi
      | and 2.5*Pi as the start and finish points.
      | ----------
      | @param startAsNewSubPath
      | 
      | if true, the arc will begin a new subpath
      | from its starting point; if false, it
      | will be added to the current sub-path,
      | continuing from the current position
      | 
      | @see addArc, arcTo
      |
      */
    pub fn add_centred_arc(
        &mut self, 
        centrex:               f32,
        centrey:               f32,
        radiusx:               f32,
        radiusy:               f32,
        rotation_of_ellipse:   f32,
        from_radians:          f32,
        to_radians:            f32,
        start_as_new_sub_path: Option<bool>

    )  {

        let start_as_new_sub_path: bool = start_as_new_sub_path.unwrap_or(false);
        
        todo!();
        /*
            if (radiusX > 0.0f && radiusY > 0.0f)
        {
            Point<float> centre (centreX, centreY);
            auto rotation = AffineTransform::rotation (rotationOfEllipse, centreX, centreY);
            auto angle = fromRadians;

            if (startAsNewSubPath)
                startNewSubPath (centre.getPointOnCircumference (radiusX, radiusY, angle).transformedBy (rotation));

            if (fromRadians < toRadians)
            {
                if (startAsNewSubPath)
                    angle += PathHelpers::ellipseAngularIncrement;

                while (angle < toRadians)
                {
                    lineTo (centre.getPointOnCircumference (radiusX, radiusY, angle).transformedBy (rotation));
                    angle += PathHelpers::ellipseAngularIncrement;
                }
            }
            else
            {
                if (startAsNewSubPath)
                    angle -= PathHelpers::ellipseAngularIncrement;

                while (angle > toRadians)
                {
                    lineTo (centre.getPointOnCircumference (radiusX, radiusY, angle).transformedBy (rotation));
                    angle -= PathHelpers::ellipseAngularIncrement;
                }
            }

            lineTo (centre.getPointOnCircumference (radiusX, radiusY, toRadians).transformedBy (rotation));
        }
        */
    }
    
    /**
      | Adds a "pie-chart" shape to the path.
      | 
      | The shape is added as a new sub-path.
      | (Any currently open paths will be left
      | open).
      | 
      | -----------
      | @note
      | 
      | when specifying the start and end angles,
      | the curve will be drawn either clockwise
      | or anti-clockwise according to whether
      | the end angle is greater than the start.
      | This means that sometimes you may need
      | to use values greater than 2*Pi for the
      | end angle.
      | 
      | -----------
      | @param x
      | 
      | the left-hand edge of the rectangle
      | in which the elliptical outline fits
      | ----------
      | @param y
      | 
      | the top edge of the rectangle in which
      | the elliptical outline fits
      | ----------
      | @param width
      | 
      | the width of the rectangle in which the
      | elliptical outline fits
      | ----------
      | @param height
      | 
      | the height of the rectangle in which
      | the elliptical outline fits
      | ----------
      | @param fromRadians
      | 
      | the angle (clockwise) in radians at
      | which to start the arc segment (where
      | 0 is the top-centre of the ellipse)
      | ----------
      | @param toRadians
      | 
      | the angle (clockwise) in radians at
      | which to end the arc segment (where 0
      | is the top-centre of the ellipse)
      | ----------
      | @param innerCircleProportionalSize
      | 
      | if this is > 0, then the pie will be drawn
      | as a curved band around a hollow ellipse
      | at its centre, where this value indicates
      | the inner ellipse's size with respect
      | to the outer one. @see addArc
      |
      */
    pub fn add_pie_segment(&mut self, 
        x:                              f32,
        y:                              f32,
        width:                          f32,
        height:                         f32,
        from_radians:                   f32,
        to_radians:                     f32,
        inner_circle_proportional_size: f32)  {
        
        todo!();
        /*
            auto radiusX = width * 0.5f;
        auto radiusY = height * 0.5f;
        Point<float> centre (x + radiusX, y + radiusY);

        startNewSubPath (centre.getPointOnCircumference (radiusX, radiusY, fromRadians));
        addArc (x, y, width, height, fromRadians, toRadians);

        if (std::abs (fromRadians - toRadians) > MathConstants<float>::pi * 1.999f)
        {
            closeSubPath();

            if (innerCircleProportionalSize > 0)
            {
                radiusX *= innerCircleProportionalSize;
                radiusY *= innerCircleProportionalSize;

                startNewSubPath (centre.getPointOnCircumference (radiusX, radiusY, toRadians));
                addArc (centre.x - radiusX, centre.y - radiusY, radiusX * 2.0f, radiusY * 2.0f, toRadians, fromRadians);
            }
        }
        else
        {
            if (innerCircleProportionalSize > 0)
            {
                radiusX *= innerCircleProportionalSize;
                radiusY *= innerCircleProportionalSize;

                addArc (centre.x - radiusX, centre.y - radiusY, radiusX * 2.0f, radiusY * 2.0f, toRadians, fromRadians);
            }
            else
            {
                lineTo (centre);
            }
        }

        closeSubPath();
        */
    }
    
    /**
      | Adds a "pie-chart" shape to the path.
      | 
      | The shape is added as a new sub-path.
      | (Any currently open paths will be left
      | open).
      | 
      | -----------
      | @note
      | 
      | when specifying the start and end angles,
      | the curve will be drawn either clockwise
      | or anti-clockwise according to whether
      | the end angle is greater than the start.
      | This means that sometimes you may need
      | to use values greater than 2*Pi for the
      | end angle.
      | 
      | -----------
      | @param segmentBounds
      | 
      | the outer rectangle in which the elliptical
      | outline fits
      | ----------
      | @param fromRadians
      | 
      | the angle (clockwise) in radians at
      | which to start the arc segment (where
      | 0 is the top-centre of the ellipse)
      | ----------
      | @param toRadians
      | 
      | the angle (clockwise) in radians at
      | which to end the arc segment (where 0
      | is the top-centre of the ellipse)
      | ----------
      | @param innerCircleProportionalSize
      | 
      | if this is > 0, then the pie will be drawn
      | as a curved band around a hollow ellipse
      | at its centre, where this value indicates
      | the inner ellipse's size with respect
      | to the outer one. @see addArc
      |
      */
    pub fn add_pie_segment_with_bounds(&mut self, 
        segment_bounds:                 Rectangle<f32>,
        from_radians:                   f32,
        to_radians:                     f32,
        inner_circle_proportional_size: f32)  {
        
        todo!();
        /*
            addPieSegment (segmentBounds.getX(),
                       segmentBounds.getY(),
                       segmentBounds.getWidth(),
                       segmentBounds.getHeight(),
                       fromRadians,
                       toRadians,
                       innerCircleProportionalSize);
        */
    }
    
    /**
      | Adds a line with a specified thickness.
      | 
      | The line is added as a new closed sub-path.
      | (Any currently open paths will be left
      | open).
      | 
      | @see addArrow
      |
      */
    pub fn add_line_segment(&mut self, 
        line:           Line<f32>,
        line_thickness: f32)  {
        
        todo!();
        /*
            auto reversed = line.reversed();
        lineThickness *= 0.5f;

        startNewSubPath (line.getPointAlongLine (0, lineThickness));
        lineTo (line.getPointAlongLine (0, -lineThickness));
        lineTo (reversed.getPointAlongLine (0, lineThickness));
        lineTo (reversed.getPointAlongLine (0, -lineThickness));
        closeSubPath();
        */
    }
    
    /**
      | Adds a line with an arrowhead on the end.
      | 
      | The arrow is added as a new closed sub-path.
      | (Any currently open paths will be left
      | open). @see PathStrokeType::createStrokeWithArrowheads
      |
      */
    pub fn add_arrow(&mut self, 
        line:             Line<f32>,
        line_thickness:   f32,
        arrowhead_width:  f32,
        arrowhead_length: f32)  {
        
        todo!();
        /*
            auto reversed = line.reversed();
        lineThickness *= 0.5f;
        arrowheadWidth *= 0.5f;
        arrowheadLength = jmin (arrowheadLength, 0.8f * line.getLength());

        startNewSubPath (line.getPointAlongLine (0, lineThickness));
        lineTo (line.getPointAlongLine (0, -lineThickness));
        lineTo (reversed.getPointAlongLine (arrowheadLength, lineThickness));
        lineTo (reversed.getPointAlongLine (arrowheadLength, arrowheadWidth));
        lineTo (line.getEnd());
        lineTo (reversed.getPointAlongLine (arrowheadLength, -arrowheadWidth));
        lineTo (reversed.getPointAlongLine (arrowheadLength, -lineThickness));
        closeSubPath();
        */
    }
    
    /**
      | Adds a polygon shape to the path. @see
      | addStar
      |
      */
    pub fn add_polygon(
        &mut self, 
        centre:          Point<f32>,
        number_of_sides: i32,
        radius:          f32,
        start_angle:     Option<f32>,

    ) {

        let start_angle: f32 = start_angle.unwrap_or(0.0);
        
        todo!();
        /*
            jassert (numberOfSides > 1); // this would be silly.

        if (numberOfSides > 1)
        {
            auto angleBetweenPoints = MathConstants<float>::twoPi / (float) numberOfSides;

            for (int i = 0; i < numberOfSides; ++i)
            {
                auto angle = startAngle + (float) i * angleBetweenPoints;
                auto p = centre.getPointOnCircumference (radius, angle);

                if (i == 0)
                    startNewSubPath (p);
                else
                    lineTo (p);
            }

            closeSubPath();
        }
        */
    }
    
    /**
      | Adds a star shape to the path. @see addPolygon
      |
      */
    pub fn add_star(
        &mut self, 
        centre:           Point<f32>,
        number_of_points: i32,
        inner_radius:     f32,
        outer_radius:     f32,
        start_angle:      Option<f32>,

    ) {

        let start_angle = start_angle.unwrap_or(0.0);
        
        todo!();
        /*
            jassert (numberOfPoints > 1); // this would be silly.

        if (numberOfPoints > 1)
        {
            auto angleBetweenPoints = MathConstants<float>::twoPi / (float) numberOfPoints;

            for (int i = 0; i < numberOfPoints; ++i)
            {
                auto angle = startAngle + (float) i * angleBetweenPoints;
                auto p = centre.getPointOnCircumference (outerRadius, angle);

                if (i == 0)
                    startNewSubPath (p);
                else
                    lineTo (p);

                lineTo (centre.getPointOnCircumference (innerRadius, angle + angleBetweenPoints * 0.5f));
            }

            closeSubPath();
        }
        */
    }
    
    /**
      | Adds a speech-bubble shape to the path.
      | 
      | -----------
      | @param bodyArea
      | 
      | the area of the body of the bubble shape
      | ----------
      | @param maximumArea
      | 
      | an area which encloses the body area
      | and defines the limits within which
      | the arrow tip can be drawn - if the tip
      | lies outside this area, the bubble will
      | be drawn without an arrow
      | ----------
      | @param arrowTipPosition
      | 
      | the location of the tip of the arrow
      | ----------
      | @param cornerSize
      | 
      | the size of the rounded corners
      | ----------
      | @param arrowBaseWidth
      | 
      | the width of the base of the arrow where
      | it joins the main rectangle
      |
      */
    pub fn add_bubble(&mut self, 
        body_area:        Rectangle<f32>,
        maximum_area:     Rectangle<f32>,
        arrow_tip:        Point<f32>,
        corner_size:      f32,
        arrow_base_width: f32)  {
        
        todo!();
        /*
            auto halfW = bodyArea.getWidth() / 2.0f;
        auto halfH = bodyArea.getHeight() / 2.0f;
        auto cornerSizeW = jmin (cornerSize, halfW);
        auto cornerSizeH = jmin (cornerSize, halfH);
        auto cornerSizeW2 = 2.0f * cornerSizeW;
        auto cornerSizeH2 = 2.0f * cornerSizeH;

        startNewSubPath (bodyArea.getX() + cornerSizeW, bodyArea.getY());

        auto targetLimit = bodyArea.reduced (jmin (halfW - 1.0f, cornerSizeW + arrowBaseWidth),
                                             jmin (halfH - 1.0f, cornerSizeH + arrowBaseWidth));

        if (Rectangle<float> (targetLimit.getX(), maximumArea.getY(),
                              targetLimit.getWidth(), bodyArea.getY() - maximumArea.getY()).contains (arrowTip))
        {
            lineTo (arrowTip.x - arrowBaseWidth, bodyArea.getY());
            lineTo (arrowTip.x, arrowTip.y);
            lineTo (arrowTip.x + arrowBaseWidth, bodyArea.getY());
        }

        lineTo (bodyArea.getRight() - cornerSizeW, bodyArea.getY());
        addArc (bodyArea.getRight() - cornerSizeW2, bodyArea.getY(), cornerSizeW2, cornerSizeH2, 0, MathConstants<float>::halfPi);

        if (Rectangle<float> (bodyArea.getRight(), targetLimit.getY(),
                              maximumArea.getRight() - bodyArea.getRight(), targetLimit.getHeight()).contains (arrowTip))
        {
            lineTo (bodyArea.getRight(), arrowTip.y - arrowBaseWidth);
            lineTo (arrowTip.x, arrowTip.y);
            lineTo (bodyArea.getRight(), arrowTip.y + arrowBaseWidth);
        }

        lineTo (bodyArea.getRight(), bodyArea.getBottom() - cornerSizeH);
        addArc (bodyArea.getRight() - cornerSizeW2, bodyArea.getBottom() - cornerSizeH2, cornerSizeW2, cornerSizeH2, MathConstants<float>::halfPi, MathConstants<float>::pi);

        if (Rectangle<float> (targetLimit.getX(), bodyArea.getBottom(),
                              targetLimit.getWidth(), maximumArea.getBottom() - bodyArea.getBottom()).contains (arrowTip))
        {
            lineTo (arrowTip.x + arrowBaseWidth, bodyArea.getBottom());
            lineTo (arrowTip.x, arrowTip.y);
            lineTo (arrowTip.x - arrowBaseWidth, bodyArea.getBottom());
        }

        lineTo (bodyArea.getX() + cornerSizeW, bodyArea.getBottom());
        addArc (bodyArea.getX(), bodyArea.getBottom() - cornerSizeH2, cornerSizeW2, cornerSizeH2, MathConstants<float>::pi, MathConstants<float>::pi * 1.5f);

        if (Rectangle<float> (maximumArea.getX(), targetLimit.getY(),
                              bodyArea.getX() - maximumArea.getX(), targetLimit.getHeight()).contains (arrowTip))
        {
            lineTo (bodyArea.getX(), arrowTip.y + arrowBaseWidth);
            lineTo (arrowTip.x, arrowTip.y);
            lineTo (bodyArea.getX(), arrowTip.y - arrowBaseWidth);
        }

        lineTo (bodyArea.getX(), bodyArea.getY() + cornerSizeH);
        addArc (bodyArea.getX(), bodyArea.getY(), cornerSizeW2, cornerSizeH2, MathConstants<float>::pi * 1.5f, MathConstants<float>::twoPi - 0.05f);

        closeSubPath();
        */
    }
    
    /**
      | Adds another path to this one.
      | 
      | The new path is added as a new sub-path.
      | (Any currently open paths in this path
      | will be left open).
      | 
      | -----------
      | @param pathToAppend
      | 
      | the path to add
      |
      */
    pub fn add_path(&mut self, other: &Path)  {
        
        todo!();
        /*
            const auto* d = other.data.begin();

        for (int i = 0; i < other.data.size();)
        {
            auto type = d[i++];

            if (isMarker (type, moveMarker))
            {
                startNewSubPath (d[i], d[i + 1]);
                i += 2;
            }
            else if (isMarker (type, lineMarker))
            {
                lineTo (d[i], d[i + 1]);
                i += 2;
            }
            else if (isMarker (type, quadMarker))
            {
                quadraticTo (d[i], d[i + 1], d[i + 2], d[i + 3]);
                i += 4;
            }
            else if (isMarker (type, cubicMarker))
            {
                cubicTo (d[i], d[i + 1], d[i + 2], d[i + 3], d[i + 4], d[i + 5]);
                i += 6;
            }
            else if (isMarker (type, closeSubPathMarker))
            {
                closeSubPath();
            }
            else
            {
                // something's gone wrong with the element list!
                jassertfalse;
            }
        }
        */
    }
    
    /**
      | Adds another path to this one, transforming
      | it on the way in.
      | 
      | The new path is added as a new sub-path,
      | its points being transformed by the
      | given matrix before being added.
      | 
      | -----------
      | @param pathToAppend
      | 
      | the path to add
      | ----------
      | @param transformToApply
      | 
      | an optional transform to apply to the
      | incoming vertices
      |
      */
    pub fn add_path_with_transform(
        &mut self, 
        other:              &Path,
        transform_to_apply: &AffineTransform
    ) {
        todo!();

        /*
            const auto* d = other.data.begin();

        for (int i = 0; i < other.data.size();)
        {
            auto type = d[i++];

            if (isMarker (type, closeSubPathMarker))
            {
                closeSubPath();
            }
            else
            {
                auto x = d[i++];
                auto y = d[i++];
                transformToApply.transformPoint (x, y);

                if (isMarker (type, moveMarker))
                {
                    startNewSubPath (x, y);
                }
                else if (isMarker (type, lineMarker))
                {
                    lineTo (x, y);
                }
                else if (isMarker (type, quadMarker))
                {
                    auto x2 = d[i++];
                    auto y2 = d[i++];
                    transformToApply.transformPoint (x2, y2);

                    quadraticTo (x, y, x2, y2);
                }
                else if (isMarker (type, cubicMarker))
                {
                    auto x2 = d[i++];
                    auto y2 = d[i++];
                    auto x3 = d[i++];
                    auto y3 = d[i++];
                    transformToApply.transformPoints (x2, y2, x3, y3);

                    cubicTo (x, y, x2, y2, x3, y3);
                }
                else
                {
                    // something's gone wrong with the element list!
                    jassertfalse;
                }
            }
        }
        */
    }
    
    /**
      | Applies a 2D transform to all the vertices
      | in the path.
      | 
      | @see AffineTransform, scaleToFit,
      | getTransformToScaleToFit
      |
      */
    pub fn apply_transform(&mut self, transform: &AffineTransform)  {
        
        todo!();
        /*
            bounds.reset();
        bool firstPoint = true;
        float* d = data.begin();
        auto* end = data.end();

        while (d < end)
        {
            auto type = *d++;

            if (isMarker (type, moveMarker))
            {
                transform.transformPoint (d[0], d[1]);
                ALOE_CHECK_COORDS_ARE_VALID (d[0], d[1])

                if (firstPoint)
                {
                    firstPoint = false;
                    bounds.reset (d[0], d[1]);
                }
                else
                {
                    bounds.extend (d[0], d[1]);
                }

                d += 2;
            }
            else if (isMarker (type, lineMarker))
            {
                transform.transformPoint (d[0], d[1]);
                ALOE_CHECK_COORDS_ARE_VALID (d[0], d[1])
                bounds.extend (d[0], d[1]);
                d += 2;
            }
            else if (isMarker (type, quadMarker))
            {
                transform.transformPoints (d[0], d[1], d[2], d[3]);
                ALOE_CHECK_COORDS_ARE_VALID (d[0], d[1])
                ALOE_CHECK_COORDS_ARE_VALID (d[2], d[3])
                bounds.extend (d[0], d[1], d[2], d[3]);
                d += 4;
            }
            else if (isMarker (type, cubicMarker))
            {
                transform.transformPoints (d[0], d[1], d[2], d[3], d[4], d[5]);
                ALOE_CHECK_COORDS_ARE_VALID (d[0], d[1])
                ALOE_CHECK_COORDS_ARE_VALID (d[2], d[3])
                ALOE_CHECK_COORDS_ARE_VALID (d[4], d[5])
                bounds.extend (d[0], d[1], d[2], d[3], d[4], d[5]);
                d += 6;
            }
        }
        */
    }
    
    /**
      | Returns a transform that can be used
      | to rescale the path to fit into a given
      | space.
      | 
      | -----------
      | @param x
      | 
      | the x position of the rectangle to fit
      | the path inside
      | ----------
      | @param y
      | 
      | the y position of the rectangle to fit
      | the path inside
      | ----------
      | @param width
      | 
      | the width of the rectangle to fit the
      | path inside
      | ----------
      | @param height
      | 
      | the height of the rectangle to fit the
      | path inside
      | ----------
      | @param preserveProportions
      | 
      | if true, it will fit the path into the
      | space without altering its horizontal/vertical
      | scale ratio; if false, it will distort
      | the path to fill the specified ratio
      | both horizontally and vertically
      | ----------
      | @param justificationType
      | 
      | if the proportions are preserved, the
      | resultant path may be smaller than the
      | available rectangle, so this describes
      | how it should be positioned within the
      | space.
      | 
      | -----------
      | @return
      | 
      | an appropriate transformation
      | 
      | @see applyTransform, scaleToFit
      |
      */
    pub fn get_transform_to_scale_to_fit_by_area(
        &self, 
        area:                 Rectangle<f32>,
        preserve_proportions: bool,
        justification:        Option<JustificationFlags>,

    ) -> AffineTransform {

        let justification: JustificationFlags = justification.unwrap_or(JustificationFlags::centred);
        
        todo!();
        /*
            return getTransformToScaleToFit (area.getX(), area.getY(), area.getWidth(), area.getHeight(),
                                         preserveProportions, justification);
        */
    }
    
    /**
      | Returns a transform that can be used
      | to rescale the path to fit into a given
      | space.
      | 
      | -----------
      | @param area
      | 
      | the rectangle to fit the path inside
      | ----------
      | @param preserveProportions
      | 
      | if true, it will fit the path into the
      | space without altering its horizontal/vertical
      | scale ratio; if false, it will distort
      | the path to fill the specified ratio
      | both horizontally and vertically
      | ----------
      | @param justificationType
      | 
      | if the proportions are preserved, the
      | resultant path may be smaller than the
      | available rectangle, so this describes
      | how it should be positioned within the
      | space.
      | 
      | -----------
      | @return
      | 
      | an appropriate transformation
      | 
      | @see applyTransform, scaleToFit
      |
      */
    pub fn get_transform_to_scale_to_fit(
        &self, 
        x:                    f32,
        y:                    f32,
        w:                    f32,
        h:                    f32,
        preserve_proportions: bool,
        justification:        Option<JustificationFlags>

    ) -> AffineTransform {

        let justification: JustificationFlags = justification.unwrap_or(JustificationFlags::centred);
        
        todo!();
        /*
            auto boundsRect = getBounds();

        if (preserveProportions)
        {
            if (w <= 0 || h <= 0 || boundsRect.isEmpty())
                return AffineTransform();

            float newW, newH;
            auto srcRatio = boundsRect.getHeight() / boundsRect.getWidth();

            if (srcRatio > h / w)
            {
                newW = h / srcRatio;
                newH = h;
            }
            else
            {
                newW = w;
                newH = w * srcRatio;
            }

            auto newXCentre = x;
            auto newYCentre = y;

            if (justification.testFlags (Justification::left))          newXCentre += newW * 0.5f;
            else if (justification.testFlags (Justification::right))    newXCentre += w - newW * 0.5f;
            else                                                        newXCentre += w * 0.5f;

            if (justification.testFlags (Justification::top))           newYCentre += newH * 0.5f;
            else if (justification.testFlags (Justification::bottom))   newYCentre += h - newH * 0.5f;
            else                                                        newYCentre += h * 0.5f;

            return AffineTransform::translation (boundsRect.getWidth()  * -0.5f - boundsRect.getX(),
                                                 boundsRect.getHeight() * -0.5f - boundsRect.getY())
                        .scaled (newW / boundsRect.getWidth(),
                                 newH / boundsRect.getHeight())
                        .translated (newXCentre, newYCentre);
        }
        else
        {
            return AffineTransform::translation (-boundsRect.getX(), -boundsRect.getY())
                        .scaled (w / boundsRect.getWidth(),
                                 h / boundsRect.getHeight())
                        .translated (x, y);
        }
        */
    }
    
    /**
      | Checks whether a point lies within the
      | path.
      | 
      | This is only relevant for closed paths
      | (see closeSubPath()), and may produce
      | false results if used on a path which
      | has open sub-paths.
      | 
      | The path's winding rule is taken into
      | account by this method.
      | 
      | The tolerance parameter is the maximum
      | error allowed when flattening the path,
      | so this method could return a false positive
      | when your point is up to this distance
      | outside the path's boundary.
      | 
      | @see closeSubPath, setUsingNonZeroWinding
      |
      */
    pub fn contains(
        &self, 
        x:         f32,
        y:         f32,
        tolerance: Option<f32>

    ) -> bool {

        let tolerance = tolerance.unwrap_or(path::DEFAULT_TOLERANCE_FOR_TESTING);
        
        todo!();
        /*
            if (x <= bounds.pathXMin || x >= bounds.pathXMax
             || y <= bounds.pathYMin || y >= bounds.pathYMax)
            return false;

        PathFlatteningIterator i (*this, AffineTransform(), tolerance);

        int positiveCrossings = 0;
        int negativeCrossings = 0;

        while (i.next())
        {
            if ((i.y1 <= y && i.y2 > y) || (i.y2 <= y && i.y1 > y))
            {
                auto intersectX = i.x1 + (i.x2 - i.x1) * (y - i.y1) / (i.y2 - i.y1);

                if (intersectX <= x)
                {
                    if (i.y1 < i.y2)
                        ++positiveCrossings;
                    else
                        ++negativeCrossings;
                }
            }
        }

        return useNonZeroWinding ? (negativeCrossings != positiveCrossings)
                                 : ((negativeCrossings + positiveCrossings) & 1) != 0;
        */
    }
    
    /**
      | Checks whether a point lies within the
      | path.
      | 
      | This is only relevant for closed paths
      | (see closeSubPath()), and may produce
      | false results if used on a path which
      | has open sub-paths.
      | 
      | The path's winding rule is taken into
      | account by this method.
      | 
      | The tolerance parameter is the maximum
      | error allowed when flattening the path,
      | so this method could return a false positive
      | when your point is up to this distance
      | outside the path's boundary.
      | 
      | @see closeSubPath, setUsingNonZeroWinding
      |
      */
    pub fn contains_point(
        &self, 
        point:     Point<f32>,
        tolerance: Option<f32>

    ) -> bool {

        let tolerance = tolerance.unwrap_or(path::DEFAULT_TOLERANCE_FOR_TESTING);
        
        todo!();
        /*
            return contains (point.x, point.y, tolerance);
        */
    }
    
    /**
      | Checks whether a line crosses the path.
      | 
      | This will return positive if the line
      | crosses any of the paths constituent
      | lines or curves. It doesn't take into
      | account whether the line is inside or
      | outside the path, or whether the path
      | is open or closed.
      | 
      | The tolerance parameter is the maximum
      | error allowed when flattening the path,
      | so this method could return a false positive
      | when your point is up to this distance
      | outside the path's boundary.
      |
      */
    pub fn intersects_line(
        &mut self, 
        line:      Line<f32>,
        tolerance: Option<f32>

    ) -> bool {

        let tolerance: f32 = tolerance.unwrap_or(path::DEFAULT_TOLERANCE_FOR_TESTING);
        
        todo!();
        /*
            PathFlatteningIterator i (*this, AffineTransform(), tolerance);
        Point<float> intersection;

        while (i.next())
            if (line.intersects (Line<float> (i.x1, i.y1, i.x2, i.y2), intersection))
                return true;

        return false;
        */
    }
    
    /**
      | Cuts off parts of a line to keep the parts
      | that are either inside or outside this
      | path.
      | 
      | -----------
      | @note
      | 
      | this isn't smart enough to cope with
      | situations where the line would need
      | to be cut into multiple pieces to correctly
      | clip against a re-entrant shape.
      | 
      | -----------
      | @param line
      | 
      | the line to clip
      | ----------
      | @param keepSectionOutsidePath
      | 
      | if true, it's the section outside the
      | path that will be kept; if false its the
      | section inside the path
      |
      */
    pub fn get_clipped_line(&self, 
        line:                      Line<f32>,
        keep_section_outside_path: bool) -> Line<f32> {
        
        todo!();
        /*
            Line<float> result (line);
        const bool startInside = contains (line.getStart());
        const bool endInside   = contains (line.getEnd());

        if (startInside == endInside)
        {
            if (keepSectionOutsidePath == startInside)
                result = Line<float>();
        }
        else
        {
            PathFlatteningIterator i (*this, AffineTransform());
            Point<float> intersection;

            while (i.next())
            {
                if (line.intersects ({ i.x1, i.y1, i.x2, i.y2 }, intersection))
                {
                    if ((startInside && keepSectionOutsidePath) || (endInside && ! keepSectionOutsidePath))
                        result.setStart (intersection);
                    else
                        result.setEnd (intersection);
                }
            }
        }

        return result;
        */
    }
    
    /**
      | Returns the length of the path. @see
      | getPointAlongPath
      |
      */
    pub fn get_length(
        &self, 
        transform: Option<&AffineTransform>,
        tolerance: Option<f32>

    ) -> f32 {

        let transform = transform.unwrap_or(&AffineTransform::default());
        let tolerance = tolerance.unwrap_or(path::DEFAULT_TOLERANCE_FOR_MEASUREMENT);
        
        todo!();
        /*
            float length = 0;
        PathFlatteningIterator i (*this, transform, tolerance);

        while (i.next())
            length += Line<float> (i.x1, i.y1, i.x2, i.y2).getLength();

        return length;
        */
    }
    
    /**
      | Returns a point that is the specified
      | distance along the path. If the distance
      | is greater than the total length of the
      | path, this will return the end point.
      | @see getLength
      |
      */
    pub fn get_point_along_path(
        &self, 
        distance_from_start: f32,
        transform:           Option<&AffineTransform>,
        tolerance:           Option<f32>

    ) -> Point<f32> {

        let transform = transform.unwrap_or(&AffineTransform::default());
        let tolerance = tolerance.unwrap_or(path::DEFAULT_TOLERANCE_FOR_MEASUREMENT);
        
        todo!();
        /*
            PathFlatteningIterator i (*this, transform, tolerance);

        while (i.next())
        {
            const Line<float> line (i.x1, i.y1, i.x2, i.y2);
            auto lineLength = line.getLength();

            if (distanceFromStart <= lineLength)
                return line.getPointAlongLine (distanceFromStart);

            distanceFromStart -= lineLength;
        }

        return { i.x2, i.y2 };
        */
    }
    
    /**
      | Finds the point along the path which
      | is nearest to a given position.
      | 
      | This sets pointOnPath to the nearest
      | point, and returns the distance of this
      | point from the start of the path.
      |
      */
    pub fn get_nearest_point(
        &self, 
        target_point:  Point<f32>,
        point_on_path: &mut Point<f32>,
        transform:     Option<&AffineTransform>,
        tolerance:     Option<f32>

    ) -> f32 {

        let transform = transform.unwrap_or(&AffineTransform::default());
        let tolerance = tolerance.unwrap_or(path::DEFAULT_TOLERANCE_FOR_MEASUREMENT);
        
        todo!();
        /*
            PathFlatteningIterator i (*this, transform, tolerance);
        float bestPosition = 0, bestDistance = std::numeric_limits<float>::max();
        float length = 0;
        Point<float> pointOnLine;

        while (i.next())
        {
            const Line<float> line (i.x1, i.y1, i.x2, i.y2);
            auto distance = line.getDistanceFromPoint (targetPoint, pointOnLine);

            if (distance < bestDistance)
            {
                bestDistance = distance;
                bestPosition = length + pointOnLine.getDistanceFrom (line.getStart());
                pointOnPath = pointOnLine;
            }

            length += line.getLength();
        }

        return bestPosition;
        */
    }
    
    /**
      | Creates a version of this path where
      | all sharp corners have been replaced
      | by curves.
      | 
      | Wherever two lines meet at an angle,
      | this will replace the corner with a curve
      | of the given radius.
      |
      */
    pub fn create_path_with_rounded_corners(&self, corner_radius: f32) -> Path {
        
        todo!();
        /*
            if (cornerRadius <= 0.01f)
            return *this;

        Path p;
        int n = 0, indexOfPathStart = 0, indexOfPathStartThis = 0;
        auto* elements = data.begin();
        bool lastWasLine = false, firstWasLine = false;

        while (n < data.size())
        {
            auto type = elements[n++];

            if (isMarker (type, moveMarker))
            {
                indexOfPathStart = p.data.size();
                indexOfPathStartThis = n - 1;
                auto x = elements[n++];
                auto y = elements[n++];
                p.startNewSubPath (x, y);
                lastWasLine = false;
                firstWasLine = (isMarker (elements[n], lineMarker));
            }
            else if (isMarker (type, lineMarker) || isMarker (type, closeSubPathMarker))
            {
                float startX = 0, startY = 0, joinX = 0, joinY = 0, endX, endY;

                if (isMarker (type, lineMarker))
                {
                    endX = elements[n++];
                    endY = elements[n++];

                    if (n > 8)
                    {
                        startX = elements[n - 8];
                        startY = elements[n - 7];
                        joinX  = elements[n - 5];
                        joinY  = elements[n - 4];
                    }
                }
                else
                {
                    endX = elements[indexOfPathStartThis + 1];
                    endY = elements[indexOfPathStartThis + 2];

                    if (n > 6)
                    {
                        startX = elements[n - 6];
                        startY = elements[n - 5];
                        joinX  = elements[n - 3];
                        joinY  = elements[n - 2];
                    }
                }

                if (lastWasLine)
                {
                    auto len1 = PathHelpers::lengthOf (startX, startY, joinX, joinY);

                    if (len1 > 0)
                    {
                        auto propNeeded = jmin (0.5, cornerRadius / len1);

                        *(p.data.end() - 2) = (float) (joinX - (joinX - startX) * propNeeded);
                        *(p.data.end() - 1) = (float) (joinY - (joinY - startY) * propNeeded);
                    }

                    auto len2 = PathHelpers::lengthOf (endX, endY, joinX, joinY);

                    if (len2 > 0)
                    {
                        auto propNeeded = jmin (0.5, cornerRadius / len2);

                        p.quadraticTo (joinX, joinY,
                                       (float) (joinX + (endX - joinX) * propNeeded),
                                       (float) (joinY + (endY - joinY) * propNeeded));
                    }

                    p.lineTo (endX, endY);
                }
                else if (isMarker (type, lineMarker))
                {
                    p.lineTo (endX, endY);
                    lastWasLine = true;
                }

                if (isMarker (type, closeSubPathMarker))
                {
                    if (firstWasLine)
                    {
                        startX = elements[n - 3];
                        startY = elements[n - 2];
                        joinX = endX;
                        joinY = endY;
                        endX = elements[indexOfPathStartThis + 4];
                        endY = elements[indexOfPathStartThis + 5];

                        auto len1 = PathHelpers::lengthOf (startX, startY, joinX, joinY);

                        if (len1 > 0)
                        {
                            auto propNeeded = jmin (0.5, cornerRadius / len1);

                            *(p.data.end() - 2) = (float) (joinX - (joinX - startX) * propNeeded);
                            *(p.data.end() - 1) = (float) (joinY - (joinY - startY) * propNeeded);
                        }

                        auto len2 = PathHelpers::lengthOf (endX, endY, joinX, joinY);

                        if (len2 > 0)
                        {
                            auto propNeeded = jmin (0.5, cornerRadius / len2);

                            endX = (float) (joinX + (endX - joinX) * propNeeded);
                            endY = (float) (joinY + (endY - joinY) * propNeeded);

                            p.quadraticTo (joinX, joinY, endX, endY);

                            p.data.begin()[indexOfPathStart + 1] = endX;
                            p.data.begin()[indexOfPathStart + 2] = endY;
                        }
                    }

                    p.closeSubPath();
                }
            }
            else if (isMarker (type, quadMarker))
            {
                lastWasLine = false;
                auto x1 = elements[n++];
                auto y1 = elements[n++];
                auto x2 = elements[n++];
                auto y2 = elements[n++];
                p.quadraticTo (x1, y1, x2, y2);
            }
            else if (isMarker (type, cubicMarker))
            {
                lastWasLine = false;
                auto x1 = elements[n++];
                auto y1 = elements[n++];
                auto x2 = elements[n++];
                auto y2 = elements[n++];
                auto x3 = elements[n++];
                auto y3 = elements[n++];
                p.cubicTo (x1, y1, x2, y2, x3, y3);
            }
        }

        return p;
        */
    }
    
    /**
      | Loads a stored path from a data stream.
      | 
      | The data in the stream must have been
      | written using writePathToStream().
      | 
      | -----------
      | @note
      | 
      | this will append the stored path to whatever
      | is currently in this path, so you might
      | need to call clear() beforehand.
      | 
      | @see loadPathFromData, writePathToStream
      |
      */
    pub fn load_path_from_stream<R: Read>(&mut self, source: &mut R)  {
        
        todo!();
        /*
            while (! source.isExhausted())
        {
            switch (source.readByte())
            {
            case 'm':
            {
                auto x = source.readFloat();
                auto y = source.readFloat();
                startNewSubPath (x, y);
                break;
            }

            case 'l':
            {
                auto x = source.readFloat();
                auto y = source.readFloat();
                lineTo (x, y);
                break;
            }

            case 'q':
            {
                auto x1 = source.readFloat();
                auto y1 = source.readFloat();
                auto x2 = source.readFloat();
                auto y2 = source.readFloat();
                quadraticTo (x1, y1, x2, y2);
                break;
            }

            case 'b':
            {
                auto x1 = source.readFloat();
                auto y1 = source.readFloat();
                auto x2 = source.readFloat();
                auto y2 = source.readFloat();
                auto x3 = source.readFloat();
                auto y3 = source.readFloat();
                cubicTo (x1, y1, x2, y2, x3, y3);
                break;
            }

            case 'c':
                closeSubPath();
                break;

            case 'n':
                useNonZeroWinding = true;
                break;

            case 'z':
                useNonZeroWinding = false;
                break;

            case 'e':
                return; // end of path marker

            default:
                jassertfalse; // illegal char in the stream
                break;
            }
        }
        */
    }
    
    /**
      | Loads a stored path from a block of data.
      | 
      | This is similar to loadPathFromStream(),
      | but just reads from a block of data. Useful
      | if you're including stored shapes in
      | your code as a block of static data.
      | 
      | @see loadPathFromStream, writePathToStream
      |
      */
    pub fn load_path_from_data(&mut self, 
        path_data:       *const c_void,
        number_of_bytes: usize)  {
        
        todo!();
        /*
            MemoryInputStream in (pathData, numberOfBytes, false);
        loadPathFromStream (in);
        */
    }
    
    /**
      | Stores the path by writing it out to a
      | stream.
      | 
      | After writing out a path, you can reload
      | it using loadPathFromStream(). @see
      | loadPathFromStream, loadPathFromData
      |
      */
    pub fn write_path_to_stream<W: Write>(&self, dest: &mut W)  {
        
        todo!();
        /*
            dest.writeByte (useNonZeroWinding ? 'n' : 'z');

        for (auto* i = data.begin(); i != data.end();)
        {
            auto type = *i++;

            if (isMarker (type, moveMarker))
            {
                dest.writeByte ('m');
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
            }
            else if (isMarker (type, lineMarker))
            {
                dest.writeByte ('l');
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
            }
            else if (isMarker (type, quadMarker))
            {
                dest.writeByte ('q');
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
            }
            else if (isMarker (type, cubicMarker))
            {
                dest.writeByte ('b');
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
                dest.writeFloat (*i++);
            }
            else if (isMarker (type, closeSubPathMarker))
            {
                dest.writeByte ('c');
            }
        }

        dest.writeByte ('e'); // marks the end-of-path
        */
    }
    
    /**
      | Creates a string containing a textual
      | representation of this path. @see restoreFromString
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            MemoryOutputStream s (2048);
        if (! useNonZeroWinding)
            s << 'a';

        float lastMarker = 0.0f;

        for (int i = 0; i < data.size();)
        {
            auto type = data.begin()[i++];
            char markerChar = 0;
            int numCoords = 0;

            if (isMarker (type, moveMarker))
            {
                markerChar = 'm';
                numCoords = 2;
            }
            else if (isMarker (type, lineMarker))
            {
                markerChar = 'l';
                numCoords = 2;
            }
            else if (isMarker (type, quadMarker))
            {
                markerChar = 'q';
                numCoords = 4;
            }
            else if (isMarker (type, cubicMarker))
            {
                markerChar = 'c';
                numCoords = 6;
            }
            else
            {
                jassert (isMarker (type, closeSubPathMarker));
                markerChar = 'z';
            }

            if (! isMarker (type, lastMarker))
            {
                if (s.getDataSize() != 0)
                    s << ' ';

                s << markerChar;
                lastMarker = type;
            }

            while (--numCoords >= 0 && i < data.size())
            {
                String coord (data.begin()[i++], 3);

                while (coord.endsWithChar ('0') && coord != "0")
                    coord = coord.dropLastCharacters (1);

                if (coord.endsWithChar ('.'))
                    coord = coord.dropLastCharacters (1);

                if (s.getDataSize() != 0)
                    s << ' ';

                s << coord;
            }
        }

        return s.toUTF8();
        */
    }
    
    /**
      | Restores this path from a string that
      | was created with the toString() method.
      | @see toString()
      |
      */
    pub fn restore_from_string(&mut self, string_version: &str)  {
        
        todo!();
        /*
            clear();
        setUsingNonZeroWinding (true);

        auto t = stringVersion.text;
        aloe_wchar marker = 'm';
        int numValues = 2;
        float values[6];

        for (;;)
        {
            auto token = PathHelpers::nextToken (t);
            auto firstChar = token[0];
            int startNum = 0;

            if (firstChar == 0)
                break;

            if (firstChar == 'm' || firstChar == 'l')
            {
                marker = firstChar;
                numValues = 2;
            }
            else if (firstChar == 'q')
            {
                marker = firstChar;
                numValues = 4;
            }
            else if (firstChar == 'c')
            {
                marker = firstChar;
                numValues = 6;
            }
            else if (firstChar == 'z')
            {
                marker = firstChar;
                numValues = 0;
            }
            else if (firstChar == 'a')
            {
                setUsingNonZeroWinding (false);
                continue;
            }
            else
            {
                ++startNum;
                values [0] = token.getFloatValue();
            }

            for (int i = startNum; i < numValues; ++i)
                values [i] = PathHelpers::nextToken (t).getFloatValue();

            switch (marker)
            {
                case 'm':   startNewSubPath (values[0], values[1]); break;
                case 'l':   lineTo (values[0], values[1]); break;
                case 'q':   quadraticTo (values[0], values[1], values[2], values[3]); break;
                case 'c':   cubicTo (values[0], values[1], values[2], values[3], values[4], values[5]); break;
                case 'z':   closeSubPath(); break;
                default:    jassertfalse; break; // illegal string format?
            }
        }
        */
    }
}
