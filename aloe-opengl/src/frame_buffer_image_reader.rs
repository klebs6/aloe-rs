crate::ix!();

pub mod framebuffer_image_reader {

    use super::*;

    pub fn read(
            frame_buffer: &mut OpenGLFrameBuffer,
            bitmap_data:  &mut ImageBitmapData,
            x:            i32,
            y:            i32)  {
        
        todo!();
            /*
                frameBuffer.readPixels ((PixelARGB*) bitmapData.data,
                                            Rectangle<int> (x, frameBuffer.getHeight() - (y + bitmapData.height), bitmapData.width, bitmapData.height));

                    verticalRowFlip ((PixelARGB*) bitmapData.data, bitmapData.width, bitmapData.height);
            */
    }

    pub fn vertical_row_flip(
            data: *mut PixelARGB,
            w:    i32,
            h:    i32)  {
        
        todo!();
            /*
                HeapBlock<PixelARGB> tempRow (w);
                    auto rowSize = (size_t) w * sizeof (PixelARGB);

                    for (int y = 0; y < h / 2; ++y)
                    {
                        PixelARGB* const row1 = data + y * w;
                        PixelARGB* const row2 = data + (h - 1 - y) * w;
                        memcpy (tempRow, row1, rowSize);
                        memcpy (row1, row2, rowSize);
                        memcpy (row2, tempRow, rowSize);
                    }
            */
    }
}

