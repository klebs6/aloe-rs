crate::ix!();

pub struct LinearGradient1Program<'a> {
    base:            ShaderBase<'a>,
    gradient_params: LinearGradientParams<'a>,
}

impl<'a> LinearGradient1Program<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*


            : ShaderBase (context, ALOE_DECLARE_LINEAR_UNIFORMS  // gradientInfo: x = x1, y = y1, z = (y2 - y1) / (x2 - x1), w = length
                              "void main()"
                              "{"
                                ALOE_CALC_LINEAR_GRAD_POS1
                                "gl_FragColor = " ALOE_GET_TEXTURE_COLOUR ";"
                              "}"),
                  gradientParams (program)
        */
    }
}
