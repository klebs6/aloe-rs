crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_AndroidViewComponent.cpp]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/embedding/aloe_AndroidViewComponent.h]

/**
  | An Android-specific class that can
  | create and embed a View inside itself.
  | 
  | To use it, create one of these, put it
  | in place and make sure it's visible in
  | a window, then use setView() to assign
  | a View to it. The view will then be moved
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
//#[cfg(target_os="android")]
#[no_copy]
#[leak_detector]
pub struct AndroidViewComponent<'a> {
    base:  Component<'a>,
    impl_: Box<AndroidViewComponentImpl<'a>>,
}

//#[cfg(target_os="android")]
impl<'a> Default for AndroidViewComponent<'a> {
    
    /**
      | Create an initially-empty container
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//#[cfg(target_os="android")]
impl<'a> AndroidViewComponent<'a> {

    /**
      | Assigns a View to this peer.
      | 
      | The view will be retained and released
      | by this component for as long as it is
      | needed. To remove the current view,
      | just call setView (nullptr).
      |
      */
    pub fn set_view(&mut self, view: *mut c_void)  {
        
        todo!();
        /*
            if (view != getView())
        {
            impl.reset();

            if (view != nullptr)
            {
                // explicitly create a new local ref here so that we don't
                // delete the users pointer
                auto* env = getEnv();
                auto localref = LocalRef<jobject>(env->NewLocalRef((jobject) view));

                impl.reset (new AndroidViewComponentImpl (localref, *this));
            }
        }
        */
    }
    
    /**
      | Returns the current View.
      |
      */
    pub fn get_view(&self)  {
        
        todo!();
        /*
            return impl == nullptr ? nullptr : (void*) impl->view;
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
            if (impl != nullptr)
            setBounds (impl->getViewBounds());
        */
    }
    
    pub fn paint(&mut self, _0: &mut Graphics)  { }
}
