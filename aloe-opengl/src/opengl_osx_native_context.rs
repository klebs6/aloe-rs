crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/native/aloe_OpenGL_osx.h]
#[cfg(target_os="macos")]
#[no_copy]
#[leak_detector]
pub struct OpenGLContextNativeContext {
    render_context:        Option<SafeNSOpenGLContext>, // default = nil
    view:                  Option<SafeNSOpenGLView>, // default = nil
    view_attachment:       ReferenceCountedObjectPtr<ReferenceCountedObject>,
    last_swap_time:        f64, // default = 0
    min_swap_time_ms:      i32, // default = 0
    underrun_counter:      i32, // default = 0
    num_frames_per_swap:   i32, // default = 0
    video_refresh_periods: f64, // default = 1.0 / 60.0
}

#[cfg(target_os="macos")]
impl Drop for OpenGLContextNativeContext {

    fn drop(&mut self) {
        todo!();
        /*
            [[NSNotificationCenter defaultCenter] removeObserver: view];
            [renderContext clearDrawable];
            [renderContext setView: nil];
            [view setOpenGLContext: nil];
            [view release];
        */
    }
}

#[cfg(target_os="macos")]
impl OpenGLContextNativeContext {

    pub fn new(
        component:                &mut Component,
        pix_format:               &OpenGLPixelFormat,
        context_to_share:         *mut c_void,
        should_use_multisampling: bool,
        version:                  OpenGLVersion) -> Self {
    
        todo!();
        /*


            NSOpenGLPixelFormatAttribute attribs[64] = { 0 };
            createAttribs (attribs, version, pixFormat, shouldUseMultisampling);

            NSOpenGLPixelFormat* format = [[NSOpenGLPixelFormat alloc] initWithAttributes: attribs];

            static OpenGLContextNativeContextMouseForwardingNSOpenGLViewClass cls;
            view = [cls.createInstance() initWithFrame: NSMakeRect (0, 0, 100.0f, 100.0f)
                                           pixelFormat: format];

            if ([view respondsToSelector: @selector (setWantsBestResolutionOpenGLSurface:)])
                [view setWantsBestResolutionOpenGLSurface: YES];

            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
            [[NSNotificationCenter defaultCenter] addObserver: view
                                                     selector: @selector (_surfaceNeedsUpdate:)
                                                         name: NSViewGlobalFrameDidChangeNotification
                                                       object: view];
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE

            renderContext = [[[NSOpenGLContext alloc] initWithFormat: format
                                                        shareContext: (NSOpenGLContext*) contextToShare] autorelease];

            [view setOpenGLContext: renderContext];
            [format release];

            viewAttachment = NSViewComponent::attachViewToComponent (component, view);
        */
    }
    
    pub fn create_attribs(
        attribs:                  *mut NSOpenGLPixelFormatAttribute,
        version:                  OpenGLVersion,
        pix_format:               &OpenGLPixelFormat,
        should_use_multisampling: bool)  {
        
        todo!();
        /*
            ignoreUnused (version);
            int numAttribs = 0;

            attribs[numAttribs++] = NSOpenGLPFAOpenGLProfile;
            attribs[numAttribs++] = version >= openGL3_2 ? NSOpenGLProfileVersion3_2Core
                                                         : NSOpenGLProfileVersionLegacy;

            attribs[numAttribs++] = NSOpenGLPFADoubleBuffer;
            attribs[numAttribs++] = NSOpenGLPFAClosestPolicy;
            attribs[numAttribs++] = NSOpenGLPFANoRecovery;
            attribs[numAttribs++] = NSOpenGLPFAColorSize;
            attribs[numAttribs++] = (NSOpenGLPixelFormatAttribute) (pixFormat.redBits + pixFormat.greenBits + pixFormat.blueBits);
            attribs[numAttribs++] = NSOpenGLPFAAlphaSize;
            attribs[numAttribs++] = (NSOpenGLPixelFormatAttribute) pixFormat.alphaBits;
            attribs[numAttribs++] = NSOpenGLPFADepthSize;
            attribs[numAttribs++] = (NSOpenGLPixelFormatAttribute) pixFormat.depthBufferBits;
            attribs[numAttribs++] = NSOpenGLPFAStencilSize;
            attribs[numAttribs++] = (NSOpenGLPixelFormatAttribute) pixFormat.stencilBufferBits;
            attribs[numAttribs++] = NSOpenGLPFAAccumSize;
            attribs[numAttribs++] = (NSOpenGLPixelFormatAttribute) (pixFormat.accumulationBufferRedBits + pixFormat.accumulationBufferGreenBits
                                                                       + pixFormat.accumulationBufferBlueBits + pixFormat.accumulationBufferAlphaBits);

            if (shouldUseMultisampling)
            {
                attribs[numAttribs++] = NSOpenGLPFAMultisample;
                attribs[numAttribs++] = NSOpenGLPFASampleBuffers;
                attribs[numAttribs++] = (NSOpenGLPixelFormatAttribute) 1;
                attribs[numAttribs++] = NSOpenGLPFASamples;
                attribs[numAttribs++] = (NSOpenGLPixelFormatAttribute) pixFormat.multisamplingLevel;
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
            return static_cast<void*> (renderContext);
        */
    }
    
    pub fn get_frame_bufferid(&self) -> GLuint {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn make_active(&self) -> bool {
        
        todo!();
        /*
            jassert (renderContext != nil);

            if ([renderContext view] != view)
                [renderContext setView: view];

            if (NSOpenGLContext* context = [view openGLContext])
            {
                [context makeCurrentContext];
                return true;
            }

            return false;
        */
    }
    
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return [NSOpenGLContext currentContext] == renderContext;
        */
    }
    
    pub fn deactivate_current_context()  {
        
        todo!();
        /*
            [NSOpenGLContext clearCurrentContext];
        */
    }
    
    pub fn swap_buffers(&mut self)  {
        
        todo!();
        /*
            auto now = Time::getMillisecondCounterHiRes();
            [renderContext flushBuffer];

            if (minSwapTimeMs > 0)
            {
                // When our window is entirely occluded by other windows, flushBuffer
                // fails to wait for the swap interval, so the render loop spins at full
                // speed, burning CPU. This hack detects when things are going too fast
                // and sleeps if necessary.

                auto swapTime = Time::getMillisecondCounterHiRes() - now;
                auto frameTime = (int) (now - lastSwapTime);

                if (swapTime < 0.5 && frameTime < minSwapTimeMs - 3)
                {
                    if (underrunCounter > 3)
                    {
                        Thread::sleep (2 * (minSwapTimeMs - frameTime));
                        now = Time::getMillisecondCounterHiRes();
                    }
                    else
                    {
                        ++underrunCounter;
                    }
                }
                else
                {
                    if (underrunCounter > 0)
                        --underrunCounter;
                }
            }

            lastSwapTime = now;
        */
    }
    
    pub fn update_window_position(&mut self, _0: Rectangle<i32>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_swap_interval(&mut self, num_frames_per_swap_in: i32) -> bool {
        
        todo!();
        /*
            numFramesPerSwap = numFramesPerSwapIn;

            // The macOS OpenGL programming guide says that numFramesPerSwap
            // can only be 0 or 1.
            jassert (isPositiveAndBelow (numFramesPerSwap, 2));

            [renderContext setValues: (const GLint*) &numFramesPerSwap
                       #if defined (MAC_OS_X_VERSION_10_12) && MAC_OS_X_VERSION_MIN_REQUIRED >= MAC_OS_X_VERSION_10_12
                        forParameter: NSOpenGLContextParameterSwapInterval];
                       #else
                        forParameter: NSOpenGLCPSwapInterval];
                       #endif

            updateMinSwapTime();

            return true;
        */
    }
    
    pub fn get_swap_interval(&self) -> i32 {
        
        todo!();
        /*
            GLint numFrames = 0;
            [renderContext getValues: &numFrames
                       #if defined (MAC_OS_X_VERSION_10_12) && MAC_OS_X_VERSION_MIN_REQUIRED >= MAC_OS_X_VERSION_10_12
                        forParameter: NSOpenGLContextParameterSwapInterval];
                       #else
                        forParameter: NSOpenGLCPSwapInterval];
                       #endif

            return numFrames;
        */
    }
    
    pub fn set_nominal_video_refresh_periods(&mut self, periods: f64)  {
        
        todo!();
        /*
            jassert (periodS > 0.0);
            videoRefreshPeriodS = periodS;
            updateMinSwapTime();
        */
    }
    
    pub fn update_min_swap_time(&mut self)  {
        
        todo!();
        /*
            minSwapTimeMs = static_cast<int> (numFramesPerSwap * 1000 * videoRefreshPeriodS);
        */
    }
}
