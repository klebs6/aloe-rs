crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/utils/aloe_OpenGLAppComponent.h]

pub trait OpenGLAppComponentInterface: Shutdown + Render + Initialize {}

/**
  | A base class for writing simple one-page
  | graphical apps.
  | 
  | A subclass can inherit from this and
  | implement just a few methods such as
  | paint() and mouse-handling. The base
  | class provides some simple abstractions
  | to take care of continuously repainting
  | itself.
  | 
  | @tags{OpenGL}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLAppComponent<'a> {
    base:            Component<'a>,

    /**
      | The GL context
      |
      */
    open_gl_context: OpenGLContext<'a>,

    frame_counter:   i32, // default = 0
}

impl<'a> OpenGLRenderer for OpenGLAppComponent<'a> {

    fn new_open_gl_context_created(&mut self)  {
        
        todo!();
        /*
            initialise();
        */
    }
    
    fn render_opengl(&mut self)  {
        
        todo!();
        /*
            ++frameCounter;
        render();
        */
    }
    
    fn open_gl_context_closing(&mut self)  {
        
        todo!();
        /*
            shutdown();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/utils/aloe_OpenGLAppComponent.cpp]
impl<'a> Drop for OpenGLAppComponent<'a> {

    fn drop(&mut self) {

        todo!();

        /*
        // Before your subclass's destructor has completed, you must call
        // shutdownOpenGL() to release the GL context. (Otherwise there's
        // a danger that it may invoke a GL callback on your class while
        // it's in the process of being deleted.
        jassert (! openGLContext.isAttached());

        shutdownOpenGL();
        */
    }
}

impl<'a> Default for OpenGLAppComponent<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        setOpaque (true);
        openGLContext.setRenderer (this);
        openGLContext.attachTo (*this);
        openGLContext.setContinuousRepainting (true);
        */
    }
}

impl<'a> OpenGLAppComponent<'a> {

    /**
      | Returns the number of times that the
      | render method has been called since
      | the component started running.
      |
      */
    pub fn get_frame_counter(&self) -> i32 {
        
        todo!();
        /*
            return frameCounter;
        */
    }

    /**
      | This must be called from your subclass's
      | destructor, to shut down the GL system
      | and stop it calling render() before
      | your class is destroyed.
      |
      */
    pub fn shutdown_opengl(&mut self)  {
        
        todo!();
        /*
            openGLContext.detach();
        */
    }
}
