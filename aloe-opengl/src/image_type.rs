crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLImage.h]

/**
  | A type of ImagePixelData that stores
  | its image data in an OpenGL framebuffer,
  | allowing a Aloe Image object to wrap
  | a framebuffer.
  | 
  | By creating an Image from an instance
  | of an OpenGLFrameBufferImage, you
  | can then use a Graphics object to draw
  | into the framebuffer using normal
  | 
  | Aloe 2D operations.
  | 
  | @see Image, ImageType, ImagePixelData,
  | OpenGLFrameBuffer
  | 
  | @tags{OpenGL}
  |
  */
pub struct OpenGLImageType {

}

impl ImageType for OpenGLImageType {

    fn get_typeid(&self) -> i32 {
        
        todo!();
        /*
            return 3;
        */
    }
    
    fn create(
        &self, 
        _0:                 ImagePixelFormat,
        width:              i32,
        height:             i32,
        should_clear_image: bool

    ) -> ImagePixelDataPtr {
        
        todo!();
        /*
            OpenGLContext* currentContext = OpenGLContext::getCurrentContext();
        jassert (currentContext != nullptr); // an OpenGL image can only be created when a valid context is active!

        std::unique_ptr<OpenGLFrameBufferImage> im (new OpenGLFrameBufferImage (*currentContext, width, height));

        if (! im->initialise())
            return ImagePixelData::Ptr();

        return *im.release();
        */
    }
}

impl OpenGLImageType {

    pub fn get_frame_buffer_from(&mut self, image: &Image) -> *mut OpenGLFrameBuffer {
        
        todo!();
        /*
            if (OpenGLFrameBufferImage* const glImage = dynamic_cast<OpenGLFrameBufferImage*> (image.getPixelData()))
            return &(glImage->frameBuffer);

        return nullptr;
        */
    }
}

