crate::ix!();

pub trait GetDesktopScaleFactor {

    /**
      | Returns the default scale factor to
      | use for this component when it is placed
      | on the desktop.
      | 
      | The default implementation of this
      | method just returns the value from Desktop::getGlobalScaleFactor(),
      | but it can be overridden if a particular
      | component has different requirements.
      | The method only used if this component
      | is added to the desktop - it has no effect
      | for child components.
      |
      */
    fn get_desktop_scale_factor(&self) -> f32;
}

pub trait AddToDesktop {

    /**
      | Makes this component appear as a window
      | on the desktop.
      | 
      | -----------
      | @note
      | 
      | before calling this, you should make
      | sure that the component's opacity is
      | set correctly using setOpaque(). If
      | the component is non-opaque, the windowing
      | system will try to create a special transparent
      | window for it, which will generally
      | take a lot more CPU to operate (and might
      | not even be possible on some platforms).
      | 
      | If the component is inside a parent component
      | at the time this method is called, it
      | will first be removed from that parent.
      | Likewise if a component is on the desktop
      | and is subsequently added to another
      | component, it'll be removed from the
      | desktop.
      | 
      | -----------
      | @param windowStyleFlags
      | 
      | a combination of the flags specified
      | in the ComponentPeer::StyleFlags
      | enum, which define the window's characteristics.
      | ----------
      | @param nativeWindowToAttachTo
      | 
      | this allows an OS object to be passed-in
      | as the window in which the aloe component
      | should place itself. On Windows, this
      | would be a HWND, a HIViewRef on the Mac.
      | Not necessarily supported on all platforms,
      | and best left as 0 unless you know what
      | you're doing. @see removeFromDesktop,
      | isOnDesktop, userTriedToCloseWindow,
      | getPeer, ComponentPeer::setMinimised,
      | ComponentPeer::StyleFlags, ComponentPeer::getStyleFlags,
      | ComponentPeer::setFullScreen
      |
      */
    fn add_to_desktop(&mut self, 
        window_style_flags:         i32,
        native_window_to_attach_to: *mut c_void);
}
