crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/Main.cpp]

/**
  | Just add a simple icon to the Window system
  | tray area or Mac menu bar..
  |
  */
#[cfg(any(any(target_os="macos",target_os="windows"),any(target_os="linux",target_os="bsd")))]
pub struct DemoTaskbarComponent<'a, StatusItem:NSStatusItem,ImageType:NSImage> {
    base: SystemTrayIconComponent<'a,StatusItem,ImageType>,
    base2: Timer,
}

#[cfg(any(any(target_os="macos",target_os="windows"),any(target_os="linux",target_os="bsd")))]
impl<'a, StatusItem:NSStatusItem,ImageType:NSImage> Default for DemoTaskbarComponent<'a,StatusItem,ImageType> {
    
    fn default() -> Self {
        todo!();
        /*


            setIconImage (getImageFromAssets ("aloe_icon.png"),
                           getImageFromAssets ("aloe_icon_template.png"));
             setIconTooltip ("Aloe demo runner!")
        */
    }
}

#[cfg(any(any(target_os="macos",target_os="windows"),any(target_os="linux",target_os="bsd")))]
impl<'a, StatusItem:NSStatusItem,ImageType:NSImage> DemoTaskbarComponent<'a,StatusItem,ImageType> {

    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            // On OSX, there can be problems launching a menu when we're not the foreground
             // process, so just in case, we'll first make our process active, and then use a
             // timer to wait a moment before opening our menu, which gives the OS some time to
             // get its act together and bring our windows to the front.

             Process::makeForegroundProcess();
             startTimer (50);
        */
    }

    /**
      | This is invoked when the menu is clicked
      | or dismissed
      |
      */
    pub fn menu_invocation_callback(
        chosen_itemid: i32,
        _1:            *mut DemoTaskbarComponent<StatusItem,ImageType>)  {
        
        todo!();
        /*
            if (chosenItemID == 1)
                 ALOEApplication::getInstance()->systemRequestedQuit();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

             PopupMenu m;
             m.addItem (1, "Quit");

             // It's always better to open menus asynchronously when possible.
             m.showMenuAsync (PopupMenu::Options(),
                              ModalCallbackFunction::forComponent (menuInvocationCallback, this));
        */
    }
}
