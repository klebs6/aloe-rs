crate::ix!();

pub trait LookAndFeelChanged {

    /**
      | Called to let the component react to
      | a change in the look-and-feel setting.
      | 
      | When the look-and-feel is changed for
      | a component, this will be called in all
      | its child components, recursively.
      | 
      | It can also be triggered manually by
      | the sendLookAndFeelChange() method,
      | in case an application uses a LookAndFeel
      | class that might have changed internally.
      | 
      | @see sendLookAndFeelChange, getLookAndFeel
      |
      */
    fn look_and_feel_changed(&mut self);
}

pub trait ColourChanged {

    /**
      | This method is called when a colour is
      | changed by the setColour() method.
      | @see setColour, findColour
      |
      */
    fn colour_changed(&mut self);
}
