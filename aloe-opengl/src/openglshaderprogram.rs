crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLShaderProgram.h]

/**
  | Manages an OpenGL shader program.
  | 
  | @tags{OpenGL}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLShaderProgram<'a> {
    context:   &'a OpenGLContext<'a>,
    programid: RefCell<u32>, // default = 0
    error_log: String,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLShaderProgram.cpp]
impl<'a> Drop for OpenGLShaderProgram<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            release();
        */
    }
}

impl<'a> OpenGLShaderProgram<'a> {

    /**
      | Get the output for the last shader compilation
      | or link that failed.
      |
      */
    pub fn get_last_error(&self) -> &String {
        
        todo!();
        /*
            return errorLog;
        */
    }

    /*
      |  Methods for setting shader uniforms
      |  without using a OpenGLShaderProgramUniform object (see
      |  below).
      |
      |  You must make sure this shader is the
      |  currently bound one before setting
      |  uniforms with these functions.
      */

    /**
      | Creates a shader for use in a particular
      | GL context.
      |
      */
    pub fn new(c: &OpenGLContext) -> Self {
    
        todo!();
        /*
        : context(c),

        
        */
    }
    
    /**
      | The ID number of the compiled program.
      |
      */
    pub fn get_programid(&self) -> u32 {
        
        todo!();
        /*
            if (programID == 0)
        {
            // This method should only be called when the current thread has an active OpenGL context.
            jassert (OpenGLHelpers::isContextActive());

            programID = context.extensions.glCreateProgram();
        }

        return programID;
        */
    }
    
    /**
      | Deletes the program.
      |
      */
    pub fn release(&mut self)  {
        
        todo!();
        /*
            if (programID != 0)
        {
            context.extensions.glDeleteProgram (programID);
            programID = 0;
        }
        */
    }
    
    /**
      | Returns the version of GLSL that the
      | current context supports.
      | 
      | -----------
      | @code
      | 
      | if (OpenGLShaderProgram::getLanguageVersion() > 1.199)
      | {
      |     // ..do something that requires GLSL 1.2 or above..
      | }
      |
      */
    pub fn get_language_version(&mut self) -> f64 {
        
        todo!();
        /*
            return String::fromUTF8 ((const char*) glGetString (GL_SHADING_LANGUAGE_VERSION))
                .retainCharacters("1234567890.").getDoubleValue();
        */
    }
    
    /**
      | Compiles and adds a shader to this program.
      | 
      | After adding all your shaders, remember
      | to call link() to link them into a usable
      | program.
      | 
      | If your app is built in debug mode, this
      | method will assert if the program fails
      | to compile correctly.
      | 
      | The shaderType parameter could be GL_VERTEX_SHADER,
      | GL_FRAGMENT_SHADER, etc.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the shader compiled successfully.
      | If not, you can call getLastError()
      | to find out what happened.
      |
      */
    pub fn add_shader(&mut self, 
        code: &String,
        ty:   GLenum) -> bool {
        
        todo!();
        /*
            GLuint shaderID = context.extensions.glCreateShader (type);

        const GLchar* c = code.toRawUTF8();
        context.extensions.glShaderSource (shaderID, 1, &c, nullptr);

        context.extensions.glCompileShader (shaderID);

        GLint status = GL_FALSE;
        context.extensions.glGetShaderiv (shaderID, GL_COMPILE_STATUS, &status);

        if (status == (GLint) GL_FALSE)
        {
            std::vector<GLchar> infoLog (16384);
            GLsizei infoLogLength = 0;
            context.extensions.glGetShaderInfoLog (shaderID, (GLsizei) infoLog.size(), &infoLogLength, infoLog.data());
            errorLog = String (infoLog.data(), (size_t) infoLogLength);

           #if ALOE_DEBUG && ! ALOE_DONT_ASSERT_ON_GLSL_COMPILE_ERROR
            // Your GLSL code contained compile errors!
            // Hopefully this compile log should help to explain what went wrong.
            DBG (errorLog);
            jassertfalse;
           #endif

            return false;
        }

        context.extensions.glAttachShader (getProgramID(), shaderID);
        context.extensions.glDeleteShader (shaderID);
        ALOE_CHECK_OPENGL_ERROR
        return true;
        */
    }
    
    /**
      | Compiles and adds a fragment shader
      | to this program.
      | 
      | This is equivalent to calling addShader()
      | with a type of GL_VERTEX_SHADER.
      |
      */
    pub fn add_vertex_shader(&mut self, code: &String) -> bool {
        
        todo!();
        /*
            return addShader (code, GL_VERTEX_SHADER);
        */
    }
    
    /**
      | Compiles and adds a fragment shader
      | to this program.
      | 
      | This is equivalent to calling addShader()
      | with a type of GL_FRAGMENT_SHADER.
      |
      */
    pub fn add_fragment_shader(&mut self, code: &String) -> bool {
        
        todo!();
        /*
            return addShader (code, GL_FRAGMENT_SHADER);
        */
    }
    
    /**
      | Links all the compiled shaders into
      | a usable program.
      | 
      | If your app is built in debug mode, this
      | method will assert if the program fails
      | to link correctly.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the program linked successfully.
      | If not, you can call getLastError()
      | to find out what happened.
      |
      */
    pub fn link(&mut self) -> bool {
        
        todo!();
        /*
            // This method can only be used when the current thread has an active OpenGL context.
        jassert (OpenGLHelpers::isContextActive());

        GLuint progID = getProgramID();

        context.extensions.glLinkProgram (progID);

        GLint status = GL_FALSE;
        context.extensions.glGetProgramiv (progID, GL_LINK_STATUS, &status);

        if (status == (GLint) GL_FALSE)
        {
            std::vector<GLchar> infoLog (16384);
            GLsizei infoLogLength = 0;
            context.extensions.glGetProgramInfoLog (progID, (GLsizei) infoLog.size(), &infoLogLength, infoLog.data());
            errorLog = String (infoLog.data(), (size_t) infoLogLength);

           #if ALOE_DEBUG && ! ALOE_DONT_ASSERT_ON_GLSL_COMPILE_ERROR
            // Your GLSL code contained link errors!
            // Hopefully this compile log should help to explain what went wrong.
            DBG (errorLog);
            jassertfalse;
           #endif
        }

        ALOE_CHECK_OPENGL_ERROR
        return status != (GLint) GL_FALSE;
        */
    }
    
    /**
      | Selects this program into the current
      | context.
      |
      */
    pub fn use_(&self)  {
        
        todo!();
        /*
            // The shader program must have been successfully linked when this method is called!
        jassert (programID != 0);

        context.extensions.glUseProgram (programID);
        */
    }
    
    /**
      | Get the uniform ID from the variable
      | name
      |
      */
    pub fn get_uniform_id_from_name(&self, uniform_name: *const u8) -> GLint {
        
        todo!();
        /*
            // The shader program must be active when this method is called!
        jassert (programID != 0);

        return (GLint) context.extensions.glGetUniformLocation (programID, uniformName);
        */
    }
    
    /**
      | Sets a float uniform.
      |
      */
    pub fn set_uniform_f32(
        &mut self, 
        name: *const u8,
        n1:   f32

    ) {
        
        todo!();
        /*
            context.extensions.glUniform1f  (getUniformIDFromName (name), n1);
        */
    }
    
    /**
      | Sets an int uniform.
      |
      */
    pub fn set_uniform_int(
        &mut self, 
        name: *const u8,
        n1:   GLint

    ) {
        
        todo!();
        /*
            context.extensions.glUniform1i  (getUniformIDFromName (name), n1);
        */
    }
    
    /**
      | Sets a vec2 uniform.
      |
      */
    pub fn set_uniform_vec2(
        &mut self, 
        name: *const u8,
        n1:   f32,
        n2:   f32)  {
        
        todo!();
        /*
            context.extensions.glUniform2f  (getUniformIDFromName (name), n1, n2);
        */
    }
    
    /**
      | Sets a vec3 uniform.
      |
      */
    pub fn set_uniform_vec3(
        &mut self, 
        name: *const u8,
        n1:   f32,
        n2:   f32,
        n3:   f32)  {
        
        todo!();
        /*
            context.extensions.glUniform3f  (getUniformIDFromName (name), n1, n2, n3);
        */
    }
    
    /**
      | Sets a vec4 uniform.
      |
      */
    pub fn set_uniform_vec4_f32(
        &mut self, 
        name: *const u8,
        n1:   f32,
        n2:   f32,
        n3:   f32,
        n4:   f32)  {
        
        todo!();
        /*
            context.extensions.glUniform4f  (getUniformIDFromName (name), n1, n2, n3, n4);
        */
    }
    
    /**
      | Sets a vec4 uniform.
      |
      */
    pub fn set_uniform_vec4(
        &mut self, 
        name: *const u8,
        n1:   GLint,
        n2:   GLint,
        n3:   GLint,
        n4:   GLint)  {
        
        todo!();
        /*
            context.extensions.glUniform4i  (getUniformIDFromName (name), n1, n2, n3, n4);
        */
    }
    
    /**
      | Sets a vector float uniform.
      |
      */
    pub fn set_uniform_vecfloat(
        &mut self, 
        name:       *const u8,
        values:     *const f32,
        num_values: GLsizei

    ) {
        
        todo!();
        /*
            context.extensions.glUniform1fv (getUniformIDFromName (name), numValues, values);
        */
    }
    
    /**
      | Sets a 2x2 matrix float uniform.
      |
      */
    pub fn set_uniform_mat2(&mut self, 
        name: *const u8,
        v:    *const f32,
        num:  GLint,
        trns: GLboolean)  {
        
        todo!();
        /*
            context.extensions.glUniformMatrix2fv (getUniformIDFromName (name), num, trns, v);
        */
    }
    
    /**
      | Sets a 3x3 matrix float uniform.
      |
      */
    pub fn set_uniform_mat3(&mut self, 
        name: *const u8,
        v:    *const f32,
        num:  GLint,
        trns: GLboolean)  {
        
        todo!();
        /*
            context.extensions.glUniformMatrix3fv (getUniformIDFromName (name), num, trns, v);
        */
    }
    
    /**
      | Sets a 4x4 matrix float uniform.
      |
      */
    pub fn set_uniform_mat4(&mut self, 
        name: *const u8,
        v:    *const f32,
        num:  GLint,
        trns: GLboolean)  {
        
        todo!();
        /*
            context.extensions.glUniformMatrix4fv (getUniformIDFromName (name), num, trns, v);
        */
    }
}
