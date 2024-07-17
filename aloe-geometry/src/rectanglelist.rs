crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_RectangleList.h]

/**
  | Maintains a set of rectangles as a complex
  | region.
  | 
  | This class allows a set of rectangles
  | to be treated as a solid shape, and can
  | add and remove rectangular sections
  | of it, and simplify overlapping or adjacent
  | rectangles.
  | 
  | @see Rectangle
  | 
  | @tags{Graphics}
  |
  */
pub struct RectangleList<ValueType> 
    where ValueType: Copy + Clone
{
    rects: Vec<<Self as HasRectangleType<ValueType>>::RectangleType>,
}

pub trait HasRectangleType<ValueType> 
    where ValueType: Copy + Clone
{
    type RectangleType = Rectangle<ValueType>;
}

impl<ValueType: Copy + Clone> HasRectangleType<ValueType> for RectangleList<ValueType> { }
    
impl<ValueType: Copy + Clone> Default for RectangleList<ValueType> {
    
    /**
      | Creates an empty RectangleList
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<ValueType: Copy + Clone> RectangleList<ValueType> {

    /**
      | Creates a copy of another list
      |
      */
    pub fn new_from_rectangle_list_ref(other: &RectangleList<ValueType>) -> Self {
    
        todo!();
        /*

            : rects (other.rects)
        */
    }

    /**
      | Creates a list containing just one rectangle.
      |
      */
    pub fn new_from_rectangle(rect: <Self as HasRectangleType<ValueType>>::RectangleType) -> Self {
    
        todo!();
        /*

            addWithoutMerging (rect);
        */
    }

    /**
      | Copies this list from another one.
      |
      */
    pub fn assign_from_rectangle_list_ref(&mut self, other: &RectangleList<ValueType>) -> &mut RectangleList<ValueType> {
        
        todo!();
        /*
            rects = other.rects;
            return *this;
        */
    }

    /**
      | Move constructor
      |
      */
    pub fn new_from_rectangle_list(other: RectangleList<ValueType>) -> Self {
    
        todo!();
        /*

            : rects (std::move (other.rects))
        */
    }

    /**
      | Move assignment operator
      |
      */
    pub fn assign_from(&mut self, other: RectangleList<ValueType>) -> &mut RectangleList<ValueType> {
        
        todo!();
        /*
            rects = std::move (other.rects);
            return *this;
        */
    }

    
    /**
      | Returns true if the region is empty.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return rects.isEmpty();
        */
    }

    /**
      | Returns the number of rectangles in
      | the list.
      |
      */
    pub fn get_num_rectangles(&self) -> i32 {
        
        todo!();
        /*
            return rects.size();
        */
    }

    /**
      | Returns one of the rectangles at a particular
      | index.
      | 
      | 
      | -----------
      | @return
      | 
      | the rectangle at the index, or an empty
      | rectangle if the index is out-of-range.
      |
      */
    pub fn get_rectangle(&self, index: i32) -> <Self as HasRectangleType<ValueType>>::RectangleType {
        
        todo!();
        /*
            return rects[index];
        */
    }

    
    /**
      | Removes all rectangles to leave an empty
      | region.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            rects.clearQuick();
        */
    }

    /**
      | Merges a new rectangle into the list.
      | 
      | The rectangle being added will first
      | be clipped to remove any parts of it that
      | overlap existing rectangles in the
      | list, and adjacent rectangles will
      | be merged into it.
      | 
      | The rectangle can have any size and may
      | be empty, but if it's floating point
      | then it's expected to not contain any
      | INF values.
      |
      */
    pub fn add_rect(&mut self, rect: <Self as HasRectangleType<ValueType>>::RectangleType )  {
        
        todo!();
        /*
            jassert (rect.isFinite()); // You must provide a valid rectangle to this method!

            if (! rect.isEmpty())
            {
                if (isEmpty())
                {
                    rects.add (rect);
                }
                else
                {
                    bool anyOverlaps = false;

                    for (int j = rects.size(); --j >= 0;)
                    {
                        auto& ourRect = rects.getReference (j);

                        if (rect.intersects (ourRect))
                        {
                            if (rect.contains (ourRect))
                                rects.remove (j);
                            else if (! ourRect.reduceIfPartlyContainedIn (rect))
                                anyOverlaps = true;
                        }
                    }

                    if (anyOverlaps && ! isEmpty())
                    {
                        RectangleList r (rect);

                        for (auto& ourRect : rects)
                        {
                            if (rect.intersects (ourRect))
                            {
                                r.subtract (ourRect);

                                if (r.isEmpty())
                                    return;
                            }
                        }

                        rects.addArray (r.rects);
                    }
                    else
                    {
                        rects.add (rect);
                    }
                }
            }
        */
    }

    /**
      | Merges a new rectangle into the list.
      | 
      | The rectangle being added will first
      | be clipped to remove any parts of it that
      | overlap existing rectangles in the
      | list.
      |
      */
    pub fn add_xywh(
        &mut self, 
        x:      ValueType,
        y:      ValueType,
        width:  ValueType,
        height: ValueType)  {
        
        todo!();
        /*
            add (RectangleType (x, y, width, height));
        */
    }

    /**
      | Dumbly adds a rectangle to the list without
      | checking for overlaps.
      | 
      | This simply adds the rectangle to the
      | end, it doesn't merge it or remove any
      | overlapping bits.
      | 
      | The rectangle can have any size and may
      | be empty, but if it's floating point
      | then it's expected to not contain any
      | INF values.
      |
      */
    pub fn add_without_merging(&mut self, rect: <Self as HasRectangleType<ValueType>>::RectangleType)  {
        
        todo!();
        /*
            jassert (rect.isFinite()); // You must provide a valid rectangle to this method!

            if (! rect.isEmpty())
                rects.add (rect);
        */
    }

    /**
      | Merges another rectangle list into
      | this one.
      | 
      | Any overlaps between the two lists will
      | be clipped, so that the result is the
      | union of both lists.
      |
      */
    pub fn add_rectangle_list_ref(&mut self, other: &RectangleList<ValueType>)  {
        
        todo!();
        /*
            for (auto& r : other)
                add (r);
        */
    }

    /**
      | Removes a rectangular region from the
      | list.
      | 
      | Any rectangles in the list which overlap
      | this will be clipped and subdivided
      | if necessary.
      |
      */
    pub fn subtract_rect(&mut self, rect: <Self as HasRectangleType<ValueType>>::RectangleType)  {
        
        todo!();
        /*
            if (auto numRects = rects.size())
            {
                auto x1 = rect.getX();
                auto y1 = rect.getY();
                auto x2 = x1 + rect.getWidth();
                auto y2 = y1 + rect.getHeight();

                for (int i = numRects; --i >= 0;)
                {
                    auto& r = rects.getReference (i);

                    auto rx1 = r.getX();
                    auto ry1 = r.getY();
                    auto rx2 = rx1 + r.getWidth();
                    auto ry2 = ry1 + r.getHeight();

                    if (! (x2 <= rx1 || x1 >= rx2 || y2 <= ry1 || y1 >= ry2))
                    {
                        if (x1 > rx1 && x1 < rx2)
                        {
                            if (y1 <= ry1 && y2 >= ry2 && x2 >= rx2)
                            {
                                r.setWidth (x1 - rx1);
                            }
                            else
                            {
                                r.setX (x1);
                                r.setWidth (rx2 - x1);

                                rects.insert (++i, RectangleType (rx1, ry1, x1 - rx1,  ry2 - ry1));
                                ++i;
                            }
                        }
                        else if (x2 > rx1 && x2 < rx2)
                        {
                            r.setX (x2);
                            r.setWidth (rx2 - x2);

                            if (y1 > ry1 || y2 < ry2 || x1 > rx1)
                            {
                                rects.insert (++i, RectangleType (rx1, ry1, x2 - rx1,  ry2 - ry1));
                                ++i;
                            }
                        }
                        else if (y1 > ry1 && y1 < ry2)
                        {
                            if (x1 <= rx1 && x2 >= rx2 && y2 >= ry2)
                            {
                                r.setHeight (y1 - ry1);
                            }
                            else
                            {
                                r.setY (y1);
                                r.setHeight (ry2 - y1);

                                rects.insert (++i, RectangleType (rx1, ry1, rx2 - rx1, y1 - ry1));
                                ++i;
                            }
                        }
                        else if (y2 > ry1 && y2 < ry2)
                        {
                            r.setY (y2);
                            r.setHeight (ry2 - y2);

                            if (x1 > rx1 || x2 < rx2 || y1 > ry1)
                            {
                                rects.insert (++i, RectangleType (rx1, ry1, rx2 - rx1, y2 - ry1));
                                ++i;
                            }
                        }
                        else
                        {
                            rects.remove (i);
                        }
                    }
                }
            }
        */
    }

    /**
      | Removes all areas in another RectangleList
      | from this one.
      | 
      | Any rectangles in the list which overlap
      | this will be clipped and subdivided
      | if necessary.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the resulting list is non-empty.
      |
      */
    pub fn subtract(&mut self, other_list: &RectangleList<ValueType>) -> bool {
        
        todo!();
        /*
            for (auto& r : otherList)
            {
                if (isEmpty())
                    return false;

                subtract (r);
            }

            return ! isEmpty();
        */
    }

    /**
      | Removes any areas of the region that
      | lie outside a given rectangle.
      | 
      | Any rectangles in the list which overlap
      | this will be clipped and subdivided
      | if necessary.
      | 
      | Returns true if the resulting region
      | is not empty, false if it is empty.
      | 
      | @see getIntersectionWith
      |
      */
    pub fn clip_to_rect(&mut self, rect: <Self as HasRectangleType<ValueType>>::RectangleType) -> bool {
        
        todo!();
        /*
            jassert (rect.isFinite()); // You must provide a valid rectangle to this method!

            bool notEmpty = false;

            if (rect.isEmpty())
            {
                clear();
            }
            else
            {
                for (int i = rects.size(); --i >= 0;)
                {
                    auto& r = rects.getReference (i);

                    if (! rect.intersectRectangle (r))
                        rects.remove (i);
                    else
                        notEmpty = true;
                }
            }

            return notEmpty;
        */
    }

    /**
      | Removes any areas of the region that
      | lie outside a given rectangle list.
      | 
      | Any rectangles in this object which
      | overlap the specified list will be clipped
      | and subdivided if necessary.
      | 
      | Returns true if the resulting region
      | is not empty, false if it is empty.
      | 
      | @see getIntersectionWith
      |
      */
    pub fn clip_to<OtherValueType>(&mut self, other: &RectangleList<OtherValueType>) -> bool 
        where OtherValueType: Copy + Clone
    {
    
        todo!();
        /*
            if (isEmpty())
                return false;

            RectangleList result;

            for (auto& rect : rects)
            {
                for (auto& r : other)
                {
                    auto clipped = r.template toType<ValueType>();

                    if (rect.intersectRectangle (clipped))
                        result.rects.add (clipped);
                }
            }

            swapWith (result);
            return ! isEmpty();
        */
    }

    /**
      | Creates a region which is the result
      | of clipping this one to a given rectangle.
      | 
      | Unlike the other clipTo method, this
      | one doesn't affect this object - it puts
      | the resulting region into the list whose
      | reference is passed-in.
      | 
      | Returns true if the resulting region
      | is not empty, false if it is empty.
      | 
      | @see clipTo
      |
      */
    pub fn get_intersection_with(&self, 
        rect:        <Self as HasRectangleType<ValueType>>::RectangleType,
        dest_region: &mut RectangleList<ValueType>) -> bool {
        
        todo!();
        /*
            jassert (rect.isFinite()); // You must provide a valid rectangle to this method!

            destRegion.clear();

            if (! rect.isEmpty())
                for (auto r : rects)
                    if (rect.intersectRectangle (r))
                        destRegion.rects.add (r);

            return ! destRegion.isEmpty();
        */
    }

    /**
      | Swaps the contents of this and another
      | list.
      | 
      | This swaps their internal pointers,
      | so is hugely faster than using copy-by-value
      | to swap them.
      |
      */
    pub fn swap_with(&mut self, other_list: &mut RectangleList<ValueType>)  {
        
        todo!();
        /*
            rects.swapWith (otherList.rects);
        */
    }

    
    /**
      | Checks whether the region contains
      | a given point.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the point lies within one of the
      | rectangles in the list
      |
      */
    pub fn contains_point(&self, point: Point<ValueType>) -> bool {
        
        todo!();
        /*
            for (auto& r : rects)
                if (r.contains (point))
                    return true;

            return false;
        */
    }

    /**
      | Checks whether the region contains
      | a given point.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the point lies within one of the
      | rectangles in the list
      |
      */
    pub fn contains_point_by_xy(
        &self, 
        x: ValueType,
        y: ValueType) -> bool {
        
        todo!();
        /*
            return containsPoint (Point<ValueType> (x, y));
        */
    }

    /**
      | Checks whether the region contains
      | the whole of a given rectangle.
      | 
      | 
      | -----------
      | @return
      | 
      | true all parts of the rectangle passed
      | in lie within the region defined by this
      | object @see intersectsRectangle,
      | containsPoint
      |
      */
    pub fn contains_rectangle(
        &self, 
        rectangle_to_check: <Self as HasRectangleType<ValueType>>::RectangleType
    ) -> bool {

        todo!();
        /*
            if (rects.size() > 1)
            {
                RectangleList r (rectangleToCheck);

                for (auto& rect : rects)
                {
                    r.subtract (rect);

                    if (r.isEmpty())
                        return true;
                }
            }
            else if (! isEmpty())
            {
                return rects.getReference (0).contains (rectangleToCheck);
            }

            return false;
        */
    }

    /**
      | Checks whether the region contains
      | any part of a given rectangle.
      | 
      | 
      | -----------
      | @return
      | 
      | true if any part of the rectangle passed
      | in lies within the region defined by
      | this object @see containsRectangle
      |
      */
    pub fn intersects_rectangle(
        &self, 
        rectangle_to_check: <Self as HasRectangleType<ValueType>>::RectangleType) -> bool {
        
        todo!();
        /*
            for (auto& r : rects)
                if (r.intersects (rectangleToCheck))
                    return true;

            return false;
        */
    }

    /**
      | Checks whether this region intersects
      | any part of another one. @see intersectsRectangle
      |
      */
    pub fn intersects(&self, other: &RectangleList<ValueType>) -> bool {
        
        todo!();
        /*
            for (auto& r : rects)
                if (other.intersectsRectangle (r))
                    return true;

            return false;
        */
    }

    
    /**
      | Returns the smallest rectangle that
      | can enclose the whole of this region.
      |
      */
    pub fn get_bounds(&self) -> <Self as HasRectangleType<ValueType>>::RectangleType {
        
        todo!();
        /*
            if (isEmpty())
                return {};

            auto& r = rects.getReference (0);

            if (rects.size() == 1)
                return r;

            auto minX = r.getX();
            auto minY = r.getY();
            auto maxX = minX + r.getWidth();
            auto maxY = minY + r.getHeight();

            for (int i = rects.size(); --i > 0;)
            {
                auto& r2 = rects.getReference (i);

                minX = jmin (minX, r2.getX());
                minY = jmin (minY, r2.getY());
                maxX = jmax (maxX, r2.getRight());
                maxY = jmax (maxY, r2.getBottom());
            }

            return { minX, minY, maxX - minX, maxY - minY };
        */
    }

    /**
      | Optimises the list into a minimum number
      | of constituent rectangles.
      | 
      | This will try to combine any adjacent
      | rectangles into larger ones where possible,
      | to simplify lists that might have been
      | fragmented by repeated add/subtract
      | calls.
      |
      */
    pub fn consolidate(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < rects.size() - 1; ++i)
            {
                auto& r = rects.getReference (i);
                auto rx1 = r.getX();
                auto ry1 = r.getY();
                auto rx2 = rx1 + r.getWidth();
                auto ry2 = ry1 + r.getHeight();

                for (int j = rects.size(); --j > i;)
                {
                    auto& r2 = rects.getReference (j);
                    auto jrx1 = r2.getX();
                    auto jry1 = r2.getY();
                    auto jrx2 = jrx1 + r2.getWidth();
                    auto jry2 = jry1 + r2.getHeight();

                    // if the vertical edges of any blocks are touching and their horizontals don't
                    // line up, split them horizontally..
                    if (jrx1 == rx2 || jrx2 == rx1)
                    {
                        if (jry1 > ry1 && jry1 < ry2)
                        {
                            r.setHeight (jry1 - ry1);
                            rects.add (RectangleType (rx1, jry1, rx2 - rx1, ry2 - jry1));
                            i = -1;
                            break;
                        }

                        if (jry2 > ry1 && jry2 < ry2)
                        {
                            r.setHeight (jry2 - ry1);
                            rects.add (RectangleType (rx1, jry2, rx2 - rx1, ry2 - jry2));
                            i = -1;
                            break;
                        }
                        else if (ry1 > jry1 && ry1 < jry2)
                        {
                            r2.setHeight (ry1 - jry1);
                            rects.add (RectangleType (jrx1, ry1, jrx2 - jrx1, jry2 - ry1));
                            i = -1;
                            break;
                        }
                        else if (ry2 > jry1 && ry2 < jry2)
                        {
                            r2.setHeight (ry2 - jry1);
                            rects.add (RectangleType (jrx1, ry2, jrx2 - jrx1, jry2 - ry2));
                            i = -1;
                            break;
                        }
                    }
                }
            }

            for (int i = 0; i < rects.size() - 1; ++i)
            {
                auto& r = rects.getReference (i);

                for (int j = rects.size(); --j > i;)
                {
                    if (r.enlargeIfAdjacent (rects.getReference (j)))
                    {
                        rects.remove (j);
                        i = -1;
                        break;
                    }
                }
            }
        */
    }

    /**
      | Adds an x and y value to all the coordinates.
      |
      */
    pub fn offset_all_from_point(&mut self, offset: Point<ValueType>)  {
        
        todo!();
        /*
            for (auto& r : rects)
                r += offset;
        */
    }

    /**
      | Adds an x and y value to all the coordinates.
      |
      */
    pub fn offset_all(
        &mut self, 
        dx: ValueType,
        dy: ValueType
    ) {
        
        todo!();
        /*
            offsetAll (Point<ValueType> (dx, dy));
        */
    }

    /**
      | Scales all the coordinates.
      |
      */
    pub fn scale_all<ScaleType>(&mut self, scale_factor: ScaleType)  {
    
        todo!();
        /*
            for (auto& r : rects)
                r *= scaleFactor;
        */
    }

    /**
      | Applies a transform to all the rectangles.
      | 
      | Obviously this will create a mess if
      | the transform involves any rotation
      | or skewing.
      |
      */
    pub fn transform_all(&mut self, transform: &AffineTransform)  {
        
        todo!();
        /*
            for (auto& r : rects)
                r = r.transformedBy (transform);
        */
    }

    /**
      | Creates a Path object to represent this
      | region.
      |
      */
    pub fn to_path(&self) -> PathBuf {
        
        todo!();
        /*
            Path p;

            for (auto& r : rects)
                p.addRectangle (r);

            return p;
        */
    }
    
    /**
      | Standard method for iterating the rectangles
      | in the list.
      |
      */
    pub fn begin(&self) -> *const <Self as HasRectangleType<ValueType>>::RectangleType {
        
        todo!();
        /*
            return rects.begin();
        */
    }

    /**
      | Standard method for iterating the rectangles
      | in the list.
      |
      */
    pub fn end(&self) -> *const <Self as HasRectangleType<ValueType>>::RectangleType {
        
        todo!();
        /*
            return rects.end();
        */
    }

    /**
      | Increases the internal storage to hold
      | a minimum number of rectangles.
      | 
      | Calling this before adding a large number
      | of rectangles means that the array won't
      | have to keep dynamically resizing itself
      | as the elements are added, and it'll
      | therefore be more efficient. @see Array::ensureStorageAllocated
      |
      */
    pub fn ensure_storage_allocated(&mut self, min_num_rectangles: i32)  {
        
        todo!();
        /*
            rects.ensureStorageAllocated (minNumRectangles);
        */
    }
}
