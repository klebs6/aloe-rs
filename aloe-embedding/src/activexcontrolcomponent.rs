crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/embedding/aloe_ActiveXControlComponent.h]

/**
  | A Windows-specific class that can create
  | and embed an ActiveX control inside
  | itself.
  | 
  | To use it, create one of these, put it
  | in place and make sure it's visible in
  | a window, then use createControl()
  | to instantiate an ActiveX control.
  | The control will then be moved and resized
  | to follow the movements of this component.
  | 
  | Of course, since the control is a heavyweight
  | window, it'll obliterate any
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
pub struct ActiveXControlComponent {
    base:                 Component,
    control:              Box<Impl>,
    mouse_events_allowed: bool, // default = true
}

#[cfg(target_os="windows")]
impl Default for ActiveXControlComponent {
    
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
impl ActiveXControlComponent {

    /**
      | Tries to create an ActiveX control and
      | embed it in this peer.
      | 
      | The peer controlIID is a pointer to an
      | IID structure - it's treated as a void*
      | because when including the Aloe headers,
      | you might not always have included windows.h
      | first, in which case IID wouldn't be
      | defined.
      | 
      | -----------
      | @code
      | 
      | const IID myIID = __uuidof (QTControl);
      | myControlComp->createControl (&myIID);
      |
      */
    pub fn create_control(&mut self, controliid: *const c_void) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Deletes the ActiveX control, if one
      | has been created.
      |
      */
    pub fn delete_control(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if a control is currently
      | in use.
      |
      */
    pub fn is_control_open(&self) -> bool {
        
        todo!();
        /*
            return control != nullptr;
        */
    }

    /**
      | Does a QueryInterface call on the embedded
      | control object.
      | 
      | This allows you to cast the control to
      | whatever type of COM object you need.
      | 
      | The iid parameter is a pointer to an IID
      | structure - it's treated as a void* because
      | when including the Aloe headers, you
      | might not always have included windows.h
      | first, in which case IID wouldn't be
      | defined, but you should just pass a pointer
      | to an IID.
      | 
      | -----------
      | @code
      | 
      | const IID iid = __uuidof (IOleWindow);
      | 
      | IOleWindow* oleWindow = (IOleWindow*) myControlComp->queryInterface (&iid);
      | 
      | if (oleWindow != nullptr)
      | {
      |     HWND hwnd;
      |     oleWindow->GetWindow (&hwnd);
      | 
      |     ...
      | 
      |     oleWindow->Release();
      | }
      |
      */
    pub fn query_interface(&self, iid: *const c_void)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Set this to false to stop mouse events
      | being allowed through to the control.
      |
      */
    pub fn set_mouse_events_allowed(&mut self, events_can_reach_control: bool)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if mouse events are allowed
      | to get through to the control.
      |
      */
    pub fn are_mouse_events_allowed(&self) -> bool {
        
        todo!();
        /*
            return mouseEventsAllowed;
        */
    }
    
    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn offer_event_to_active_xcontrol(&mut self, _0: *mut c_void) -> libc::intptr_t {
        
        todo!();
        /*
        
        */
    }
    
    pub fn offer_event_to_active_xcontrol_static(_0: *mut c_void) -> libc::intptr_t {
        
        todo!();
        /*
        
        */
    }
}
