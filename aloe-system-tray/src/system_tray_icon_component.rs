crate::ix!();

pub trait ConfigureIcon {

    fn configure_icon(&mut self);
}

pub trait SetHighlighted {

    fn set_highlighted(&mut self, should_highlight: bool);
}

pub trait HideInfoBubble {

    fn hide_info_bubble(&mut self);
}

pub trait GetNativeHandle {

    fn get_native_handle(&self);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_mac_SystemTrayIcon.cpp]

lazy_static!{
    /*
    extern NSMenu* createNSMenu (const PopupMenu&, const String& name, int topLevelMenuId,
                                 int topLevelIndex, bool addDelegate);
    */
}

#[cfg(not(any(
            target_os="win32",
            target_os="linux",
            target_os="bsd",
            target_os="macos")))]
pub struct SystemTrayIconComponent {}

#[cfg(not(any(any(target_os="win32",target_os="linux"),any(target_os="bsd",target_os="macos"))))]
impl SetIconImage for SystemTrayIconComponent {

    fn set_icon_image(&mut self, 
        _0:             &Image,
        template_image: &Image)  {
        
        todo!();
        /*
            if (templateImage.isValid())
        {
            if (pimpl == nullptr)
                pimpl.reset (new SystemTrayIconComponentPimpl (*this, templateImage));
            else
                pimpl->statusItemHolder->updateIcon (templateImage);
        }
        else
        {
            pimpl.reset();
        }
        */
    }
}

#[cfg(not(any(any(target_os="win32",target_os="linux"),any(target_os="bsd",target_os="macos"))))]
impl SetIconTooltip for SystemTrayIconComponent {
    
    fn set_icon_tooltip(&mut self, _0: &str)  {
        
        todo!();
        /*
            // xxx not yet implemented!
        */
    }
}

#[cfg(not(any(any(target_os="win32",target_os="linux"),any(target_os="bsd",target_os="macos"))))]
impl SetHighlighted for SystemTrayIconComponent {
    
    fn set_highlighted(&mut self, should_highlight: bool)  {
        
        todo!();
        /*
            if (pimpl != nullptr)
            pimpl->statusItemHolder->setHighlighted (shouldHighlight);
        */
    }
}

#[cfg(not(any(any(target_os="win32",target_os="linux"),any(target_os="bsd",target_os="macos"))))]
impl ShowInfoBubble for SystemTrayIconComponent {
    
    fn show_info_bubble(&mut self, title: &str, content: &str)  {
        
        todo!();
        /*
            // xxx Not implemented!
        */
    }
}

#[cfg(not(any(any(target_os="win32",target_os="linux"),any(target_os="bsd",target_os="macos"))))]
impl HideInfoBubble for SystemTrayIconComponent {
    
    fn hide_info_bubble(&mut self)  {
        
        todo!();
        /*
            // xxx Not implemented!
        */
    }
}

#[cfg(not(any(any(target_os="win32",target_os="linux"),any(target_os="bsd",target_os="macos"))))]
impl GetNativeHandle for SystemTrayIconComponent {
    
    fn get_native_handle(&self)  {
        
        todo!();
        /*
            return pimpl != nullptr ? pimpl->statusItemHolder->statusItem.get() : nullptr;
        */
    }
}

#[cfg(not(any(any(target_os="win32",target_os="linux"),any(target_os="bsd",target_os="macos"))))]
impl ShowDropdownMenu for SystemTrayIconComponent {
    
    fn show_dropdown_menu(&mut self, menu: &PopupMenu)  {
        
        todo!();
        /*
            if (pimpl != nullptr)
            pimpl->statusItemHolder->showMenu (menu);
        */
    }
}
