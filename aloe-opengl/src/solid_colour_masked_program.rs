crate::ix!();

pub struct SolidColourMaskedProgram<'a> {
    base:        ShaderBase<'a>,
    mask_params: MaskedShaderParams<'a>,
}

impl<'a> SolidColourMaskedProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*
            : ShaderBase (context,
                              ALOE_DECLARE_MASK_UNIFORMS ALOE_DECLARE_VARYING_COLOUR ALOE_DECLARE_VARYING_PIXELPOS
                              "void main() {"
                                "gl_FragColor = frontColour * " ALOE_GET_MASK_ALPHA ";"
                              "}"),
                  maskParams (program)
        */
    }
}
