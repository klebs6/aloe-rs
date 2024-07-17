crate::ix!();

pub struct RadialGradientParams<'a> {
    gradient_texture: OpenGLShaderProgramUniform<'a>,
    matrix:           OpenGLShaderProgramUniform<'a>,
}

impl<'a> RadialGradientParams<'a> {

    pub fn new(program: &mut OpenGLShaderProgram) -> Self {
    
        todo!();
        /*


            : gradientTexture (program, "gradientTexture"),
                  matrix (program, "matrix")
        */
    }
    
    pub fn set_matrix(&mut self, 
        p1: Point<f32>,
        p2: Point<f32>,
        p3: Point<f32>)  {
        
        todo!();
        /*
            auto t = AffineTransform::fromTargetPoints (p1, Point<float>(),
                                                            p2, Point<float> (1.0f, 0.0f),
                                                            p3, Point<float> (0.0f, 1.0f));
                const GLfloat m[] = { t.mat00, t.mat01, t.mat02, t.mat10, t.mat11, t.mat12 };
                matrix.set (m, 6);
        */
    }
}
