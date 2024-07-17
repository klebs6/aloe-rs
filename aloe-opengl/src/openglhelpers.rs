crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLHelpers.h]

/**
  | A set of miscellaneous openGL helper
  | functions.
  | 
  | @tags{OpenGL}
  |
  */
pub struct OpenGLHelpers {

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLHelpers.cpp]
impl OpenGLHelpers {

    /**
      | Clears the GL error state.
      |
      */
    pub fn reset_error_state(&mut self)  {
        
        todo!();
        /*
            while (glGetError() != GL_NO_ERROR) {}
        */
    }
    
    /**
      | Returns the address of a named GL extension
      | function
      |
      */
    pub fn get_extension_function(&mut self, function_name: *const u8)  {
        
        todo!();
        /*
            #if ALOE_WINDOWS
        return (void*) wglGetProcAddress (functionName);
       #elif ALOE_LINUX || ALOE_BSD
        return (void*) glXGetProcAddress ((const GLubyte*) functionName);
       #else
        static void* handle = dlopen (nullptr, RTLD_LAZY);
        return dlsym (handle, functionName);
       #endif
        */
    }
    
    /**
      | Checks whether the current context
      | supports the specified extension.
      |
      */
    pub fn is_extension_supported(&mut self, extension_name: *const u8) -> bool {
        
        todo!();
        /*
            jassert (extensionName != nullptr); // you must supply a genuine string for this.
        jassert (isContextActive()); // An OpenGL context will need to be active before calling this.

        const char* extensions = (const char*) glGetString (GL_EXTENSIONS);
        jassert (extensions != nullptr); // Perhaps you didn't activate an OpenGL context before calling this?

        for (;;)
        {
            const char* found = strstr (extensions, extensionName);

            if (found == nullptr)
                break;

            extensions = found + strlen (extensionName);

            if (extensions[0] == ' ' || extensions[0] == 0)
                return true;
        }

        return false;
        */
    }
    
    /**
      | Clears the current context using the
      | given colour.
      |
      */
    pub fn clear(&mut self, colour: Colour)  {
        
        todo!();
        /*
            glClearColor (colour.getFloatRed(), colour.getFloatGreen(),
                      colour.getFloatBlue(), colour.getFloatAlpha());

        glClear (GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT);
        */
    }
    
    pub fn enable_scissor_test(&mut self, clip: Rectangle<i32>)  {
        
        todo!();
        /*
            glEnable (GL_SCISSOR_TEST);
        glScissor (clip.getX(), clip.getY(), clip.getWidth(), clip.getHeight());
        */
    }
    
    /**
      | Returns a version string such as "#version
      | 150" suitable for prefixing a GLSL shader
      | on this platform.
      |
      */
    pub fn get_glsl_version_string(&mut self) -> String {
        
        todo!();
        /*
            if (getOpenGLVersion() >= Version (3, 2))
        {
           #if ALOE_OPENGL_ES
            return "#version 300 es";
           #else
            return "#version 150";
           #endif
        }

        return "#version 110";
        */
    }
    
    /**
      | Makes some simple textual changes to
      | a shader program to try to convert old
      | GLSL keywords to their v3 equivalents.
      | 
      | Before doing this, the function will
      | check whether the current context is
      | actually using a later version of the
      | language, and if not it will not make
      | any changes.
      | 
      | Obviously this is not a real parser,
      | so will only work on simple code!
      |
      */
    pub fn translate_vertex_shader_tov3(&mut self, code: &String) -> String {
        
        todo!();
        /*
            if (getOpenGLVersion() >= Version (3, 2))
        {
            String output;

           #if ALOE_ANDROID
            {
                int numAttributes = 0;

                for (int p = code.indexOf (0, "attribute "); p >= 0; p = code.indexOf (p + 1, "attribute "))
                    numAttributes++;

                int last = 0;

                for (int p = code.indexOf (0, "attribute "); p >= 0; p = code.indexOf (p + 1, "attribute "))
                {
                    output += code.substring (last, p) + "layout(location=" + String (--numAttributes) + ") in ";

                    last = p + 10;
                }

                output += code.substring (last);
            }
           #else
            output = code.replace ("attribute", "in");
           #endif

            return getGLSLVersionString() + "\n" + output.replace ("varying", "out");
        }

        return code;
        */
    }
    
    /**
      | Makes some simple textual changes to
      | a shader program to try to convert old
      | GLSL keywords to their v3 equivalents.
      | 
      | Before doing this, the function will
      | check whether the current context is
      | actually using a later version of the
      | language, and if not it will not make
      | any changes.
      | 
      | Obviously this is not a real parser,
      | so will only work on simple code!
      |
      */
    pub fn translate_fragment_shader_tov3(&mut self, code: &String) -> String {
        
        todo!();
        /*
            if (getOpenGLVersion() >= Version (3, 2))
            return getGLSLVersionString() + "\n"
                   "out " ALOE_MEDIUMP " vec4 fragColor;\n"
                    + code.replace ("varying", "in")
                          .replace ("texture2D", "texture")
                          .replace ("gl_FragColor", "fragColor");

        return code;
        */
    }
}
