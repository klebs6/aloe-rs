crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_linux_XEmbedComponent.cpp]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/embedding/aloe_XEmbedComponent.h]

/**
  | A Linux-specific class that can embed
  | a foreign X11 widget.
  | 
  | Use this class to embed a foreign X11
  | widget from other toolkits such as
  | 
  | GTK+ or QT.
  | 
  | There are two ways to initiate the Xembed
  | protocol. Either the client creates
  | a window and passes this to the host (client
  | initiated) or the host creates a window
  | in which the client can reparent it's
  | client widget (host initiated). XEmbedComponent
  | supports both protocol types.
  | 
  | This is how you embed a GTK+ widget: if
  | you are using the client initiated version
  | of the protocol, then create a new gtk
  | widget with gtk_plug_new (0). Then
  | query the window id of the plug via gtk_plug_get_id().
  | 
  | Pass this id to the constructor of this
  | class.
  | 
  | If you are using the host initiated version
  | of the protocol, then first create the
  | XEmbedComponent using the default
  | constructor. Use getHostWindowID
  | to get the window id of the host, use this
  | to construct your gtk plug via gtk_plug_new.
  | 
  | A similar approach can be used to embed
  | QT widgets via QT's QX11EmbedWidget
  | class.
  | 
  | Other toolkits or raw X11 widgets should
  | follow the X11 embed protocol: https://specifications.freedesktop.org/xembed-spec/xembed-spec-latest.html
  | 
  | @tags{GUI}
  |
  */
#[cfg(any(target_os="linux", target_os="freebsd", target_os="openbsd", target_os="netbsd"))]
pub struct XEmbedComponent {
    base:  Component,
    impl_: Box<Impl>,
}

#[cfg(any(target_os="linux", target_os="freebsd", target_os="openbsd", target_os="netbsd"))]
impl XEmbedComponent {

    /**
      | Creates a Aloe component wrapping a
      | foreign widget
      | 
      | Use this constructor if you are using
      | the host initiated version of the XEmbedProtocol.
      | When using this version of the protocol
      | you must call getHostWindowID() and
      | pass this id to the foreign toolkit.
      |
      */
    pub fn new(
        wants_keyboard_focus:                     Option<bool>,
        allow_foreign_widget_to_resize_component: Option<bool>

    ) -> Self {

        let wants_keyboard_focus: bool =
                 wants_keyboard_focus.unwrap_or(true);

        let allow_foreign_widget_to_resize_component: bool =
                 allow_foreign_widget_to_resize_component.unwrap_or(false);
    
        todo!();
        /*


            : impl (new XembedComponentImpl (*this, 0, wantsKeyboardFocus, false, allowForeignWidgetToResizeComponent))
        setOpaque (true);
        */
    }
    
    /**
      | Create a Aloe component wrapping the
      | foreign widget with id wID
      | 
      | Use this constructor if you are using
      | the client initiated version of the
      | XEmbedProtocol.
      |
      */
    pub fn new(
        wid:                                      u64,
        wants_keyboard_focus:                     bool,
        allow_foreign_widget_to_resize_component: bool

    ) -> Self {

        let wants_keyboard_focus: bool =
                 wants_keyboard_focus.unwrap_or(true);
        let allow_foreign_widget_to_resize_component: bool =
                 allow_foreign_widget_to_resize_component.unwrap_or(false);
    
        todo!();
        /*


            : impl (new XembedComponentImpl (*this, wID, wantsKeyboardFocus, true, allowForeignWidgetToResizeComponent))
        setOpaque (true);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::lightgrey);
        */
    }
    
    pub fn focus_gained(&mut self, change_type: FocusChangeType)  {
        
        todo!();
        /*
            impl->focusGained (changeType);
        */
    }
    
    pub fn focus_lost(&mut self, change_type: FocusChangeType)  {
        
        todo!();
        /*
            impl->focusLost   (changeType);
        */
    }
    
    pub fn brought_to_front(&mut self)  {
        
        todo!();
        /*
            impl->broughtToFront();
        */
    }
    
    /**
      | Use this method to retrieve the host's
      | window id when using the host initiated
      | version of the XEmbedProtocol
      |
      */
    pub fn get_host_windowid(&mut self) -> u64 {
        
        todo!();
        /*
            return impl->getHostWindowID();
        */
    }
    
    /**
      | Removes the client window from the host.
      |
      */
    pub fn remove_client(&mut self)  {
        
        todo!();
        /*
            impl->setClient (0, true);
        */
    }
}
