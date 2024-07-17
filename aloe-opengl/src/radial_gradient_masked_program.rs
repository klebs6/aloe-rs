crate::ix!();

pub struct RadialGradientMaskedProgram<'a> {
    base:            ShaderBase<'a>,
    gradient_params: RadialGradientParams<'a>,
    mask_params:     MaskedShaderParams<'a>,
}

impl<'a> RadialGradientMaskedProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*


            : ShaderBase (context, ALOE_DECLARE_VARYING_PIXELPOS
                              ALOE_DECLARE_RADIAL_UNIFORMS ALOE_DECLARE_VARYING_COLOUR
                              ALOE_DECLARE_MASK_UNIFORMS
                              "void main()"
                              "{"
                                ALOE_MEDIUMP " float gradientPos = length (" ALOE_MATRIX_TIMES_FRAGCOORD ");"
                                "gl_FragColor = " ALOE_GET_TEXTURE_COLOUR " * " ALOE_GET_MASK_ALPHA ";"
                              "}"),
                  gradientParams (program),
                  maskParams (program)
        */
    }
}
