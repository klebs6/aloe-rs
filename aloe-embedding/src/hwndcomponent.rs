crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/embedding/aloe_HWNDComponent.h]

/**
  | A Windows-specific class that can create
  | and embed a HWND inside itself.
  | 
  | To use it, create one of these, put it
  | in place and make sure it's visible in
  | a window, then use setHWND() to assign
  | a HWND to it. The window will then be moved
  | and resized to follow the movements
  | of this component.
  | 
  | Of course, since the window is a native
  | object, it'll obliterate any
  | 
  | Aloe components that may overlap this
  | component, but that's life.
  | 
  | @tags{GUI}
  |
  */
#[cfg(target_os="windows")]
#[no_copy]
#[leak_detector]
pub struct HWNDComponent {
    base:  Component,
    pimpl: Box<Pimpl>,
}

#[cfg(target_os="windows")]
impl Default for HWNDComponent {
    
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

#[cfg(target_os="windows")]
impl HWNDComponent {

    /**
      | Assigns a HWND to this peer.
      | 
      | The window will be retained and released
      | by this component for as long as it is
      | needed. To remove the current HWND,
      | just call setHWND (nullptr).
      | 
      | -----------
      | @note
      | 
      | A void* is used here to avoid including
      | the Windows headers as part of AloeHeader.h,
      | but the method expects a HWND.
      |
      */
    pub fn sethwnd(&mut self, hwnd: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the current HWND.
      | 
      | -----------
      | @note
      | 
      | A void* is returned here to avoid the
      | needing to include the Windows headers,
      | so you should just cast the return value
      | to a HWND.
      |
      */
    pub fn gethwnd(&self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Resizes this component to fit the HWND
      | that it contains.
      |
      */
    pub fn resize_to_fit(&mut self)  {
        
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
