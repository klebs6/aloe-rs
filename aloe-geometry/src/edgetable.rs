crate::ix!();

pub fn get_edge_table_allocation_size(
        line_stride: i32,
        height:      i32) -> usize {
    
    todo!();
    /*
        // (leave an extra line at the end for use as scratch space)
        return (size_t) (lineStride * (2 + jmax (0, height)));
    */
}

pub const aloe_edge_table_default_edges_per_line: i32 = 32;

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_EdgeTable.h]

/**
  | A table of horizontal scan-line segments
  | - used for rasterising Paths.
  | 
  | @see Path, Graphics
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct EdgeTable {
    table:                   HeapBlock<i32>,
    bounds:                  Rectangle<i32>,
    max_edges_per_line:      i32,
    line_stride_elements:    i32,
    need_to_check_emptiness: bool, // default = true
}

impl Default for EdgeTable {

    fn default() -> Self {
        Self {
            table:                   HeapBlock::default(),
            bounds:                  Rectangle::default(),
            max_edges_per_line:      0,
            line_stride_elements:    0,
            need_to_check_emptiness: true, // default = true
        }
    }
}

pub mod edge_table {
    use super::*;
    
    /**
      | table line format: number of points; point0
      | x, point0 levelDelta, point1 x, point1
      | levelDelta, etc
      */
    #[derive(PartialEq,Eq)]
    pub struct LineItem
    {
        x:     i32,
        level: i32,
    }

    impl Ord for LineItem {
        
        #[inline] fn cmp(&self, other: &LineItem) -> std::cmp::Ordering {
            todo!();
            /*
                return x < other.x;
            */
        }
    }

    impl PartialOrd<LineItem> for LineItem {
        #[inline] fn partial_cmp(&self, other: &LineItem) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_EdgeTable.cpp]
impl EdgeTable {

    pub fn get_maximum_bounds(&self) -> &Rectangle<i32> {
        
        todo!();
        /*
            return bounds;
        */
    }
    
    /**
      | Iterates the lines in the table, for
      | rendering.
      | 
      | This function will iterate each line
      | in the table, and call a user-defined
      | class to render each pixel or continuous
      | line of pixels that the table contains.
      | 
      | -----------
      | @param iterationCallback
      | 
      | this templated class must contain the
      | following methods:
      | 
      | -----------
      | @code
      | 
      | inline void setEdgeTableYPos (int y);
      | inline void handleEdgeTablePixel (int x, int alphaLevel) const;
      | inline void handleEdgeTablePixelFull (int x) const;
      | inline void handleEdgeTableLine (int x, int width, int alphaLevel) const;
      | inline void handleEdgeTableLineFull (int x, int width) const;
      | 
      | (these don't necessarily have to be
      | 'const', but it might help it go faster)
      |
      */
    pub fn iterate<EdgeTableIterationCallback>(&self, iteration_callback: &mut EdgeTableIterationCallback)  {
    
        todo!();
        /*
            const int* lineStart = table;

            for (int y = 0; y < bounds.getHeight(); ++y)
            {
                const int* line = lineStart;
                lineStart += lineStrideElements;
                int numPoints = line[0];

                if (--numPoints > 0)
                {
                    int x = *++line;
                    jassert ((x >> 8) >= bounds.getX() && (x >> 8) < bounds.getRight());
                    int levelAccumulator = 0;

                    iterationCallback.setEdgeTableYPos (bounds.getY() + y);

                    while (--numPoints >= 0)
                    {
                        const int level = *++line;
                        jassert (isPositiveAndBelow (level, 256));
                        const int endX = *++line;
                        jassert (endX >= x);
                        const int endOfRun = (endX >> 8);

                        if (endOfRun == (x >> 8))
                        {
                            // small segment within the same pixel, so just save it for the next
                            // time round..
                            levelAccumulator += (endX - x) * level;
                        }
                        else
                        {
                            // plot the fist pixel of this segment, including any accumulated
                            // levels from smaller segments that haven't been drawn yet
                            levelAccumulator += (0x100 - (x & 0xff)) * level;
                            levelAccumulator >>= 8;
                            x >>= 8;

                            if (levelAccumulator > 0)
                            {
                                if (levelAccumulator >= 255)
                                    iterationCallback.handleEdgeTablePixelFull (x);
                                else
                                    iterationCallback.handleEdgeTablePixel (x, levelAccumulator);
                            }

                            // if there's a run of similar pixels, do it all in one go..
                            if (level > 0)
                            {
                                jassert (endOfRun <= bounds.getRight());
                                const int numPix = endOfRun - ++x;

                                if (numPix > 0)
                                    iterationCallback.handleEdgeTableLine (x, numPix, level);
                            }

                            // save the bit at the end to be drawn next time round the loop.
                            levelAccumulator = (endX & 0xff) * level;
                        }

                        x = endX;
                    }

                    levelAccumulator >>= 8;

                    if (levelAccumulator > 0)
                    {
                        x >>= 8;
                        jassert (x >= bounds.getX() && x < bounds.getRight());

                        if (levelAccumulator >= 255)
                            iterationCallback.handleEdgeTablePixelFull (x);
                        else
                            iterationCallback.handleEdgeTablePixel (x, levelAccumulator);
                    }
                }
            }
        */
    }
    
    /**
      | Creates an edge table containing a path.
      | 
      | A table is created with a fixed vertical
      | range, and only sections of the path
      | which lie within this range will be added
      | to the table.
      | 
      | -----------
      | @param clipLimits
      | 
      | only the region of the path that lies
      | within this area will be added
      | ----------
      | @param pathToAdd
      | 
      | the path to add to the table
      | ----------
      | @param transform
      | 
      | a transform to apply to the path being
      | added
      |
      */
    pub fn new_from_area_path_and_transform(
        area:      Rectangle<i32>,
        path:      &Path,
        transform: &AffineTransform) -> Self {
    
        todo!();
        /*


            : bounds (area),
         // this is a very vague heuristic to make a rough guess at a good table size
         // for a given path, such that it's big enough to mostly avoid remapping, but also
         // not so big that it's wasteful for simple paths.
         maxEdgesPerLine (jmax (aloe_edgeTableDefaultEdgesPerLine / 2,
                                4 * (int) std::sqrt (path.data.size()))),
         lineStrideElements (maxEdgesPerLine * 2 + 1)

        allocate();
        int* t = table;

        for (int i = bounds.getHeight(); --i >= 0;)
        {
            *t = 0;
            t += lineStrideElements;
        }

        auto leftLimit   = bounds.getX() * 256;
        auto topLimit    = bounds.getY() * 256;
        auto rightLimit  = bounds.getRight() * 256;
        auto heightLimit = bounds.getHeight() * 256;

        PathFlatteningIterator iter (path, transform);

        while (iter.next())
        {
            auto y1 = roundToInt (iter.y1 * 256.0f);
            auto y2 = roundToInt (iter.y2 * 256.0f);

            if (y1 != y2)
            {
                y1 -= topLimit;
                y2 -= topLimit;

                auto startY = y1;
                int direction = -1;

                if (y1 > y2)
                {
                    std::swap (y1, y2);
                    direction = 1;
                }

                if (y1 < 0)
                    y1 = 0;

                if (y2 > heightLimit)
                    y2 = heightLimit;

                if (y1 < y2)
                {
                    const double startX = 256.0f * iter.x1;
                    const double multiplier = (iter.x2 - iter.x1) / (iter.y2 - iter.y1);
                    auto stepSize = jlimit (1, 256, 256 / (1 + (int) std::abs (multiplier)));

                    do
                    {
                        auto step = jmin (stepSize, y2 - y1, 256 - (y1 & 255));
                        auto x = roundToInt (startX + multiplier * ((y1 + (step >> 1)) - startY));

                        if (x < leftLimit)
                            x = leftLimit;
                        else if (x >= rightLimit)
                            x = rightLimit - 1;

                        addEdgePoint (x, y1 >> 8, direction * step);
                        y1 += step;
                    }
                    while (y1 < y2);
                }
            }
        }

        sanitiseLevels (path.isUsingNonZeroWinding());
        */
    }
    
    /**
      | Creates an edge table containing a rectangle.
      |
      */
    pub fn new_from_i32_rectangle(rectangle_to_add: Rectangle<i32>) -> Self {
    
        todo!();
        /*


            : bounds (rectangleToAdd),
         maxEdgesPerLine (aloe_edgeTableDefaultEdgesPerLine),
         lineStrideElements (aloe_edgeTableDefaultEdgesPerLine * 2 + 1)

        allocate();
        table[0] = 0;

        auto x1 = rectangleToAdd.getX() << 8;
        auto x2 = rectangleToAdd.getRight() << 8;
        int* t = table;

        for (int i = rectangleToAdd.getHeight(); --i >= 0;)
        {
            t[0] = 2;
            t[1] = x1;
            t[2] = 255;
            t[3] = x2;
            t[4] = 0;
            t += lineStrideElements;
        }
        */
    }
    
    /**
      | Creates an edge table containing a rectangle
      | list.
      |
      */
    pub fn new_from_i32_rectangle_list(rectangles_to_add: &RectangleList<i32>) -> Self {
    
        todo!();
        /*


            : bounds (rectanglesToAdd.getBounds()),
         maxEdgesPerLine (aloe_edgeTableDefaultEdgesPerLine),
         lineStrideElements (aloe_edgeTableDefaultEdgesPerLine * 2 + 1),
         needToCheckEmptiness (true)
        allocate();
        clearLineSizes();

        for (auto& r : rectanglesToAdd)
        {
            auto x1 = r.getX() << 8;
            auto x2 = r.getRight() << 8;
            auto y = r.getY() - bounds.getY();

            for (int j = r.getHeight(); --j >= 0;)
                addEdgePointPair (x1, x2, y++, 255);
        }

        sanitiseLevels (true);
        */
    }
    
    /**
      | Creates an edge table containing a rectangle
      | list.
      |
      */
    pub fn new_from_f32_rectangle_list(rectangles_to_add: &RectangleList<f32>) -> Self {
    
        todo!();
        /*


            : bounds (rectanglesToAdd.getBounds().getSmallestIntegerContainer()),
         maxEdgesPerLine (rectanglesToAdd.getNumRectangles() * 2),
         lineStrideElements (rectanglesToAdd.getNumRectangles() * 4 + 1)

        bounds.setHeight (bounds.getHeight() + 1);
        allocate();
        clearLineSizes();

        for (auto& r : rectanglesToAdd)
        {
            auto x1 = roundToInt (r.getX() * 256.0f);
            auto x2 = roundToInt (r.getRight() * 256.0f);

            auto y1 = roundToInt (r.getY() * 256.0f) - (bounds.getY() << 8);
            auto y2 = roundToInt (r.getBottom() * 256.0f) - (bounds.getY() << 8);

            if (x2 <= x1 || y2 <= y1)
                continue;

            auto y = y1 >> 8;
            auto lastLine = y2 >> 8;

            if (y == lastLine)
            {
                addEdgePointPair (x1, x2, y, y2 - y1);
            }
            else
            {
                addEdgePointPair (x1, x2, y++, 255 - (y1 & 255));

                while (y < lastLine)
                    addEdgePointPair (x1, x2, y++, 255);

                jassert (y < bounds.getHeight());
                addEdgePointPair (x1, x2, y, y2 & 255);
            }
        }

        sanitiseLevels (true);
        */
    }
    
    /**
      | Creates an edge table containing a rectangle.
      |
      */
    pub fn new_from_f32_rectangle(rectangle_to_add: Rectangle<f32>) -> Self {
    
        todo!();
        /*


            : bounds ((int) std::floor (rectangleToAdd.getX()),
                 roundToInt (rectangleToAdd.getY() * 256.0f) >> 8,
                 2 + (int) rectangleToAdd.getWidth(),
                 2 + (int) rectangleToAdd.getHeight()),
         maxEdgesPerLine (aloe_edgeTableDefaultEdgesPerLine),
         lineStrideElements ((aloe_edgeTableDefaultEdgesPerLine * 2) + 1)

        jassert (! rectangleToAdd.isEmpty());
        allocate();
        table[0] = 0;

        auto x1 = roundToInt (rectangleToAdd.getX()      * 256.0f);
        auto x2 = roundToInt (rectangleToAdd.getRight()  * 256.0f);
        auto y1 = roundToInt (rectangleToAdd.getY()      * 256.0f) - (bounds.getY() << 8);
        auto y2 = roundToInt (rectangleToAdd.getBottom() * 256.0f) - (bounds.getY() << 8);
        jassert (y1 < 256);

        if (x2 <= x1 || y2 <= y1)
        {
            bounds.setHeight (0);
            return;
        }

        int lineY = 0;
        int* t = table;

        if ((y1 >> 8) == (y2 >> 8))
        {
            t[0] = 2;
            t[1] = x1;
            t[2] = y2 - y1;
            t[3] = x2;
            t[4] = 0;
            ++lineY;
            t += lineStrideElements;
        }
        else
        {
            t[0] = 2;
            t[1] = x1;
            t[2] = 255 - (y1 & 255);
            t[3] = x2;
            t[4] = 0;
            ++lineY;
            t += lineStrideElements;

            while (lineY < (y2 >> 8))
            {
                t[0] = 2;
                t[1] = x1;
                t[2] = 255;
                t[3] = x2;
                t[4] = 0;
                ++lineY;
                t += lineStrideElements;
            }

            jassert (lineY < bounds.getHeight());
            t[0] = 2;
            t[1] = x1;
            t[2] = y2 & 255;
            t[3] = x2;
            t[4] = 0;
            ++lineY;
            t += lineStrideElements;
        }

        while (lineY < bounds.getHeight())
        {
            t[0] = 0;
            t += lineStrideElements;
            ++lineY;
        }
        */
    }
    
    /**
      | Creates a copy of another edge table.
      |
      */
    pub fn new_from_edge_table(other: &EdgeTable) -> Self {
    
        todo!();
        /*
            operator= (other);
        */
    }
    
    /**
      | Copies from another edge table.
      |
      */
    pub fn assign_from(&mut self, other: &EdgeTable) -> &mut EdgeTable {
        
        todo!();
        /*
            bounds = other.bounds;
        maxEdgesPerLine = other.maxEdgesPerLine;
        lineStrideElements = other.lineStrideElements;
        needToCheckEmptiness = other.needToCheckEmptiness;

        allocate();
        copyEdgeTableData (table, lineStrideElements, other.table, lineStrideElements, bounds.getHeight());
        return *this;
        */
    }
    
    pub fn allocate(&mut self)  {
        
        todo!();
        /*
            table.malloc (getEdgeTableAllocationSize (lineStrideElements, bounds.getHeight()));
        */
    }
    
    pub fn clear_line_sizes(&mut self)  {
        
        todo!();
        /*
            int* t = table;

        for (int i = bounds.getHeight(); --i >= 0;)
        {
            *t = 0;
            t += lineStrideElements;
        }
        */
    }
    
    pub fn copy_edge_table_data(&mut self, 
        dest:             *mut i32,
        dest_line_stride: i32,
        src:              *const i32,
        src_line_stride:  i32,
        num_lines:        i32)  {
        
        todo!();
        /*
            while (--numLines >= 0)
        {
            memcpy (dest, src, (size_t) (src[0] * 2 + 1) * sizeof (int));
            src += srcLineStride;
            dest += destLineStride;
        }
        */
    }
    
    pub fn sanitise_levels(&mut self, use_non_zero_winding: bool)  {
        
        todo!();
        /*
            // Convert the table from relative windings to absolute levels..
        int* lineStart = table;

        for (int y = bounds.getHeight(); --y >= 0;)
        {
            auto num = lineStart[0];

            if (num > 0)
            {
                auto* items = reinterpret_cast<LineItem*> (lineStart + 1);
                auto* itemsEnd = items + num;

                // sort the X coords
                std::sort (items, itemsEnd);

                auto* src = items;
                auto correctedNum = num;
                int level = 0;

                while (src < itemsEnd)
                {
                    level += src->level;
                    auto x = src->x;
                    ++src;

                    while (src < itemsEnd && src->x == x)
                    {
                        level += src->level;
                        ++src;
                        --correctedNum;
                    }

                    auto corrected = std::abs (level);

                    if (corrected >> 8)
                    {
                        if (useNonZeroWinding)
                        {
                            corrected = 255;
                        }
                        else
                        {
                            corrected &= 511;

                            if (corrected >> 8)
                                corrected = 511 - corrected;
                        }
                    }

                    items->x = x;
                    items->level = corrected;
                    ++items;
                }

                lineStart[0] = correctedNum;
                (items - 1)->level = 0; // force the last level to 0, just in case something went wrong in creating the table
            }

            lineStart += lineStrideElements;
        }
        */
    }
    
    pub fn remap_table_for_num_edges(&mut self, new_num_edges_per_line: i32)  {
        
        todo!();
        /*
            if (newNumEdgesPerLine != maxEdgesPerLine)
        {
            maxEdgesPerLine = newNumEdgesPerLine;

            jassert (bounds.getHeight() > 0);
            auto newLineStrideElements = maxEdgesPerLine * 2 + 1;

            HeapBlock<int> newTable (getEdgeTableAllocationSize (newLineStrideElements, bounds.getHeight()));

            copyEdgeTableData (newTable, newLineStrideElements, table, lineStrideElements, bounds.getHeight());

            table.swapWith (newTable);
            lineStrideElements = newLineStrideElements;
        }
        */
    }
    
    #[inline] pub fn remap_with_extra_space(&mut self, num_points: i32)  {
        
        todo!();
        /*
            remapTableForNumEdges (numPoints * 2);
        jassert (numPoints < maxEdgesPerLine);
        */
    }
    
    /**
      | Reduces the amount of space the table
      | has allocated.
      | 
      | This will shrink the table down to use
      | as little memory as possible - useful
      | for read-only tables that get stored
      | and re-used for rendering.
      |
      */
    pub fn optimise_table(&mut self)  {
        
        todo!();
        /*
            int maxLineElements = 0;

        for (int i = bounds.getHeight(); --i >= 0;)
            maxLineElements = jmax (maxLineElements, table[i * lineStrideElements]);

        remapTableForNumEdges (maxLineElements);
        */
    }
    
    pub fn add_edge_point(&mut self, 
        x:       i32,
        y:       i32,
        winding: i32)  {
        
        todo!();
        /*
            jassert (y >= 0 && y < bounds.getHeight());

        auto* line = table + lineStrideElements * y;
        auto numPoints = line[0];

        if (numPoints >= maxEdgesPerLine)
        {
            remapWithExtraSpace (numPoints);
            line = table + lineStrideElements * y;
        }

        line[0] = numPoints + 1;
        line += numPoints * 2;
        line[1] = x;
        line[2] = winding;
        */
    }
    
    pub fn add_edge_point_pair(&mut self, 
        x1:      i32,
        x2:      i32,
        y:       i32,
        winding: i32)  {
        
        todo!();
        /*
            jassert (y >= 0 && y < bounds.getHeight());

        auto* line = table + lineStrideElements * y;
        auto numPoints = line[0];

        if (numPoints + 1 >= maxEdgesPerLine)
        {
            remapWithExtraSpace (numPoints + 1);
            line = table + lineStrideElements * y;
        }

        line[0] = numPoints + 2;
        line += numPoints * 2;
        line[1] = x1;
        line[2] = winding;
        line[3] = x2;
        line[4] = -winding;
        */
    }
    
    pub fn translate(&mut self, dx: f32, dy: i32)  {
        
        todo!();
        /*
            bounds.translate ((int) std::floor (dx), dy);

        int* lineStart = table;
        auto intDx = (int) (dx * 256.0f);

        for (int i = bounds.getHeight(); --i >= 0;)
        {
            auto* line = lineStart;
            lineStart += lineStrideElements;
            auto num = *line++;

            while (--num >= 0)
            {
                *line += intDx;
                line += 2;
            }
        }
        */
    }
    
    /**
      | Scales all the alpha-levels in the table
      | by the given multiplier.
      |
      */
    pub fn multiply_levels(&mut self, amount: f32)  {
        
        todo!();
        /*
            int* lineStart = table;
        auto multiplier = (int) (amount * 256.0f);

        for (int y = 0; y < bounds.getHeight(); ++y)
        {
            auto numPoints = lineStart[0];
            auto* item = reinterpret_cast<LineItem*> (lineStart + 1);
            lineStart += lineStrideElements;

            while (--numPoints > 0)
            {
                item->level = jmin (255, (item->level * multiplier) >> 8);
                ++item;
            }
        }
        */
    }
    
    pub fn intersect_with_edge_table_line(&mut self, 
        y:          i32,
        other_line: *const i32)  {
        
        todo!();
        /*
            jassert (y >= 0 && y < bounds.getHeight());

        auto* srcLine = table + lineStrideElements * y;
        auto srcNum1 = *srcLine;

        if (srcNum1 == 0)
            return;

        auto srcNum2 = *otherLine;

        if (srcNum2 == 0)
        {
            *srcLine = 0;
            return;
        }

        auto right = bounds.getRight() << 8;

        // optimise for the common case where our line lies entirely within a
        // single pair of points, as happens when clipping to a simple rect.
        if (srcNum2 == 2 && otherLine[2] >= 255)
        {
            clipEdgeTableLineToRange (srcLine, otherLine[1], jmin (right, otherLine[3]));
            return;
        }

        bool isUsingTempSpace = false;

        const int* src1 = srcLine + 1;
        auto x1 = *src1++;

        const int* src2 = otherLine + 1;
        auto x2 = *src2++;

        int destIndex = 0, destTotal = 0;
        int level1 = 0, level2 = 0;
        int lastX = std::numeric_limits<int>::min(), lastLevel = 0;

        while (srcNum1 > 0 && srcNum2 > 0)
        {
            int nextX;

            if (x1 <= x2)
            {
                if (x1 == x2)
                {
                    level2 = *src2++;
                    x2 = *src2++;
                    --srcNum2;
                }

                nextX = x1;
                level1 = *src1++;
                x1 = *src1++;
                --srcNum1;
            }
            else
            {
                nextX = x2;
                level2 = *src2++;
                x2 = *src2++;
                --srcNum2;
            }

            if (nextX > lastX)
            {
                if (nextX >= right)
                    break;

                lastX = nextX;

                auto nextLevel = (level1 * (level2 + 1)) >> 8;
                jassert (isPositiveAndBelow (nextLevel, 256));

                if (nextLevel != lastLevel)
                {
                    if (destTotal >= maxEdgesPerLine)
                    {
                        srcLine[0] = destTotal;

                        if (isUsingTempSpace)
                        {
                            auto tempSize = (size_t) srcNum1 * 2 * sizeof (int);
                            auto oldTemp = static_cast<int*> (alloca (tempSize));
                            memcpy (oldTemp, src1, tempSize);

                            remapTableForNumEdges (jmax (256, destTotal * 2));
                            srcLine = table + lineStrideElements * y;

                            auto* newTemp = table + lineStrideElements * bounds.getHeight();
                            memcpy (newTemp, oldTemp, tempSize);
                            src1 = newTemp;
                        }
                        else
                        {
                            remapTableForNumEdges (jmax (256, destTotal * 2));
                            srcLine = table + lineStrideElements * y;
                        }
                    }

                    ++destTotal;
                    lastLevel = nextLevel;

                    if (! isUsingTempSpace)
                    {
                        isUsingTempSpace = true;
                        auto* temp = table + lineStrideElements * bounds.getHeight();
                        memcpy (temp, src1, (size_t) srcNum1 * 2 * sizeof (int));
                        src1 = temp;
                    }

                    srcLine[++destIndex] = nextX;
                    srcLine[++destIndex] = nextLevel;
                }
            }
        }

        if (lastLevel > 0)
        {
            if (destTotal >= maxEdgesPerLine)
            {
                srcLine[0] = destTotal;
                remapTableForNumEdges (jmax (256, destTotal * 2));
                srcLine = table + lineStrideElements * y;
            }

            ++destTotal;
            srcLine[++destIndex] = right;
            srcLine[++destIndex] = 0;
        }

        srcLine[0] = destTotal;
        */
    }
    
    pub fn clip_edge_table_line_to_range(&mut self, 
        dest: *mut i32,
        x1:   i32,
        x2:   i32)  {
        
        todo!();
        /*
            int* lastItem = dest + (dest[0] * 2 - 1);

        if (x2 < lastItem[0])
        {
            if (x2 <= dest[1])
            {
                dest[0] = 0;
                return;
            }

            while (x2 < lastItem[-2])
            {
                --(dest[0]);
                lastItem -= 2;
            }

            lastItem[0] = x2;
            lastItem[1] = 0;
        }

        if (x1 > dest[1])
        {
            while (lastItem[0] > x1)
                lastItem -= 2;

            auto itemsRemoved = (int) (lastItem - (dest + 1)) / 2;

            if (itemsRemoved > 0)
            {
                dest[0] -= itemsRemoved;
                memmove (dest + 1, lastItem, (size_t) dest[0] * (sizeof (int) * 2));
            }

            dest[1] = x1;
        }
        */
    }
    
    pub fn clip_to_rectangle(&mut self, r: Rectangle<i32>)  {
        
        todo!();
        /*
            auto clipped = r.getIntersection (bounds);

        if (clipped.isEmpty())
        {
            needToCheckEmptiness = false;
            bounds.setHeight (0);
        }
        else
        {
            auto top = clipped.getY() - bounds.getY();
            auto bottom = clipped.getBottom() - bounds.getY();

            if (bottom < bounds.getHeight())
                bounds.setHeight (bottom);

            for (int i = 0; i < top; ++i)
                table[lineStrideElements * i] = 0;

            if (clipped.getX() > bounds.getX() || clipped.getRight() < bounds.getRight())
            {
                auto x1 = clipped.getX() << 8;
                auto x2 = jmin (bounds.getRight(), clipped.getRight()) << 8;
                int* line = table + lineStrideElements * top;

                for (int i = bottom - top; --i >= 0;)
                {
                    if (line[0] != 0)
                        clipEdgeTableLineToRange (line, x1, x2);

                    line += lineStrideElements;
                }
            }

            needToCheckEmptiness = true;
        }
        */
    }
    
    pub fn exclude_rectangle(&mut self, r: Rectangle<i32>)  {
        
        todo!();
        /*
            auto clipped = r.getIntersection (bounds);

        if (! clipped.isEmpty())
        {
            auto top = clipped.getY() - bounds.getY();
            auto bottom = clipped.getBottom() - bounds.getY();

            const int rectLine[] = { 4, std::numeric_limits<int>::min(), 255,
                                     clipped.getX() << 8, 0,
                                     clipped.getRight() << 8, 255,
                                     std::numeric_limits<int>::max(), 0 };

            for (int i = top; i < bottom; ++i)
                intersectWithEdgeTableLine (i, rectLine);

            needToCheckEmptiness = true;
        }
        */
    }
    
    pub fn clip_to_edge_table(&mut self, other: &EdgeTable)  {
        
        todo!();
        /*
            auto clipped = other.bounds.getIntersection (bounds);

        if (clipped.isEmpty())
        {
            needToCheckEmptiness = false;
            bounds.setHeight (0);
        }
        else
        {
            auto top = clipped.getY() - bounds.getY();
            auto bottom = clipped.getBottom() - bounds.getY();

            if (bottom < bounds.getHeight())
                bounds.setHeight (bottom);

            if (clipped.getRight() < bounds.getRight())
                bounds.setRight (clipped.getRight());

            for (int i = 0; i < top; ++i)
                table[lineStrideElements * i] = 0;

            auto* otherLine = other.table + other.lineStrideElements * (clipped.getY() - other.bounds.getY());

            for (int i = top; i < bottom; ++i)
            {
                intersectWithEdgeTableLine (i, otherLine);
                otherLine += other.lineStrideElements;
            }

            needToCheckEmptiness = true;
        }
        */
    }
    
    pub fn clip_line_to_mask(&mut self, 
        x:           i32,
        y:           i32,
        mask:        *const u8,
        mask_stride: i32,
        num_pixels:  i32)  {
        
        todo!();
        /*
            y -= bounds.getY();

        if (y < 0 || y >= bounds.getHeight())
            return;

        needToCheckEmptiness = true;

        if (numPixels <= 0)
        {
            table[lineStrideElements * y] = 0;
            return;
        }

        auto* tempLine = static_cast<int*> (alloca ((size_t) (numPixels * 2 + 4) * sizeof (int)));
        int destIndex = 0, lastLevel = 0;

        while (--numPixels >= 0)
        {
            auto alpha = *mask;
            mask += maskStride;

            if (alpha != lastLevel)
            {
                tempLine[++destIndex] = (x << 8);
                tempLine[++destIndex] = alpha;
                lastLevel = alpha;
            }

            ++x;
        }

        if (lastLevel > 0)
        {
            tempLine[++destIndex] = (x << 8);
            tempLine[++destIndex] = 0;
        }

        tempLine[0] = destIndex >> 1;

        intersectWithEdgeTableLine (y, tempLine);
        */
    }
    
    pub fn is_empty(&mut self) -> bool {
        
        todo!();
        /*
            if (needToCheckEmptiness)
        {
            needToCheckEmptiness = false;
            int* t = table;

            for (int i = bounds.getHeight(); --i >= 0;)
            {
                if (t[0] > 1)
                    return false;

                t += lineStrideElements;
            }

            bounds.setHeight (0);
        }

        return bounds.getHeight() == 0;
        */
    }
}
