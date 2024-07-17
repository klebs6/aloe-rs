crate::ix!();

pub struct CopyTextureProgram<'a> {
    base:         ShaderBase<'a>,
    image_params: ImageParams<'a>,
}

impl<'a> CopyTextureProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*


            : ShaderBase (context, ALOE_DECLARE_IMAGE_UNIFORMS
                              "void main()"
                              "{"
                                ALOE_MOD_TEXTURE_COORD
                                "gl_FragColor = frontColour.a * " ALOE_GET_IMAGE_PIXEL ";"
                              "}"),
                  imageParams (program)
        */
    }
}
