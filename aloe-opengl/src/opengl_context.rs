crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLContext.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLContext.cpp]

/**
  | Creates an OpenGL context, which can
  | be attached to a component.
  | 
  | To render some OpenGL, you should create
  | an instance of an OpenGLContext, and
  | call attachTo() to make it use a component
  | as its render target.
  | 
  | To provide threaded rendering, you
  | can supply an OpenGLRenderer object
  | that will be used to render each frame.
  | 
  | Before your target component or OpenGLRenderer
  | is deleted, you MUST call detach() or
  | delete the OpenGLContext to allow the
  | background thread to stop and the native
  | resources to be freed safely.
  | 
  | @see OpenGLRenderer
  | 
  | @tags{OpenGL}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLContext<'a> {

    /**
      | This structure holds a set of dynamically
      | loaded GL functions for use on this context.
      |
      */
    extensions:            OpenGLExtensionFunctions,
    native_context:        *mut OpenGLContextNativeContext, // default = nullptr
    renderer:              *mut dyn OpenGLRenderer, // default = nullptr
    current_render_scale:  f64, // default = 1.0
    attachment:            Box<OpenGLContextAttachment<'a>>,
    open_gl_pixel_format:  OpenGLPixelFormat,
    context_to_share_with: *mut c_void, // default = nullptr
    version_required:      OpenGLContextOpenGLVersion, // default = defaultGLVersion
    image_cache_max_size:  usize, // default = 8 * 1024 * 1024
    render_components:     bool, // default = true
    use_multisampling:     bool, // default = false
    override_can_attach:   bool, // default = false
    continuous_repaint:    AtomicBool, // default = { false  }
    tex_mag_filter:        OpenGLContextTextureMagnificationFilter, // default = linear
}

lazy_static!{
    /*
    static ThreadLocalValue<OpenGLContext*> currentThreadActiveContext;
    */
}

pub fn context_has_texture_npot_feature() -> bool {
    
    todo!();
        /*
            if (getOpenGLVersion() >= Version (2))
            return true;

        // If the version is < 2, we can't use the newer extension-checking API
        // so we have to use glGetString
        const auto* extensionsBegin = glGetString (GL_EXTENSIONS);

        if (extensionsBegin == nullptr)
            return false;

        const auto* extensionsEnd = findNullTerminator (extensionsBegin);
        const std::string extensionsString (extensionsBegin, extensionsEnd);
        const auto stringTokens = StringArray::fromTokens (extensionsString.c_str(), false);
        return stringTokens.contains ("GL_ARB_texture_non_power_of_two");
        */
}

impl<'a> Drop for OpenGLContext<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            detach();
        */
    }
}

impl<'a> OpenGLContext<'a> {

    /**
      | Gives the context an OpenGLRenderer
      | to use to do the drawing.
      | 
      | The object that you give it will not be
      | owned by the context, so it's the caller's
      | responsibility to manage its lifetime
      | and make sure that it doesn't get deleted
      | while the context may be using it. To
      | stop the context using a renderer, just
      | call this method with a null pointer.
      | 
      | -----------
      | @note
      | 
      | This must be called BEFORE attaching
      | your context to a target component!
      |
      */
    pub fn set_renderer(&mut self, renderer_to_use: *mut dyn OpenGLRenderer)  {
        
        todo!();
        /*
            // This method must not be called when the context has already been attached!
        // Call it before attaching your context, or use detach() first, before calling this!
        jassert (nativeContext == nullptr);

        renderer = rendererToUse;
        */
    }
    
    /**
      | Enables or disables the use of the GL
      | context to perform 2D rendering of the
      | component to which it is attached.
      | 
      | If this is false, then only your OpenGLRenderer
      | will be used to perform any rendering.
      | If true, then each time your target's
      | paint() method needs to be called, an
      | OpenGLGraphicsContext will be used
      | to render it, (after calling your OpenGLRenderer
      | if there is one).
      | 
      | By default this is set to true. If you're
      | not using any paint() method functionality
      | and are doing all your rendering in an
      | OpenGLRenderer, you should disable
      | it to improve performance.
      | 
      | -----------
      | @note
      | 
      | This must be called BEFORE attaching
      | your context to a target component!
      |
      */
    pub fn set_component_painting_enabled(&mut self, should_paint_component: bool)  {
        
        todo!();
        /*
            // This method must not be called when the context has already been attached!
        // Call it before attaching your context, or use detach() first, before calling this!
        jassert (nativeContext == nullptr);

        renderComponents = shouldPaintComponent;
        */
    }
    
    /**
      | Enables or disables continuous repainting.
      | 
      | If set to true, the context will run a
      | loop, re-rendering itself without
      | waiting for triggerRepaint() to be
      | called, at a frequency determined by
      | the swap interval (see setSwapInterval).
      | If false, then after each render callback,
      | it will wait for another call to triggerRepaint()
      | before rendering again.
      | 
      | This is disabled by default. @see setSwapInterval
      |
      */
    pub fn set_continuous_repainting(&mut self, should_continuously_repaint: bool)  {
        
        todo!();
        /*
            continuousRepaint = shouldContinuouslyRepaint;

        #if ALOE_MAC
         if (auto* component = getTargetComponent())
         {
             detach();
             attachment.reset (new OpenGLContextAttachment (*this, *component));
         }
        #endif

        triggerRepaint();
        */
    }
    
    /**
      | Sets the pixel format which you'd like
      | to use for the target GL surface.
      | 
      | -----------
      | @note
      | 
      | This must be called BEFORE attaching
      | your context to a target component!
      |
      */
    pub fn set_pixel_format(&mut self, preferred_pixel_format: &OpenGLPixelFormat)  {
        
        todo!();
        /*
            // This method must not be called when the context has already been attached!
        // Call it before attaching your context, or use detach() first, before calling this!
        jassert (nativeContext == nullptr);

        openGLPixelFormat = preferredPixelFormat;
        */
    }
    
    /**
      | Sets the texture magnification filter.
      | By default the texture magnification
      | filter is linear. However, for faster
      | rendering you may want to use the 'nearest'
      | magnification filter. This option
      | will not affect any textures created
      | before this function was called.
      |
      */
    pub fn set_texture_magnification_filter(&mut self, mag_filter_mode: OpenGLContextTextureMagnificationFilter)  {
        
        todo!();
        /*
            texMagFilter = magFilterMode;
        */
    }
    
    /**
      | Provides a context with which you'd
      | like this context's resources to be
      | shared.
      | 
      | The object passed-in here is a platform-dependent
      | native context object, and must not
      | be deleted while this context may still
      | be using it! To turn off sharing, you
      | can call this method with a null pointer.
      | 
      | -----------
      | @note
      | 
      | This must be called BEFORE attaching
      | your context to a target component!
      |
      */
    pub fn set_native_shared_context(&mut self, native_context_to_share_with: *mut c_void)  {
        
        todo!();
        /*
            // This method must not be called when the context has already been attached!
        // Call it before attaching your context, or use detach() first, before calling this!
        jassert (nativeContext == nullptr);

        contextToShareWith = nativeContextToShareWith;
        */
    }
    
    /**
      | Enables multisampling on platforms
      | where this is implemented.
      | 
      | If enabling this, you must call this
      | method before attachTo().
      |
      */
    pub fn set_multisampling_enabled(&mut self, b: bool)  {
        
        todo!();
        /*
            // This method must not be called when the context has already been attached!
        // Call it before attaching your context, or use detach() first, before calling this!
        jassert (nativeContext == nullptr);

        useMultisampling = b;
        */
    }
    
    /**
      | Sets a preference for the version of
      | GL that this context should use, if possible.
      | 
      | Some platforms may ignore this value.
      |
      */
    pub fn set_open_gl_version_required(&mut self, v: OpenGLContextOpenGLVersion)  {
        
        todo!();
        /*
            versionRequired = v;
        */
    }
    
    /**
      | Attaches the context to a target component.
      | 
      | If the component is not fully visible,
      | this call will wait until the component
      | is shown before actually creating a
      | native context for it.
      | 
      | When a native context is created, a thread
      | is started, and will be used to call the
      | OpenGLRenderer methods. The context
      | will be floated above the target component,
      | and when the target moves, it will track
      | it. If the component is hidden/shown,
      | the context may be deleted and re-created.
      |
      */
    pub fn attach_to(&mut self, component: &mut Component)  {
        
        todo!();
        /*
            component.repaint();

        if (getTargetComponent() != &component)
        {
            detach();
            attachment.reset (new OpenGLContextAttachment (*this, component));
        }
        */
    }
    
    /**
      | Detaches the context from its target
      | component and deletes any native resources.
      | 
      | If the context has not been attached,
      | this will do nothing. Otherwise, it
      | will block until the context and its
      | thread have been cleaned up.
      |
      */
    pub fn detach(&mut self)  {
        
        todo!();
        /*
            if (auto* a = attachment.get())
        {
            a->detach(); // must detach before nulling our pointer
            attachment.reset();
        }

        nativeContext = nullptr;
        */
    }
    
    /**
      | Returns true if the context is attached
      | to a component and is on-screen.
      | 
      | -----------
      | @note
      | 
      | if you call attachTo() for a non-visible
      | component, this method will return
      | false until the component is made visible.
      |
      */
    pub fn is_attached(&self) -> bool {
        
        todo!();
        /*
            return nativeContext != nullptr;
        */
    }
    
    /**
      | Returns the component to which this
      | context is currently attached, or nullptr.
      |
      */
    pub fn get_target_component(&self) -> *mut Component {
        
        todo!();
        /*
            return attachment != nullptr ? attachment->getComponent() : nullptr;
        */
    }
    
    /**
      | If the given component has an OpenGLContext
      | attached, then this will return it.
      |
      */
    pub fn get_context_attached_to(&mut self, c: &mut Component) -> *mut OpenGLContext {
        
        todo!();
        /*
            if (auto* ci = OpenGLContextCachedImage::get (c))
            return &(ci->context);

        return nullptr;
        */
    }
    
    /**
      | Returns the context that's currently
      | in active use by the calling thread,
      | or nullptr if no context is active.
      |
      */
    pub fn get_current_context(&mut self) -> *mut OpenGLContext {
        
        todo!();
        /*
            return currentThreadActiveContext.get();
        */
    }
    
    /**
      | Makes this context the currently active
      | one.
      | 
      | You should never need to call this in
      | normal use - the context will already
      | be active when OpenGLRenderer::renderOpenGL()
      | is invoked.
      |
      */
    pub fn make_active(&self) -> bool {
        
        todo!();
        /*
            auto& current = currentThreadActiveContext.get();

        if (nativeContext != nullptr && nativeContext->makeActive())
        {
            current = const_cast<OpenGLContext*> (this);
            return true;
        }

        current = nullptr;
        return false;
        */
    }
    
    /**
      | Returns true if this context is currently
      | active for the calling thread.
      |
      */
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return nativeContext != nullptr && nativeContext->isActive();
        */
    }
    
    /**
      | If any context is active on the current
      | thread, this deactivates it.
      | 
      | -----------
      | @note
      | 
      | on some platforms, like Android, this
      | isn't possible.
      |
      */
    pub fn deactivate_current_context(&mut self)  {
        
        todo!();
        /*
            OpenGLContextNativeContext::deactivateCurrentContext();
        currentThreadActiveContext.get() = nullptr;
        */
    }
    
    /**
      | Asynchronously causes a repaint to
      | be made.
      |
      */
    pub fn trigger_repaint(&mut self)  {
        
        todo!();
        /*
            if (auto* cachedImage = getCachedImage())
            cachedImage->triggerRepaint();
        */
    }
    
    /**
      | Swaps the buffers (if the context can
      | do this).
      | 
      | There's normally no need to call this
      | directly - the buffers will be swapped
      | automatically after your OpenGLRenderer::renderOpenGL()
      | method has been called.
      |
      */
    pub fn swap_buffers(&mut self)  {
        
        todo!();
        /*
            if (nativeContext != nullptr)
            nativeContext->swapBuffers();
        */
    }
    
    /**
      | If this context is backed by a frame buffer,
      | this returns its ID number, or 0 if the
      | context does not use a framebuffer.
      |
      */
    pub fn get_frame_bufferid(&self) -> u32 {
        
        todo!();
        /*
            return nativeContext != nullptr ? nativeContext->getFrameBufferID() : 0;
        */
    }
    
    /**
      | Sets whether the context checks the
      | vertical sync before swapping.
      | 
      | The value is the number of frames to allow
      | between buffer-swapping. This is fairly
      | system-dependent, but 0 turns off syncing,
      | 1 makes it swap on frame-boundaries,
      | and greater numbers indicate that it
      | should swap less often.
      | 
      | By default, this will be set to 1.
      | 
      | Returns true if it sets the value successfully
      | - some platforms won't support this
      | setting.
      | 
      | @see setContinuousRepainting
      |
      */
    pub fn set_swap_interval(&mut self, num_frames_per_swap: i32) -> bool {
        
        todo!();
        /*
            return nativeContext != nullptr && nativeContext->setSwapInterval (numFramesPerSwap);
        */
    }
    
    /**
      | Returns the current swap-sync interval.
      | 
      | See setSwapInterval() for info about
      | the value returned.
      |
      */
    pub fn get_swap_interval(&self) -> i32 {
        
        todo!();
        /*
            return nativeContext != nullptr ? nativeContext->getSwapInterval() : 0;
        */
    }
    
    /**
      | Returns an OS-dependent handle to some
      | kind of underlying OS-provided GL context.
      | 
      | The exact type of the value returned
      | will depend on the OS and may change if
      | the implementation changes. If you
      | want to use this, digging around in the
      | native code is probably the best way
      | to find out what it is.
      |
      */
    pub fn get_raw_context(&self)  {
        
        todo!();
        /*
            return nativeContext != nullptr ? nativeContext->getRawContext() : nullptr;
        */
    }
    
    pub fn get_cached_image(&self) -> *mut OpenGLContextCachedImage {
        
        todo!();
        /*
            if (auto* comp = getTargetComponent())
            return OpenGLContextCachedImage::get (*comp);

        return nullptr;
        */
    }
    
    /**
      | Returns true if shaders can be used in
      | this context.
      |
      */
    pub fn are_shaders_available(&self) -> bool {
        
        todo!();
        /*
            auto* c = getCachedImage();
        return c != nullptr && c->shadersAvailable;
        */
    }
    
    /**
      | Returns true if non-power-of-two textures
      | are supported in this context.
      |
      */
    pub fn is_texture_npot_supported(&self) -> bool {
        
        todo!();
        /*
            auto* c = getCachedImage();
        return c != nullptr && c->textureNpotSupported;
        */
    }
    
    /**
      | This retrieves an object that was previously
      | stored with setAssociatedObject().
      | 
      | If no object is found with the given name,
      | this will return nullptr.
      | 
      | This method must only be called from
      | within the GL rendering methods. @see
      | setAssociatedObject
      |
      */
    pub fn get_associated_object(&self, name: *const u8) -> *mut ReferenceCountedObject {
        
        todo!();
        /*
            jassert (name != nullptr);

        auto* c = getCachedImage();

        // This method must only be called from an openGL rendering callback.
        jassert (c != nullptr && nativeContext != nullptr);
        jassert (getCurrentContext() != nullptr);

        auto index = c->associatedObjectNames.indexOf (name);
        return index >= 0 ? c->associatedObjects.getUnchecked (index).get() : nullptr;
        */
    }
    
    /**
      | Attaches a named object to the context,
      | which will be deleted when the context
      | is destroyed.
      | 
      | This allows you to store an object which
      | will be released before the context
      | is deleted. The main purpose is for caching
      | GL objects such as shader programs,
      | which will become invalid when the context
      | is deleted.
      | 
      | This method must only be called from
      | within the GL rendering methods.
      |
      */
    pub fn set_associated_object(
        &mut self, 
        name:       *const u8,
        new_object: *mut ReferenceCountedObject

    ) {
        
        todo!();
        /*
            jassert (name != nullptr);

        if (auto* c = getCachedImage())
        {
            // This method must only be called from an openGL rendering callback.
            jassert (nativeContext != nullptr);
            jassert (getCurrentContext() != nullptr);

            const int index = c->associatedObjectNames.indexOf (name);

            if (index >= 0)
            {
                if (newObject != nullptr)
                {
                    c->associatedObjects.set (index, newObject);
                }
                else
                {
                    c->associatedObjectNames.remove (index);
                    c->associatedObjects.remove (index);
                }
            }
            else if (newObject != nullptr)
            {
                c->associatedObjectNames.add (name);
                c->associatedObjects.add (newObject);
            }
        }
        */
    }
    
    /**
      | Changes the amount of GPU memory that
      | the internal cache for Images is allowed
      | to use.
      |
      */
    pub fn set_image_cache_size(&mut self, new_size: usize)  {
        
        todo!();
        /*
            imageCacheMaxSize = newSize;
        */
    }
    
    /**
      | Returns the amount of GPU memory that
      | the internal cache for Images is allowed
      | to use.
      |
      */
    pub fn get_image_cache_size(&self) -> usize {
        
        todo!();
        /*
            return imageCacheMaxSize;
        */
    }
    
    pub fn execute(
        &mut self, 
        worker_to_use: OpenGLContextAsyncWorkerPtr,
        should_block:  bool

    ) {
        
        todo!();
        /*
            if (auto* c = getCachedImage())
            c->execute (std::move (workerToUse), shouldBlock);
        else
            jassertfalse; // You must have attached the context to a component
        */
    }
    
    /**
      | Draws the currently selected texture
      | into this context at its original size.
      | 
      | -----------
      | @param targetClipArea
      | 
      | the target area to draw into (in top-left
      | origin coords)
      | ----------
      | @param anchorPosAndTextureSize
      | 
      | the position of this rectangle is the
      | texture's top-left anchor position
      | in the target space, and the size must
      | be the total size of the texture.
      | ----------
      | @param contextWidth
      | 
      | the width of the context or framebuffer
      | that is being drawn into, used for scaling
      | of the coordinates.
      | ----------
      | @param contextHeight
      | 
      | the height of the context or framebuffer
      | that is being drawn into, used for vertical
      | flipping of the y coordinates.
      | ----------
      | @param textureOriginIsBottomLeft
      | 
      | if true, the texture's origin is treated
      | as being at (0, 0). If false, it is assumed
      | to be (0, 1)
      |
      */
    pub fn copy_texture(&mut self, 
        target_clip_area:            &Rectangle<i32>,
        anchor_pos_and_texture_size: &Rectangle<i32>,
        context_width:               i32,
        context_height:              i32,
        flipped_vertically:          bool)  {
        
        todo!();
        /*
            if (contextWidth <= 0 || contextHeight <= 0)
            return;

        ALOE_CHECK_OPENGL_ERROR
        glBlendFunc (GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
        glEnable (GL_BLEND);

        DepthTestDisabler depthDisabler;

        if (areShadersAvailable())
        {
            struct OverlayShaderProgram  : public ReferenceCountedObject
            {
                OverlayShaderProgram (OpenGLContext& context)
                    : program (context), builder (program), params (program)
                {}

                static const OverlayShaderProgram& select (OpenGLContext& context)
                {
                    static const char programValueID[] = "aloeGLComponentOverlayShader";
                    OverlayShaderProgram* program = static_cast<OverlayShaderProgram*> (context.getAssociatedObject (programValueID));

                    if (program == nullptr)
                    {
                        program = new OverlayShaderProgram (context);
                        context.setAssociatedObject (programValueID, program);
                    }

                    program->program.use();
                    return *program;
                }

                struct ProgramBuilder
                {
                    ProgramBuilder (OpenGLShaderProgram& prog)
                    {
                        prog.addVertexShader (OpenGLHelpers::translateVertexShaderToV3 (
                            "attribute " ALOE_HIGHP " vec2 position;"
                            "uniform " ALOE_HIGHP " vec2 screenSize;"
                            "uniform " ALOE_HIGHP " float textureBounds[4];"
                            "uniform " ALOE_HIGHP " vec2 vOffsetAndScale;"
                            "varying " ALOE_HIGHP " vec2 texturePos;"
                            "void main()"
                            "{"
                              ALOE_HIGHP " vec2 scaled = position / (0.5 * screenSize.xy);"
                              "gl_Position = vec4 (scaled.x - 1.0, 1.0 - scaled.y, 0, 1.0);"
                              "texturePos = (position - vec2 (textureBounds[0], textureBounds[1])) / vec2 (textureBounds[2], textureBounds[3]);"
                              "texturePos = vec2 (texturePos.x, vOffsetAndScale.x + vOffsetAndScale.y * texturePos.y);"
                            "}"));

                        prog.addFragmentShader (OpenGLHelpers::translateFragmentShaderToV3 (
                            "uniform sampler2D imageTexture;"
                            "varying " ALOE_HIGHP " vec2 texturePos;"
                            "void main()"
                            "{"
                              "gl_FragColor = texture2D (imageTexture, texturePos);"
                            "}"));

                        prog.link();
                    }
                };

                struct Params
                {
                    Params (OpenGLShaderProgram& prog)
                        : positionAttribute (prog, "position"),
                          screenSize (prog, "screenSize"),
                          imageTexture (prog, "imageTexture"),
                          textureBounds (prog, "textureBounds"),
                          vOffsetAndScale (prog, "vOffsetAndScale")
                    {}

                    void set (const float targetWidth, const float targetHeight, const Rectangle<float>& bounds, bool flipVertically) const
                    {
                        const GLfloat m[] = { bounds.getX(), bounds.getY(), bounds.getWidth(), bounds.getHeight() };
                        textureBounds.set (m, 4);
                        imageTexture.set (0);
                        screenSize.set (targetWidth, targetHeight);

                        vOffsetAndScale.set (flipVertically ? 0.0f : 1.0f,
                                             flipVertically ? 1.0f : -1.0f);
                    }

                    OpenGLShaderProgram::Attribute positionAttribute;
                    OpenGLShaderProgram::Uniform screenSize, imageTexture, textureBounds, vOffsetAndScale;
                };

                OpenGLShaderProgram program;
                ProgramBuilder builder;
                Params params;
            };

            auto left   = (GLshort) targetClipArea.getX();
            auto top    = (GLshort) targetClipArea.getY();
            auto right  = (GLshort) targetClipArea.getRight();
            auto bottom = (GLshort) targetClipArea.getBottom();
            const GLshort vertices[] = { left, bottom, right, bottom, left, top, right, top };

            auto& program = OverlayShaderProgram::select (*this);
            program.params.set ((float) contextWidth, (float) contextHeight, anchorPosAndTextureSize.toFloat(), flippedVertically);

            GLuint vertexBuffer = 0;
            extensions.glGenBuffers (1, &vertexBuffer);
            extensions.glBindBuffer (GL_ARRAY_BUFFER, vertexBuffer);
            extensions.glBufferData (GL_ARRAY_BUFFER, sizeof (vertices), vertices, GL_STATIC_DRAW);

            auto index = (GLuint) program.params.positionAttribute.attributeID;
            extensions.glVertexAttribPointer (index, 2, GL_SHORT, GL_FALSE, 4, nullptr);
            extensions.glEnableVertexAttribArray (index);
            ALOE_CHECK_OPENGL_ERROR

            if (extensions.glCheckFramebufferStatus (GL_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE)
            {
                glDrawArrays (GL_TRIANGLE_STRIP, 0, 4);

                extensions.glBindBuffer (GL_ARRAY_BUFFER, 0);
                extensions.glUseProgram (0);
                extensions.glDisableVertexAttribArray (index);
                extensions.glDeleteBuffers (1, &vertexBuffer);
            }
            else
            {
                clearGLError();
            }
        }
        else
        {
            jassert (attachment == nullptr); // Running on an old graphics card!
        }

        ALOE_CHECK_OPENGL_ERROR
        */
    }

    /**
      | Returns the scale factor used by the
      | display that is being rendered.
      | 
      | The scale is that of the display - see
      | Displays::Display::scale
      | 
      | -----------
      | @note
      | 
      | this should only be called during an
      | OpenGLRenderer::renderOpenGL()
      | callback - at other times the value it
      | returns is undefined.
      |
      */
    pub fn get_rendering_scale(&self) -> f64 {
        
        todo!();
        /*
            return currentRenderScale;
        */
    }

    /**
      | Execute a lambda, function or functor
      | on the OpenGL thread with an active context.
      | 
      | This method will attempt to execute
      | functor on the OpenGL thread. If blockUntilFinished
      | is true then the method will block until
      | the functor has finished executing.
      | 
      | This function can only be called if the
      | context is attached to a component.
      | 
      | Otherwise, this function will assert.
      | 
      | This function is useful when you need
      | to execute house-keeping tasks such
      | as allocating, deallocating textures
      | or framebuffers. As such, the functor
      | will execute without locking the message
      | thread. Therefore, it is not intended
      | for any drawing commands or GUI code.
      | Any GUI code should be executed in the
      | OpenGLRenderer::renderOpenGL callback
      | instead.
      |
      */
    pub fn execute_on_gl_thread<FunctionType>(
        &mut self, 
        f:            FunctionType,
        should_block: bool

    ) {
    
        todo!();
        /*
            execute (new OpenGLContextAsyncWorkerFunctor<FunctionType> (f), shouldBlock);
        */
    }
}
