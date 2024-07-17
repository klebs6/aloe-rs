crate::ix!();

/**
  | Enumeration used by the focusGained()
  | and focusLost() methods.
  |
  */
pub enum FocusChangeType
{
    /**
      | Means that the user clicked the mouse
      | to change focus.
      |
      */
    focusChangedByMouseClick,   

    /**
      | Means that the user pressed the tab key
      | to move the focus.
      |
      */
    focusChangedByTabKey,       

    /**
      | Means that the focus was changed by a
      | call to grabKeyboardFocus().
      |
      */
    focusChangedDirectly,        
}

pub trait FocusGained {

    /**
      | Called to indicate that this component
      | has just acquired the keyboard focus.
      | @see focusLost, setWantsKeyboardFocus,
      | getCurrentlyFocusedComponent, hasKeyboardFocus
      |
      */
    fn focus_gained(&mut self, cause: FocusChangeType);
}

pub trait FocusLost {

    /**
      | Called to indicate that this component
      | has just lost the keyboard focus. @see
      | focusGained, setWantsKeyboardFocus,
      | getCurrentlyFocusedComponent, hasKeyboardFocus
      |
      */
    fn focus_lost(&mut self, cause: FocusChangeType);
}

pub trait FocusOfChildComponentChanged {

    /**
      | Called to indicate a change in whether
      | or not this component is the parent of
      | the currently-focused component.
      | 
      | Essentially this is called when the
      | return value of a call to hasKeyboardFocus
      | (true) has changed. It happens when
      | focus moves from one of this component's
      | children (at any depth) to a component
      | that isn't contained in this one, (or
      | vice-versa).
      | 
      | -----------
      | @note
      | 
      | this method does NOT get called to when
      | focus simply moves from one of its child
      | components to another.
      | 
      | @see focusGained, setWantsKeyboardFocus,
      | getCurrentlyFocusedComponent, hasKeyboardFocus
      |
      */
    fn focus_of_child_component_changed(&mut self, cause: FocusChangeType);
}

/**
  | A focus container type that can be passed
  | to setFocusContainer().
  | 
  | If a component is marked as a focus container
  | or keyboard focus container then it
  | will act as the top-level component
  | within which focus or keyboard focus
  | is passed around. By default components
  | are considered "focusable" if they
  | are visible and enabled and "keyboard
  | focusable" if `getWantsKeyboardFocus()
  | == true`.
  | 
  | The order of traversal within a focus
  | container is determined by the objects
  | returned by createFocusTraverser()
  | and createKeyboardFocusTraverser(),
  | respectively - see the documentation
  | of the default FocusContainer and KeyboardFocusContainer
  | implementations for more information.
  |
  */
pub enum FocusContainerType 
{
    /**
      | The component will not act as a focus
      | container.
      | 
      | This is the default setting for non top-level
      | components and means that it and any
      | sub-components are navigable within
      | their containing focus container.
      |
      */
    none,

    /**
      | The component will act as a top-level
      | component within which focus is passed
      | around.
      | 
      | The default traverser implementation
      | returned by createFocusTraverser()
      | will use this flag to find the first parent
      | component (of the currently focused
      | one) that wants to be a focus container.
      | 
      | This is currently used when determining
      | the hierarchy of accessible UI elements
      | presented to screen reader clients
      | on supported platforms. See the AccessibilityHandler
      | class for more information.
      |
      */
    focusContainer,

    /**
      | The component will act as a top-level
      | component within which keyboard focus
      | is passed around.
      | 
      | The default traverser implementation
      | returned by createKeyboardFocusTraverser()
      | will use this flag to find the first parent
      | component (of the currently focused
      | one) that wants to be a keyboard focus
      | container.
      | 
      | This is currently used when determining
      | how keyboard focus is passed between
      | components that have been marked as
      | keyboard focusable with setWantsKeyboardFocus()
      | when clicking on components and navigating
      | with the tab key.
      |
      */
    keyboardFocusContainer
}

pub trait CreateFocusTraverser {

    /**
      | Creates a ComponentTraverser object
      | to determine the logic by which focus
      | should be passed from this component.
      | 
      | The default implementation of this
      | method will return an instance of FocusTraverser
      | if this component is a focus container
      | (as determined by the setFocusContainer()
      | method). If the component isn't a focus
      | container, then it will recursively
      | call createFocusTraverser() on its
      | parents.
      | 
      | If you override this to return a custom
      | traverser object, then this component
      | and all its sub-components will use
      | the new object to make their focusing
      | decisions.
      |
      */
    fn create_focus_traverser(&mut self) -> Box<dyn ComponentTraverser>;
}

pub trait CreateKeyboardFocusTraverser {

    /**
      | Creates a ComponentTraverser object
      | to use to determine the logic by which
      | keyboard focus should be passed from
      | this component.
      | 
      | The default implementation of this
      | method will return an instance of KeyboardFocusTraverser
      | if this component is a keyboard focus
      | container (as determined by the setFocusContainer()
      | method). If the component isn't a keyboard
      | focus container, then it will recursively
      | call createKeyboardFocusTraverser()
      | on its parents.
      | 
      | If you override this to return a custom
      | traverser object, then this component
      | and all its sub-components will use
      | the new object to make their keyboard
      | focusing decisions.
      |
      */
    fn create_keyboard_focus_traverser(&mut self) -> Box<dyn ComponentTraverser>;
}

pub trait IsFocused {

    /**
      | True if the window has the keyboard focus.
      |
      */
    fn is_focused(&self) -> bool;
}

pub trait GrabFocus {

    /**
      | Tries to give the window keyboard focus.
      |
      */
    fn grab_focus(&mut self);
}


