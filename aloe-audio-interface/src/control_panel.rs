crate::ix!();

pub trait HasControlPanel {

    /**
      | True if this device can show a pop-up
      | control panel for editing its settings.
      | 
      | This is generally just true of ASIO devices.
      | If true, you can call showControlPanel()
      | to display it.
      |
      */
    fn has_control_panel(&self) -> bool { false }
}

pub trait ShowControlPanel {

    /**
      | Shows a device-specific control panel
      | if there is one.
      | 
      | This should only be called for devices
      | which return true from hasControlPanel().
      |
      */
    fn show_control_panel(&mut self) -> bool {
        
        todo!();
        /*
        jassertfalse;    // this should only be called for devices which return true from
                         // their hasControlPanel() method.
        return false;
        
        */
    }
}
