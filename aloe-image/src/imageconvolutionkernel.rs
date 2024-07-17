crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_ImageConvolutionKernel.h]

/**
  | Represents a filter kernel to use in
  | convoluting an image.
  | 
  | @see typename Image::applyConvolution
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ImageConvolutionKernel {
    values: HeapBlock<f32>,
    size:   i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_ImageConvolutionKernel.cpp]
impl ImageConvolutionKernel {

    /**
      | Returns the size of the kernel.
      | 
      | E.g. if it's a 3x3 kernel, this returns
      | 3.
      |
      */
    pub fn get_kernel_size(&self) -> i32 {
        
        todo!();
        /*
            return size;
        */
    }
    
    /**
      | Creates an empty convolution kernel.
      | 
      | -----------
      | @param size
      | 
      | the length of each dimension of the kernel,
      | so e.g. if the size is 5, it will create
      | a 5x5 kernel
      |
      */
    pub fn new(size_to_use: i32) -> Self {
    
        todo!();
        /*

            : values ((size_t) (sizeToUse * sizeToUse)),
          size (sizeToUse)
        clear();
        */
    }
    
    /**
      | Returns one of the kernel values.
      |
      */
    pub fn get_kernel_value(&self, x: i32, y: i32) -> f32 {
        
        todo!();
        /*
            if (isPositiveAndBelow (x, size) && isPositiveAndBelow (y, size))
            return values [x + y * size];

        jassertfalse;
        return 0;
        */
    }
    
    /**
      | Sets the value of a specific cell in the
      | kernel.
      | 
      | The x and y parameters must be in the range
      | 0 < x < getKernelSize().
      | 
      | @see setOverallSum
      |
      */
    pub fn set_kernel_value(&mut self, 
        x:     i32,
        y:     i32,
        value: f32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (x, size) && isPositiveAndBelow (y, size))
        {
            values [x + y * size] = value;
        }
        else
        {
            jassertfalse;
        }
        */
    }
    
    /**
      | Resets all values in the kernel to zero.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            for (int i = size * size; --i >= 0;)
            values[i] = 0;
        */
    }
    
    /**
      | Rescales all values in the kernel to
      | make the total add up to a fixed value.
      | 
      | This will multiply all values in the
      | kernel by (desiredTotalSum / currentTotalSum).
      |
      */
    pub fn set_overall_sum(&mut self, desired_total_sum: f32)  {
        
        todo!();
        /*
            double currentTotal = 0.0;

        for (int i = size * size; --i >= 0;)
            currentTotal += values[i];

        rescaleAllValues ((float) (desiredTotalSum / currentTotal));
        */
    }
    
    /**
      | Multiplies all values in the kernel
      | by a value.
      |
      */
    pub fn rescale_all_values(&mut self, multiplier: f32)  {
        
        todo!();
        /*
            for (int i = size * size; --i >= 0;)
            values[i] *= multiplier;
        */
    }
    
    /**
      | Initialises the kernel for a gaussian
      | blur.
      | 
      | -----------
      | @param blurRadius
      | 
      | this may be larger or smaller than the
      | kernel's actual size but this will obviously
      | be wasteful or clip at the edges. Ideally
      | the kernel should be just larger than
      | (blurRadius * 2).
      |
      */
    pub fn create_gaussian_blur(&mut self, radius: f32)  {
        
        todo!();
        /*
            const double radiusFactor = -1.0 / (radius * radius * 2);
        const int centre = size >> 1;

        for (int y = size; --y >= 0;)
        {
            for (int x = size; --x >= 0;)
            {
                auto cx = x - centre;
                auto cy = y - centre;

                values [x + y * size] = (float) std::exp (radiusFactor * (cx * cx + cy * cy));
            }
        }

        setOverallSum (1.0f);
        */
    }
    
    /**
      | Applies the kernel to an image.
      | 
      | -----------
      | @param destImage
      | 
      | the image that will receive the resultant
      | convoluted pixels.
      | ----------
      | @param sourceImage
      | 
      | the source image to read from - this can
      | be the same image as the destination,
      | but if different, it must be exactly
      | the same size and format.
      | ----------
      | @param destinationArea
      | 
      | the region of the image to apply the filter
      | to
      |
      */
    pub fn apply_to_image(&self, 
        dest_image:       &mut Image,
        source_image:     &Image,
        destination_area: &Rectangle<i32>)  {
        
        todo!();
        /*
            if (sourceImage == destImage)
        {
            destImage.duplicateIfShared();
        }
        else
        {
            if (sourceImage.getWidth() != destImage.getWidth()
                 || sourceImage.getHeight() != destImage.getHeight()
                 || sourceImage.getFormat() != destImage.getFormat())
            {
                jassertfalse;
                return;
            }
        }

        auto area = destinationArea.getIntersection (destImage.getBounds());

        if (area.isEmpty())
            return;

        auto right = area.getRight();
        auto bottom = area.getBottom();

        const typename ImageBitmapData destData (destImage, area.getX(), area.getY(), area.getWidth(), area.getHeight(),
                                          typename ImageBitmapData::writeOnly);
        uint8* line = destData.data;

        const typename ImageBitmapData srcData (sourceImage, typename ImageBitmapData::readOnly);

        if (destData.pixelStride == 4)
        {
            for (int y = area.getY(); y < bottom; ++y)
            {
                uint8* dest = line;
                line += destData.lineStride;

                for (int x = area.getX(); x < right; ++x)
                {
                    float c1 = 0;
                    float c2 = 0;
                    float c3 = 0;
                    float c4 = 0;

                    for (int yy = 0; yy < size; ++yy)
                    {
                        const int sy = y + yy - (size >> 1);

                        if (sy >= srcData.height)
                            break;

                        if (sy >= 0)
                        {
                            int sx = x - (size >> 1);
                            const uint8* src = srcData.getPixelPointer (sx, sy);

                            for (int xx = 0; xx < size; ++xx)
                            {
                                if (sx >= srcData.width)
                                    break;

                                if (sx >= 0)
                                {
                                    const float kernelMult = values [xx + yy * size];
                                    c1 += kernelMult * *src++;
                                    c2 += kernelMult * *src++;
                                    c3 += kernelMult * *src++;
                                    c4 += kernelMult * *src++;
                                }
                                else
                                {
                                    src += 4;
                                }

                                ++sx;
                            }
                        }
                    }

                    *dest++ = (uint8) jmin (0xff, roundToInt (c1));
                    *dest++ = (uint8) jmin (0xff, roundToInt (c2));
                    *dest++ = (uint8) jmin (0xff, roundToInt (c3));
                    *dest++ = (uint8) jmin (0xff, roundToInt (c4));
                }
            }
        }
        else if (destData.pixelStride == 3)
        {
            for (int y = area.getY(); y < bottom; ++y)
            {
                uint8* dest = line;
                line += destData.lineStride;

                for (int x = area.getX(); x < right; ++x)
                {
                    float c1 = 0;
                    float c2 = 0;
                    float c3 = 0;

                    for (int yy = 0; yy < size; ++yy)
                    {
                        const int sy = y + yy - (size >> 1);

                        if (sy >= srcData.height)
                            break;

                        if (sy >= 0)
                        {
                            int sx = x - (size >> 1);
                            const uint8* src = srcData.getPixelPointer (sx, sy);

                            for (int xx = 0; xx < size; ++xx)
                            {
                                if (sx >= srcData.width)
                                    break;

                                if (sx >= 0)
                                {
                                    const float kernelMult = values [xx + yy * size];
                                    c1 += kernelMult * *src++;
                                    c2 += kernelMult * *src++;
                                    c3 += kernelMult * *src++;
                                }
                                else
                                {
                                    src += 3;
                                }

                                ++sx;
                            }
                        }
                    }

                    *dest++ = (uint8) roundToInt (c1);
                    *dest++ = (uint8) roundToInt (c2);
                    *dest++ = (uint8) roundToInt (c3);
                }
            }
        }
        else if (destData.pixelStride == 1)
        {
            for (int y = area.getY(); y < bottom; ++y)
            {
                uint8* dest = line;
                line += destData.lineStride;

                for (int x = area.getX(); x < right; ++x)
                {
                    float c1 = 0;

                    for (int yy = 0; yy < size; ++yy)
                    {
                        const int sy = y + yy - (size >> 1);

                        if (sy >= srcData.height)
                            break;

                        if (sy >= 0)
                        {
                            int sx = x - (size >> 1);
                            const uint8* src = srcData.getPixelPointer (sx, sy);

                            for (int xx = 0; xx < size; ++xx)
                            {
                                if (sx >= srcData.width)
                                    break;

                                if (sx >= 0)
                                {
                                    const float kernelMult = values [xx + yy * size];
                                    c1 += kernelMult * *src++;
                                }
                                else
                                {
                                    src += 3;
                                }

                                ++sx;
                            }
                        }
                    }

                    *dest++ = (uint8) roundToInt (c1);
                }
            }
        }
        */
    }
}
