crate::ix!();

#[cfg(target_os="android")]
#[no_copy]
#[leak_detector]
pub struct OpenGLContextNativeContext<'a> {
    base:                    SurfaceHolderCallback,
    component:               &'a mut Component<'a>,
    has_initialised:         bool, // default = false
    surface_view:            GlobalRef,
    last_bounds:             Rectangle<i32>,
    aloe_context:            *mut OpenGLContext<'a>, // default = nullptr
    surface:                 EGLSurface,
    context:                 EGLContext,
    surface_holder_callback: GlobalRef,
}

#[cfg(target_os="android")]
impl<'a> Drop for OpenGLContextNativeContext<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            auto env = getEnv();

            if (jobject viewParent = env->CallObjectMethod (surfaceView.get(), AloeOpenGLViewSurface.getParent))
                env->CallVoidMethod (viewParent, AndroidViewGroup.removeView, surfaceView.get());
        */
    }
}

#[cfg(target_os="android")]
lazy_static!{
    /*
    static EGLDisplay native_context_display;
        static EGLConfig native_context_config;
    */
}

#[cfg(target_os="android")]
impl<'a> OpenGLContextNativeContext<'a> {
    
    pub fn new(
        comp:                  &mut Component,
        pixel_format:          &OpenGLPixelFormat,
        context_to_share_with: *mut c_void,
        use_multisampling:     bool,
        _4:                    OpenGLVersion) -> Self {
    
        todo!();
        /*
        : component(comp),
        : surface(EGL_NO_SURFACE),
        : context(EGL_NO_CONTEXT),

            auto env = getEnv();

            // Do we have a native peer that we can attach to?
            if (component.getPeer()->getNativeHandle() == nullptr)
                return;

            // Initialise the EGL display
            if (! initEGLDisplay())
                return;

            // create a native surface view
            surfaceView = GlobalRef (LocalRef<jobject>(env->NewObject (AloeOpenGLViewSurface,
                                                                       AloeOpenGLViewSurface.constructor,
                                                                       getAppContext().get(),
                                                                       reinterpret_cast<jlong> (this))));
            if (surfaceView.get() == nullptr)
                return;

            // add the view to the view hierarchy
            // after this the nativecontext can receive callbacks
            env->CallVoidMethod ((jobject) component.getPeer()->getNativeHandle(),
                                 AndroidViewGroup.addView, surfaceView.get());

            // initialise the geometry of the view
            auto bounds = component.getTopLevelComponent()->getLocalArea (&component, component.getLocalBounds());
            bounds *= component.getDesktopScaleFactor();

            updateWindowPosition (bounds);
            hasInitialised = true;
        */
    }
    
    pub fn initialise_on_render_thread(&mut self, a_context: &mut OpenGLContext) -> bool {
        
        todo!();
        /*
            jassert (hasInitialised);

            // has the context already attached?
            jassert (surface == EGL_NO_SURFACE && context == EGL_NO_CONTEXT);

            auto env = getEnv();

            ANativeWindow* window = nullptr;

            LocalRef<jobject> holder (env->CallObjectMethod (surfaceView.get(), AloeOpenGLViewSurface.getHolder));

            if (holder != nullptr)
            {
                LocalRef<jobject> jSurface (env->CallObjectMethod (holder.get(), AndroidSurfaceHolder.getSurface));

                if (jSurface != nullptr)
                {
                    window = ANativeWindow_fromSurface(env, jSurface.get());

                    // if we didn't succeed the first time, wait 25ms and try again
                    if (window == nullptr)
                    {
                        Thread::sleep (200);
                        window = ANativeWindow_fromSurface (env, jSurface.get());
                    }
                }
            }

            if (window == nullptr)
            {
                // failed to get a pointer to the native window after second try so bail out
                jassertfalse;
                return false;
            }

            // create the surface
            surface = eglCreateWindowSurface (display, config, window, nullptr);
            jassert (surface != EGL_NO_SURFACE);

            ANativeWindow_release (window);

            // create the OpenGL context
            EGLint contextAttribs[] = { EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE };
            context = eglCreateContext (display, config, EGL_NO_CONTEXT, contextAttribs);
            jassert (context != EGL_NO_CONTEXT);

            aloeContext = &aContext;
            return true;
        */
    }
    
    pub fn shutdown_on_render_thread(&mut self)  {
        
        todo!();
        /*
            jassert (hasInitialised);

            // is there a context available to detach?
            jassert (surface != EGL_NO_SURFACE && context != EGL_NO_CONTEXT);

            eglDestroyContext (display, context);
            context = EGL_NO_CONTEXT;

            eglDestroySurface (display, surface);
            surface = EGL_NO_SURFACE;
        */
    }
    
    pub fn make_active(&self) -> bool {
        
        todo!();
        /*
            if (! hasInitialised)
                return false;

            if (surface == EGL_NO_SURFACE || context == EGL_NO_CONTEXT)
                return false;

            if (! eglMakeCurrent (display, surface, surface, context))
                return false;

            return true;
        */
    }
    
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return eglGetCurrentContext() == context;
        */
    }
    
    pub fn deactivate_current_context()  {
        
        todo!();
        /*
            eglMakeCurrent (display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
        */
    }
    
    pub fn swap_buffers(&self)  {
        
        todo!();
        /*
            eglSwapBuffers (display, surface);
        */
    }
    
    pub fn set_swap_interval(&mut self, _0: i32) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_swap_interval(&self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn created_ok(&self) -> bool {
        
        todo!();
        /*
            return hasInitialised;
        */
    }
    
    pub fn get_raw_context(&self)  {
        
        todo!();
        /*
            return surfaceView.get();
        */
    }
    
    pub fn get_frame_bufferid(&self) -> GLuint {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn update_window_position(&mut self, bounds: Rectangle<i32>)  {
        
        todo!();
        /*
            if (lastBounds != bounds)
            {
                auto env = getEnv();

                lastBounds = bounds;
                auto r = bounds * Desktop::getInstance().getDisplays().getPrimaryDisplay()->scale;

                env->CallVoidMethod (surfaceView.get(), AloeOpenGLViewSurface.layout,
                                     (jint) r.getX(), (jint) r.getY(), (jint) r.getRight(), (jint) r.getBottom());
            }
        */
    }

    /**
      | Android Surface Callbacks:
      |
      */
    pub fn surface_changed(&mut self, 
        holder: LocalRef<jobject>,
        format: i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            ignoreUnused (holder, format, width, height);
        */
    }
    
    pub fn surface_created(&mut self, holder: LocalRef<jobject>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn surface_destroyed(&mut self, holder: LocalRef<jobject>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn attached_to_window(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

            LocalRef<jobject> holder (env->CallObjectMethod (surfaceView.get(), AloeOpenGLViewSurface.getHolder));

            if (surfaceHolderCallback == nullptr)
                surfaceHolderCallback = GlobalRef (CreateJavaInterface (this, "android/view/SurfaceHolder$Callback"));

            env->CallVoidMethod (holder, AndroidSurfaceHolder.addCallback, surfaceHolderCallback.get());
        */
    }
    
    pub fn detached_from_window(&mut self)  {
        
        todo!();
        /*
            if (surfaceHolderCallback != nullptr)
            {
                auto* env = getEnv();

                LocalRef<jobject> holder (env->CallObjectMethod (surfaceView.get(), AloeOpenGLViewSurface.getHolder));

                env->CallVoidMethod (holder.get(), AndroidSurfaceHolder.removeCallback, surfaceHolderCallback.get());
                surfaceHolderCallback.clear();
            }
        */
    }
    
    pub fn dispatch_draw(&mut self, canvas: jobject)  {
        
        todo!();
        /*
            if (aloeContext != nullptr)
                aloeContext->triggerRepaint();
        */
    }
    
    pub fn init_egl_display(&mut self) -> bool {
        
        todo!();
        /*
            // already initialised?
            if (display != EGL_NO_DISPLAY)
                return true;

            const EGLint attribs[] =
            {
                EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
                EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
                EGL_BLUE_SIZE, 8,
                EGL_GREEN_SIZE, 8,
                EGL_RED_SIZE, 8,
                EGL_ALPHA_SIZE, 0,
                EGL_DEPTH_SIZE, 16,
                EGL_NONE
            };

            EGLint numConfigs;

            if ((display = eglGetDisplay (EGL_DEFAULT_DISPLAY)) == EGL_NO_DISPLAY)
            {
                jassertfalse;
                return false;
            }

            if (! eglInitialize (display, nullptr, nullptr))
            {
                jassertfalse;
                return false;
            }

            if (! eglChooseConfig (display, attribs, &config, 1, &numConfigs))
            {
                eglTerminate (display);
                jassertfalse;
                return false;
            }

            return true;
        */
    }
    
    #[cfg(target_os="android")]
    pub fn surface_created(&mut self, holder: LocalRef<jobject>)  {
        
        todo!();
        /*
            ignoreUnused (holder);

            if (auto* cachedImage = OpenGLContextCachedImage::get (component))
            {
                if (auto* pool = cachedImage->renderThread.get())
                {
                    if (! pool->contains (cachedImage))
                    {
                        cachedImage->resume();
                        cachedImage->context.triggerRepaint();
                    }
                }
            }
        */
    }
    
    #[cfg(target_os="android")]
    pub fn surface_destroyed(&mut self, holder: LocalRef<jobject>)  {
        
        todo!();
        /*
            ignoreUnused (holder);

            // unlike the name suggests this will be called just before the
            // surface is destroyed. We need to pause the render thread.
            if (auto* cachedImage = OpenGLContextCachedImage::get (component))
            {
                cachedImage->pause();

                if (auto* threadPool = cachedImage->renderThread.get())
                    threadPool->waitForJobToFinish (cachedImage, -1);
            }
        */
    }
}
