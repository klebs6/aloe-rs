crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLTexture.h]

/**
  | Creates an openGL texture from an Image.
  | 
  | @tags{OpenGL}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLTexture<'a> {
    textureid:     u32,
    width:         i32,
    height:        i32,
    owner_context: *mut OpenGLContext<'a>,
}

impl<'a> Drop for OpenGLTexture<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            release();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLTexture.cpp]
impl<'a> OpenGLTexture<'a> {

    /**
      | Returns the GL texture ID number.
      |
      */
    pub fn get_textureid(&self) -> u32 {
        
        todo!();
        /*
            return textureID;
        */
    }
    
    pub fn get_width(&self) -> i32 {
        
        todo!();
        /*
            return width;
        */
    }
    
    pub fn get_height(&self) -> i32 {
        
        todo!();
        /*
            return height;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
        : textureid(0),
        : width(0),
        : height(0),
        : owner_context(nullptr),

        
        */
    }
    
    /**
      | Returns true if a texture can be created
      | with the given size.
      | 
      | Some systems may require that the sizes
      | are powers-of-two.
      |
      */
    pub fn is_valid_size(&mut self, 
        width:  i32,
        height: i32) -> bool {
        
        todo!();
        /*
            return isPowerOfTwo (width) && isPowerOfTwo (height);
        */
    }
    
    pub fn create(&mut self, 
        w:        i32,
        h:        i32,
        pixels:   *const c_void,
        ty:       GLenum,
        top_left: bool)  {
        
        todo!();
        /*
            ownerContext = OpenGLContext::getCurrentContext();

        // Texture objects can only be created when the current thread has an active OpenGL
        // context. You'll need to create this object in one of the OpenGLContext's callbacks.
        jassert (ownerContext != nullptr);

        if (textureID == 0)
        {
            ALOE_CHECK_OPENGL_ERROR
            glGenTextures (1, &textureID);
            glBindTexture (GL_TEXTURE_2D, textureID);
            glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);

            auto glMagFilter = (GLint) (ownerContext->texMagFilter == OpenGLContext::linear ? GL_LINEAR : GL_NEAREST);
            glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, glMagFilter);
            glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
            glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
            ALOE_CHECK_OPENGL_ERROR
        }
        else
        {
            glBindTexture (GL_TEXTURE_2D, textureID);
            ALOE_CHECK_OPENGL_ERROR;
        }

        glPixelStorei (GL_UNPACK_ALIGNMENT, 1);
        ALOE_CHECK_OPENGL_ERROR

        const auto textureNpotSupported = ownerContext->isTextureNpotSupported();

        const auto getAllowedTextureSize = [&] (int n)
        {
            return textureNpotSupported ? n : nextPowerOfTwo (n);
        };

        width  = getAllowedTextureSize (w);
        height = getAllowedTextureSize (h);

        const GLint internalformat = type == GL_ALPHA ? GL_ALPHA : GL_RGBA;

        if (width != w || height != h)
        {
            glTexImage2D (GL_TEXTURE_2D, 0, internalformat,
                          width, height, 0, type, GL_UNSIGNED_BYTE, nullptr);

            glTexSubImage2D (GL_TEXTURE_2D, 0, 0, topLeft ? (height - h) : 0, w, h,
                             type, GL_UNSIGNED_BYTE, pixels);
        }
        else
        {
            glTexImage2D (GL_TEXTURE_2D, 0, internalformat,
                          w, h, 0, type, GL_UNSIGNED_BYTE, pixels);
        }

        ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    /**
      | Creates a texture from the given image.
      | 
      | -----------
      | @note
      | 
      | if the image's dimensions aren't a power-of-two,
      | the texture may be created with a larger
      | size.
      | 
      | The image will be arranged so that its
      | top-left corner is at texture coordinate
      | (0, 1).
      |
      */
    pub fn load_image(&mut self, image: &Image)  {
        
        todo!();
        /*
            const int imageW = image.getWidth();
        const int imageH = image.getHeight();

        HeapBlock<PixelARGB> dataCopy;
        ImageBitmapData srcData (image, ImageBitmapData::readOnly);

        switch (srcData.pixelFormat)
        {
            case Image::ARGB:           Flipper<PixelARGB> ::flip (dataCopy, srcData.data, srcData.lineStride, imageW, imageH); break;
            case Image::RGB:            Flipper<PixelRGB>  ::flip (dataCopy, srcData.data, srcData.lineStride, imageW, imageH); break;
            case Image::SingleChannel:  Flipper<PixelAlpha>::flip (dataCopy, srcData.data, srcData.lineStride, imageW, imageH); break;
            case Image::UnknownFormat:
            default: break;
        }

        create (imageW, imageH, dataCopy, ALOE_RGBA_FORMAT, true);
        */
    }
    
    /**
      | Creates a texture from a raw array of
      | pixels.
      | 
      | If width and height are not powers-of-two,
      | the texture will be created with a larger
      | size, and only the subsection (0, 0,
      | width, height) will be initialised.
      | 
      | The data is sent directly to the OpenGL
      | driver without being flipped vertically,
      | so the first pixel will be mapped onto
      | texture coordinate (0, 0).
      |
      */
    pub fn loadargb(&mut self, 
        pixels: *const PixelARGB,
        w:      i32,
        h:      i32)  {
        
        todo!();
        /*
            create (w, h, pixels, ALOE_RGBA_FORMAT, false);
        */
    }
    
    /**
      | Creates an alpha-channel texture from
      | an array of alpha values.
      | 
      | If width and height are not powers-of-two,
      | the texture will be created with a larger
      | size, and only the subsection (0, 0,
      | width, height) will be initialised.
      | 
      | The data is sent directly to the OpenGL
      | driver without being flipped vertically,
      | so the first pixel will be mapped onto
      | texture coordinate (0, 0).
      |
      */
    pub fn load_alpha(&mut self, 
        pixels: *const u8,
        w:      i32,
        h:      i32)  {
        
        todo!();
        /*
            create (w, h, pixels, GL_ALPHA, false);
        */
    }
    
    /**
      | Creates a texture from a raw array of
      | pixels.
      | 
      | This is like loadARGB, but will vertically
      | flip the data so that the first pixel
      | ends up at texture coordinate (0, 1),
      | and if the width and height are not powers-of-two,
      | it will compensate by using a larger
      | texture size.
      |
      */
    pub fn load_argb_flipped(&mut self, 
        pixels: *const PixelARGB,
        w:      i32,
        h:      i32)  {
        
        todo!();
        /*
            HeapBlock<PixelARGB> flippedCopy;
        Flipper<PixelARGB>::flip (flippedCopy, (const uint8*) pixels, 4 * w, w, h);

        create (w, h, flippedCopy, ALOE_RGBA_FORMAT, true);
        */
    }
    
    /**
      | Frees the texture, if there is one.
      |
      */
    pub fn release(&mut self)  {
        
        todo!();
        /*
            if (textureID != 0)
        {
            // If the texture is deleted while the owner context is not active, it's
            // impossible to delete it, so this will be a leak until the context itself
            // is deleted.
            jassert (ownerContext == OpenGLContext::getCurrentContext());

            if (ownerContext == OpenGLContext::getCurrentContext())
            {
                glDeleteTextures (1, &textureID);

                textureID = 0;
                width = 0;
                height = 0;
            }
        }
        */
    }
    
    /**
      | Binds the texture to the currently active
      | openGL context.
      |
      */
    pub fn bind(&self)  {
        
        todo!();
        /*
            glBindTexture (GL_TEXTURE_2D, textureID);
        */
    }
    
    /**
      | Unbinds the texture to the currently
      | active openGL context.
      |
      */
    pub fn unbind(&self)  {
        
        todo!();
        /*
            glBindTexture (GL_TEXTURE_2D, 0);
        */
    }
}
