/*!
  | Contains classes for calculating the
  | colour of pixels within various types
  | of gradient.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_RenderingHelpers.h]

/**
  | Iterates the colour of pixels in a linear
  | gradient
  |
  */
#[no_copy]
pub struct GradientPixelColorLinear {
    lookup_table: *const PixelARGB,
    num_entries:  i32,
    line_pix:     PixelARGB,
    start:        i32,
    scale:        i32,
    grad:         f64,
    y_term:       f64,
    vertical:     bool,
    horizontal:   bool,
}

impl GradientPixelColorLinear {

    pub const numScaleBits: usize = 12;

    pub fn new(
        gradient:    &ColourGradient,
        transform:   &AffineTransform,
        colours:     *const PixelARGB,
        num_colours: i32) -> Self {
    
        todo!();
        /*


            : lookupTable (colours),
                  numEntries (numColours)

                jassert (numColours >= 0);
                auto p1 = gradient.point1;
                auto p2 = gradient.point2;

                if (! transform.isIdentity())
                {
                    auto p3 = Line<float> (p2, p1).getPointAlongLine (0.0f, 100.0f);

                    p1.applyTransform (transform);
                    p2.applyTransform (transform);
                    p3.applyTransform (transform);

                    p2 = Line<float> (p2, p3).findNearestPointTo (p1);
                }

                vertical   = std::abs (p1.x - p2.x) < 0.001f;
                horizontal = std::abs (p1.y - p2.y) < 0.001f;

                if (vertical)
                {
                    scale = roundToInt ((double) ((int64_t) numEntries << (int) numScaleBits) / (double) (p2.y - p1.y));
                    start = roundToInt (p1.y * (float) scale);
                }
                else if (horizontal)
                {
                    scale = roundToInt ((double) ((int64_t) numEntries << (int) numScaleBits) / (double) (p2.x - p1.x));
                    start = roundToInt (p1.x * (float) scale);
                }
                else
                {
                    grad = (p2.getY() - p1.y) / (double) (p1.x - p2.x);
                    yTerm = p1.getY() - p1.x / grad;
                    scale = roundToInt ((double) ((int64_t) numEntries << (int) numScaleBits) / (yTerm * grad - (p2.y * grad - p2.x)));
                    grad *= scale;
                }
        */
    }
    
    #[inline(always)] pub fn sety(&mut self, y: i32)  {
        
        todo!();
        /*
            if (vertical)
                    linePix = lookupTable[jlimit (0, numEntries, (y * scale - start) >> (int) numScaleBits)];
                else if (! horizontal)
                    start = roundToInt ((y - yTerm) * grad);
        */
    }
    
    #[inline] pub fn get_pixel(&self, x: i32) -> PixelARGB {
        
        todo!();
        /*
            return vertical ? linePix
                                : lookupTable[jlimit (0, numEntries, (x * scale - start) >> (int) numScaleBits)];
        */
    }
}

/**
  | Iterates the colour of pixels in a circular
  | radial gradient
  |
  */
#[no_copy]
pub struct GradientPixelColorRadial {
    lookup_table: *const PixelARGB,
    num_entries:  i32,
    gx1:          f64,
    gy1:          f64,
    max_dist:     f64,
    inv_scale:    f64,
    dy:           f64,
}

impl GradientPixelColorRadial {

    pub fn new(
        gradient:    &ColourGradient,
        _1:          &AffineTransform,
        colours:     *const PixelARGB,
        num_colours: i32) -> Self {
    
        todo!();
        /*


            : lookupTable (colours),
                  numEntries (numColours),
                  gx1 (gradient.point1.x),
                  gy1 (gradient.point1.y)
                jassert (numColours >= 0);
                auto diff = gradient.point1 - gradient.point2;
                maxDist = diff.x * diff.x + diff.y * diff.y;
                invScale = numEntries / std::sqrt (maxDist);
                jassert (roundToInt (std::sqrt (maxDist) * invScale) <= numEntries);
        */
    }
    
    #[inline(always)] pub fn sety(&mut self, y: i32)  {
        
        todo!();
        /*
            dy = y - gy1;
                dy *= dy;
        */
    }
    
    #[inline] pub fn get_pixel(&self, px: i32) -> PixelARGB {
        
        todo!();
        /*
            auto x = px - gx1;
                x *= x;
                x += dy;

                return lookupTable[x >= maxDist ? numEntries : roundToInt (std::sqrt (x) * invScale)];
        */
    }
}

/**
  | Iterates the colour of pixels in a skewed
  | radial gradient
  |
  */
#[no_copy]
pub struct GradientPixelColorTransformedRadial {
    base:              GradientPixelColorRadial,
    tm10:              f64,
    tm00:              f64,
    lineym01:          f64,
    lineym11:          f64,
    inverse_transform: AffineTransform,
}

impl GradientPixelColorTransformedRadial {

    pub fn new(
        gradient:    &ColourGradient,
        transform:   &AffineTransform,
        colours:     *const PixelARGB,
        num_colours: i32) -> Self {
    
        todo!();
        /*


            : Radial (gradient, transform, colours, numColours),
                  inverseTransform (transform.inverted())

                tM10 = inverseTransform.mat10;
                tM00 = inverseTransform.mat00;
        */
    }
    
    #[inline(always)] pub fn sety(&mut self, y: i32)  {
        
        todo!();
        /*
            auto floatY = (float) y;
                lineYM01 = inverseTransform.mat01 * floatY + inverseTransform.mat02 - gx1;
                lineYM11 = inverseTransform.mat11 * floatY + inverseTransform.mat12 - gy1;
        */
    }
    
    #[inline] pub fn get_pixel(&self, px: i32) -> PixelARGB {
        
        todo!();
        /*
            double x = px;
                auto y = tM10 * x + lineYM11;
                x = tM00 * x + lineYM01;
                x *= x;
                x += y * y;

                if (x >= maxDist)
                    return lookupTable[numEntries];

                return lookupTable[jmin (numEntries, roundToInt (std::sqrt (x) * invScale))];
        */
    }
}

macro_rules! aloe_perform_pixel_op_loop {
    ($op:ident) => {
        /*
        
        { 
            const int destStride = destData.pixelStride;  
            do { dest->op; dest = addBytesToPointer (dest, destStride); } while (--width > 0); 
        }
        */
    }
}
