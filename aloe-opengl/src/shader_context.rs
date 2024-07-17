crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ShaderContext<'a> {
    base:     StackBasedLowLevelGraphicsContext<SavedState<'a>>,
    gl_state: GLState<'a>,
}

impl<'a> ShaderContext<'a> {

    pub fn new(target: &RenderingTarget) -> Self {
    
        todo!();
        /*
        : gl_state(target),

            stack.initialise (new SavedState (&glState));
        */
    }
    
    pub fn fill_rect_with_custom_shader(&mut self, 
        shader: &mut ShaderBase,
        area:   Rectangle<i32>)  {
        
        todo!();
        /*
            static_cast<SavedState&> (*stack).fillRectWithCustomShader (shader, area);
        */
    }
}
