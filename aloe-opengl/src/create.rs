crate::ix!();

pub fn clear_open_gl_glyph_cache_callback()  {
    
    todo!();
        /*
            SavedState::GlyphCacheType::getInstance().reset();
        */
}

pub fn create_open_gl_context(target: &RenderingTarget) -> Box<dyn LowLevelGraphicsContext> {
    
    todo!();
        /*
            clearOpenGLGlyphCache = clearOpenGLGlyphCacheCallback;

        if (target.context.areShadersAvailable())
            return std::make_unique<ShaderContext> (target);

        Image tempImage (Image::ARGB, target.bounds.getWidth(), target.bounds.getHeight(), true, SoftwareImageType());
        return std::make_unique<NonShaderContext> (target, tempImage);
        */
}

/**
  | Creates a graphics context object that
  | will render into the given OpenGL target.
  |
  */
pub fn create_open_gl_graphics_context_with_width_height(
    context: &mut OpenGLContext,
    width:   i32,
    height:  i32
) -> Box<dyn LowLevelGraphicsContext> {
    
    todo!();
        /*
            return createOpenGLGraphicsContext (context, context.getFrameBufferID(), width, height);
        */
}

/**
  | Creates a graphics context object that
  | will render into the given OpenGL framebuffer.
  |
  */
pub fn create_open_gl_graphics_context_with_framebuffer(
    context: &mut OpenGLContext,
    target:  &mut OpenGLFrameBuffer

) -> Box<dyn LowLevelGraphicsContext> {
    
    todo!();
        /*
            return OpenGLRendering::createOpenGLContext (OpenGLRendering::RenderingTarget (context, target, {}));
        */
}

/**
  | Creates a graphics context object that
  | will render into the given OpenGL framebuffer,
  | with the given size.
  |
  */
pub fn create_open_gl_graphics_context_with_bufferid_width_height(
    context:        &mut OpenGLContext,
    frame_bufferid: u32,
    width:          i32,
    height:         i32

) -> Box<dyn LowLevelGraphicsContext> {
    
    todo!();
        /*
            return OpenGLRendering::createOpenGLContext (OpenGLRendering::RenderingTarget (context, frameBufferID, width, height));
        */
}
