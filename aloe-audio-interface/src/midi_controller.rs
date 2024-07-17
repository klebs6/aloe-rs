crate::ix!();

pub trait IsController {

    /**
      | Returns true if this is a midi controller
      | message. @see getControllerNumber,
      | getControllerValue, controllerEvent
      |
      */
    fn is_controller(&self) -> bool;
}

pub trait IsControllerOfType {

    /** 
      | Returns true if this message is
      | a controller message and if it has the
      | specified
      | controller type.
      */
    fn is_controller_of_type(&self, controller_type: i32) -> bool;
}

pub trait GetControllerNumber {

    /**
      | Returns the controller number of a controller
      | message.
      | 
      | -----------
      | @note
      | 
      | The name of the controller can be looked
      | up using the getControllerName() method.
      | 
      | Note that the value returned is invalid
      | for messages that aren't controller
      | changes.
      | 
      | @see isController, getControllerName,
      | getControllerValue
      |
      */
    fn get_controller_number(&self) -> i32;
}

pub trait GetControllerValue {

    /**
      | Returns the controller value from a
      | controller message.
      | 
      | -----------
      | @note
      | 
      | A value 0 to 127 is returned to indicate
      | the new controller position.
      | 
      | Note that the value returned is invalid
      | for messages that aren't controller
      | changes.
      | 
      | @see isController, getControllerNumber
      |
      */
    fn get_controller_value(&self) -> i32;
}

pub trait ControllerEvent {

    /**
      | Creates a controller message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param controllerType
      | 
      | the type of controller
      | ----------
      | @param value
      | 
      | the controller value
      | 
      | @see isController
      |
      */
    fn controller_event(
        &mut self, 
        channel:         i32,
        controller_type: i32,
        value:           i32
    ) -> dyn MidiMessageInterface;
}
