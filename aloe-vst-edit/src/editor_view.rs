crate::ix!();

/**
  | View related to an edit controller.
  | \ingroup vstClasses
  |
  */
pub struct EditorView {
    base:       CPluginView,
    controller: *mut EditController,
}

#[cfg(not(NO_PLUGUI))]
impl Drop for EditorView {

    fn drop(&mut self) {
        todo!();
        /*
            if (controller)
        {
            controller->editorDestroyed (this);
            controller->release ();
        }
        */
    }
}

impl EditorView {

    /**
      | Gets its controller part.
      |
      */
    pub fn get_controller(&self) -> *mut EditController {
        
        todo!();
        /*
            return controller;
        */
    }
    
    #[cfg(not(NO_PLUGUI))]
    pub fn new(
        controller: *mut EditController,
        size:       *mut ViewRect) -> Self {
    
        todo!();
        /*
        : plugin_view(size),
        : controller(_controller),

            if (controller)
        {
            controller->addRef ();
        }
        */
    }
    
    #[cfg(not(NO_PLUGUI))]
    pub fn attached_to_parent(&mut self)  {
        
        todo!();
        /*
            if (controller)
        {
            controller->editorAttached (this);
        }
        */
    }
    
    #[cfg(not(NO_PLUGUI))]
    pub fn removed_from_parent(&mut self)  {
        
        todo!();
        /*
            if (controller)
        {
            controller->editorRemoved (this);
        }
        */
    }
}
