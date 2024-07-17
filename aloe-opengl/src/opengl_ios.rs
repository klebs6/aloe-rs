crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/native/aloe_OpenGL_ios.h]

#[cfg(target_os="ios")]
lazy_static!{
    /*
    @interface AloeGLView   : UIView
    {
    }
    + (Class) layerClass;
    @end

    @implementation AloeGLView
    + (Class) layerClass
    {
        return [CAEAGLLayer class];
    }
    @end
    */
}

#[cfg(target_os="ios")]
pub fn gl_resolve_multisample_framebufferapple() -> GLvoid {
    
    todo!();
        /*
        
        */
}

#[cfg(target_os="ios")]
#[no_copy]
#[leak_detector]
pub struct OpenGLContextNativeContext {
    component:               &mut Component,
    view:                    *mut AloeGLView, // default = nil
    gl_layer:                *mut CAEAGLLayer, // default = nil
    context:                 *mut EAGLContext, // default = nil
    open_glversion:          OpenGLVersion,
    use_depth_buffer:        bool,
    usemsaa:                 bool,
    frame_buffer_handle:     GLuint, // default = 0
    color_buffer_handle:     GLuint, // default = 0
    depth_buffer_handle:     GLuint, // default = 0
    msaa_color_handle:       GLuint, // default = 0
    msaa_buffer_handle:      GLuint, // default = 0
    last_bounds:             Rectangle<i32>,
    swap_frames:             i32, // default = 0
    need_to_rebuild_buffers: bool, // default = false
}

#[cfg(target_os="ios")]
pub struct OpenGLContextNativeContextLocker { 

}

#[cfg(target_os="ios")]
impl OpenGLContextNativeContextLocker {
    
    pub fn new(_0: &mut OpenGLContextNativeContext) -> Self {
    
        todo!();
        /*


        
        */
    }
}

#[cfg(target_os="ios")]
impl Drop for OpenGLContextNativeContext {

    fn drop(&mut self) {
        todo!();
        /*
            releaseContext();
                [view removeFromSuperview];
                [view release];
        */
    }
}

#[cfg(target_os="ios")]
impl OpenGLContextNativeContext {

    pub fn new(
        c:                &mut Component,
        pix_format:       &OpenGLPixelFormat,
        context_to_share: *mut c_void,
        multisampling:    bool,
        version:          OpenGLVersion) -> Self {
    
        todo!();
        /*


            : component (c), openGLversion (version),
                useDepthBuffer (pixFormat.depthBufferBits > 0),
                useMSAA (multisampling)

            ALOE_AUTORELEASEPOOL
            {
                if (auto* peer = component.getPeer())
                {
                    auto bounds = peer->getAreaCoveredBy (component);

                    view = [[AloeGLView alloc] initWithFrame: convertToCGRect (bounds)];
                    view.opaque = YES;
                    view.hidden = NO;
                    view.backgroundColor = [UIColor blackColor];
                    view.userInteractionEnabled = NO;

                    glLayer = (CAEAGLLayer*) [view layer];
                    glLayer.opaque = true;

                    updateWindowPosition (bounds);

                    [((UIView*) peer->getNativeHandle()) addSubview: view];

                    if (version == openGL3_2 && [[UIDevice currentDevice].systemVersion floatValue] >= 7.0)
                    {
                        if (! createContext (kEAGLRenderingAPIOpenGLES3, contextToShare))
                        {
                            releaseContext();
                            createContext (kEAGLRenderingAPIOpenGLES2, contextToShare);
                        }
                    }
                    else
                    {
                        createContext (kEAGLRenderingAPIOpenGLES2, contextToShare);
                    }

                    if (context != nil)
                    {
                        // I'd prefer to put this stuff in the initialiseOnRenderThread() call, but doing
                        // so causes mysterious timing-related failures.
                        [EAGLContext setCurrentContext: context];
                        gl::loadFunctions();
                        createGLBuffers();
                        deactivateCurrentContext();
                    }
                    else
                    {
                        jassertfalse;
                    }
                }
                else
                {
                    jassertfalse;
                }
            }
        */
    }
    
    pub fn initialise_on_render_thread(&mut self, _0: &mut OpenGLContext) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn shutdown_on_render_thread(&mut self)  {
        
        todo!();
        /*
            ALOE_CHECK_OPENGL_ERROR
                    freeGLBuffers();
                deactivateCurrentContext();
        */
    }
    
    pub fn created_ok(&self) -> bool {
        
        todo!();
        /*
            return getRawContext() != nullptr;
        */
    }
    
    pub fn get_raw_context(&self)  {
        
        todo!();
        /*
            return context;
        */
    }
    
    pub fn get_frame_bufferid(&self) -> GLuint {
        
        todo!();
        /*
            return useMSAA ? msaaBufferHandle : frameBufferHandle;
        */
    }
    
    pub fn make_active(&self) -> bool {
        
        todo!();
        /*
            if (! [EAGLContext setCurrentContext: context])
                    return false;

                glBindFramebuffer (GL_FRAMEBUFFER, useMSAA ? msaaBufferHandle
                        : frameBufferHandle);
                return true;
        */
    }
    
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return [EAGLContext currentContext] == context;
        */
    }
    
    pub fn deactivate_current_context()  {
        
        todo!();
        /*
            [EAGLContext setCurrentContext: nil];
        */
    }
    
    pub fn swap_buffers(&mut self)  {
        
        todo!();
        /*
            if (useMSAA)
                {
                    glBindFramebuffer (GL_DRAW_FRAMEBUFFER, frameBufferHandle);
                    glBindFramebuffer (GL_READ_FRAMEBUFFER, msaaBufferHandle);

                    if (openGLversion >= openGL3_2)
                    {
                        auto w = roundToInt (lastBounds.getWidth()  * glLayer.contentsScale);
                        auto h = roundToInt (lastBounds.getHeight() * glLayer.contentsScale);

                        glBlitFramebuffer (0, 0, w, h,
                                0, 0, w, h,
                                GL_COLOR_BUFFER_BIT,
                                GL_NEAREST);
                    }
                    else
                    {
                        ::glResolveMultisampleFramebufferAPPLE();
                    }
                }

                glBindRenderbuffer (GL_RENDERBUFFER, colorBufferHandle);
                [context presentRenderbuffer: GL_RENDERBUFFER];

                if (needToRebuildBuffers)
                {
                    needToRebuildBuffers = false;

                    freeGLBuffers();
                    createGLBuffers();
                    makeActive();
                }
        */
    }
    
    pub fn update_window_position(&mut self, bounds: Rectangle<i32>)  {
        
        todo!();
        /*
            view.frame = convertToCGRect (bounds);
                glLayer.contentsScale = (CGFloat) (Desktop::getInstance().getDisplays().getPrimaryDisplay()->scale
                        / component.getDesktopScaleFactor());

                if (lastBounds != bounds)
                {
                    lastBounds = bounds;
                    needToRebuildBuffers = true;
                }
        */
    }
    
    pub fn set_swap_interval(&mut self, num_frames_per_swap: i32) -> bool {
        
        todo!();
        /*
            swapFrames = numFramesPerSwap;
                return false;
        */
    }
    
    pub fn get_swap_interval(&self) -> i32 {
        
        todo!();
        /*
            return swapFrames;
        */
    }
    
    pub fn create_context(&mut self, 
        ty:               EAGLRenderingAPI,
        context_to_share: *mut c_void) -> bool {
        
        todo!();
        /*
            jassert (context == nil);
                context = [EAGLContext alloc];

                context = contextToShare != nullptr
                    ? [context initWithAPI: type  sharegroup: [(EAGLContext*) contextToShare sharegroup]]
                                          : [context initWithAPI: type];

                return context != nil;
        */
    }
    
    pub fn release_context(&mut self)  {
        
        todo!();
        /*
            [context release];
                context = nil;
        */
    }
    
    pub fn create_gl_buffers(&mut self)  {
        
        todo!();
        /*
            glGenFramebuffers (1, &frameBufferHandle);
                glGenRenderbuffers (1, &colorBufferHandle);

                glBindFramebuffer (GL_FRAMEBUFFER, frameBufferHandle);
                glBindRenderbuffer (GL_RENDERBUFFER, colorBufferHandle);

                glFramebufferRenderbuffer (GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_RENDERBUFFER, colorBufferHandle);

                bool ok = [context renderbufferStorage: GL_RENDERBUFFER fromDrawable: glLayer];
                jassert (ok); ignoreUnused (ok);

                GLint width, height;
                glGetRenderbufferParameteriv (GL_RENDERBUFFER, GL_RENDERBUFFER_WIDTH, &width);
                glGetRenderbufferParameteriv (GL_RENDERBUFFER, GL_RENDERBUFFER_HEIGHT, &height);

                if (useMSAA)
                {
                    glGenFramebuffers (1, &msaaBufferHandle);
                    glGenRenderbuffers (1, &msaaColorHandle);

                    glBindFramebuffer (GL_FRAMEBUFFER, msaaBufferHandle);
                    glBindRenderbuffer (GL_RENDERBUFFER, msaaColorHandle);

                    glRenderbufferStorageMultisample (GL_RENDERBUFFER, 4, GL_RGBA8, width, height);

                    glFramebufferRenderbuffer (GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_RENDERBUFFER, msaaColorHandle);
                }

                if (useDepthBuffer)
                {
                    glGenRenderbuffers (1, &depthBufferHandle);
                    glBindRenderbuffer (GL_RENDERBUFFER, depthBufferHandle);

                    if (useMSAA)
                        glRenderbufferStorageMultisample (GL_RENDERBUFFER, 4, GL_DEPTH_COMPONENT16, width, height);
                    else
                        glRenderbufferStorage (GL_RENDERBUFFER, GL_DEPTH_COMPONENT16, width, height);

                    glFramebufferRenderbuffer (GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_RENDERBUFFER, depthBufferHandle);
                }

                jassert (glCheckFramebufferStatus (GL_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE);
                ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn free_gl_buffers(&mut self)  {
        
        todo!();
        /*
            ALOE_CHECK_OPENGL_ERROR
                    [context renderbufferStorage: GL_RENDERBUFFER fromDrawable: nil];

                deleteFrameBuffer (frameBufferHandle);
                deleteFrameBuffer (msaaBufferHandle);
                deleteRenderBuffer (colorBufferHandle);
                deleteRenderBuffer (depthBufferHandle);
                deleteRenderBuffer (msaaColorHandle);

                ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn delete_frame_buffer(i: &mut GLuint)  {
        
        todo!();
        /*
            if (i != 0) glDeleteFramebuffers  (1, &i); i = 0;
        */
    }
    
    pub fn delete_render_buffer(i: &mut GLuint)  {
        
        todo!();
        /*
            if (i != 0) glDeleteRenderbuffers (1, &i); i = 0;
        */
    }
}
