crate::ix!();

pub struct MaskedShaderParams<'a> {
    mask_texture: OpenGLShaderProgramUniform<'a>,
    mask_bounds:  OpenGLShaderProgramUniform<'a>,
}

impl<'a> MaskedShaderParams<'a> {

    pub fn new(program: &mut OpenGLShaderProgram) -> Self {
    
        todo!();
        /*
        : mask_texture(program, "maskTexture"),
        : mask_bounds(program, "maskBounds"),

        
        */
    }
    
    pub fn set_bounds(
        &self, 
        area:          Rectangle<i32>,
        target:        &RenderingTarget,
        texture_index: GLint

    ) {
        
        todo!();
        /*
            maskTexture.set (textureIndex);
                maskBounds.set (area.getX() - target.bounds.getX(),
                                area.getY() - target.bounds.getY(),
                                area.getWidth(), area.getHeight());
        */
    }
}
