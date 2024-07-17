crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/aloe_opengl.h]

pub const ALOE_OPENGL: usize = 1;

#[cfg(any(target_os="ios",target_os="android"))]
pub const ALOE_OPENGL_ES: bool = 1;

/**
  | This macro is a helper for use in GLSL
  | shader code which needs to compile on
  | both GLES and desktop GL.
  | 
  | Since it's mandatory in GLES to mark
  | a variable with a precision, but the
  | keywords don't exist in normal GLSL,
  | these macros define the various precision
  | keywords only on GLES.
  |
  */
#[cfg(ALOE_OPENGL_ES)]
pub const ALOE_MEDIUMP: &'static str = "mediump";

/**
  | This macro is a helper for use in GLSL
  | shader code which needs to compile on
  | both GLES and desktop GL.
  | 
  | Since it's mandatory in GLES to mark
  | a variable with a precision, but the
  | keywords don't exist in normal GLSL,
  | these macros define the various precision
  | keywords only on GLES.
  |
  */
#[cfg(ALOE_OPENGL_ES)]
pub const ALOE_HIGHP: &'static str = "highp";

/**
  | This macro is a helper for use in GLSL
  | shader code which needs to compile on
  | both GLES and desktop GL.
  | 
  | Since it's mandatory in GLES to mark
  | a variable with a precision, but the
  | keywords don't exist in normal GLSL,
  | these macros define the various precision
  | keywords only on GLES.
  |
  */
#[cfg(ALOE_OPENGL_ES)]
pub const ALOE_LOWP: &'static str = "lowp";

#[cfg(not(ALOE_OPENGL_ES))]
pub const ALOE_MEDIUMP: &'static str = "";

#[cfg(not(ALOE_OPENGL_ES))]
pub const ALOE_HIGHP:   &'static str = "";

#[cfg(not(ALOE_OPENGL_ES))]
pub const ALOE_LOWP:    &'static str = "";

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/aloe_opengl.cpp]

pub const ALOE_CORE_INCLUDE_OBJC_HELPERS:                             usize = 1;
pub const ALOE_CORE_INCLUDE_JNI_HELPERS:                              usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS:                           usize = 1;
pub const ALOE_GRAPHICS_INCLUDE_COREGRAPHICS_HELPERS:                 usize = 1;
pub const ALOE_GUI_BASICS_INCLUDE_XHEADERS:                           usize = 1;
pub const ALOE_GUI_BASICS_INCLUDE_SCOPED_THREAD_DPI_AWARENESS_SETTER: usize = 1;

pub const ALOE_STATIC_LINK_GL_VERSION_1_0: usize = 1;
pub const ALOE_STATIC_LINK_GL_VERSION_1_1: usize = 1;

#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_1_2: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_1_3: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_1_4: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_1_5: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_2_0: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_2_1: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_3_0: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_3_1: usize = 1;
#[cfg(target_os="macos")] pub const ALOE_STATIC_LINK_GL_VERSION_3_2: usize = 1;

pub const ALOE_STATIC_LINK_GL_ES_VERSION_2_0: usize = 1;

#[cfg(any(not(target_os="android"),ALOE_ANDROID_GL_ES_VERSION_3_0))]
pub const ALOE_STATIC_LINK_GL_ES_VERSION_3_0: usize = 1;

//TODO
pub struct OpenGLExtensionFunctions {}

impl OpenGLExtensionFunctions {

    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            gl::loadFunctions();
        */
    }
}

lazy_static!{
    /*
    #define X(name) decltype (::gl::name)& OpenGLExtensionFunctions::name = ::gl::name;
    ALOE_GL_BASE_FUNCTIONS
    ALOE_GL_EXTENSION_FUNCTIONS
    ALOE_GL_VERTEXBUFFER_FUNCTIONS
    #undef X
    */
}

#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
pub fn get_gl_error_message(e: GLenum) -> *const u8 {
    
    todo!();
        /*
            switch (e)
        {
            case GL_INVALID_ENUM:                   return "GL_INVALID_ENUM";
            case GL_INVALID_VALUE:                  return "GL_INVALID_VALUE";
            case GL_INVALID_OPERATION:              return "GL_INVALID_OPERATION";
            case GL_OUT_OF_MEMORY:                  return "GL_OUT_OF_MEMORY";
           #ifdef GL_STACK_OVERFLOW
            case GL_STACK_OVERFLOW:                 return "GL_STACK_OVERFLOW";
           #endif
           #ifdef GL_STACK_UNDERFLOW
            case GL_STACK_UNDERFLOW:                return "GL_STACK_UNDERFLOW";
           #endif
           #ifdef GL_INVALID_FRAMEBUFFER_OPERATION
            case GL_INVALID_FRAMEBUFFER_OPERATION:  return "GL_INVALID_FRAMEBUFFER_OPERATION";
           #endif
            default: break;
        }

        return "Unknown error";
        */
}

#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
#[cfg(any(target_os="macos",target_os="ios"))] #[cfg(not(ALOE_IOS_MAC_VIEW))] #[cfg(target_os="ios")] pub type ALOE_IOS_MAC_VIEW   = UIView;

#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
#[cfg(any(target_os="macos",target_os="ios"))] #[cfg(not(ALOE_IOS_MAC_VIEW))] #[cfg(target_os="ios")] pub type ALOE_IOS_MAC_WINDOW = UIWindow;

#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
#[cfg(any(target_os="macos",target_os="ios"))] #[cfg(ALOE_IOS_MAC_VIEW)] #[cfg(not(target_os="ios"))] pub type ALOE_IOS_MAC_VIEW   = NSView;

#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
#[cfg(any(target_os="macos",target_os="ios"))] #[cfg(ALOE_IOS_MAC_VIEW)] #[cfg(not(target_os="ios"))] pub type ALOE_IOS_MAC_WINDOW = NSWindow;

#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
pub fn check_peer_is_valid(context: *mut OpenGLContext) -> bool {
    
    todo!();
        /*
            jassert (context != nullptr);

        if (context != nullptr)
        {
            if (auto* comp = context->getTargetComponent())
            {
                if (auto* peer = comp->getPeer())
                {
                   #if ALOE_MAC || ALOE_IOS
                    if (auto* nsView = (ALOE_IOS_MAC_VIEW*) peer->getNativeHandle())
                    {
                        if (auto nsWindow = [nsView window])
                        {
                           #if ALOE_MAC
                            return ([nsWindow isVisible]
                                      && (! [nsWindow hidesOnDeactivate] || [NSApp isActive]));
                           #else
                            ignoreUnused (nsWindow);
                            return true;
                           #endif
                        }
                    }
                   #else
                    ignoreUnused (peer);
                    return true;
                   #endif
                }
            }
        }

        return false;
        */
}

#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
pub fn check_gl_error(
        file: *const u8,
        line: i32)  {
    
    todo!();
        /*
            for (;;)
        {
            const GLenum e = glGetError();

            if (e == GL_NO_ERROR)
                break;

            // if the peer is not valid then ignore errors
            if (! checkPeerIsValid (OpenGLContext::getCurrentContext()))
                continue;

            DBG ("***** " << getGLErrorMessage (e) << "  at " << file << " : " << line);
            jassertfalse;
        }
        */
}


#[cfg(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR)))]
macro_rules! aloe_check_opengl_error {
    () => {
        /*
                checkGLError (__FILE__, __LINE__);
        */
    }
}

#[cfg(not(all(ALOE_DEBUG,not(ALOE_CHECK_OPENGL_ERROR))))]
macro_rules! aloe_check_opengl_error {
    () => {
        /*
                ;
        */
    }
}

pub fn clear_gl_error()  {
    
    todo!();
        /*
            while (glGetError() != GL_NO_ERROR) {}
        */
}
