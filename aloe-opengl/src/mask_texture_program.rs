crate::ix!();

pub struct MaskTextureProgram<'a> {
    base:         ShaderBase<'a>,
    image_params: ImageParams<'a>,
}

impl<'a> MaskTextureProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*


            : ShaderBase (context, ALOE_DECLARE_IMAGE_UNIFORMS
                              "void main()"
                              "{"
                                ALOE_HIGHP " vec2 texturePos = " ALOE_MATRIX_TIMES_FRAGCOORD ";"
                                ALOE_HIGHP " float roundingError = 0.00001;"
                                "if (texturePos.x >= -roundingError"
                                     "&& texturePos.y >= -roundingError"
                                     "&& texturePos.x <= imageLimits.x + roundingError"
                                     "&& texturePos.y <= imageLimits.y + roundingError)"
                                 "gl_FragColor = frontColour * " ALOE_GET_IMAGE_PIXEL ".a;"
                                "else "
                                 "gl_FragColor = vec4 (0, 0, 0, 0);"
                              "}"),
                  imageParams (program)
        */
    }
}
