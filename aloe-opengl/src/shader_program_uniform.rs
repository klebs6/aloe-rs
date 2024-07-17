crate::ix!();

/**
  | Represents an openGL uniform value.
  | 
  | After a program has been linked, you
  | can create OpenGLShaderProgramUniform objects to let you
  | set the uniforms that your shaders use.
  | 
  | Be careful not to call the set() functions
  | unless the appropriate program is loaded
  | into the current context.
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLShaderProgramUniform<'a> {

    /**
      | The uniform's ID number.
      | 
      | If the uniform couldn't be found, this
      | value will be < 0.
      |
      */
    uniformid: GLint,

    context:   &'a OpenGLContext<'a>,
}

impl<'a> OpenGLShaderProgramUniform<'a> {

    /**
      | Initialises a uniform.
      | 
      | The program must have been successfully
      | linked when this constructor is called.
      |
      */
    pub fn new(
        program: &OpenGLShaderProgram,
        name:    *const u8) -> Self {
    
        todo!();
        /*


            : uniformID (program.context.extensions.glGetUniformLocation (program.getProgramID(), name)), context (program.context)
       #if ALOE_DEBUG && ! ALOE_DONT_ASSERT_ON_GLSL_COMPILE_ERROR
        jassert (uniformID >= 0);
       #endif
        */
    }
    
    /**
      | Sets a float uniform.
      |
      */
    pub fn set_f32(&self, n1: f32)  {
        
        todo!();
        /*
            context.extensions.glUniform1f (uniformID, n1);
        */
    }
    
    /**
      | Sets an int uniform.
      |
      */
    pub fn set_int(&self, n1: GLint)  {
        
        todo!();
        /*
            context.extensions.glUniform1i (uniformID, n1);
        */
    }
    
    /**
      | Sets a vec2 uniform.
      |
      */
    pub fn set_vec2(&self, n1: f32, n2: f32)  {
        
        todo!();
        /*
            context.extensions.glUniform2f (uniformID, n1, n2);
        */
    }
    
    /**
      | Sets a vec3 uniform.
      |
      */
    pub fn set_vec3(
        &self, 
        n1: f32,
        n2: f32,
        n3: f32)  {
        
        todo!();
        /*
            context.extensions.glUniform3f (uniformID, n1, n2, n3);
        */
    }
    
    /**
      | Sets a vec4 uniform.
      |
      */
    pub fn set_vec4(
        &self, 
        n1: f32,
        n2: f32,
        n3: f32,
        n4: f32)  {
        
        todo!();
        /*
            context.extensions.glUniform4f (uniformID, n1, n2, n3, n4);
        */
    }
    
    /**
      | Sets an ivec4 uniform.
      |
      */
    pub fn set_ivec4(
        &self, 
        n1: GLint,
        n2: GLint,
        n3: GLint,
        n4: GLint)  {
        
        todo!();
        /*
            context.extensions.glUniform4i (uniformID, n1, n2, n3, n4);
        */
    }
    
    /**
      | Sets a vector float uniform.
      |
      */
    pub fn set_vecf(
        &self, 
        values:     *const f32,
        num_values: GLsizei

    ) {
        
        todo!();
        /*
            context.extensions.glUniform1fv (uniformID, numValues, values);
        */
    }
    
    /**
      | Sets a 2x2 matrix float uniform.
      |
      */
    pub fn set_matrix2(&self, 
        v:    *const f32,
        num:  GLint,
        trns: GLboolean)  {
        
        todo!();
        /*
            context.extensions.glUniformMatrix2fv (uniformID, num, trns, v);
        */
    }
    
    /**
      | Sets a 3x3 matrix float uniform.
      |
      */
    pub fn set_matrix3(&self, 
        v:    *const f32,
        num:  GLint,
        trns: GLboolean)  {
        
        todo!();
        /*
            context.extensions.glUniformMatrix3fv (uniformID, num, trns, v);
        */
    }
    
    /**
      | Sets a 4x4 matrix float uniform.
      |
      */
    pub fn set_matrix4(&self, 
        v:    *const f32,
        num:  GLint,
        trns: GLboolean)  {
        
        todo!();
        /*
            context.extensions.glUniformMatrix4fv (uniformID, num, trns, v);
        */
    }
}
