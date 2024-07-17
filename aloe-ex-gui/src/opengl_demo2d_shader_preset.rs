crate::ix!();

pub const OPENGL_DEMO2D_SHADER_LINK_DELAY: usize = 500;

pub struct OpenGLAppDemo2DShaderPreset
{
    name:            *const u8,
    fragment_shader: *const u8,
}

pub fn get_presets() -> Vec<OpenGLAppDemo2DShaderPreset> {
    
    todo!();
    /*
        #define SHADER_2DDEMO_HEADER \
                "/*  This demo shows the use of the OpenGLGraphicsContextCustomShader,\n" \
                "    which allows a 2D area to be filled using a GL shader program.\n" \
                "\n" \
                "    Edit the shader program below and it will be \n" \
                "    recompiled in real-time!\n" \
                "*/\n\n"

            OpenGLAppDemo2DShaderPreset presets[] =
            {
                {
                    "Simple Gradient",

                    SHADER_2DDEMO_HEADER
                    "void main()\n"
                    "{\n"
                    "    " ALOE_MEDIUMP " vec4 colour1 = vec4 (1.0, 0.4, 0.6, 1.0);\n"
                    "    " ALOE_MEDIUMP " vec4 colour2 = vec4 (0.0, 0.8, 0.6, 1.0);\n"
                    "    " ALOE_MEDIUMP " float alpha = pixelPos.x / 1000.0;\n"
                    "    gl_FragColor = pixelAlpha * mix (colour1, colour2, alpha);\n"
                    "}\n"
                },

                {
                    "Circular Gradient",

                    SHADER_2DDEMO_HEADER
                    "void main()\n"
                    "{\n"
                    "    " ALOE_MEDIUMP " vec4 colour1 = vec4 (1.0, 0.4, 0.6, 1.0);\n"
                    "    " ALOE_MEDIUMP " vec4 colour2 = vec4 (0.3, 0.4, 0.4, 1.0);\n"
                    "    " ALOE_MEDIUMP " float alpha = distance (pixelPos, vec2 (600.0, 500.0)) / 400.0;\n"
                    "    gl_FragColor = pixelAlpha * mix (colour1, colour2, alpha);\n"
                    "}\n"
                },

                {
                    "Circle",

                    SHADER_2DDEMO_HEADER
                    "void main()\n"
                    "{\n"
                    "    " ALOE_MEDIUMP " vec4 colour1 = vec4 (0.1, 0.1, 0.9, 1.0);\n"
                    "    " ALOE_MEDIUMP " vec4 colour2 = vec4 (0.0, 0.8, 0.6, 1.0);\n"
                    "    " ALOE_MEDIUMP " float distance = distance (pixelPos, vec2 (600.0, 500.0));\n"
                    "\n"
                    "    " ALOE_MEDIUMP " float innerRadius = 200.0;\n"
                    "    " ALOE_MEDIUMP " float outerRadius = 210.0;\n"
                    "\n"
                    "    if (distance < innerRadius)\n"
                    "        gl_FragColor = colour1;\n"
                    "    else if (distance > outerRadius)\n"
                    "        gl_FragColor = colour2;\n"
                    "    else\n"
                    "        gl_FragColor = mix (colour1, colour2, (distance - innerRadius) / (outerRadius - innerRadius));\n"
                    "\n"
                    "    gl_FragColor *= pixelAlpha;\n"
                    "}\n"
                },

                {
                    "Solid Colour",

                    SHADER_2DDEMO_HEADER
                    "void main()\n"
                    "{\n"
                    "    gl_FragColor = vec4 (1.0, 0.6, 0.1, pixelAlpha);\n"
                    "}\n"
                }
            };

            return Vec<OpenGLAppDemo2DShaderPreset> (presets, numElementsInArray (presets));
    */
}
