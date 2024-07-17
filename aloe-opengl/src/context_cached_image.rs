crate::ix!();

#[cfg(target_os="android")]
lazy_static!{
    /*
    EGLDisplay OpenGLContext::OpenGLContextNativeContext::display = EGL_NO_DISPLAY;
    EGLDisplay OpenGLContext::OpenGLContextNativeContext::config;
    */
}

#[no_copy]
#[leak_detector]
pub struct OpenGLContextCachedImage<'a> {
    base2:                     ThreadPoolJob<'a>,
    native_context:            Box<OpenGLContextNativeContext>,
    context:                   &'a mut OpenGLContext<'a>,
    component:                 &'a mut Component<'a>,
    open_gl_version:           OpenGLVersion,
    cached_image_frame_buffer: OpenGLFrameBuffer<'a>,
    valid_area:                RectangleList<i32>,
    viewport_area:             Rectangle<i32>,
    last_screen_bounds:        Rectangle<i32>,
    last_display:              *const Display, // default = nullptr
    scale:                     f64, // default = 1.0
    transform:                 AffineTransform,
    vertex_array_object:       GLuint, // default = 0
    associated_object_names:   Vec<String>,
    associated_objects:        ReferenceCountedArray<ReferenceCountedObject>,
    can_paint_now_flag:        WaitableEvent,
    finished_painting_flag:    WaitableEvent,
    repaint_event:             WaitableEvent, // default = { true  }

    #[cfg(ALOE_OPENGL_ES)]
    shaders_available: bool, // default = true

    #[cfg(not(ALOE_OPENGL_ES))]
    shaders_available: bool, // default = false

    texture_npot_supported:    bool, // default = false
    has_initialised:           AtomicBool, // default = { false  }
    needs_update:              AtomicBool, // default = { true  }
    destroying:                AtomicBool, // default = { false  }
    last_mm_lock_release_time: u32, // default = 0
  
    #[cfg(target_os="macos")]
    cv_display_link_wrapper:   Box<OpenGLContextCachedImageCVDisplayLinkWrapper<'a>>,

    render_thread:             Box<ThreadPool<'a>>,
    work_queue:                ReferenceCountedArray<Box<dyn OpenGLContextAsyncWorker>,CriticalSection>,
    message_manager_lock:      MessageManagerLock,

    #[cfg(target_os="ios")]
    background_process_check: iOSBackgroundProcessCheck,
}

/*
impl<'a> CachedComponentImage for OpenGLContextCachedImage<'a> {

}

impl<'a> Paint for OpenGLContextCachedImage<'a> {

    fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
            updateViewportSize (false);
        */
    }
}
*/

impl<'a> Invalidate for OpenGLContextCachedImage<'a> {

    fn invalidate(&mut self, area: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            validArea.subtract (area.toFloat().transformedBy (transform).getSmallestIntegerContainer());
            triggerRepaint();
            return false;
        */
    }
}

impl<'a> ReleaseResources for OpenGLContextCachedImage<'a> {

    fn release_resources(&mut self)  {
        
        todo!();
        /*
            stop();
        */
    }
}

impl<'a> InvalidateAll for OpenGLContextCachedImage<'a> {

    fn invalidate_all(&mut self) -> bool {
        
        todo!();
        /*
            validArea.clear();
            triggerRepaint();
            return false;
        */
    }
}

impl<'a> Drop for OpenGLContextCachedImage<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            stop();
        */
    }
}

impl<'a> OpenGLContextCachedImage<'a> {

    pub fn new(
        c:                &mut OpenGLContext,
        comp:             &mut Component,
        pix_format:       &OpenGLPixelFormat,
        context_to_share: *mut c_void) -> Self {
    
        todo!();
        /*


            : ThreadPoolJob ("OpenGL Rendering"),
              context (c), component (comp)

            nativeContext.reset (new OpenGLContextNativeContext (component, pixFormat, contextToShare,
                                                    c.useMultisampling, c.versionRequired));

            if (nativeContext->createdOk())
                context.nativeContext = nativeContext.get();
            else
                nativeContext.reset();
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            if (nativeContext != nullptr)
            {
                renderThread.reset (new ThreadPool (1));
                resume();
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (renderThread != nullptr)
            {
                // make sure everything has finished executing
                destroying = true;

                if (workQueue.size() > 0)
                {
                    if (! renderThread->contains (this))
                        resume();

                    while (workQueue.size() != 0)
                        Thread::sleep (20);
                }

                pause();
                renderThread.reset();
            }

            hasInitialised = false;
        */
    }
    
    pub fn pause(&mut self)  {
        
        todo!();
        /*
            signalJobShouldExit();
            messageManagerLock.abort();

            if (renderThread != nullptr)
            {
                repaintEvent.signal();
                renderThread->removeJob (this, true, -1);
            }
        */
    }
    
    pub fn resume(&mut self)  {
        
        todo!();
        /*
            if (renderThread != nullptr)
                renderThread->addJob (this, false);
        */
    }
    
    pub fn trigger_repaint(&mut self)  {
        
        todo!();
        /*
            needsUpdate = 1;
            repaintEvent.signal();
        */
    }
    
    pub fn ensure_frame_buffer_size(&mut self) -> bool {
        
        todo!();
        /*
            auto fbW = cachedImageFrameBuffer.getWidth();
            auto fbH = cachedImageFrameBuffer.getHeight();

            if (fbW != viewportArea.getWidth() || fbH != viewportArea.getHeight() || ! cachedImageFrameBuffer.isValid())
            {
                if (! cachedImageFrameBuffer.initialise (context, viewportArea.getWidth(), viewportArea.getHeight()))
                    return false;

                validArea.clear();
                ALOE_CHECK_OPENGL_ERROR
            }

            return true;
        */
    }
    
    pub fn clear_region_in_frame_buffer(&mut self, list: &RectangleList<i32>)  {
        
        todo!();
        /*
            glClearColor (0, 0, 0, 0);
            glEnable (GL_SCISSOR_TEST);

            auto previousFrameBufferTarget = OpenGLFrameBuffer::getCurrentFrameBufferTarget();
            cachedImageFrameBuffer.makeCurrentRenderingTarget();
            auto imageH = cachedImageFrameBuffer.getHeight();

            for (auto& r : list)
            {
                glScissor (r.getX(), imageH - r.getBottom(), r.getWidth(), r.getHeight());
                glClear (GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT);
            }

            glDisable (GL_SCISSOR_TEST);
            context.extensions.glBindFramebuffer (GL_FRAMEBUFFER, previousFrameBufferTarget);
            ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn render_frame(&mut self) -> bool {
        
        todo!();
        /*
            MessageManager::Lock::ScopedTryLockType mmLock (messageManagerLock, false);

            auto isUpdatingTestValue = true;
            auto isUpdating = needsUpdate.compare_exchange_strong (isUpdatingTestValue, false);

            if (context.renderComponents && isUpdating)
            {
                // This avoids hogging the message thread when doing intensive rendering.
                if (lastMMLockReleaseTime + 1 >= Time::getMillisecondCounter())
                    Thread::sleep (2);

                while (! shouldExit())
                {
                    doWorkWhileWaitingForLock (false);

                    if (mmLock.retryLock())
                        break;
                }

                if (shouldExit())
                    return false;
            }

            if (! context.makeActive())
                return false;

            OpenGLContextNativeContext::Locker locker (*nativeContext);

            ALOE_CHECK_OPENGL_ERROR

            doWorkWhileWaitingForLock (true);

            if (context.renderer != nullptr)
            {
                glViewport (0, 0, viewportArea.getWidth(), viewportArea.getHeight());
                context.currentRenderScale = scale;
                context.renderer->renderOpenGL();
                clearGLError();

                bindVertexArray();
            }

            if (context.renderComponents)
            {
                if (isUpdating)
                {
                    paintComponent();

                    if (! hasInitialised)
                        return false;

                    messageManagerLock.exit();
                    lastMMLockReleaseTime = Time::getMillisecondCounter();
                }

                glViewport (0, 0, viewportArea.getWidth(), viewportArea.getHeight());
                drawComponentBuffer();
            }

            context.swapBuffers();

            OpenGLContext::deactivateCurrentContext();
            return true;
        */
    }
    
    pub fn update_viewport_size(&mut self, can_trigger_update: bool)  {
        
        todo!();
        /*
            if (auto* peer = component.getPeer())
            {
                auto localBounds = component.getLocalBounds();
                const auto currentDisplay = Desktop::getInstance().getDisplays().getDisplayForRect (component.getTopLevelComponent()->getScreenBounds());

                if (currentDisplay != lastDisplay)
                {
                   #if ALOE_MAC
                    if (cvDisplayLinkWrapper != nullptr)
                    {
                        cvDisplayLinkWrapper->updateActiveDisplay (currentDisplay->totalArea.getTopLeft());
                        nativeContext->setNominalVideoRefreshPeriodS (cvDisplayLinkWrapper->getNominalVideoRefreshPeriodS());
                    }
                   #endif
                    lastDisplay = currentDisplay;
                }

                const auto displayScale = currentDisplay->scale;

                auto newArea = peer->getComponent().getLocalArea (&component, localBounds).withZeroOrigin() * displayScale;

               #if ALOE_WINDOWS && ALOE_WIN_PER_MONITOR_DPI_AWARE
                auto newScale = getScaleFactorForWindow (nativeContext->getNativeHandle());
                auto desktopScale = Desktop::getInstance().getGlobalScaleFactor();

                if (! approximatelyEqual (1.0f, desktopScale))
                    newScale *= desktopScale;
               #else
                auto newScale = displayScale;
               #endif

                if (scale != newScale || viewportArea != newArea)
                {
                    scale = newScale;
                    viewportArea = newArea;
                    transform = AffineTransform::scale ((float) newArea.getWidth()  / (float) localBounds.getWidth(),
                                                        (float) newArea.getHeight() / (float) localBounds.getHeight());

                    nativeContext->updateWindowPosition (peer->getAreaCoveredBy (component));

                    if (canTriggerUpdate)
                        invalidateAll();
                }
            }
        */
    }
    
    pub fn bind_vertex_array(&mut self)  {
        
        todo!();
        /*
            if (openGLVersion.major >= 3)
                if (vertexArrayObject != 0)
                    context.extensions.glBindVertexArray (vertexArrayObject);
        */
    }
    
    pub fn check_viewport_bounds(&mut self)  {
        
        todo!();
        /*
            auto screenBounds = component.getTopLevelComponent()->getScreenBounds();

            if (lastScreenBounds != screenBounds)
            {
                updateViewportSize (true);
                lastScreenBounds = screenBounds;
            }
        */
    }
    
    pub fn paint_component(&mut self)  {
        
        todo!();
        /*
            // you mustn't set your own cached image object when attaching a GL context!
            jassert (get (component) == this);

            if (! ensureFrameBufferSize())
                return;

            RectangleList<int> invalid (viewportArea);
            invalid.subtract (validArea);
            validArea = viewportArea;

            if (! invalid.isEmpty())
            {
                clearRegionInFrameBuffer (invalid);

                {
                    std::unique_ptr<LowLevelGraphicsContext> g (createOpenGLGraphicsContext (context, cachedImageFrameBuffer));
                    g->clipToRectangleList (invalid);
                    g->addTransform (transform);

                    paintOwner (*g);
                    ALOE_CHECK_OPENGL_ERROR
                }

                if (! context.isActive())
                    context.makeActive();
            }

            ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn draw_component_buffer(&mut self)  {
        
        todo!();
        /*
            #if ! ALOE_ANDROID
            glEnable (GL_TEXTURE_2D);
            clearGLError();
           #endif

           #if ALOE_WINDOWS
            // some stupidly old drivers are missing this function, so try to at least avoid a crash here,
            // but if you hit this assertion you may want to have your own version check before using the
            // component rendering stuff on such old drivers.
            jassert (context.extensions.glActiveTexture != nullptr);
            if (context.extensions.glActiveTexture != nullptr)
           #endif
                context.extensions.glActiveTexture (GL_TEXTURE0);

            glBindTexture (GL_TEXTURE_2D, cachedImageFrameBuffer.getTextureID());
            bindVertexArray();

            const Rectangle<int> cacheBounds (cachedImageFrameBuffer.getWidth(), cachedImageFrameBuffer.getHeight());
            context.copyTexture (cacheBounds, cacheBounds, cacheBounds.getWidth(), cacheBounds.getHeight(), false);
            glBindTexture (GL_TEXTURE_2D, 0);
            ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn paint_owner(&mut self, llgc: &mut dyn LowLevelGraphicsContext)  {
        
        todo!();
        /*
            Graphics g (llgc);

          #if ALOE_ENABLE_REPAINT_DEBUGGING
           #ifdef ALOE_IS_REPAINT_DEBUGGING_ACTIVE
            if (ALOE_IS_REPAINT_DEBUGGING_ACTIVE)
           #endif
            {
                g.saveState();
            }
           #endif

            ALOE_TRY
            {
                component.paintEntireComponent (g, false);
            }
            ALOE_CATCH_EXCEPTION

          #if ALOE_ENABLE_REPAINT_DEBUGGING
           #ifdef ALOE_IS_REPAINT_DEBUGGING_ACTIVE
            if (ALOE_IS_REPAINT_DEBUGGING_ACTIVE)
           #endif
            {
                // enabling this code will fill all areas that get repainted with a colour overlay, to show
                // clearly when things are being repainted.
                g.restoreState();

                static Random rng;
                g.fillAll (Colour ((uint8) rng.nextInt (255),
                                   (uint8) rng.nextInt (255),
                                   (uint8) rng.nextInt (255),
                                   (uint8) 0x50));
            }
           #endif
        */
    }
    
    pub fn handle_resize(&mut self)  {
        
        todo!();
        /*
            updateViewportSize (true);

           #if ALOE_MAC
            if (hasInitialised)
            {
                [nativeContext->view update];
                renderFrame();
            }
           #endif
        */
    }
    
    pub fn run_job(&mut self) -> ThreadPoolJobStatus {
        
        todo!();
        /*
            {
                // Allow the message thread to finish setting-up the context before using it..
                MessageManager::Lock::ScopedTryLockType mmLock (messageManagerLock, false);

                do
                {
                    if (shouldExit())
                        return ThreadPoolJob::jobHasFinished;

                } while (! mmLock.retryLock());
            }

            if (! initialiseOnThread())
            {
                hasInitialised = false;

                return ThreadPoolJob::jobHasFinished;
            }

            hasInitialised = true;

            while (! shouldExit())
            {
               #if ALOE_IOS
                if (backgroundProcessCheck.isBackgroundProcess())
                {
                    repaintEvent.wait (300);
                    repaintEvent.reset();
                    continue;
                }
               #endif

                if (shouldExit())
                    break;

               #if ALOE_MAC
                if (context.continuousRepaint)
                {
                    repaintEvent.wait (-1);
                    renderFrame();
                }
                else
               #endif
                if (! renderFrame())
                    repaintEvent.wait (5); // failed to render, so avoid a tight fail-loop.
                else if (! context.continuousRepaint && ! shouldExit())
                    repaintEvent.wait (-1);

                repaintEvent.reset();
            }

            hasInitialised = false;
            context.makeActive();
            shutdownOnThread();
            OpenGLContext::deactivateCurrentContext();

            return ThreadPoolJob::jobHasFinished;
        */
    }
    
    pub fn initialise_on_thread(&mut self) -> bool {
        
        todo!();
        /*
            // On android, this can get called twice, so drop any previous state..
            associatedObjectNames.clear();
            associatedObjects.clear();
            cachedImageFrameBuffer.release();

            context.makeActive();

            if (! nativeContext->initialiseOnRenderThread (context))
                return false;

           #if ALOE_ANDROID
            // On android the context may be created in initialiseOnRenderThread
            // and we therefore need to call makeActive again
            context.makeActive();
           #endif

            gl::loadFunctions();

            openGLVersion = getOpenGLVersion();

            if (openGLVersion.major >= 3)
            {
                context.extensions.glGenVertexArrays (1, &vertexArrayObject);
                bindVertexArray();
            }

            glViewport (0, 0, component.getWidth(), component.getHeight());

            nativeContext->setSwapInterval (1);

           #if ! ALOE_OPENGL_ES
            ALOE_CHECK_OPENGL_ERROR
            shadersAvailable = OpenGLShaderProgram::getLanguageVersion() > 0;
            clearGLError();
           #endif

            textureNpotSupported = contextHasTextureNpotFeature();

            if (context.renderer != nullptr)
                context.renderer->newOpenGLContextCreated();

           #if ALOE_MAC
            cvDisplayLinkWrapper = std::make_unique<OpenGLContextCachedImageCVDisplayLinkWrapper> (*this);
            nativeContext->setNominalVideoRefreshPeriodS (cvDisplayLinkWrapper->getNominalVideoRefreshPeriodS());
           #endif

            return true;
        */
    }
    
    pub fn shutdown_on_thread(&mut self)  {
        
        todo!();
        /*
            #if ALOE_MAC
            cvDisplayLinkWrapper = nullptr;
           #endif

            if (context.renderer != nullptr)
                context.renderer->openGLContextClosing();

            if (vertexArrayObject != 0)
                context.extensions.glDeleteVertexArrays (1, &vertexArrayObject);

            associatedObjectNames.clear();
            associatedObjects.clear();
            cachedImageFrameBuffer.release();
            nativeContext->shutdownOnRenderThread();
        */
    }
    
    pub fn do_work_while_waiting_for_lock(&mut self, context_is_already_active: bool) -> bool {
        
        todo!();
        /*
            bool contextActivated = false;

            for (OpenGLContext::OpenGLContextAsyncWorker::OpenGLContextAsyncWorkerPtr work = workQueue.removeAndReturn (0);
                 work != nullptr && (! shouldExit()); work = workQueue.removeAndReturn (0))
            {
                if ((! contextActivated) && (! contextIsAlreadyActive))
                {
                    if (! context.makeActive())
                        break;

                    contextActivated = true;
                }

                OpenGLContextNativeContext::Locker locker (*nativeContext);

                (*work) (context);
                clearGLError();
            }

            if (contextActivated)
                OpenGLContext::deactivateCurrentContext();

            return shouldExit();
        */
    }
    
    pub fn execute(
        &mut self, 
        worker_to_use:          OpenGLContextAsyncWorkerPtr,
        should_block:           bool,
        called_from_destructor: Option<bool>

    ) {

        let called_from_destructor: bool = called_from_destructor.unwrap_or(false);

        todo!();
        /*
            if (calledFromDestructor || ! destroying)
            {
                if (shouldBlock)
                {
                    auto blocker = new OpenGLContextCachedImageBlockingWorker (std::move (workerToUse));
                    OpenGLContext::OpenGLContextAsyncWorker::OpenGLContextAsyncWorkerPtr worker (*blocker);
                    workQueue.add (worker);

                    messageManagerLock.abort();
                    context.triggerRepaint();

                    blocker->block();
                }
                else
                {
                    workQueue.add (std::move (workerToUse));

                    messageManagerLock.abort();
                    context.triggerRepaint();
                }
            }
            else
            {
                jassertfalse; // you called execute AFTER you detached your openglcontext
            }
        */
    }
    
    pub fn get(c: &mut Component) -> *mut OpenGLContextCachedImage<'a> {
        
        todo!();
        /*
            return dynamic_cast<OpenGLContextCachedImage*> (c.getCachedComponentImage());
        */
    }
}
