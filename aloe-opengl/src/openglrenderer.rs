crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLRenderer.h]

/**
  | A base class that should be implemented
  | by classes which want to render openGL
  | on a background thread.
  | 
  | @see OpenGLContext
  | 
  | @tags{OpenGL}
  |
  */
pub trait OpenGLRenderer {

    /**
      | Called when a new GL context has been
      | created.
      | 
      | You can use this as an opportunity to
      | create your textures, shaders, etc.
      | 
      | When the method is invoked, the new GL
      | context will be active.
      | 
      | -----------
      | @note
      | 
      | this callback will be made on a background
      | thread, so make sure that your implementation
      | is thread-safe.
      |
      */
    fn new_open_gl_context_created(&mut self);

    /**
      | Called when you should render the next
      | openGL frame.
      | 
      | -----------
      | @note
      | 
      | this callback will be made on a background
      | thread.
      | 
      | If the context is attached to a component
      | in order to do component rendering,
      | then the MessageManager will be locked
      | when this callback is made.
      | 
      | If no component rendering is being done,
      | then the MessageManager will not be
      | locked, and you'll need to make sure
      | your code is thread-safe in any interactions
      | it has with your GUI classes.
      | 
      | For information about how to trigger
      | a render callback, see
      | 
      | OpenGLContext::triggerRepaint()
      | and OpenGLContext::setContinuousRepainting().
      |
      */
    fn render_opengl(&mut self);

    /**
      | Called when the current openGL context
      | is about to close.
      | 
      | You can use this opportunity to release
      | any GL resources that you may have created.
      | 
      | -----------
      | @note
      | 
      | this callback will be made on a background
      | thread, so make sure that your implementation
      | is thread-safe.
      | 
      | (Also note that on Android, this callback
      | won't happen, because there's currently
      | no way to implement it..)
      |
      */
    fn open_gl_context_closing(&mut self);
}
