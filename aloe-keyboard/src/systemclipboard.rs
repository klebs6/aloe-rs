crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_SystemClipboard.h]

/**
  | Handles reading/writing to the system's
  | clipboard.
  | 
  | @tags{GUI}
  |
  */
pub struct SystemClipboard {

}

impl SystemClipboard {

    /**
      | Copies a string of text onto the clipboard
      |
      */
    pub fn copy_text_to_clipboard(text: &String)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Gets the current clipboard's contents.
      | 
      | Obviously this might have come from
      | another app, so could contain anything..
      |
      */
    pub fn get_text_from_clipboard() -> String {
        
        todo!();
        /*
        
        */
    }

    #[cfg(target_os="linux")]
    pub fn copy_text_to_clipboard(&mut self, clip_text: &String)  {
        
        todo!();
        /*
            XWindowSystem::getInstance()->copyTextToClipboard (clipText);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn get_text_from_clipboard(&mut self) -> String {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->getTextFromClipboard();
        */

    }
}
