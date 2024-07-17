crate::ix!();

#[no_copy]
#[leak_detector]
pub struct OpenFramebufferSavedState {
    width:  i32,
    height: i32,
    data:   HeapBlock<PixelARGB>,
}

impl OpenFramebufferSavedState {

    pub fn new(
        buffer: &mut OpenGLFrameBuffer,
        w:      i32,
        h:      i32) -> Self {
    
        todo!();
        /*

            : width (w), height (h),
                data ((size_t) (w * h))

                buffer.readPixels (data, Rectangle<int> (w, h));
        */
    }
    
    pub fn restore(&mut self, 
        context: &mut OpenGLContext,
        buffer:  &mut OpenGLFrameBuffer) -> bool {
        
        todo!();
        /*
            if (buffer.initialise (context, width, height))
                {
                    buffer.writePixels (data, Rectangle<int> (width, height));
                    return true;
                }

                return false;
        */
    }
}
