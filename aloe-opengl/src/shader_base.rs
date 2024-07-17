crate::ix!();

pub struct ShaderBase<'a> {
    base:                ShaderProgramHolder<'a>,
    position_attribute:  OpenGLShaderProgramAttribute,
    colour_attribute:    OpenGLShaderProgramAttribute,
    screen_bounds:       OpenGLShaderProgramUniform<'a>,
    on_shader_activated: fn(_0: &mut OpenGLShaderProgram) -> (),
}

impl<'a> ShaderBase<'a> {

    pub fn new(
        context:         &mut OpenGLContext,
        fragment_shader: *const u8,
        vertex_shader:   *const u8) -> Self {

        todo!();

        /*


            : ShaderProgramHolder (context, fragmentShader, vertexShader),
                  positionAttribute (program, "position"),
                  colourAttribute (program, "colour"),
                  screenBounds (program, "screenBounds")
        */
    }
    
    pub fn set2d_bounds(&mut self, bounds: Rectangle<f32>)  {
        
        todo!();
        /*
            screenBounds.set (bounds.getX(), bounds.getY(), 0.5f * bounds.getWidth(), 0.5f * bounds.getHeight());
        */
    }
    
    pub fn bind_attributes(&mut self)  {
        
        todo!();
        /*
            gl::glVertexAttribPointer ((GLuint) positionAttribute.attributeID, 2, GL_SHORT, GL_FALSE, 8, nullptr);
                gl::glVertexAttribPointer ((GLuint) colourAttribute.attributeID, 4, GL_UNSIGNED_BYTE, GL_TRUE, 8, (void*) 4);
                gl::glEnableVertexAttribArray ((GLuint) positionAttribute.attributeID);
                gl::glEnableVertexAttribArray ((GLuint) colourAttribute.attributeID);
        */
    }
    
    pub fn unbind_attributes(&mut self)  {
        
        todo!();
        /*
            gl::glDisableVertexAttribArray ((GLuint) positionAttribute.attributeID);
                gl::glDisableVertexAttribArray ((GLuint) colourAttribute.attributeID);
        */
    }
}

