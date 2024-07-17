crate::ix!();

pub struct LinearGradient1MaskedProgram<'a> {
    base:            ShaderBase<'a>,
    gradient_params: LinearGradientParams<'a>,
    mask_params:     MaskedShaderParams<'a>,
}

impl<'a> LinearGradient1MaskedProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*


            : ShaderBase (context, ALOE_DECLARE_LINEAR_UNIFORMS  // gradientInfo: x = x1, y = y1, z = (y2 - y1) / (x2 - x1), w = length
                              ALOE_DECLARE_MASK_UNIFORMS
                              "void main()"
                              "{"
                                ALOE_CALC_LINEAR_GRAD_POS1
                                "gl_FragColor = " ALOE_GET_TEXTURE_COLOUR " * " ALOE_GET_MASK_ALPHA ";"
                              "}"),
                  gradientParams (program),
                  maskParams (program)
        */
    }
}
