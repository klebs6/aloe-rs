crate::ix!();

#[no_copy]
pub struct OpenFramebufferPimpl<'a> {
    context:                 &'a mut OpenGLContext<'a>,
    width:                   i32,
    height:                  i32,
    textureid:               GLuint,
    frame_bufferid:          GLuint,
    depth_or_stencil_buffer: GLuint,
    has_depth_buffer:        bool,
    has_stencil_buffer:      bool,
}

impl<'a> Drop for OpenFramebufferPimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (OpenGLHelpers::isContextActive())
                {
                    if (textureID != 0)
                        glDeleteTextures (1, &textureID);

                    if (depthOrStencilBuffer != 0)
                        context.extensions.glDeleteRenderbuffers (1, &depthOrStencilBuffer);

                    if (frameBufferID != 0)
                        context.extensions.glDeleteFramebuffers (1, &frameBufferID);

                    ALOE_CHECK_OPENGL_ERROR
                }
        */
    }
}

impl<'a> OpenFramebufferPimpl<'a> {
    
    pub fn new(
        c:                    &mut OpenGLContext,
        w:                    i32,
        h:                    i32,
        wants_depth_buffer:   bool,
        wants_stencil_buffer: bool) -> Self {
    
        todo!();
        /*
        : context(c),
        : width(w),
        : height(h),
        : textureid(0),
        : frame_bufferid(0),
        : depth_or_stencil_buffer(0),
        : has_depth_buffer(false),
        : has_stencil_buffer(false),

            // Framebuffer objects can only be created when the current thread has an active OpenGL
                // context. You'll need to create this object in one of the OpenGLContext's callbacks.
                jassert (OpenGLHelpers::isContextActive());

                #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
                if (context.extensions.glGenFramebuffers == nullptr)
                    return;
                #endif

                context.extensions.glGenFramebuffers (1, &frameBufferID);
                bind();

                glGenTextures (1, &textureID);
                glBindTexture (GL_TEXTURE_2D, textureID);
                ALOE_CHECK_OPENGL_ERROR

                    glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
                glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
                glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
                glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
                ALOE_CHECK_OPENGL_ERROR

                    glTexImage2D (GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, nullptr);
                ALOE_CHECK_OPENGL_ERROR

                    context.extensions.glFramebufferTexture2D (GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, textureID, 0);

                if (wantsDepthBuffer || wantsStencilBuffer)
                {
                    context.extensions.glGenRenderbuffers (1, &depthOrStencilBuffer);
                    context.extensions.glBindRenderbuffer (GL_RENDERBUFFER, depthOrStencilBuffer);
                    jassert (context.extensions.glIsRenderbuffer (depthOrStencilBuffer));

                    context.extensions.glRenderbufferStorage (GL_RENDERBUFFER,
                        (wantsDepthBuffer && wantsStencilBuffer) ? (GLenum) GL_DEPTH24_STENCIL8
                        #if ALOE_OPENGL_ES
                        : (GLenum) GL_DEPTH_COMPONENT16,
                        #else
                        : (GLenum) GL_DEPTH_COMPONENT,
                        #endif
                        width, height);

                    GLint params = 0;
                    context.extensions.glGetRenderbufferParameteriv (GL_RENDERBUFFER, GL_RENDERBUFFER_DEPTH_SIZE, &params);
                    context.extensions.glFramebufferRenderbuffer (GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_RENDERBUFFER, depthOrStencilBuffer);

                    if (wantsStencilBuffer)
                        context.extensions.glFramebufferRenderbuffer (GL_FRAMEBUFFER, GL_STENCIL_ATTACHMENT, GL_RENDERBUFFER, depthOrStencilBuffer);

                    hasDepthBuffer = wantsDepthBuffer;
                    hasStencilBuffer = wantsStencilBuffer;
                }

                unbind();
        */
    }
    
    pub fn created_ok(&self) -> bool {
        
        todo!();
        /*
            return frameBufferID != 0 && textureID != 0;
        */
    }
    
    pub fn bind(&mut self)  {
        
        todo!();
        /*
            context.extensions.glBindFramebuffer (GL_FRAMEBUFFER, frameBufferID);
                ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn unbind(&mut self)  {
        
        todo!();
        /*
            context.extensions.glBindFramebuffer (GL_FRAMEBUFFER, context.getFrameBufferID());
                ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn check_status(&mut self) -> bool {
        
        todo!();
        /*
            const GLenum status = context.extensions.glCheckFramebufferStatus (GL_FRAMEBUFFER);

                return status == GL_NO_ERROR
                    || status == GL_FRAMEBUFFER_COMPLETE;
        */
    }
}
