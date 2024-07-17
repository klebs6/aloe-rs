crate::ix!();

pub struct ShaderProgramHolder<'a> {
    program:    OpenGLShaderProgram<'a>,
    last_error: String,
}

impl<'a> ShaderProgramHolder<'a> {

    pub fn new(
        context:         &mut OpenGLContext,
        fragment_shader: *const u8,
        vertex_shader:   *const u8) -> Self {
    
        todo!();
        /*
        : program(context),

            ALOE_CHECK_OPENGL_ERROR

                if (vertexShader == nullptr)
                    vertexShader = "attribute vec2 position;"
                                   "attribute vec4 colour;"
                                   "uniform vec4 screenBounds;"
                                   "varying " ALOE_MEDIUMP " vec4 frontColour;"
                                   "varying " ALOE_HIGHP " vec2 pixelPos;"
                                   "void main()"
                                   "{"
                                     "frontColour = colour;"
                                     "vec2 adjustedPos = position - screenBounds.xy;"
                                     "pixelPos = adjustedPos;"
                                     "vec2 scaledPos = adjustedPos / screenBounds.zw;"
                                     "gl_Position = vec4 (scaledPos.x - 1.0, 1.0 - scaledPos.y, 0, 1.0);"
                                   "}";

                if (program.addVertexShader (OpenGLHelpers::translateVertexShaderToV3 (vertexShader))
                     && program.addFragmentShader (OpenGLHelpers::translateFragmentShaderToV3 (fragmentShader))
                     && program.link())
                {
                    ALOE_CHECK_OPENGL_ERROR
                }
                else
                {
                    lastError = program.getLastError();
                }
        */
    }
}

