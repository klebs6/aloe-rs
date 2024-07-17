crate::ix!();

pub struct SolidColourProgram<'a> {
    base: ShaderBase<'a>,
}

impl<'a> SolidColourProgram<'a> {

    pub fn new(context: &mut OpenGLContext) -> Self {
    
        todo!();
        /*
            : ShaderBase (context, ALOE_DECLARE_VARYING_COLOUR
                              "void main() { gl_FragColor = frontColour; }")
        */
    }
}
