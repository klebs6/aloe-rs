crate::ix!();

#[no_copy]
pub struct FrameBufferImageWriter<'a> {
    frame_buffer: &'a mut OpenGLFrameBuffer<'a>,
    area:         Rectangle<i32>,
}

impl<'a> FrameBufferImageWriter<'a> {

    pub fn new(
        fb: &mut OpenGLFrameBuffer,
        x:  i32,
        y:  i32,
        w:  i32,
        h:  i32) -> Self {
    
        todo!();
        /*
        : frame_buffer(fb),
        : area(x, y, w, h),

        
        */
    }
    
    pub fn write(&self, data: *const PixelARGB)  {
        
        todo!();
        /*
            HeapBlock<PixelARGB> invertedCopy (area.getWidth() * area.getHeight());
                auto rowSize = (size_t) area.getWidth() * sizeof (PixelARGB);

                for (int y = 0; y < area.getHeight(); ++y)
                    memcpy (invertedCopy + area.getWidth() * y,
                            data + area.getWidth() * (area.getHeight() - 1 - y), rowSize);

                frameBuffer.writePixels (invertedCopy, area);
        */
    }
}
