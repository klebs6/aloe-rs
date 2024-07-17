crate::ix!();

//-------------------------[cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLImage.cpp]
#[no_copy]
#[leak_detector]
pub struct OpenGLFrameBufferImage<'a> {
    base:         ImagePixelData,
    context:      &'a mut OpenGLContext<'a>,
    frame_buffer: OpenGLFrameBuffer<'a>,
    pixel_stride: i32,
    line_stride:  i32,
}

impl<'a> OpenGLFrameBufferImage<'a> {

    pub fn new(
        c: &mut OpenGLContext,
        w: i32,
        h: i32) -> Self {
    
        todo!();
        /*


            : ImagePixelData (Image::ARGB, w, h),
              context (c),
              pixelStride (4),
              lineStride (width * pixelStride)
        */
    }
    
    pub fn initialise(&mut self) -> bool {
        
        todo!();
        /*
            if (! frameBuffer.initialise (context, width, height))
                return false;

            frameBuffer.clear (Colours::transparentBlack);
            return true;
        */
    }
    
    pub fn create_low_level_context(&mut self) -> Box<dyn LowLevelGraphicsContext> {
        
        todo!();
        /*
            sendDataChangeMessage();
            return createOpenGLGraphicsContext (context, frameBuffer);
        */
    }
    
    pub fn create_type(&self) -> Box<dyn ImageType> {
        
        todo!();
        /*
            return std::make_unique<OpenGLImageType>();
        */
    }
    
    pub fn clone(&mut self) -> ImagePixelDataPtr {
        
        todo!();
        /*
            std::unique_ptr<OpenGLFrameBufferImage> im (new OpenGLFrameBufferImage (context, width, height));

            if (! im->initialise())
                return ImagePixelData::Ptr();

            Image newImage (im.release());
            Graphics g (newImage);
            g.drawImageAt (Image (*this), 0, 0, false);

            return ImagePixelData::Ptr (newImage.getPixelData());
        */
    }
    
    pub fn initialise_bitmap_data(
        &mut self, 
        bitmap_data: &mut ImageBitmapData,
        x:           i32,
        y:           i32,
        mode:        ImageBitmapDataReadWriteMode

    ) {
        
        todo!();
        /*
            bitmapData.pixelFormat = pixelFormat;
            bitmapData.lineStride  = lineStride;
            bitmapData.pixelStride = pixelStride;

            switch (mode)
            {
                case ImageBitmapData::writeOnly:  DataReleaser<Dummy,  Writer>::initialise (frameBuffer, bitmapData, x, y); break;
                case ImageBitmapData::readOnly:   DataReleaser<Reader, Dummy> ::initialise (frameBuffer, bitmapData, x, y); break;
                case ImageBitmapData::readWrite:  DataReleaser<Reader, Writer>::initialise (frameBuffer, bitmapData, x, y); break;
                default:                            jassertfalse; break;
            }

            if (mode != ImageBitmapData::readOnly)
                sendDataChangeMessage();
        */
    }
}
