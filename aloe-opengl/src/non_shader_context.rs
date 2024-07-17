crate::ix!();

#[no_copy]
#[leak_detector]
pub struct NonShaderContext<'a> {
    base:   LowLevelGraphicsSoftwareRenderer,
    target: RenderingTarget<'a>,
    image:  Image,
}

impl<'a> Drop for NonShaderContext<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            ALOE_CHECK_OPENGL_ERROR
            auto previousFrameBufferTarget = OpenGLFrameBuffer::getCurrentFrameBufferTarget();

           #if ! ALOE_ANDROID
            target.context.extensions.glActiveTexture (GL_TEXTURE0);
            glEnable (GL_TEXTURE_2D);
            clearGLError();
           #endif

            OpenGLTexture texture;
            texture.loadImage (image);
            texture.bind();

            target.makeActive();
            target.context.copyTexture (target.bounds, Rectangle<int> (texture.getWidth(),
                                                                       texture.getHeight()),
                                        target.bounds.getWidth(), target.bounds.getHeight(),
                                        false);
            glBindTexture (GL_TEXTURE_2D, 0);

           #if ALOE_WINDOWS
            if (target.context.extensions.glBindFramebuffer != nullptr)
           #endif
                target.context.extensions.glBindFramebuffer (GL_FRAMEBUFFER, previousFrameBufferTarget);

            ALOE_CHECK_OPENGL_ERROR
        */
    }
}

impl<'a> NonShaderContext<'a> {

    pub fn new(
        t:  &RenderingTarget,
        im: &Image) -> Self {
    
        todo!();
        /*
        : low_level_graphics_software_renderer(im),
        : target(t),
        : image(im),

        
        */
    }
}
