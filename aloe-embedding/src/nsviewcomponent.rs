crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/embedding/aloe_NSViewComponent.h]

/**
  | A Mac-specific class that can create
  | and embed an NSView inside itself.
  | 
  | To use it, create one of these, put it
  | in place and make sure it's visible in
  | a window, then use setView() to assign
  | an NSView to it. The view will then be
  | moved and resized to follow the movements
  | of this component.
  | 
  | Of course, since the view is a native
  | object, it'll obliterate any
  | 
  | Aloe components that may overlap this
  | component, but that's life.
  | 
  | @tags{GUI}
  |
  */
#[cfg(target_os="macos")]
#[no_copy]
#[leak_detector]
pub struct NSViewComponent<'a> {
    base:       Component<'a>,
    attachment: Rc<RefCell<ReferenceCountedObject>>,
}

#[cfg(target_os="macos")]
impl<'a> Default for NSViewComponent<'a> {
    
    /**
      | Create an initially-empty container.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

#[cfg(target_os="macos")]
impl<'a> NSViewComponent<'a> {

    /**
      | Assigns an NSView to this peer.
      | 
      | The view will be retained and released
      | by this component for as long as it is
      | needed. To remove the current view,
      | just call setView (nullptr).
      | 
      | -----------
      | @note
      | 
      | A void* is used here to avoid including
      | the cocoa headers as part of AloeHeader.h,
      | but the method expects an NSView*.
      |
      */
    pub fn set_view(&mut self, ns_view: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the current NSView.
      | 
      | -----------
      | @note
      | 
      | A void* is returned here to avoid the
      | needing to include the cocoa headers,
      | so you should just cast the return value
      | to an NSView*.
      |
      */
    pub fn get_view(&self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Resizes this component to fit the view
      | that it contains.
      |
      */
    pub fn resize_to_fit_view(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn alpha_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn attach_view_to_component(
        _0: &mut Component,
        _1: *mut c_void) -> *mut ReferenceCountedObject {
        
        todo!();
        /*
        
        */
    }
}
