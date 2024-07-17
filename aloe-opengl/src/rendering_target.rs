crate::ix!();

pub struct RenderingTarget<'a> {
    context:        &'a mut OpenGLContext<'a>,
    frame_bufferid: GLuint,
    bounds:         Rectangle<i32>,
}

impl<'a> RenderingTarget<'a> {

    pub fn new_with_wh(
        c:      &mut OpenGLContext,
        fbid:   GLuint,
        width:  i32,
        height: i32) -> Self {
    
        todo!();
        /*
        : context(c),
        : frame_bufferid(fbID),
        : bounds(width, height),
        */
    }
    
    pub fn new(
        c:      &mut OpenGLContext,
        fb:     &mut OpenGLFrameBuffer,
        origin: Point<i32>) -> Self {
    
        todo!();
        /*
        : context (c), frameBufferID (fb.getFrameBufferID()),
          bounds (origin.x, origin.y, fb.getWidth(), fb.getHeight())

          jassert (frameBufferID != 0); // trying to render into an uninitialised framebuffer object.
        */
    }
    
    pub fn new_from_rendering_target(other: &RenderingTarget) -> Self {
    
        todo!();
        /*


            : context (other.context), frameBufferID (other.frameBufferID), bounds (other.bounds)
        */
    }
    
    pub fn assign_from(&mut self, other: &RenderingTarget) -> &mut RenderingTarget {
        
        todo!();
        /*
            frameBufferID = other.frameBufferID;
            bounds = other.bounds;
            return *this;
        */
    }
    
    pub fn make_active(&self)  {
        
        todo!();
        /*
            #if ALOE_WINDOWS
            if (context.extensions.glBindFramebuffer != nullptr)
           #endif
                context.extensions.glBindFramebuffer (GL_FRAMEBUFFER, frameBufferID);

            glViewport (0, 0, bounds.getWidth(), bounds.getHeight());
            glDisable (GL_DEPTH_TEST);
        */
    }
}
