crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_ModifierKeys.h]

/**
  | Represents the state of the mouse buttons
  | and modifier keys.
  | 
  | This is used both by mouse events and
  | by KeyPress objects to describe the
  | state of keys such as shift, control,
  | alt, etc.
  | 
  | @see KeyPress, MouseEvent::mods
  | 
  | @tags{GUI}
  |
  */
#[derive(Copy,Clone)]
pub struct ModifierKeys {
    flags: i32, // default = 0
}

impl Default for ModifierKeys {
    
    /**
      | Creates a ModifierKeys object with
      | no flags set.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

/**
  | Command key flag - on windows this is
  | the same as the CTRL key flag.
  |
  */
#[cfg(any(target_os="macos",target_os="ios"))]
pub const COMMAND_MODIFIER: u32 = 8;

/**
  | Command key flag - on windows this is
  | the same as the CTRL key flag.
  |
  */
#[cfg(not(any(target_os="macos",target_os="ios")))]
pub const COMMAND_MODIFIER: u32 = ModifierKeysFlags::ctrlModifier.bits;

/**
  | Popup menu flag - on windows this is the
  | same as rightButtonModifier, on the
  | Mac it's the same as (rightButtonModifier
  | | ctrlModifier).
  |
  */
#[cfg(any(target_os="macos",target_os="ios"))]
pub const POPUP_MENU_CLICK_MODIFIER: u32 = 
ModifierKeysFlags::rightButtonModifier.bits() 
    | ModifierKeysFlags::ctrlModifier.bits();

/**
  | Popup menu flag - on windows this is the
  | same as rightButtonModifier, on the
  | 
  | Mac it's the same as (rightButtonModifier
  | | ctrlModifier).
  |
  */
#[cfg(not(any(target_os="macos",target_os="ios")))]
pub const POPUP_MENU_CLICK_MODIFIER: u32 = ModifierKeysFlags::rightButtonModifier.bits();

bitflags!{

    /**
      | Flags that represent the different
      | keys.
      |
      */
    pub struct ModifierKeysFlags: u32
    {
        /**
          | Indicates no modifier keys.
          |
          */
        const noModifiers            = 0;

        /**
          | Shift key flag.
          |
          */
        const shiftModifier          = 1;

        /**
          | CTRL key flag.
          |
          */
        const ctrlModifier           = 2;

        /**
          | ALT key flag.
          |
          */
        const altModifier            = 4;

        /**
          | Left mouse button flag.
          |
          */
        const leftButtonModifier     = 16;

        /**
          | Right mouse button flag.
          |
          */
        const rightButtonModifier    = 32;

        /**
          | Middle mouse button flag.
          |
          */
        const middleButtonModifier   = 64;

        const commandModifier        = COMMAND_MODIFIER;

        const popupMenuClickModifier = POPUP_MENU_CLICK_MODIFIER;

        /**
          | Represents a combination of all the
          | shift, alt, ctrl and command key modifiers.
          |
          */
        const allKeyboardModifiers = 
            Self::shiftModifier.bits() 
            | Self::ctrlModifier.bits() 
            | Self::altModifier.bits() 
            | Self::commandModifier.bits();

        /**
          | Represents a combination of all the
          | mouse buttons at once.
          |
          */
        const allMouseButtonModifiers = 
            Self::leftButtonModifier.bits() 
            | Self::rightButtonModifier.bits() 
            | Self::middleButtonModifier.bits();

        /**
          | Represents a combination of all the
          | alt, ctrl and command key modifiers.
          |
          */
        const ctrlAltCommandModifiers = 
            Self::ctrlModifier.bits() 
            | Self::altModifier.bits() 
            | Self::commandModifier.bits();
    }

}

pub mod modifier_keys {

    use super::*;

    /**
      | This object represents the last-known
      | state of the keyboard and mouse buttons.
      |
      */
    lazy_static!{
        /*
        static ModifierKeys currentModifiers;
        ModifierKeys ModifierKeys::currentModifiers;
        */
    }
}

impl PartialEq<ModifierKeys> for ModifierKeys {
    
    #[inline] fn eq(&self, other: &ModifierKeys) -> bool {
        todo!();
        /*
            return flags == other.flags;
        */
    }
}

impl Eq for ModifierKeys {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_ModifierKeys.cpp]
impl ModifierKeys {

    /**
      | Creates a ModifierKeys object from
      | a raw set of flags.
      | 
      | -----------
      | @param flags
      | 
      | to represent the keys that are down @see
      | shiftModifier, ctrlModifier, altModifier,
      | leftButtonModifier, rightButtonModifier,
      | commandModifier, popupMenuClickModifier
      |
      */
    pub fn new(raw_flags: i32) -> Self {
    
        todo!();
        /*
        : flags(rawFlags),
        */
    }

    /**
      | Checks whether the 'command' key flag
      | is set (or 'ctrl' on Windows/Linux).
      | 
      | This is a platform-agnostic way of checking
      | for the operating system's preferred
      | command-key modifier - so on the Mac
      | it tests for the cmd key, on
      | 
      | Windows/Linux, it's actually checking
      | for the CTRL key.
      |
      */
    #[inline] pub fn is_command_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (commandModifier);
        */
    }

    /**
      | Checks whether the user is trying to
      | launch a pop-up menu.
      | 
      | This checks for platform-specific
      | modifiers that might indicate that
      | the user is following the operating
      | system's normal method of showing a
      | pop-up menu.
      | 
      | So on Windows/Linux, this method is
      | really testing for a right-click.
      | 
      | On the Mac, it tests for either the CTRL
      | key being down, or a right-click.
      |
      */
    #[inline] pub fn is_popup_menu(&self) -> bool {
        
        todo!();
        /*
            return testFlags (popupMenuClickModifier);
        */
    }

    /**
      | Checks whether the flag is set for the
      | left mouse-button.
      |
      */
    #[inline] pub fn is_left_button_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (leftButtonModifier);
        */
    }

    /**
      | Checks whether the flag is set for the
      | right mouse-button.
      | 
      | -----------
      | @note
      | 
      | for detecting popup-menu clicks, you
      | should be using isPopupMenu() instead,
      | as this is platform-independent (and
      | makes your code more explanatory too).
      |
      */
    #[inline] pub fn is_right_button_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (rightButtonModifier);
        */
    }
    
    #[inline] pub fn is_middle_button_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (middleButtonModifier);
        */
    }

    /**
      | Tests for any of the mouse-button flags.
      |
      */
    #[inline] pub fn is_any_mouse_button_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (allMouseButtonModifiers);
        */
    }

    /**
      | Tests for any of the modifier key flags.
      |
      */
    #[inline] pub fn is_any_modifier_key_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags ((shiftModifier | ctrlModifier | altModifier | commandModifier));
        */
    }

    /**
      | Checks whether the shift key's flag
      | is set.
      |
      */
    #[inline] pub fn is_shift_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (shiftModifier);
        */
    }

    /**
      | Checks whether the CTRL key's flag is
      | set.
      | 
      | Remember that it's better to use the
      | platform-agnostic routines to test
      | for command-key and popup-menu modifiers.
      | 
      | @see isCommandDown, isPopupMenu
      |
      */
    #[inline] pub fn is_ctrl_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (ctrlModifier);
        */
    }

    /**
      | Checks whether the ALT key's flag is
      | set.
      |
      */
    #[inline] pub fn is_alt_down(&self) -> bool {
        
        todo!();
        /*
            return testFlags (altModifier);
        */
    }
    
    /**
      | Returns a copy of only the mouse-button
      | flags
      |
      */
    pub fn with_only_mouse_buttons(&self) -> ModifierKeys {
        
        todo!();
        /*
            return ModifierKeys (flags & allMouseButtonModifiers);
        */
    }

    /**
      | Returns a copy of only the non-mouse
      | flags
      |
      */
    pub fn without_mouse_buttons(&self) -> ModifierKeys {
        
        todo!();
        /*
            return ModifierKeys (flags & ~allMouseButtonModifiers);
        */
    }

    
    /**
      | Returns the raw flags for direct testing.
      |
      */
    #[inline] pub fn get_raw_flags(&self) -> i32 {
        
        todo!();
        /*
            return flags;
        */
    }
    
    pub fn without_flags(&self, raw_flags_to_clear: i32) -> ModifierKeys {
        
        todo!();
        /*
            return ModifierKeys (flags & ~rawFlagsToClear);
        */
    }
    
    pub fn with_flags(&self, raw_flags_to_set: i32) -> ModifierKeys {
        
        todo!();
        /*
            return ModifierKeys (flags | rawFlagsToSet);
        */
    }

    /**
      | Tests a combination of flags and returns
      | true if any of them are set.
      |
      */
    pub fn test_flags(&self, flags_to_test: i32) -> bool {
        
        todo!();
        /*
            return (flags & flagsToTest) != 0;
        */
    }

    /**
      | Creates a ModifierKeys object to represent
      | the last-known state of the keyboard
      | and mouse buttons.
      | 
      | This method is here for backwards compatibility
      | and there's no need to call it anymore,
      | you should use the public currentModifiers
      | member directly.
      |
      */
    pub fn get_current_modifiers() -> ModifierKeys {
        
        todo!();
        /*
            return currentModifiers;
        */
    }

    
    /**
      | Returns the total number of mouse buttons
      | that are down.
      |
      */
    pub fn get_num_mouse_buttons_down(&self) -> i32 {
        
        todo!();
        /*
            int num = 0;

        if (isLeftButtonDown())     ++num;
        if (isRightButtonDown())    ++num;
        if (isMiddleButtonDown())   ++num;

        return num;
        */
    }
    
    /**
      | Creates a ModifierKeys object to represent
      | the current state of the keyboard and
      | mouse buttons.
      | 
      | This method is here for backwards compatibility
      | and you should call ComponentPeer::getCurrentModifiersRealtime()
      | instead (which is what this method now
      | does).
      |
      */
    pub fn get_current_modifiers_realtime(&mut self) -> ModifierKeys {
        
        todo!();
        /*
            return ComponentPeer::getCurrentModifiersRealtime();
        */
    }
}
