crate::ix!();

/**
  | Represents an openGL vertex attribute
  | value.
  | 
  | After a program has been linked, you
  | can create OpenGLShaderProgramAttribute objects to let
  | you set the attributes that your vertex
  | shaders use.
  |
  */
pub struct OpenGLShaderProgramAttribute {

    /**
      | The attribute's ID number.
      | 
      | If the uniform couldn't be found, this
      | value will be < 0.
      |
      */
    attributeid: u32,
}

impl OpenGLShaderProgramAttribute {

    /**
      | Initialises an attribute.
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
            : attributeID ((GLuint) program.context.extensions.glGetAttribLocation (program.getProgramID(), name))

       #if ALOE_DEBUG && ! ALOE_DONT_ASSERT_ON_GLSL_COMPILE_ERROR
        jassert ((GLint) attributeID >= 0);
       #endif
        */
    }
}
