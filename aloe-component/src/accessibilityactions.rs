crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/enums/aloe_AccessibilityActions.h]

/**
  | An action that can be performed by an
  | accessible UI element.
  | 
  | @tags{Accessibility}
  |
  */
pub enum AccessibilityActionType
{
    /**
      | Represents a "press" action.
      | 
      | This will be called when the user "clicks"
      | the UI element using an accessibility
      | client.
      |
      */
    press,

    /**
      | Represents a "toggle" action.
      | 
      | This will be called when the user toggles
      | the state of a UI element, for example
      | a toggle button or the selection of a
      | list item.
      |
      */
    toggle,

    /**
      | Indicates that the UI element has received
      | focus.
      | 
      | This will be called when a UI element
      | receives focus from an accessibility
      | client, or keyboard focus from the application.
      |
      */
    focus,

    /**
      | Represents the user showing a contextual
      | menu for a UI element.
      | 
      | This will be called for UI elements which
      | expand and collapse to show contextual
      | information or menus, or show a popup.
      |
      */
    showMenu
}

/**
  | A simple wrapper for building a collection
  | of supported accessibility actions
  | and corresponding callbacks for a UI
  | element.
  | 
  | Pass one of these when constructing
  | an `AccessibilityHandler` to enable
  | users to interact with a UI element via
  | the supported actions.
  | 
  | @tags{Accessibility}
  |
  */
pub struct AccessibilityActions {
    action_map: HashMap<AccessibilityActionType,fn() -> ()>,
}

impl Default for AccessibilityActions {
    
    /**
      | Constructor.
      | 
      | Creates a default AccessibilityActions
      | object with no action callbacks.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl AccessibilityActions {

    /**
      | Adds an action.
      | 
      | When the user performs this action with
      | an accessibility client `actionCallback`
      | will be called.
      | 
      | Returns a reference to itself so that
      | several calls can be chained.
      |
      */
    pub fn add_action(&mut self, 
        ty:              AccessibilityActionType,
        action_callback: fn() -> ()) -> &mut AccessibilityActions {
        
        todo!();
        /*
            actionMap[type] = std::move (actionCallback);
            return *this;
        */
    }

    /**
      | Returns true if the specified action
      | is supported.
      |
      */
    pub fn contains(&self, ty: AccessibilityActionType) -> bool {
        
        todo!();
        /*
            return actionMap.find (type) != actionMap.end();
        */
    }

    /**
      | If an action has been registered for
      | the provided action type, invokes the
      | action and returns true. Otherwise,
      | returns false.
      |
      */
    pub fn invoke(&self, ty: AccessibilityActionType) -> bool {
        
        todo!();
        /*
            auto iter = actionMap.find (type);

            if (iter == actionMap.end())
                return false;

            iter->second();
            return true;
        */
    }
}
