crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/embedding/aloe_UIViewComponent.h]

/**
  | An iOS-specific class that can create
  | and embed an UIView inside itself.
  | 
  | To use it, create one of these, put it
  | in place and make sure it's visible in
  | a window, then use setView() to assign
  | a UIView to it. The view will then be moved
  | and resized to follow the movements
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
//#[cfg(target_os="ios")]
#[no_copy]
#[leak_detector]
pub struct UIViewComponent<'a> {
    base:  Component<'a>,
    impl_: Box<UIViewComponentImpl>,
}

pub struct UIViewComponentImpl {}

//#[cfg(target_os="ios")]
impl<'a> Default for UIViewComponent<'a> {
    
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

//#[cfg(target_os="ios")]
impl<'a> UIViewComponent<'a> {

    /**
      | Assigns an UIView to this peer.
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
      | but the method expects an UIView*.
      |
      */
    pub fn set_view(&mut self, ui_view: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the current UIView.
      | 
      | -----------
      | @note
      | 
      | A void* is returned here to avoid the
      | needing to include the cocoa headers,
      | so you should just cast the return value
      | to an UIView*.
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
}
