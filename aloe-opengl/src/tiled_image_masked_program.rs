crate::ix!();

pub struct TiledImageMaskedProgram<'a> {
    base:         ShaderBase<'a>,
    image_params: ImageParams<'a>,
    mask_params:  MaskedShaderParams<'a>,
}

impl<'a> TiledImageMaskedProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*
            : ShaderBase (context, ALOE_DECLARE_IMAGE_UNIFORMS ALOE_DECLARE_MASK_UNIFORMS
                              "void main()"
                              "{"
                                ALOE_MOD_TEXTURE_COORD
                                "gl_FragColor = frontColour.a * " ALOE_GET_IMAGE_PIXEL " * " ALOE_GET_MASK_ALPHA ";"
                              "}"),
                  imageParams (program),
                  maskParams (program)
        */
    }
}
