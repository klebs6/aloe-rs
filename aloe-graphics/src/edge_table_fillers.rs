/*!
  | Contains classes for filling edge tables
  | with various fill types.
  |
  */
crate::ix!();

/**
  | Fills an edge-table with a solid colour.
  |
  */
#[no_copy]
pub struct SolidColour<'a, PixelType,const replaceExisting: bool = false> {
    dest_data:                &'a ImageBitmapData,
    line_pixels:              *mut PixelType,
    source_colour:            PixelARGB,
    are_rgb_components_equal: bool,
}

impl<'a, PixelType,const ReplaceExisting: bool> SolidColour<'a, PixelType,ReplaceExisting> {

    pub fn new(
        image:  &ImageBitmapData,
        colour: PixelARGB) -> Self {
    
        todo!();
        /*
        : dest_data(image),
        : source_colour(colour),

            if (sizeof (PixelType) == 3 && (size_t) destData.pixelStride == sizeof (PixelType))
                    areRGBComponentsEqual = sourceColour.getRed() == sourceColour.getGreen()
                                                && sourceColour.getGreen() == sourceColour.getBlue();
                else
                    areRGBComponentsEqual = false;
        */
    }
    
    #[inline(always)] pub fn set_edge_table_ypos(&mut self, y: i32)  {
        
        todo!();
        /*
            linePixels = (PixelType*) destData.getLinePointer (y);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel(&self, 
        x:           i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            if (replaceExisting)
                    getPixel (x)->set (sourceColour);
                else
                    getPixel (x)->blend (sourceColour, (uint32) alphaLevel);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel_full(&self, x: i32)  {
        
        todo!();
        /*
            if (replaceExisting)
                    getPixel (x)->set (sourceColour);
                else
                    getPixel (x)->blend (sourceColour);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_line(&self, 
        x:           i32,
        width:       i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            auto p = sourceColour;
                p.multiplyAlpha (alphaLevel);

                auto* dest = getPixel (x);

                if (replaceExisting || p.getAlpha() >= 0xff)
                    replaceLine (dest, p, width);
                else
                    blendLine (dest, p, width);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_line_full(&self, 
        x:     i32,
        width: i32)  {
        
        todo!();
        /*
            auto* dest = getPixel (x);

                if (replaceExisting || sourceColour.getAlpha() >= 0xff)
                    replaceLine (dest, sourceColour, width);
                else
                    blendLine (dest, sourceColour, width);
        */
    }
    
    pub fn handle_edge_table_rectangle(&mut self, 
        x:           i32,
        y:           i32,
        width:       i32,
        height:      i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            auto p = sourceColour;
                p.multiplyAlpha (alphaLevel);

                setEdgeTableYPos (y);
                auto* dest = getPixel (x);

                if (replaceExisting || p.getAlpha() >= 0xff)
                {
                    while (--height >= 0)
                    {
                        replaceLine (dest, p, width);
                        dest = addBytesToPointer (dest, destData.lineStride);
                    }
                }
                else
                {
                    while (--height >= 0)
                    {
                        blendLine (dest, p, width);
                        dest = addBytesToPointer (dest, destData.lineStride);
                    }
                }
        */
    }
    
    pub fn handle_edge_table_rectangle_full(&mut self, 
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            handleEdgeTableRectangle (x, y, width, height, 255);
        */
    }
    
    #[inline(always)] pub fn get_pixel(&self, x: i32) -> *mut PixelType {
        
        todo!();
        /*
            return addBytesToPointer (linePixels, x * destData.pixelStride);
        */
    }
    
    #[inline] pub fn blend_line(&self, 
        dest:   *mut PixelType,
        colour: PixelARGB,
        width:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_PIXEL_OP_LOOP (blend (colour))
        */
    }
    
    #[inline(always)] 
    pub fn replace_line_rgb(
        &self, 
        dest:   *mut PixelRGB,
        colour: PixelARGB,
        width:  i32)  {
        
        todo!();
        /*
            if ((size_t) destData.pixelStride == sizeof (*dest) && areRGBComponentsEqual)
                    memset ((void*) dest, colour.getRed(), (size_t) width * 3);   // if all the component values are the same, we can cheat..
                else
                    ALOE_PERFORM_PIXEL_OP_LOOP (set (colour));
        */
    }
    
    #[inline(always)] 
    pub fn replace_line_alpha(
        &self, 
        dest:   *mut PixelAlpha,
        colour: PixelARGB,
        width:  i32)  {
        
        todo!();
        /*
            if ((size_t) destData.pixelStride == sizeof (*dest))
                    memset ((void*) dest, colour.getAlpha(), (size_t) width);
                else
                    ALOE_PERFORM_PIXEL_OP_LOOP (setAlpha (colour.getAlpha()))
        */
    }
    
    #[inline(always)] 
    pub fn replace_line_argb(
        &self, 
        dest:   *mut PixelARGB,
        colour: PixelARGB,
        width:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_PIXEL_OP_LOOP (set (colour))
        */
    }
}

/**
  | Fills an edge-table with a gradient.
  |
  */
#[no_copy]
pub struct Gradient<'a, PixelType,GradientType> {
    base:        GradientType,
    dest_data:   &'a ImageBitmapData,
    line_pixels: *mut PixelType,
}

impl<'a,PixelType,GradientType> Gradient<'a,PixelType,GradientType> {

    pub fn new(
        dest:        &ImageBitmapData,
        gradient:    &ColourGradient,
        transform:   &AffineTransform,
        colours:     *const PixelARGB,
        num_colours: i32) -> Self {
    
        todo!();
        /*


            : GradientType (gradient, transform, colours, numColours - 1),
                  destData (dest)
        */
    }
    
    #[inline(always)] pub fn set_edge_table_ypos(&mut self, y: i32)  {
        
        todo!();
        /*
            linePixels = (PixelType*) destData.getLinePointer (y);
                GradientType::setY (y);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel(&self, 
        x:           i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            getPixel (x)->blend (GradientType::getPixel (x), (uint32) alphaLevel);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel_full(&self, x: i32)  {
        
        todo!();
        /*
            getPixel (x)->blend (GradientType::getPixel (x));
        */
    }
    
    pub fn handle_edge_table_line(&self, 
        x:           i32,
        width:       i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            auto* dest = getPixel (x);

                if (alphaLevel < 0xff)
                    ALOE_PERFORM_PIXEL_OP_LOOP (blend (GradientType::getPixel (x++), (uint32) alphaLevel))
                else
                    ALOE_PERFORM_PIXEL_OP_LOOP (blend (GradientType::getPixel (x++)))
        */
    }
    
    pub fn handle_edge_table_line_full(&self, 
        x:     i32,
        width: i32)  {
        
        todo!();
        /*
            auto* dest = getPixel (x);
                ALOE_PERFORM_PIXEL_OP_LOOP (blend (GradientType::getPixel (x++)))
        */
    }
    
    pub fn handle_edge_table_rectangle(&mut self, 
        x:           i32,
        y:           i32,
        width:       i32,
        height:      i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLine (x, width, alphaLevel);
                }
        */
    }
    
    pub fn handle_edge_table_rectangle_full(&mut self, 
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLineFull (x, width);
                }
        */
    }
    
    #[inline(always)] pub fn get_pixel(&self, x: i32) -> *mut PixelType {
        
        todo!();
        /*
            return addBytesToPointer (linePixels, x * destData.pixelStride);
        */
    }
}

/**
  | Fills an edge-table with a non-transformed
  | image.
  |
  */
#[no_copy]
pub struct ImageFill<'a, DestPixelType,SrcPixelType,const repeatPattern: bool> {
    dest_data:         &'a ImageBitmapData,
    src_data:          &'a ImageBitmapData,
    extra_alpha:       i32,
    x_offset:          i32,
    y_offset:          i32,
    line_pixels:       *mut DestPixelType,
    source_line_start: *mut SrcPixelType,
}

impl<'a, DestPixelType, SrcPixelType, const RepeatPattern: bool> 

    ImageFill<'a, DestPixelType,SrcPixelType,RepeatPattern> {

    pub fn new(
        dest:  &ImageBitmapData,
        src:   &ImageBitmapData,
        alpha: i32,
        x:     i32,
        y:     i32) -> Self {
    
        todo!();
        /*


            : destData (dest),
                  srcData (src),
                  extraAlpha (alpha + 1),
                  xOffset (repeatPattern ? negativeAwareModulo (x, src.width)  - src.width  : x),
                  yOffset (repeatPattern ? negativeAwareModulo (y, src.height) - src.height : y)
        */
    }
    
    #[inline(always)] pub fn set_edge_table_ypos(&mut self, y: i32)  {
        
        todo!();
        /*
            linePixels = (DestPixelType*) destData.getLinePointer (y);
                y -= yOffset;

                if (repeatPattern)
                {
                    jassert (y >= 0);
                    y %= srcData.height;
                }

                sourceLineStart = (SrcPixelType*) srcData.getLinePointer (y);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel(&self, 
        x:           i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            alphaLevel = (alphaLevel * extraAlpha) >> 8;

                getDestPixel (x)->blend (*getSrcPixel (repeatPattern ? ((x - xOffset) % srcData.width) : (x - xOffset)), (uint32) alphaLevel);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel_full(&self, x: i32)  {
        
        todo!();
        /*
            getDestPixel (x)->blend (*getSrcPixel (repeatPattern ? ((x - xOffset) % srcData.width) : (x - xOffset)), (uint32) extraAlpha);
        */
    }
    
    pub fn handle_edge_table_line(&self, 
        x:           i32,
        width:       i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            auto* dest = getDestPixel (x);
                alphaLevel = (alphaLevel * extraAlpha) >> 8;
                x -= xOffset;

                if (repeatPattern)
                {
                    if (alphaLevel < 0xfe)
                        ALOE_PERFORM_PIXEL_OP_LOOP (blend (*getSrcPixel (x++ % srcData.width), (uint32) alphaLevel))
                    else
                        ALOE_PERFORM_PIXEL_OP_LOOP (blend (*getSrcPixel (x++ % srcData.width)))
                }
                else
                {
                    jassert (x >= 0 && x + width <= srcData.width);

                    if (alphaLevel < 0xfe)
                        ALOE_PERFORM_PIXEL_OP_LOOP (blend (*getSrcPixel (x++), (uint32) alphaLevel))
                    else
                        copyRow (dest, getSrcPixel (x), width);
                }
        */
    }
    
    pub fn handle_edge_table_line_full(&self, 
        x:     i32,
        width: i32)  {
        
        todo!();
        /*
            auto* dest = getDestPixel (x);
                x -= xOffset;

                if (repeatPattern)
                {
                    if (extraAlpha < 0xfe)
                        ALOE_PERFORM_PIXEL_OP_LOOP (blend (*getSrcPixel (x++ % srcData.width), (uint32) extraAlpha))
                    else
                        ALOE_PERFORM_PIXEL_OP_LOOP (blend (*getSrcPixel (x++ % srcData.width)))
                }
                else
                {
                    jassert (x >= 0 && x + width <= srcData.width);

                    if (extraAlpha < 0xfe)
                        ALOE_PERFORM_PIXEL_OP_LOOP (blend (*getSrcPixel (x++), (uint32) extraAlpha))
                    else
                        copyRow (dest, getSrcPixel (x), width);
                }
        */
    }
    
    pub fn handle_edge_table_rectangle(&mut self, 
        x:           i32,
        y:           i32,
        width:       i32,
        height:      i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLine (x, width, alphaLevel);
                }
        */
    }
    
    pub fn handle_edge_table_rectangle_full(&mut self, 
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLineFull (x, width);
                }
        */
    }
    
    pub fn clip_edge_table_line(&mut self, 
        et:    &mut EdgeTable,
        x:     i32,
        y:     i32,
        width: i32)  {
        
        todo!();
        /*
            jassert (x - xOffset >= 0 && x + width - xOffset <= srcData.width);
                auto* s = (SrcPixelType*) srcData.getLinePointer (y - yOffset);
                auto* mask = (uint8*) (s + x - xOffset);

                if (sizeof (SrcPixelType) == sizeof (PixelARGB))
                    mask += PixelARGB::indexA;

                et.clipLineToMask (x, y, mask, sizeof (SrcPixelType), width);
        */
    }
    
    #[inline(always)] pub fn get_dest_pixel(&self, x: i32) -> *mut DestPixelType {
        
        todo!();
        /*
            return addBytesToPointer (linePixels, x * destData.pixelStride);
        */
    }
    
    #[inline(always)] pub fn get_src_pixel(&self, x: i32) -> *mut SrcPixelType {
        
        todo!();
        /*
            return addBytesToPointer (sourceLineStart, x * srcData.pixelStride);
        */
    }
    
    #[inline(always)] pub fn copy_row(&self, 
        dest:  *mut DestPixelType,
        src:   *const SrcPixelType,
        width: i32)  {
        
        todo!();
        /*
            auto destStride = destData.pixelStride;
                auto srcStride  = srcData.pixelStride;

                if (destStride == srcStride
                     && srcData.pixelFormat  == typename Image::RGB
                     && destData.pixelFormat == typename Image::RGB)
                {
                    memcpy ((void*) dest, src, (size_t) (width * srcStride));
                }
                else
                {
                    do
                    {
                        dest->blend (*src);
                        dest = addBytesToPointer (dest, destStride);
                        src  = addBytesToPointer (src, srcStride);
                    } while (--width > 0);
                }
        */
    }
}

/**
  | Fills an edge-table with a transformed
  | image.
  |
  */
#[no_copy]
pub struct TransformedImageFill<'a, DestPixelType,SrcPixelType,const repeatPattern: bool> {
    interpolator:   transformed_image_fill::TransformedImageSpanInterpolator,
    dest_data:      &'a ImageBitmapData,
    src_data:       &'a ImageBitmapData,
    extra_alpha:    i32,
    quality:        GraphicsResamplingQuality,
    maxx:           i32,
    maxy:           i32,
    currenty:       i32,
    line_pixels:    *mut DestPixelType,
    scratch_buffer: HeapBlock<SrcPixelType>,
    scratch_size:   usize, // default = 2048
}

pub mod transformed_image_fill {

    use super::*;
        
    #[no_copy]
    pub struct TransformedImageSpanInterpolator {
        inverse_transform: AffineTransform,
        x_bresenham:       transformed_image_span_interpolator::BresenhamInterpolator,
        y_bresenham:       transformed_image_span_interpolator::BresenhamInterpolator,
        pixel_offset:      f32,
        pixel_offset_int:  i32,
    }

    pub mod transformed_image_span_interpolator {

        use super::*;

        #[derive(Default)]
        pub struct BresenhamInterpolator {
            n:         i32,
            num_steps: i32,
            step:      i32,
            modulo:    i32,
            remainder: i32,
        }

        impl BresenhamInterpolator {

            pub fn set(&mut self, 
                n1:         i32,
                n2:         i32,
                steps:      i32,
                offset_int: i32)  {
                
                todo!();
                /*
                    numSteps = steps;
                                step = (n2 - n1) / numSteps;
                                remainder = modulo = (n2 - n1) % numSteps;
                                n = n1 + offsetInt;

                                if (modulo <= 0)
                                {
                                    modulo += numSteps;
                                    remainder += numSteps;
                                    --step;
                                }

                                modulo -= numSteps;
                */
            }
            
            #[inline(always)] pub fn step_to_next(&mut self)  {
                
                todo!();
                /*
                    modulo += remainder;
                                n += step;

                                if (modulo > 0)
                                {
                                    modulo -= numSteps;
                                    ++n;
                                }
                */
            }
        }
    }

    impl TransformedImageSpanInterpolator {

        pub fn new(
            transform:    &AffineTransform,
            offset_float: f32,
            offset_int:   i32) -> Self {
        
            todo!();
            /*


                : inverseTransform (transform.inverted()),
                          pixelOffset (offsetFloat), pixelOffsetInt (offsetInt)
            */
        }
        
        pub fn set_start_of_line(&mut self, 
            sx:         f32,
            sy:         f32,
            num_pixels: i32)  {
            
            todo!();
            /*
                jassert (numPixels > 0);

                        sx += pixelOffset;
                        sy += pixelOffset;
                        auto x1 = sx, y1 = sy;
                        sx += (float) numPixels;
                        inverseTransform.transformPoints (x1, y1, sx, sy);

                        xBresenham.set ((int) (x1 * 256.0f), (int) (sx * 256.0f), numPixels, pixelOffsetInt);
                        yBresenham.set ((int) (y1 * 256.0f), (int) (sy * 256.0f), numPixels, pixelOffsetInt);
            */
        }
        
        pub fn next(&mut self, 
            px: &mut i32,
            py: &mut i32)  {
            
            todo!();
            /*
                px = xBresenham.n;  xBresenham.stepToNext();
                        py = yBresenham.n;  yBresenham.stepToNext();
            */
        }
    }
}

impl<'a, DestPixelType,SrcPixelType,const RepeatPattern: bool> 

    TransformedImageFill<'a, DestPixelType,SrcPixelType,RepeatPattern> {

    pub fn new(
        dest:      &ImageBitmapData,
        src:       &ImageBitmapData,
        transform: &AffineTransform,
        alpha:     i32,
        q:         GraphicsResamplingQuality) -> Self {
    
        todo!();
        /*


            : interpolator (transform,
                                q != typename Graphics::lowResamplingQuality ? 0.5f : 0.0f,
                                q != typename Graphics::lowResamplingQuality ? -128 : 0),
                  destData (dest),
                  srcData (src),
                  extraAlpha (alpha + 1),
                  quality (q),
                  maxX (src.width  - 1),
                  maxY (src.height - 1)

                scratchBuffer.malloc (scratchSize);
        */
    }
    
    #[inline(always)] pub fn set_edge_table_ypos(&mut self, newy: i32)  {
        
        todo!();
        /*
            currentY = newY;
                linePixels = (DestPixelType*) destData.getLinePointer (newY);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel(&mut self, 
        x:           i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            SrcPixelType p;
                generate (&p, x, 1);

                getDestPixel (x)->blend (p, (uint32) (alphaLevel * extraAlpha) >> 8);
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_pixel_full(&mut self, x: i32)  {
        
        todo!();
        /*
            SrcPixelType p;
                generate (&p, x, 1);

                getDestPixel (x)->blend (p, (uint32) extraAlpha);
        */
    }
    
    pub fn handle_edge_table_line(&mut self, 
        x:           i32,
        width:       i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            if (width > (int) scratchSize)
                {
                    scratchSize = (size_t) width;
                    scratchBuffer.malloc (scratchSize);
                }

                SrcPixelType* span = scratchBuffer;
                generate (span, x, width);

                auto* dest = getDestPixel (x);
                alphaLevel *= extraAlpha;
                alphaLevel >>= 8;

                if (alphaLevel < 0xfe)
                    ALOE_PERFORM_PIXEL_OP_LOOP (blend (*span++, (uint32) alphaLevel))
                else
                    ALOE_PERFORM_PIXEL_OP_LOOP (blend (*span++))
        */
    }
    
    #[inline(always)] pub fn handle_edge_table_line_full(&mut self, 
        x:     i32,
        width: i32)  {
        
        todo!();
        /*
            handleEdgeTableLine (x, width, 255);
        */
    }
    
    pub fn handle_edge_table_rectangle(&mut self, 
        x:           i32,
        y:           i32,
        width:       i32,
        height:      i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLine (x, width, alphaLevel);
                }
        */
    }
    
    pub fn handle_edge_table_rectangle_full(&mut self, 
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLineFull (x, width);
                }
        */
    }
    
    pub fn clip_edge_table_line(&mut self, 
        et:    &mut EdgeTable,
        x:     i32,
        y:     i32,
        width: i32)  {
        
        todo!();
        /*
            if (width > (int) scratchSize)
                {
                    scratchSize = (size_t) width;
                    scratchBuffer.malloc (scratchSize);
                }

                currentY = y;
                generate (scratchBuffer.get(), x, width);

                et.clipLineToMask (x, y,
                                   reinterpret_cast<uint8*> (scratchBuffer.get()) + SrcPixelType::indexA,
                                   sizeof (SrcPixelType), width);
        */
    }
    
    #[inline(always)] pub fn get_dest_pixel(&self, x: i32) -> *mut DestPixelType {
        
        todo!();
        /*
            return addBytesToPointer (linePixels, x * destData.pixelStride);
        */
    }
    
    
    pub fn generate<PixelType>(&mut self, 
        dest:       *mut PixelType,
        x:          i32,
        num_pixels: i32)  {
    
        todo!();
        /*
            this->interpolator.setStartOfLine ((float) x, (float) currentY, numPixels);

                do
                {
                    int hiResX, hiResY;
                    this->interpolator.next (hiResX, hiResY);

                    int loResX = hiResX >> 8;
                    int loResY = hiResY >> 8;

                    if (repeatPattern)
                    {
                        loResX = negativeAwareModulo (loResX, srcData.width);
                        loResY = negativeAwareModulo (loResY, srcData.height);
                    }

                    if (quality != typename Graphics::lowResamplingQuality)
                    {
                        if (isPositiveAndBelow (loResX, maxX))
                        {
                            if (isPositiveAndBelow (loResY, maxY))
                            {
                                // In the centre of the image..
                                render4PixelAverage (dest, this->srcData.getPixelPointer (loResX, loResY),
                                                     hiResX & 255, hiResY & 255);
                                ++dest;
                                continue;
                            }

                            if (! repeatPattern)
                            {
                                // At a top or bottom edge..
                                if (loResY < 0)
                                    render2PixelAverageX (dest, this->srcData.getPixelPointer (loResX, 0), hiResX & 255);
                                else
                                    render2PixelAverageX (dest, this->srcData.getPixelPointer (loResX, maxY), hiResX & 255);

                                ++dest;
                                continue;
                            }
                        }
                        else
                        {
                            if (isPositiveAndBelow (loResY, maxY) && ! repeatPattern)
                            {
                                // At a left or right hand edge..
                                if (loResX < 0)
                                    render2PixelAverageY (dest, this->srcData.getPixelPointer (0, loResY), hiResY & 255);
                                else
                                    render2PixelAverageY (dest, this->srcData.getPixelPointer (maxX, loResY), hiResY & 255);

                                ++dest;
                                continue;
                            }
                        }
                    }

                    if (! repeatPattern)
                    {
                        if (loResX < 0)     loResX = 0;
                        if (loResY < 0)     loResY = 0;
                        if (loResX > maxX)  loResX = maxX;
                        if (loResY > maxY)  loResY = maxY;
                    }

                    dest->set (*(const PixelType*) this->srcData.getPixelPointer (loResX, loResY));
                    ++dest;

                } while (--numPixels > 0);
        */
    }
    
    pub fn render_4pixel_average_argb(
        &mut self, 
        dest:       *mut PixelARGB,
        src:        *const u8,
        sub_pixelx: i32,
        sub_pixely: i32)  {
        
        todo!();
        /*
            uint32 c[4] = { 256 * 128, 256 * 128, 256 * 128, 256 * 128 };

                auto weight = (uint32) ((256 - subPixelX) * (256 - subPixelY));
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                src += this->srcData.pixelStride;

                weight = (uint32) (subPixelX * (256 - subPixelY));
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                src += this->srcData.lineStride;

                weight = (uint32) (subPixelX * subPixelY);
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                src -= this->srcData.pixelStride;

                weight = (uint32) ((256 - subPixelX) * subPixelY);
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                dest->setARGB ((uint8) (c[PixelARGB::indexA] >> 16),
                               (uint8) (c[PixelARGB::indexR] >> 16),
                               (uint8) (c[PixelARGB::indexG] >> 16),
                               (uint8) (c[PixelARGB::indexB] >> 16));
        */
    }
    
    pub fn render_2pixel_averagex_argb(
        &mut self, 
        dest:       *mut PixelARGB,
        src:        *const u8,
        sub_pixelx: u32)  {
        
        todo!();
        /*
            uint32 c[4] = { 128, 128, 128, 128 };

                uint32 weight = 256 - subPixelX;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                src += this->srcData.pixelStride;

                weight = subPixelX;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                dest->setARGB ((uint8) (c[PixelARGB::indexA] >> 8),
                               (uint8) (c[PixelARGB::indexR] >> 8),
                               (uint8) (c[PixelARGB::indexG] >> 8),
                               (uint8) (c[PixelARGB::indexB] >> 8));
        */
    }
    
    pub fn render_2pixel_averagey_argb(
        &mut self, 
        dest:       *mut PixelARGB,
        src:        *const u8,
        sub_pixely: u32)  {
        
        todo!();
        /*
            uint32 c[4] = { 128, 128, 128, 128 };

                uint32 weight = 256 - subPixelY;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                src += this->srcData.lineStride;

                weight = subPixelY;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];
                c[3] += weight * src[3];

                dest->setARGB ((uint8) (c[PixelARGB::indexA] >> 8),
                               (uint8) (c[PixelARGB::indexR] >> 8),
                               (uint8) (c[PixelARGB::indexG] >> 8),
                               (uint8) (c[PixelARGB::indexB] >> 8));
        */
    }
    
    pub fn render_4pixel_average_rgb(
        &mut self, 
        dest:       *mut PixelRGB,
        src:        *const u8,
        sub_pixelx: u32,
        sub_pixely: u32)  {
        
        todo!();
        /*
            uint32 c[3] = { 256 * 128, 256 * 128, 256 * 128 };

                uint32 weight = (256 - subPixelX) * (256 - subPixelY);
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];

                src += this->srcData.pixelStride;

                weight = subPixelX * (256 - subPixelY);
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];

                src += this->srcData.lineStride;

                weight = subPixelX * subPixelY;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];

                src -= this->srcData.pixelStride;

                weight = (256 - subPixelX) * subPixelY;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];

                dest->setARGB ((uint8) 255,
                               (uint8) (c[PixelRGB::indexR] >> 16),
                               (uint8) (c[PixelRGB::indexG] >> 16),
                               (uint8) (c[PixelRGB::indexB] >> 16));
        */
    }
    
    pub fn render_2pixel_averagex_rgb(
        &mut self, 
        dest:       *mut PixelRGB,
        src:        *const u8,
        sub_pixelx: u32)  {
        
        todo!();
        /*
            uint32 c[3] = { 128, 128, 128 };

                const uint32 weight = 256 - subPixelX;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];

                src += this->srcData.pixelStride;

                c[0] += subPixelX * src[0];
                c[1] += subPixelX * src[1];
                c[2] += subPixelX * src[2];

                dest->setARGB ((uint8) 255,
                               (uint8) (c[PixelRGB::indexR] >> 8),
                               (uint8) (c[PixelRGB::indexG] >> 8),
                               (uint8) (c[PixelRGB::indexB] >> 8));
        */
    }
    
    pub fn render_2pixel_averagey_rgb(
        &mut self, 
        dest:       *mut PixelRGB,
        src:        *const u8,
        sub_pixely: u32)  {
        
        todo!();
        /*
            uint32 c[3] = { 128, 128, 128 };

                const uint32 weight = 256 - subPixelY;
                c[0] += weight * src[0];
                c[1] += weight * src[1];
                c[2] += weight * src[2];

                src += this->srcData.lineStride;

                c[0] += subPixelY * src[0];
                c[1] += subPixelY * src[1];
                c[2] += subPixelY * src[2];

                dest->setARGB ((uint8) 255,
                               (uint8) (c[PixelRGB::indexR] >> 8),
                               (uint8) (c[PixelRGB::indexG] >> 8),
                               (uint8) (c[PixelRGB::indexB] >> 8));
        */
    }
    
    pub fn render_4pixel_average_alpha(
        &mut self, 
        dest:       *mut PixelAlpha,
        src:        *const u8,
        sub_pixelx: u32,
        sub_pixely: u32)  {
        
        todo!();
        /*
            uint32 c = 256 * 128;
                c += src[0] * ((256 - subPixelX) * (256 - subPixelY));
                src += this->srcData.pixelStride;
                c += src[0] * (subPixelX * (256 - subPixelY));
                src += this->srcData.lineStride;
                c += src[0] * (subPixelX * subPixelY);
                src -= this->srcData.pixelStride;

                c += src[0] * ((256 - subPixelX) * subPixelY);

                *((uint8*) dest) = (uint8) (c >> 16);
        */
    }
    
    pub fn render_2pixel_averagex_alpha(
        &mut self, 
        dest:       *mut PixelAlpha,
        src:        *const u8,
        sub_pixelx: u32)  {
        
        todo!();
        /*
            uint32 c = 128;
                c += src[0] * (256 - subPixelX);
                src += this->srcData.pixelStride;
                c += src[0] * subPixelX;
                *((uint8*) dest) = (uint8) (c >> 8);
        */
    }
    
    pub fn render_2pixel_averagey_alpha(
        &mut self, 
        dest:       *mut PixelAlpha,
        src:        *const u8,
        sub_pixely: u32)  {
        
        todo!();
        /*
            uint32 c = 128;
                c += src[0] * (256 - subPixelY);
                src += this->srcData.lineStride;
                c += src[0] * subPixelY;
                *((uint8*) dest) = (uint8) (c >> 8);
        */
    }
}

///------------------------
pub fn render_image_transformed<Iterator>(
        iter:       &mut Iterator,
        dest_data:  &ImageBitmapData,
        src_data:   &ImageBitmapData,
        alpha:      i32,
        transform:  &AffineTransform,
        quality:    GraphicsResamplingQuality,
        tiled_fill: bool)  {

    todo!();
        /*
            switch (destData.pixelFormat)
            {
            case typename Image::ARGB:
                switch (srcData.pixelFormat)
                {
                case typename Image::ARGB:
                    if (tiledFill)  { TransformedImageFill<PixelARGB, PixelARGB, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelARGB, PixelARGB, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                case typename Image::RGB:
                    if (tiledFill)  { TransformedImageFill<PixelARGB, PixelRGB, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelARGB, PixelRGB, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:
                    if (tiledFill)  { TransformedImageFill<PixelARGB, PixelAlpha, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelARGB, PixelAlpha, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                }
                break;

            case typename Image::RGB:
            {
                switch (srcData.pixelFormat)
                {
                case typename Image::ARGB:
                    if (tiledFill)  { TransformedImageFill<PixelRGB, PixelARGB, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelRGB, PixelARGB, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                case typename Image::RGB:
                    if (tiledFill)  { TransformedImageFill<PixelRGB, PixelRGB, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelRGB, PixelRGB, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:
                    if (tiledFill)  { TransformedImageFill<PixelRGB, PixelAlpha, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelRGB, PixelAlpha, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                }
                break;
            }

            case typename Image::SingleChannel:
            case typename Image::UnknownFormat:
            default:
                switch (srcData.pixelFormat)
                {
                case typename Image::ARGB:
                    if (tiledFill)  { TransformedImageFill<PixelAlpha, PixelARGB, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelAlpha, PixelARGB, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                case typename Image::RGB:
                    if (tiledFill)  { TransformedImageFill<PixelAlpha, PixelRGB, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelAlpha, PixelRGB, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:
                    if (tiledFill)  { TransformedImageFill<PixelAlpha, PixelAlpha, true>  r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    else            { TransformedImageFill<PixelAlpha, PixelAlpha, false> r (destData, srcData, transform, alpha, quality); iter.iterate (r); }
                    break;
                }
                break;
            }
        */
}

///------------------------
pub fn render_image_untransformed<Iterator>(
        iter:       &mut Iterator,
        dest_data:  &ImageBitmapData,
        src_data:   &ImageBitmapData,
        alpha:      i32,
        x:          i32,
        y:          i32,
        tiled_fill: bool)  {

    todo!();
        /*
            switch (destData.pixelFormat)
            {
            case typename Image::ARGB:
                switch (srcData.pixelFormat)
                {
                case typename Image::ARGB:
                    if (tiledFill)  { ImageFill<PixelARGB, PixelARGB, true>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelARGB, PixelARGB, false> r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                case typename Image::RGB:
                    if (tiledFill)  { ImageFill<PixelARGB, PixelRGB, true>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelARGB, PixelRGB, false> r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:
                    if (tiledFill)  { ImageFill<PixelARGB, PixelAlpha, true>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelARGB, PixelAlpha, false> r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                }
                break;

            case typename Image::RGB:
                switch (srcData.pixelFormat)
                {
                case typename Image::ARGB:
                    if (tiledFill)  { ImageFill<PixelRGB, PixelARGB, true>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelRGB, PixelARGB, false> r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                case typename Image::RGB:
                    if (tiledFill)  { ImageFill<PixelRGB, PixelRGB, true>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelRGB, PixelRGB, false> r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:
                    if (tiledFill)  { ImageFill<PixelRGB, PixelAlpha, true>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelRGB, PixelAlpha, false> r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                }
                break;

            case typename Image::SingleChannel:
            case typename Image::UnknownFormat:
            default:
                switch (srcData.pixelFormat)
                {
                case typename Image::ARGB:
                    if (tiledFill)  { ImageFill<PixelAlpha, PixelARGB, true>   r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelAlpha, PixelARGB, false>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                case typename Image::RGB:
                    if (tiledFill)  { ImageFill<PixelAlpha, PixelRGB, true>    r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelAlpha, PixelRGB, false>   r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:
                    if (tiledFill)  { ImageFill<PixelAlpha, PixelAlpha, true>  r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    else            { ImageFill<PixelAlpha, PixelAlpha, false> r (destData, srcData, alpha, x, y); iter.iterate (r); }
                    break;
                }
                break;
            }
        */
}

///------------------------
pub fn render_solid_fill<Iterator, DestPixelType>(
        iter:             &mut Iterator,
        dest_data:        &ImageBitmapData,
        fill_colour:      PixelARGB,
        replace_contents: bool,
        _4:               *mut DestPixelType)  {

    todo!();
        /*
            if (replaceContents)
            {
                EdgeTableFillers::SolidColour<DestPixelType, true> r (destData, fillColour);
                iter.iterate (r);
            }
            else
            {
                EdgeTableFillers::SolidColour<DestPixelType, false> r (destData, fillColour);
                iter.iterate (r);
            }
        */
}

///------------------------
pub fn render_gradient<Iterator, DestPixelType>(
        iter:               &mut Iterator,
        dest_data:          &ImageBitmapData,
        g:                  &ColourGradient,
        transform:          &AffineTransform,
        lookup_table:       *const PixelARGB,
        num_lookup_entries: i32,
        is_identity:        bool,
        _7:                 *mut DestPixelType)  {

    todo!();
        /*
            if (g.isRadial)
            {
                if (isIdentity)
                {
                    EdgeTableFillers::Gradient<DestPixelType, GradientPixelIterators::Radial> renderer (destData, g, transform, lookupTable, numLookupEntries);
                    iter.iterate (renderer);
                }
                else
                {
                    EdgeTableFillers::Gradient<DestPixelType, GradientPixelIterators::TransformedRadial> renderer (destData, g, transform, lookupTable, numLookupEntries);
                    iter.iterate (renderer);
                }
            }
            else
            {
                EdgeTableFillers::Gradient<DestPixelType, GradientPixelIterators::Linear> renderer (destData, g, transform, lookupTable, numLookupEntries);
                iter.iterate (renderer);
            }
        */
}
