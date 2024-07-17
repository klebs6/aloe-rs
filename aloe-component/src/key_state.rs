crate::ix!();

pub trait KeyPressed {

    /**
      | Called when a key is pressed.
      | 
      | When a key is pressed, the component
      | that has the keyboard focus will have
      | this method called. Remember that a
      | component will only be given the focus
      | if its setWantsKeyboardFocus() method
      | has been used to enable this.
      | 
      | If your implementation returns true,
      | the event will be consumed and not passed
      | on to any other listeners. If it returns
      | false, the key will be passed to any KeyListeners
      | that have been registered with this
      | component. As soon as one of these returns
      | true, the process will stop, but if they
      | all return false, the event will be passed
      | upwards to this component's parent,
      | and so on.
      | 
      | The default implementation of this
      | method does nothing and returns false.
      | 
      | @see keyStateChanged, getCurrentlyFocusedComponent,
      | addKeyListener
      |
      */
    fn key_pressed(&mut self, key: &KeyPress) -> bool;
}

pub trait KeyStateChanged {

    /**
      | Called when a key is pressed or released.
      | 
      | Whenever a key on the keyboard is pressed
      | or released (including modifier keys
      | like shift and ctrl), this method will
      | be called on the component that currently
      | has the keyboard focus. Remember that
      | a component will only be given the focus
      | if its setWantsKeyboardFocus() method
      | has been used to enable this.
      | 
      | If your implementation returns true,
      | the event will be consumed and not passed
      | on to any other listeners. If it returns
      | false, then any KeyListeners that have
      | been registered with this component
      | will have their keyStateChanged methods
      | called. As soon as one of these returns
      | true, the process will stop, but if they
      | all return false, the event will be passed
      | upwards to this component's parent,
      | and so on.
      | 
      | The default implementation of this
      | method does nothing and returns false.
      | 
      | To find out which keys are up or down at
      | any time, see the KeyPress::isKeyCurrentlyDown()
      | method.
      | 
      | -----------
      | @param isKeyDown
      | 
      | true if a key has been pressed; false
      | if it has been released
      | 
      | @see keyPressed, KeyPress, getCurrentlyFocusedComponent,
      | addKeyListener
      |
      */
    fn key_state_changed(&mut self, is_key_down: bool) -> bool;
}

pub trait ModifierKeysChanged {

    /**
      | Called when a modifier key is pressed
      | or released.
      | 
      | Whenever the shift, control, alt or
      | command keys are pressed or released,
      | this method will be called.
      | 
      | The component that is currently under
      | the main mouse pointer will be tried
      | first and, if there is no component currently
      | under the pointer, the component that
      | currently has the keyboard focus will
      | have this method called. Remember that
      | a component will only be given the focus
      | if its setWantsKeyboardFocus() method
      | has been used to enable this.
      | 
      | The default implementation of this
      | method actually calls its parent's
      | modifierKeysChanged method, so that
      | focused components which aren't interested
      | in this will give their parents a chance
      | to act on the event instead.
      | 
      | @see keyStateChanged, ModifierKeys
      |
      */
    fn modifier_keys_changed(&mut self, modifiers: &ModifierKeys);
}
