crate::ix!();

pub struct CurrentShader<'a> {
    context:        &'a mut OpenGLContext<'a>,
    programs:       ShaderProgramsPtr<'a>,
    active_shader:  *mut ShaderBase<'a>, // default = nullptr
    current_bounds: Rectangle<i32>,
}

impl<'a> Drop for CurrentShader<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            jassert (activeShader == nullptr);
        */
    }
}

impl<'a> CurrentShader<'a> {

    pub fn new(c: &mut OpenGLContext) -> Self {
    
        todo!();
        /*
        : context(c),

            auto programValueID = "GraphicsContextPrograms";
                programs = static_cast<ShaderPrograms*> (context.getAssociatedObject (programValueID));

                if (programs == nullptr)
                {
                    programs = new ShaderPrograms (context);
                    context.setAssociatedObject (programValueID, programs.get());
                }
        */
    }
    
    pub fn set_shader_with_bounds(
        &mut self, 
        bounds:     Rectangle<i32>,
        quad_queue: &mut ShaderQuadQueue,
        shader:     &mut ShaderBase

    ) {
        
        todo!();
        /*
            if (activeShader != &shader)
                {
                    clearShader (quadQueue);

                    activeShader = &shader;
                    shader.program.use();
                    shader.bindAttributes();

                    if (shader.onShaderActivated)
                        shader.onShaderActivated (shader.program);

                    currentBounds = bounds;
                    shader.set2DBounds (bounds.toFloat());

                    ALOE_CHECK_OPENGL_ERROR
                }
                else if (bounds != currentBounds)
                {
                    currentBounds = bounds;
                    shader.set2DBounds (bounds.toFloat());
                }
        */
    }
    
    pub fn set_shader_with_rendering_target(
        &mut self, 
        target:     &mut RenderingTarget,
        quad_queue: &mut ShaderQuadQueue,
        shader:     &mut ShaderBase
    )  {

        todo!();
        /*
            setShader (target.bounds, quadQueue, shader);
        */
    }
    
    pub fn clear_shader(&mut self, quad_queue: &mut ShaderQuadQueue)  {
        
        todo!();
        /*
            if (activeShader != nullptr)
                {
                    quadQueue.flush();
                    activeShader->unbindAttributes();
                    activeShader = nullptr;
                    context.extensions.glUseProgram (0);
                }
        */
    }
    
    pub fn assign_from(&mut self, _0: &CurrentShader) -> &mut CurrentShader {
        
        todo!();
        /*
        
        */
    }
}
