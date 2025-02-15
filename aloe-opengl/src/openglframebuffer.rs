crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLFrameBuffer.h]

/**
  | Creates an openGL frame buffer.
  | 
  | @tags{OpenGL}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLFrameBuffer<'a> {
    impl_:       Box<OpenFramebufferImpl<'a>>,
    saved_state: Box<OpenFramebufferSavedState>,
}

impl<'a> Default for OpenGLFrameBuffer<'a> {
    
    /**
      | Creates an uninitialised buffer.
      | 
      | To actually allocate the buffer, use
      | initialise().
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLFrameBuffer.cpp]
impl<'a> OpenGLFrameBuffer<'a> {
    
    /**
      | Returns true if a valid buffer has been
      | allocated.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return impl != nullptr;
        */
    }

    /**
      | Tries to allocates a buffer of the given
      | size.
      | 
      | -----------
      | @note
      | 
      | a valid openGL context must be selected
      | when you call this method, or it will
      | fail.
      |
      */
    pub fn initialise_with_wh(
        &mut self, 
        context: &mut OpenGLContext,
        width:   i32,
        height:  i32

    ) -> bool {
        
        todo!();
        /*
            jassert (context.isActive()); // The context must be active when creating a framebuffer!

        impl.reset();
        impl.reset (new OpenFramebufferImpl (context, width, height, false, false));

        if (! impl->createdOk())
            impl.reset();

        return impl != nullptr;
        */
    }
    
    /**
      | Tries to allocates a buffer containing
      | a copy of a given image.
      | 
      | -----------
      | @note
      | 
      | a valid openGL context must be selected
      | when you call this method, or it will
      | fail.
      |
      */
    pub fn initialise(
        &mut self, 
        context: &mut OpenGLContext,
        image:   &Image

    ) -> bool {
        
        todo!();
        /*
            if (! image.isARGB())
            return initialise (context, image.convertedToFormat (Image::ARGB));

        ImageBitmapData bitmap (image, ImageBitmapData::readOnly);

        return initialise (context, bitmap.width, bitmap.height)
                && writePixels ((const PixelARGB*) bitmap.data, image.getBounds());
        */
    }
    
    /**
      | Tries to allocate a copy of another framebuffer.
      |
      */
    pub fn initialise_with_framebuffer(&mut self, other: &mut OpenGLFrameBuffer) -> bool {
        
        todo!();
        /*
            auto* p = other.impl.get();

        if (p == nullptr)
        {
            impl.reset();
            return true;
        }

        const Rectangle<int> area (impl->width, impl->height);

        if (initialise (p->context, area.getWidth(), area.getHeight()))
        {
            impl->bind();

           #if ! ALOE_ANDROID
            glEnable (GL_TEXTURE_2D);
            clearGLError();
           #endif
            glBindTexture (GL_TEXTURE_2D, p->textureID);
            impl->context.copyTexture (area, area, area.getWidth(), area.getHeight(), false);
            glBindTexture (GL_TEXTURE_2D, 0);
            ALOE_CHECK_OPENGL_ERROR

            impl->unbind();
            return true;
        }

        return false;
        */
    }
    
    /**
      | Releases the buffer, if one has been
      | allocated.
      | 
      | Any saved state that was created with
      | saveAndRelease() will also be freed
      | by this call.
      |
      */
    pub fn release(&mut self)  {
        
        todo!();
        /*
            impl.reset();
        savedState.reset();
        */
    }
    
    /**
      | If the framebuffer is active, this will
      | save a stashed copy of its contents in
      | main memory, and will release the GL
      | buffer.
      | 
      | After saving, the original state can
      | be restored again by calling reloadSavedCopy().
      |
      */
    pub fn save_and_release(&mut self)  {
        
        todo!();
        /*
            if (impl != nullptr)
        {
            savedState.reset (new OpenFramebufferSavedState (*this, impl->width, impl->height));
            impl.reset();
        }
        */
    }
    
    /**
      | Restores the framebuffer content that
      | was previously saved using saveAndRelease().
      | 
      | After saving to main memory, the original
      | state can be restored by calling restoreToGPUMemory().
      |
      */
    pub fn reload_saved_copy(&mut self, context: &mut OpenGLContext) -> bool {
        
        todo!();
        /*
            if (savedState != nullptr)
        {
            std::unique_ptr<OpenFramebufferSavedState> state;
            std::swap (state, savedState);

            if (state->restore (context, *this))
                return true;

            std::swap (state, savedState);
        }

        return false;
        */
    }
    
    /**
      | Returns the width of the buffer.
      |
      */
    pub fn get_width(&self) -> i32 {
        
        todo!();
        /*
            return impl != nullptr ? impl->width : 0;
        */
    }
    
    /**
      | Returns the height of the buffer.
      |
      */
    pub fn get_height(&self) -> i32 {
        
        todo!();
        /*
            return impl != nullptr ? impl->height : 0;
        */
    }
    
    /**
      | Returns the texture ID number for using
      | this buffer as a texture.
      |
      */
    pub fn get_textureid(&self) -> GLuint {
        
        todo!();
        /*
            return impl != nullptr ? impl->textureID : 0;
        */
    }
    
    /**
      | Selects this buffer as the current OpenGL
      | rendering target.
      |
      */
    pub fn make_current_rendering_target(&mut self) -> bool {
        
        todo!();
        /*
            // trying to use a framebuffer after saving it with saveAndRelease()! Be sure to call
        // reloadSavedCopy() to put it back into GPU memory before using it..
        jassert (savedState == nullptr);

        if (impl == nullptr)
            return false;

        impl->bind();
        return true;
        */
    }
    
    /**
      | Returns the ID of this framebuffer,
      | or 0 if it isn't initialised.
      |
      */
    pub fn get_frame_bufferid(&self) -> GLuint {
        
        todo!();
        /*
            return impl != nullptr ? impl->frameBufferID : 0;
        */
    }
    
    /**
      | Returns the current frame buffer ID
      | for the current context.
      |
      */
    pub fn get_current_frame_buffer_target(&mut self) -> GLuint {
        
        todo!();
        /*
            GLint fb = {};
        glGetIntegerv (GL_FRAMEBUFFER_BINDING, &fb);
        return (GLuint) fb;
        */
    }
    
    /**
      | Deselects this buffer as the current
      | OpenGL rendering target.
      |
      */
    pub fn release_as_rendering_target(&mut self)  {
        
        todo!();
        /*
            if (impl != nullptr)
            impl->unbind();
        */
    }
    
    /**
      | Clears the framebuffer with the specified
      | colour.
      |
      */
    pub fn clear(&mut self, colour: Colour)  {
        
        todo!();
        /*
            if (makeCurrentRenderingTarget())
        {
            OpenGLHelpers::clear (colour);
            releaseAsRenderingTarget();
        }
        */
    }
    
    /**
      | Selects the framebuffer as the current
      | target, and clears it to transparent.
      |
      */
    pub fn make_current_and_clear(&mut self)  {
        
        todo!();
        /*
            if (makeCurrentRenderingTarget())
        {
            glClearColor (0, 0, 0, 0);
            glClear (GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT);
        }
        */
    }
    
    /**
      | Reads an area of pixels from the framebuffer
      | into a 32-bit ARGB pixel array.
      | 
      | The lineStride is measured as a number
      | of pixels, not bytes - pass a stride of
      | 0 to indicate a packed array.
      |
      */
    pub fn read_pixels(&mut self, 
        target: *mut PixelARGB,
        area:   &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            if (! makeCurrentRenderingTarget())
            return false;

        glPixelStorei (GL_PACK_ALIGNMENT, 4);
        glReadPixels (area.getX(), area.getY(), area.getWidth(), area.getHeight(),
                      ALOE_RGBA_FORMAT, GL_UNSIGNED_BYTE, target);

        impl->unbind();
        return true;
        */
    }
    
    /**
      | Writes an area of pixels into the framebuffer
      | from a specified pixel array.
      | 
      | The lineStride is measured as a number
      | of pixels, not bytes - pass a stride of
      | 0 to indicate a packed array.
      |
      */
    pub fn write_pixels(&mut self, 
        data: *const PixelARGB,
        area: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            OpenGLTargetSaver ts (impl->context);

        if (! makeCurrentRenderingTarget())
            return false;

        glDisable (GL_DEPTH_TEST);
        glDisable (GL_BLEND);
        ALOE_CHECK_OPENGL_ERROR

        OpenGLTexture tex;
        tex.loadARGB (data, area.getWidth(), area.getHeight());

        glViewport (0, 0, impl->width, impl->height);
        impl->context.copyTexture (area, Rectangle<int> (area.getX(), area.getY(),
                                                          tex.getWidth(), tex.getHeight()),
                                    impl->width, impl->height, true);

        ALOE_CHECK_OPENGL_ERROR
        return true;
        */
    }
}
