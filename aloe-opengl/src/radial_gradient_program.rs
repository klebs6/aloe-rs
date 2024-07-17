crate::ix!();

pub struct RadialGradientProgram<'a> {
    base:            ShaderBase<'a>,
    gradient_params: RadialGradientParams<'a>,
}

impl<'a> RadialGradientProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*


            : ShaderBase (context, ALOE_DECLARE_VARYING_PIXELPOS
                              ALOE_DECLARE_RADIAL_UNIFORMS ALOE_DECLARE_VARYING_COLOUR
                              "void main()"
                              "{"
                                ALOE_MEDIUMP " float gradientPos = length (" ALOE_MATRIX_TIMES_FRAGCOORD ");"
                                "gl_FragColor = " ALOE_GET_TEXTURE_COLOUR ";"
                              "}"),
                  gradientParams (program)
        */
    }
}
