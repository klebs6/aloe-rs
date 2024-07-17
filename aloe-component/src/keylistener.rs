crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_KeyListener.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_KeyListener.cpp]

/**
  | Receives callbacks when keys are pressed.
  |
  | You can add a key listener to a component to
  | be informed when that component gets key
  | events. See the Component::addListener method
  | for more details.
  |
  | @see KeyPress, Component::addKeyListener,
  | KeyPressMappingSet
  |
  | @tags{GUI}
  */
pub trait KeyListener {
    
    /**
      | 
      | Called to indicate that a key has been
      | pressed.
      | 
      | If your implementation returns true,
      | then the key event is considered to have
      | 
      | been consumed, and will not be passed
      | on to any other components. If it returns
      | 
      | false, then the key will be passed to
      | other components that might want to
      | use it.
      | 
      | -----------
      | @param key
      | 
      | the keystroke, including modifier
      | keys
      | ----------
      | @param originatingComponent
      | 
      | the component that received the key
      | event
      | 
      | @see keyStateChanged, Component::keyPressed
      |
      */
    fn key_pressed(&mut self, 
        key:                   &KeyPress,
        originating_component: *mut Component) -> bool;

    /**
      | Called when any key is pressed or released.
      | 
      | When this is called, classes that might
      | be interested in
      | 
      | the state of one or more keys can use 
      | KeyPress::isKeyCurrentlyDown() to
      | 
      | check whether their key has changed.
      | 
      | If your implementation returns true,
      | then the key event is considered to have
      | 
      | been consumed, and will not be passed
      | on to any other components. If it returns
      | 
      | false, then the key will be passed to
      | other components that might want to
      | use it.
      | 
      | -----------
      | @param originatingComponent
      | 
      | the component that received the key
      | event
      | ----------
      | @param isKeyDown
      | 
      | true if a key is being pressed, false
      | if one is being released
      | 
      | @see KeyPress, Component::keyStateChanged
      |
      */
    fn key_state_changed(&mut self, 
        is_key_down:           bool,
        originating_component: *mut Component) -> bool 
    {
        false
    }
}
