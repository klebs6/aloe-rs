crate::ix!();

impl OpenGLHelpers {

    /**
      | Returns true if the current thread has
      | an active OpenGL context.
      |
      */
    pub fn is_context_active(&mut self) -> bool {
        
        #[cfg(target_os="android")]
        fn is_context_active() -> bool {
            
            todo!();
            /*
                return eglGetCurrentContext() != EGL_NO_CONTEXT;
            */
        }

        #[cfg(target_os="ios")]
        fn is_context_active() -> bool {
            
            todo!();
            /*
                return [EAGLContext currentContext] != nil;
            */
        }

        #[cfg(target_os="linux")]
        #[cfg(feature = "x11")]
        fn is_context_active() -> bool {
            
            todo!();
            /*
            XWindowSystemUtilities::ScopedXLock xLock;
            return glXGetCurrentContext() != nullptr;
            */
        }
        
        #[cfg(target_os="macos")]
        fn is_context_active() -> bool {
            
            todo!();
            /*
                return CGLGetCurrentContext() != CGLContextObj();
            */
        }

        is_context_active()
    }
}
