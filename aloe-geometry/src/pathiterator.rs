crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_PathIterator.h]

/**
  | Flattens a Path object into a series
  | of straight-line sections.
  | 
  | Use one of these to iterate through a
  | Path object, and it will convert all
  | the curves into line sections so it's
  | easy to render or perform geometric
  | operations on.
  | 
  | @see Path
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct PathFlatteningIterator<'a> {

    /**
      | The x position of the start of the current
      | line segment.
      |
      */
    x1:                    f32,

    /**
      | The y position of the start of the current
      | line segment.
      |
      */
    y1:                    f32,

    /**
      | The x position of the end of the current
      | line segment.
      |
      */
    x2:                    f32,

    /**
      | The y position of the end of the current
      | line segment.
      |
      */
    y2:                    f32,

    /**
      | Indicates whether the current line segment is closing
      | a sub-path. If the current line is the one that
      | connects the end of a sub-path back to the start
      | again, this will be true.
      */
    closes_sub_path:       bool,

    /**
      | The index of the current line within the current
      | sub-path. E.g. you can use this to see whether
      | the line is the first one in the subpath by seeing
      | if it's 0.
      */
    sub_path_index:        i32,

    path:                  &'a Path,
    transform:             AffineTransform,
    source:                *const f32,
    tolerance_squared:     f32,
    sub_path_closex:       f32, // default = 0
    sub_path_closey:       f32, // default = 0
    is_identity_transform: bool,
    stack_base:            HeapBlock<f32>, // default = { 32  }
    stack_pos:             *mut f32,
    stack_size:            usize, // default = 32
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_PathIterator.cpp]
impl<'a> PathFlatteningIterator<'a> {

    /**
      | Creates a PathFlatteningIterator.
      | 
      | After creation, use the next() method
      | to initialise the fields in the object
      | with the first line's position.
      | 
      | -----------
      | @param path
      | 
      | the path to iterate along
      | ----------
      | @param transform
      | 
      | a transform to apply to each point in
      | the path being iterated
      | ----------
      | @param tolerance
      | 
      | the amount by which the curves are allowed
      | to deviate from the lines into which
      | they are being broken down - a higher
      | tolerance contains less lines, so can
      | be generated faster, but will be less
      | smooth.
      |
      */
    pub fn new(
        path_to_use: &Path,
        transform:   Option<&AffineTransform>,
        tolerance:   Option<f32>

    ) -> Self {
    
        let transform: &AffineTransform =
            transform.unwrap_or(&AffineTransform::default());

        let tolerance: f32 =
            tolerance.unwrap_or(path::DEFAULT_TOLERANCE_FOR_MEASUREMENT);

        todo!();
        /*
        : x2(0),
        : y2(0),
        : closes_sub_path(false),
        : sub_path_index(-1),
        : path(pathToUse),
        : transform(transform),
        : source(path.data.begin()),
        : tolerance_squared(tolerance * tolerance),
        : is_identity_transform(transform.isIdentity()),

            stackPos = stackBase;
        */
    }
    
    /**
      | Returns true if the current segment
      | is the last in the current sub-path.
      |
      */
    pub fn is_last_in_subpath(&self) -> bool {
        
        todo!();
        /*
            return stackPos == stackBase.get()
                 && (source == path.data.end() || isMarker (*source, Path::moveMarker));
        */
    }
    
    /**
      | Fetches the next line segment from the
      | path.
      | 
      | This will update the member variables
      | x1, y1, x2, y2, subPathIndex and closesSubPath
      | so that they describe the new line segment.
      | 
      | -----------
      | @return
      | 
      | false when there are no more lines to
      | fetch.
      |
      */
    pub fn next(&mut self) -> bool {
        
        todo!();
        /*
            x1 = x2;
        y1 = y2;

        float x3 = 0;
        float y3 = 0;
        float x4 = 0;
        float y4 = 0;

        for (;;)
        {
            float type;

            if (stackPos == stackBase.get())
            {
                if (source == path.data.end())
                    return false;

                type = *source++;

                if (! isMarker (type, Path::closeSubPathMarker))
                {
                    x2 = *source++;
                    y2 = *source++;

                    if (isMarker (type, Path::quadMarker))
                    {
                        x3 = *source++;
                        y3 = *source++;

                        if (! isIdentityTransform)
                            transform.transformPoints (x2, y2, x3, y3);
                    }
                    else if (isMarker (type, Path::cubicMarker))
                    {
                        x3 = *source++;
                        y3 = *source++;
                        x4 = *source++;
                        y4 = *source++;

                        if (! isIdentityTransform)
                            transform.transformPoints (x2, y2, x3, y3, x4, y4);
                    }
                    else
                    {
                        if (! isIdentityTransform)
                            transform.transformPoint (x2, y2);
                    }
                }
            }
            else
            {
                type = *--stackPos;

                if (! isMarker (type, Path::closeSubPathMarker))
                {
                    x2 = *--stackPos;
                    y2 = *--stackPos;

                    if (isMarker (type, Path::quadMarker))
                    {
                        x3 = *--stackPos;
                        y3 = *--stackPos;
                    }
                    else if (isMarker (type, Path::cubicMarker))
                    {
                        x3 = *--stackPos;
                        y3 = *--stackPos;
                        x4 = *--stackPos;
                        y4 = *--stackPos;
                    }
                }
            }

            if (isMarker (type, Path::lineMarker))
            {
                ++subPathIndex;

                closesSubPath = stackPos == stackBase.get()
                                 && source != path.data.end()
                                 && *source == Path::closeSubPathMarker
                                 && x2 == subPathCloseX
                                 && y2 == subPathCloseY;

                return true;
            }

            if (isMarker (type, Path::quadMarker))
            {
                const size_t offset = (size_t) (stackPos - stackBase);

                if (offset >= stackSize - 10)
                {
                    stackSize <<= 1;
                    stackBase.realloc (stackSize);
                    stackPos = stackBase + offset;
                }

                auto m1x = (x1 + x2) * 0.5f;
                auto m1y = (y1 + y2) * 0.5f;
                auto m2x = (x2 + x3) * 0.5f;
                auto m2y = (y2 + y3) * 0.5f;
                auto m3x = (m1x + m2x) * 0.5f;
                auto m3y = (m1y + m2y) * 0.5f;

                auto errorX = m3x - x2;
                auto errorY = m3y - y2;

                auto outsideTolerance = errorX * errorX + errorY * errorY > toleranceSquared;
                auto canBeSubdivided = (m3x != m1x && m3x != m2x)
                                    || (m3y != m1y && m3y != m2y);

                if (outsideTolerance && canBeSubdivided)
                {
                    *stackPos++ = y3;
                    *stackPos++ = x3;
                    *stackPos++ = m2y;
                    *stackPos++ = m2x;
                    *stackPos++ = Path::quadMarker;

                    *stackPos++ = m3y;
                    *stackPos++ = m3x;
                    *stackPos++ = m1y;
                    *stackPos++ = m1x;
                    *stackPos++ = Path::quadMarker;
                }
                else
                {
                    *stackPos++ = y3;
                    *stackPos++ = x3;
                    *stackPos++ = Path::lineMarker;

                    *stackPos++ = m3y;
                    *stackPos++ = m3x;
                    *stackPos++ = Path::lineMarker;
                }

                jassert (stackPos < stackBase + stackSize);
            }
            else if (isMarker (type, Path::cubicMarker))
            {
                const size_t offset = (size_t) (stackPos - stackBase);

                if (offset >= stackSize - 16)
                {
                    stackSize <<= 1;
                    stackBase.realloc (stackSize);
                    stackPos = stackBase + offset;
                }

                auto m1x = (x1 + x2) * 0.5f;
                auto m1y = (y1 + y2) * 0.5f;
                auto m2x = (x3 + x2) * 0.5f;
                auto m2y = (y3 + y2) * 0.5f;
                auto m3x = (x3 + x4) * 0.5f;
                auto m3y = (y3 + y4) * 0.5f;
                auto m4x = (m1x + m2x) * 0.5f;
                auto m4y = (m1y + m2y) * 0.5f;
                auto m5x = (m3x + m2x) * 0.5f;
                auto m5y = (m3y + m2y) * 0.5f;

                auto error1X = m4x - x2;
                auto error1Y = m4y - y2;
                auto error2X = m5x - x3;
                auto error2Y = m5y - y3;

                auto outsideTolerance = error1X * error1X + error1Y * error1Y > toleranceSquared
                                     || error2X * error2X + error2Y * error2Y > toleranceSquared;
                auto canBeSubdivided = (m4x != m1x && m4x != m2x)
                                    || (m4y != m1y && m4y != m2y)
                                    || (m5x != m3x && m5x != m2x)
                                    || (m5y != m3y && m5y != m2y);

                if (outsideTolerance && canBeSubdivided)
                {
                    *stackPos++ = y4;
                    *stackPos++ = x4;
                    *stackPos++ = m3y;
                    *stackPos++ = m3x;
                    *stackPos++ = m5y;
                    *stackPos++ = m5x;
                    *stackPos++ = Path::cubicMarker;

                    *stackPos++ = (m4y + m5y) * 0.5f;
                    *stackPos++ = (m4x + m5x) * 0.5f;
                    *stackPos++ = m4y;
                    *stackPos++ = m4x;
                    *stackPos++ = m1y;
                    *stackPos++ = m1x;
                    *stackPos++ = Path::cubicMarker;
                }
                else
                {
                    *stackPos++ = y4;
                    *stackPos++ = x4;
                    *stackPos++ = Path::lineMarker;

                    *stackPos++ = m5y;
                    *stackPos++ = m5x;
                    *stackPos++ = Path::lineMarker;

                    *stackPos++ = m4y;
                    *stackPos++ = m4x;
                    *stackPos++ = Path::lineMarker;
                }
            }
            else if (isMarker (type, Path::closeSubPathMarker))
            {
                if (x2 != subPathCloseX || y2 != subPathCloseY)
                {
                    x1 = x2;
                    y1 = y2;
                    x2 = subPathCloseX;
                    y2 = subPathCloseY;
                    closesSubPath = true;

                    return true;
                }
            }
            else
            {
                jassert (isMarker (type, Path::moveMarker));

                subPathIndex = -1;
                subPathCloseX = x1 = x2;
                subPathCloseY = y1 = y2;
            }
        }
        */
    }
}
