crate::ix!();

pub struct FrameBufferImageDataReleaser<ReaderType,WriterType> {
    base:    ImageBitmapDataReleaser,
    data:    HeapBlock<PixelARGB>,
    writer:  WriterType,

    phantom: PhantomData<ReaderType>,
}

impl<ReaderType,WriterType> Drop for FrameBufferImageDataReleaser<ReaderType,WriterType> {

    fn drop(&mut self) {
        todo!();
        /*
            writer.write (data);
        */
    }
}

impl<ReaderType,WriterType> FrameBufferImageDataReleaser<ReaderType,WriterType> {

    pub fn new(
        fb: &mut OpenGLFrameBuffer,
        x:  i32,
        y:  i32,
        w:  i32,
        h:  i32) -> Self {
    
        todo!();
        /*
           : data ((size_t) (w * h)),
           writer (fb, x, y, w, h)
        */
    }
    
    pub fn initialise(
        frame_buffer: &mut OpenGLFrameBuffer,
        bitmap_data:  &mut ImageBitmapData,
        x:            i32,
        y:            i32)  {
        
        todo!();
        /*
           auto* r = new FrameBufferImageDataReleaser (frameBuffer, x, y, bitmapData.width, bitmapData.height);
           bitmapData.dataReleaser.reset (r);

           bitmapData.data = (uint8*) r->data.get();
           bitmapData.lineStride = (bitmapData.width * bitmapData.pixelStride + 3) & ~3;

           ReaderType::read (frameBuffer, bitmapData, x, y);
           */
    }
}
