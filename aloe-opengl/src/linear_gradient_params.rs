crate::ix!();

pub struct LinearGradientParams<'a> {
    gradient_texture: OpenGLShaderProgramUniform<'a>,
    gradient_info:    OpenGLShaderProgramUniform<'a>,
}

impl<'a> LinearGradientParams<'a> {

    pub fn new(program: &mut OpenGLShaderProgram) -> Self {
    
        todo!();
        /*


            : gradientTexture (program, "gradientTexture"),
                  gradientInfo (program, "gradientInfo")
        */
    }
}
