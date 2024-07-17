crate::ix!();

/**
  | This component sits in the taskbar tray
  | as a small icon.
  | 
  | (NB: The exact behaviour of this class
  | will differ between OSes, and it isn't
  | fully implemented for all OSes)
  | 
  | To use it, just create one of these components,
  | but don't attempt to make it visible,
  | add it to a parent, or put it on the desktop.
  | 
  | You can then call setIconImage() to
  | create an icon for it in the taskbar.
  | 
  | To change the icon's tooltip, you can
  | use setIconTooltip().
  | 
  | To respond to mouse-events, you can
  | override the normal mouseDown(), mouseUp(),
  | mouseDoubleClick() and mouseMove()
  | methods, and although the x, y position
  | will not be valid, you can use this to
  | respond to clicks. Traditionally you'd
  | use a left-click to show your application's
  | window, and a right-click to show a pop-up
  | menu.
  | 
  | @tags{GUI}
  |
  */
#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
#[no_copy]
#[leak_detector]
pub struct SystemTrayIconComponent<'a,StatusItem:NSStatusItem,ImageType:NSImage> {
    base:  Component<'a>,
    pimpl: Box<SystemTrayIconComponentPimpl<'a,StatusItem,ImageType>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_SystemTrayIconComponent.cpp]
#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> SetIconImage for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    fn set_icon_image(
        &mut self, 
        colour_image:   &Image,
        template_image: &Image
    )
    {
        todo!();
    }
}

#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> SetIconTooltip for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    fn set_icon_tooltip(&mut self, tooltip: &String)
    {
        todo!();
    }
}

#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> SetHighlighted for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    fn set_highlighted(&mut self, _0: bool)
    {
        todo!();
    }
}

#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> ShowInfoBubble for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    fn show_info_bubble(
        &mut self, 
        title:   &String,
        content: &String
    )
    {
        todo!();
    }
}

#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> HidInfoBubble for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    fn hide_info_bubble(&mut self)
    {
        todo!();
    }
}

#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> GetIconNativeHandle for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    fn get_native_handle(&self)
    {
        todo!();
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> PaintGraphics for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (pimpl != nullptr)
            g.drawImage (pimpl->image, getLocalBounds().toFloat(),
                         RectanglePlacement::xLeft | RectanglePlacement::yTop | RectanglePlacement::onlyReduceInSize);
        */
    }
}
        
#[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="macos"))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> Default for SystemTrayIconComponent<'a,StatusItem,ImageType> {
    
    fn default() -> Self {
    
        todo!();
        /*


            addToDesktop (0);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_linux_X11_SystemTrayIcon.cpp]

#[cfg(target_os="linux")]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> SystemTrayIconComponent<'a,StatusItem,ImageType> {

    pub fn set_icon_image(&mut self, 
        colour_image: &Image,
        _1:           &Image)  {
        
        todo!();
        /*
            pimpl.reset();

        if (colourImage.isValid())
        {
            if (! isOnDesktop())
                addToDesktop (0);

            pimpl.reset (new SystemTrayIconComponentPimpl (colourImage, (Window) getWindowHandle()));

            setVisible (true);
            toFront (false);
        }

        repaint();
        */
    }
    
    pub fn set_icon_tooltip(&mut self, tooltip: &String)  {
        
        todo!();
        /*
            // xxx Not implemented!
        */
    }
    
    pub fn set_highlighted(&mut self, _0: bool)  {
        
        todo!();
        /*
            // xxx Not implemented!
        */
    }
    
    pub fn show_info_bubble(&mut self, 
        title:   &String,
        content: &String)  {
        
        todo!();
        /*
            // xxx Not implemented!
        */
    }
    
    pub fn hide_info_bubble(&mut self)  {
        
        todo!();
        /*
            // xxx Not implemented!
        */
    }
    
    pub fn get_native_handle(&self)  {
        
        todo!();
        /*
            return getWindowHandle();
        */
    }
}
