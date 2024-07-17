crate::ix!();

pub struct ImageProgram<'a> {
    base:         ShaderBase<'a>,
    image_params: ImageParams<'a>,
}

impl<'a> ImageProgram<'a> {
    
    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*


            : ShaderBase (context, ALOE_DECLARE_VARYING_COLOUR
                              "uniform sampler2D imageTexture;"
                              "varying " ALOE_HIGHP " vec2 texturePos;"
                              "void main()"
                              "{"
                                "gl_FragColor = frontColour.a * " ALOE_GET_IMAGE_PIXEL ";"
                              "}",
                              "uniform " ALOE_MEDIUMP " vec2 imageLimits;"
                              ALOE_DECLARE_MATRIX_UNIFORM
                              "attribute vec2 position;"
                              "attribute vec4 colour;"
                              "uniform vec4 screenBounds;"
                              "varying " ALOE_MEDIUMP " vec4 frontColour;"
                              "varying " ALOE_HIGHP " vec2 texturePos;"
                              "void main()"
                              "{"
                                "frontColour = colour;"
                                "vec2 adjustedPos = position - screenBounds.xy;"
                                "vec2 pixelPos = adjustedPos;"
                                "texturePos = clamp (" ALOE_MATRIX_TIMES_FRAGCOORD ", vec2 (0, 0), imageLimits);"
                                "vec2 scaledPos = adjustedPos / screenBounds.zw;"
                                "gl_Position = vec4 (scaledPos.x - 1.0, 1.0 - scaledPos.y, 0, 1.0);"
                              "}"),
                  imageParams (program)
        */
    }
}
