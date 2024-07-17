crate::ix!();

pub struct OpenGLTargetSaver<'a> {
    context:         &'a OpenGLContext<'a>,
    old_framebuffer: u32,
    old_viewport:    [GLint; 4],
}

impl<'a> Drop for OpenGLTargetSaver<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            context.extensions.glBindFramebuffer (GL_FRAMEBUFFER, oldFramebuffer);
            glViewport (oldViewport[0], oldViewport[1], oldViewport[2], oldViewport[3]);
        */
    }
}

impl<'a> OpenGLTargetSaver<'a> {

    pub fn new(c: &OpenGLContext) -> Self {
    
        todo!();
        /*


            : context (c), oldFramebuffer (OpenGLFrameBuffer::getCurrentFrameBufferTarget())

            glGetIntegerv (GL_VIEWPORT, oldViewport);
        */
    }
    
    pub fn assign_from(&mut self, _0: &OpenGLTargetSaver) -> &mut OpenGLTargetSaver {
        
        todo!();
        /*
        
        */
    }
}
