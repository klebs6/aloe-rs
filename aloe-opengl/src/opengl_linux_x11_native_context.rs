crate::ix!();

#[cfg(target_os="linux")]
#[no_copy]
#[leak_detector]
pub struct OpenGLContextNativeContext {
    component:             &mut Component,
    render_context:        GLXContext,
    embedded_window:       Window,
    swap_frames:           i32, // default = 1
    bounds:                Rectangle<i32>,
    best_visual:           *mut XVisualInfo, // default = nullptr
    context_to_share_with: *mut c_void,
    context:               *mut OpenGLContext, // default = nullptr
    dummy:                 OpenGLContextNativeContextDummyComponent,
    display:               *mut Display, // default = nullptr
}

#[cfg(target_os="linux")]
impl Drop for OpenGLContextNativeContext {

    fn drop(&mut self) {
        todo!();
        /*
            if (auto* peer = component.getPeer())
                {
                    aloe_LinuxRemoveRepaintListener (peer, &dummy);

                    if (embeddedWindow != 0)
                    {
                        XWindowSystemUtilities::ScopedXLock xLock;

                        X11Symbols::getInstance()->xUnmapWindow (display, embeddedWindow);
                        X11Symbols::getInstance()->xDestroyWindow (display, embeddedWindow);
                        X11Symbols::getInstance()->xSync (display, False);

                        XEvent event;
                        while (X11Symbols::getInstance()->xCheckWindowEvent (display,
                                                                             embeddedWindow,
                                                                             embeddedWindowEventMask,
                                                                             &event) == True)
                        {
                        }
                    }
                }

                if (bestVisual != nullptr)
                    X11Symbols::getInstance()->xFree (bestVisual);
        */
    }
}

#[cfg(target_os="linux")]
impl OpenGLContextNativeContext {

    pub fn new(
        comp:              &mut Component,
        pixel_format:      &OpenGLPixelFormat,
        share_context:     *mut c_void,
        use_multisampling: bool,
        _4:                OpenGLVersion) -> Self {
    
        todo!();
        /*


            : component (comp), contextToShareWith (shareContext), dummy (*this)

                display = XWindowSystem::getInstance()->getDisplay();

                XWindowSystemUtilities::ScopedXLock xLock;

                X11Symbols::getInstance()->xSync (display, False);

                GLint attribs[] =
                {
                    GLX_RGBA,
                    GLX_DOUBLEBUFFER,
                    GLX_RED_SIZE,         cPixelFormat.redBits,
                    GLX_GREEN_SIZE,       cPixelFormat.greenBits,
                    GLX_BLUE_SIZE,        cPixelFormat.blueBits,
                    GLX_ALPHA_SIZE,       cPixelFormat.alphaBits,
                    GLX_DEPTH_SIZE,       cPixelFormat.depthBufferBits,
                    GLX_STENCIL_SIZE,     cPixelFormat.stencilBufferBits,
                    GLX_ACCUM_RED_SIZE,   cPixelFormat.accumulationBufferRedBits,
                    GLX_ACCUM_GREEN_SIZE, cPixelFormat.accumulationBufferGreenBits,
                    GLX_ACCUM_BLUE_SIZE,  cPixelFormat.accumulationBufferBlueBits,
                    GLX_ACCUM_ALPHA_SIZE, cPixelFormat.accumulationBufferAlphaBits,
                    None
                };

                bestVisual = glXChooseVisual (display, X11Symbols::getInstance()->xDefaultScreen (display), attribs);
                if (bestVisual == nullptr)
                    return;

                auto* peer = component.getPeer();
                jassert (peer != nullptr);

                auto windowH = (Window) peer->getNativeHandle();
                auto colourMap = X11Symbols::getInstance()->xCreateColormap (display, windowH, bestVisual->visual, AllocNone);

                XSetWindowAttributes swa;
                swa.colormap = colourMap;
                swa.border_pixel = 0;
                swa.event_mask = embeddedWindowEventMask;

                auto glBounds = component.getTopLevelComponent()
                                   ->getLocalArea (&component, component.getLocalBounds());

                glBounds = Desktop::getInstance().getDisplays().logicalToPhysical (glBounds);

                embeddedWindow = X11Symbols::getInstance()->xCreateWindow (display, windowH,
                                                                           glBounds.getX(), glBounds.getY(),
                                                                           (unsigned int) jmax (1, glBounds.getWidth()),
                                                                           (unsigned int) jmax (1, glBounds.getHeight()),
                                                                           0, bestVisual->depth,
                                                                           InputOutput,
                                                                           bestVisual->visual,
                                                                           CWBorderPixel | CWColormap | CWEventMask,
                                                                           &swa);

                X11Symbols::getInstance()->xSaveContext (display, (XID) embeddedWindow, windowHandleXContext, (XPointer) peer);

                X11Symbols::getInstance()->xMapWindow (display, embeddedWindow);
                X11Symbols::getInstance()->xFreeColormap (display, colourMap);

                X11Symbols::getInstance()->xSync (display, False);

                aloe_LinuxAddRepaintListener (peer, &dummy);
        */
    }
    
    pub fn initialise_on_render_thread(&mut self, c: &mut OpenGLContext) -> bool {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;
                renderContext = glXCreateContext (display, bestVisual, (GLXContext) contextToShareWith, GL_TRUE);
                c.makeActive();
                context = &c;

                return true;
        */
    }
    
    pub fn shutdown_on_render_thread(&mut self)  {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;
                context = nullptr;
                deactivateCurrentContext();
                glXDestroyContext (display, renderContext);
                renderContext = nullptr;
        */
    }
    
    pub fn make_active(&self) -> bool {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;
                return renderContext != nullptr
                         && glXMakeCurrent (display, embeddedWindow, renderContext);
        */
    }
    
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;
                return glXGetCurrentContext() == renderContext && renderContext != nullptr;
        */
    }
    
    pub fn deactivate_current_context()  {
        
        todo!();
        /*
            if (auto* display = XWindowSystem::getInstance()->getDisplay())
                {
                    XWindowSystemUtilities::ScopedXLock xLock;
                    glXMakeCurrent (display, None, nullptr);
                }
        */
    }
    
    pub fn swap_buffers(&mut self)  {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;
                glXSwapBuffers (display, embeddedWindow);
        */
    }
    
    pub fn update_window_position(&mut self, new_bounds: Rectangle<i32>)  {
        
        todo!();
        /*
            bounds = newBounds;
                auto physicalBounds = Desktop::getInstance().getDisplays().logicalToPhysical (bounds);

                XWindowSystemUtilities::ScopedXLock xLock;
                X11Symbols::getInstance()->xMoveResizeWindow (display, embeddedWindow,
                                                              physicalBounds.getX(), physicalBounds.getY(),
                                                              (unsigned int) jmax (1, physicalBounds.getWidth()),
                                                              (unsigned int) jmax (1, physicalBounds.getHeight()));
        */
    }
    
    pub fn set_swap_interval(&mut self, num_frames_per_swap: i32) -> bool {
        
        todo!();
        /*
            if (numFramesPerSwap == swapFrames)
                    return true;

                if (auto GLXSwapIntervalSGI
                      = (PFNGLXSWAPINTERVALSGIPROC) OpenGLHelpers::getExtensionFunction ("glXSwapIntervalSGI"))
                {
                    XWindowSystemUtilities::ScopedXLock xLock;
                    swapFrames = numFramesPerSwap;
                    GLXSwapIntervalSGI (numFramesPerSwap);
                    return true;
                }

                return false;
        */
    }
    
    pub fn get_swap_interval(&self) -> i32 {
        
        todo!();
        /*
            return swapFrames;
        */
    }
    
    pub fn created_ok(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_raw_context(&self)  {
        
        todo!();
        /*
            return renderContext;
        */
    }
    
    pub fn get_frame_bufferid(&self) -> GLuint {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn trigger_repaint(&mut self)  {
        
        todo!();
        /*
            if (context != nullptr)
                    context->triggerRepaint();
        */
    }
}
