crate::ix!();

pub trait Shutdown {

    /**
      | Implement this method to free any GL
      | objects that you created during rendering.
      | 
      | The GL context will still be active when
      | this method is called.
      | 
      | -----------
      | @note
      | 
      | because the GL context could be destroyed
      | and re-created ad-hoc by the underlying
      | platform, the shutdown() and initialise()
      | calls could be called multiple times
      | while your app is running. So don't make
      | your code assume that this will only
      | be called once!
      |
      */
    fn shutdown(&mut self);
}

pub trait Render {

    /**
      | Called to render your openGL. @see OpenGLRenderer::render()
      |
      */
    fn render(&mut self);
}

pub trait Initialize {

    /**
      | Implement this method to set up any GL
      | objects that you need for rendering.
      | 
      | The GL context will be active when this
      | method is called.
      | 
      | -----------
      | @note
      | 
      | because the GL context could be destroyed
      | and re-created ad-hoc by the underlying
      | platform, the shutdown() and initialise()
      | calls could be called multiple times
      | while your app is running. So don't make
      | your code assume that this will only
      | be called once!
      |
      */
    fn initialise(&mut self);
}
