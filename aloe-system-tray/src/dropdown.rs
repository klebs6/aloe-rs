crate::ix!();

#[cfg(any(any(target_os="windows",target_os="linux"),any(target_os="bsd",target_os="macos")))]
impl<'a,StatusItem:NSStatusItem,ImageType:NSImage> ShowDropdownMenu for SystemTrayIconComponent<'a,StatusItem,ImageType> {

    /**
      | Shows a menu attached to the OSX menu
      | bar icon.
      |
      */
    #[cfg(target_os="macos")]
    fn show_dropdown_menu(&mut self, menu: &PopupMenu)  {
        
        todo!();
        /*
        
        */
    }
}
