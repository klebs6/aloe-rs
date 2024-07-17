crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/aloe_AccessibilityState.h]

/**
  | Represents the state of an accessible
  | UI element.
  | 
  | An instance of this class is returned
  | by `AccessibilityHandler::getCurrentState()`
  | to convey its current state to an accessibility
  | client.
  | 
  | @see AccessibilityHandler
  | 
  | @tags{Accessibility}
  |
  */
pub struct AccessibleState {
    flags: i32, // default = 0
}

pub mod accessible_state {
    use super::*;
    pub enum Flags
    {
        checkable           = 1 << 0,
        checked             = 1 << 1,
        collapsed           = 1 << 2,
        expandable          = 1 << 3,
        expanded            = 1 << 4,
        focusable           = 1 << 5,
        focused             = 1 << 6,
        ignored             = 1 << 7,
        multiSelectable     = 1 << 8,
        selectable          = 1 << 9,
        selected            = 1 << 10,
        accessibleOffscreen = 1 << 11
    }
}

impl Default for AccessibleState {
    
    /**
      | Constructor.
      | 
      | Represents a "default" state with no
      | flags set. To set a flag, use one of the
      | `withX()` methods - these can be chained
      | together to set multiple flags.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl AccessibleState {

    /**
      | Sets the checkable flag and returns
      | the new state.
      | 
      | @see isCheckable
      |
      */
    pub fn with_checkable(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::checkable);
        */
    }

    /**
      | Sets the checked flag and returns the
      | new state.
      | 
      | @see isChecked
      |
      */
    pub fn with_checked(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::checked);
        */
    }

    /**
      | Sets the collapsed flag and returns
      | the new state.
      | 
      | @see isCollapsed
      |
      */
    pub fn with_collapsed(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::collapsed);
        */
    }

    /**
      | Sets the expandable flag and returns
      | the new state.
      | 
      | @see isExpandable
      |
      */
    pub fn with_expandable(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::expandable);
        */
    }

    /**
      | Sets the expanded flag and returns the
      | new state.
      | 
      | @see isExpanded
      |
      */
    pub fn with_expanded(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::expanded);
        */
    }

    /**
      | Sets the focusable flag and returns
      | the new state.
      | 
      | @see isFocusable
      |
      */
    pub fn with_focusable(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::focusable);
        */
    }

    /**
      | Sets the focused flag and returns the
      | new state.
      | 
      | @see isFocused
      |
      */
    pub fn with_focused(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::focused);
        */
    }

    /**
      | Sets the ignored flag and returns the
      | new state.
      | 
      | @see isIgnored
      |
      */
    pub fn with_ignored(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::ignored);
        */
    }

    /**
      | Sets the selectable flag and returns
      | the new state.
      | 
      | @see isSelectable
      |
      */
    pub fn with_selectable(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::selectable);
        */
    }

    /**
      | Sets the multiSelectable flag and returns
      | the new state.
      | 
      | @see isMultiSelectable
      |
      */
    pub fn with_multi_selectable(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::multiSelectable);
        */
    }

    /**
      | Sets the selected flag and returns the
      | new state.
      | 
      | @see isSelected
      |
      */
    pub fn with_selected(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::selected);
        */
    }

    /**
      | Sets the accessible offscreen flag
      | and returns the new state.
      | 
      | @see isSelected
      |
      */
    pub fn with_accessible_offscreen(&self) -> AccessibleState {
        
        todo!();
        /*
            return withFlag (Flags::accessibleOffscreen);
        */
    }

    
    /**
      | Returns true if the UI element is checkable.
      | 
      | @see withCheckable
      |
      */
    pub fn is_checkable(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::checkable);
        */
    }

    /**
      | Returns true if the UI element is checked.
      | 
      | @see withChecked
      |
      */
    pub fn is_checked(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::checked);
        */
    }

    /**
      | Returns true if the UI element is collapsed.
      | 
      | @see withCollapsed
      |
      */
    pub fn is_collapsed(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::collapsed);
        */
    }

    /**
      | Returns true if the UI element is expandable.
      | 
      | @see withExpandable
      |
      */
    pub fn is_expandable(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::expandable);
        */
    }

    /**
      | Returns true if the UI element is expanded.
      | 
      | @see withExpanded
      |
      */
    pub fn is_expanded(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::expanded);
        */
    }

    /**
      | Returns true if the UI element is focusable.
      | 
      | @see withFocusable
      |
      */
    pub fn is_focusable(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::focusable);
        */
    }

    /**
      | Returns true if the UI element is focused.
      | 
      | @see withFocused
      |
      */
    pub fn is_focused(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::focused);
        */
    }

    /**
      | Returns true if the UI element is ignored.
      | 
      | @see withIgnored
      |
      */
    pub fn is_ignored(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::ignored);
        */
    }

    /**
      | Returns true if the UI element supports
      | multiple item selection.
      | 
      | @see withMultiSelectable
      |
      */
    pub fn is_multi_selectable(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::multiSelectable);
        */
    }

    /**
      | Returns true if the UI element is selectable.
      | 
      | @see withSelectable
      |
      */
    pub fn is_selectable(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::selectable);
        */
    }

    /**
      | Returns true if the UI element is selected.
      | 
      | @see withSelected
      |
      */
    pub fn is_selected(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::selected);
        */
    }

    /**
      | Returns true if the UI element is accessible
      | offscreen.
      | 
      | @see withSelected
      |
      */
    pub fn is_accessible_offscreen(&self) -> bool {
        
        todo!();
        /*
            return isFlagSet (Flags::accessibleOffscreen);
        */
    }
    
    pub fn with_flag(&self, flag: i32) -> AccessibleState {
        
        todo!();
        /*
            auto copy = *this;
            copy.flags |= flag;

            return copy;
        */
    }
    
    pub fn is_flag_set(&self, flag: i32) -> bool {
        
        todo!();
        /*
            return (flags & flag) != 0;
        */
    }
}

