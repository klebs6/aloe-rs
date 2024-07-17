crate::ix!();

/**
  | This class just manages the uniform
  | values that the demo shaders use.
  |
  */
pub struct OpenGLAppDemoUniforms<'a> {
    projection_matrix: Box<OpenGLShaderProgramUniform<'a>>,
    view_matrix:       Box<OpenGLShaderProgramUniform<'a>>,
}

impl<'a> OpenGLAppDemoUniforms<'a> {

    pub fn new(shader_program: &mut OpenGLShaderProgram) -> Self {
    
        todo!();
        /*


            projectionMatrix.reset (createUniform (shaderProgram, "projectionMatrix"));
                viewMatrix      .reset (createUniform (shaderProgram, "viewMatrix"));
        */
    }
    
    pub fn create_uniform(
        shader_program: &mut OpenGLShaderProgram,
        uniform_name:   *const u8

    ) -> *mut OpenGLShaderProgramUniform<'a> {
        
        todo!();
        /*
            using namespace ::gl;

                if (glGetUniformLocation (shaderProgram.getProgramID(), uniformName) < 0)
                    return nullptr;

                return new OpenGLShaderProgram::Uniform (shaderProgram, uniformName);
        */
    }
}

/**
  | This class just manages the uniform
  | values that the demo shaders use.
  |
  */
pub struct OpenGLAppDemoUniforms2<'a> {
    projection_matrix: Box<OpenGLShaderProgramUniform<'a>>,
    view_matrix:       Box<OpenGLShaderProgramUniform<'a>>,
    texture:           Box<OpenGLShaderProgramUniform<'a>>,
    light_position:    Box<OpenGLShaderProgramUniform<'a>>,
    bouncing_number:   Box<OpenGLShaderProgramUniform<'a>>,
}

impl<'a> OpenGLAppDemoUniforms2<'a> {

    pub fn new(shader: &mut OpenGLShaderProgram) -> Self {
    
        todo!();
        /*


            projectionMatrix.reset (createUniform (shader, "projectionMatrix"));
                viewMatrix      .reset (createUniform (shader, "viewMatrix"));
                texture         .reset (createUniform (shader, "demoTexture"));
                lightPosition   .reset (createUniform (shader, "lightPosition"));
                bouncingNumber  .reset (createUniform (shader, "bouncingNumber"));
        */
    }
    
    pub fn create_uniform(
        shader:       &mut OpenGLShaderProgram,
        uniform_name: *const u8

    ) -> *mut OpenGLShaderProgramUniform<'a> {
        
        todo!();
        /*
            using namespace ::gl;

                if (glGetUniformLocation (shader.getProgramID(), uniformName) < 0)
                    return nullptr;

                return new OpenGLShaderProgram::Uniform (shader, uniformName);
        */
    }
}
