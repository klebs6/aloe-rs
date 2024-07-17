crate::ix!();

pub trait SliderInterface:
    StartedDragging 
    + StoppedDragging 
    + ValueChanged 
    + GetValueFromText 
    + GetTextFromValue 
    + ProportionOfLengthToValue 
    + ValueToProportionOfLength 
    + SnapValue  {}

pub trait StartedDragging {

    /**
      | Callback to indicate that the user is
      | about to start dragging the slider.
      | @see Slider::SliderListener::sliderDragStarted
      |
      */
    fn started_dragging(&mut self);
}

pub trait StoppedDragging {

    /**
      | Callback to indicate that the user has
      | just stopped dragging the slider. @see
      | Slider::SliderListener::sliderDragEnded
      |
      */
    fn stopped_dragging(&mut self);

}

pub trait ValueChanged {

    /**
      | Callback to indicate that the user has
      | just moved the slider. @see Slider::SliderListener::sliderValueChanged
      |
      */
    fn value_changed(&mut self);
}

pub trait GetValueFromText {

    /**
      | Subclasses can override this to convert
      | a text string to a value.
      | 
      | When the user enters something into
      | the text-entry box, this method is called
      | to convert it to a value.
      | 
      | The default implementation just tries
      | to convert it to a double.
      | 
      | @see getTextFromValue
      |
      */
    fn get_value_from_text(&mut self, text: &String) -> f64;
}

pub trait GetTextFromValue {

    /**
      | Turns the slider's current value into
      | a text string.
      | 
      | Subclasses can override this to customise
      | the formatting of the text-entry box.
      | 
      | The default implementation just turns
      | the value into a string, using a number
      | of decimal places based on the range
      | interval. If a suffix string has been
      | set using setTextValueSuffix(), this
      | will be appended to the text.
      | 
      | @see getValueFromText
      |
      */
    fn get_text_from_value(&mut self, value: f64) -> String;
}

pub trait ProportionOfLengthToValue {

    /**
      | Allows a user-defined mapping of distance
      | along the slider to its value.
      | 
      | The default implementation for this
      | performs the skewing operation that
      | can be set up in the setSkewFactor()
      | method. Override it if you need some
      | kind of custom mapping instead, but
      | make sure you also implement the inverse
      | function in valueToProportionOfLength().
      | 
      | -----------
      | @param proportion
      | 
      | a value 0 to 1.0, indicating a distance
      | along the slider
      | 
      | -----------
      | @return
      | 
      | the slider value that is represented
      | by this position @see valueToProportionOfLength
      |
      */
    fn proportion_of_length_to_value(&mut self, proportion: f64) -> f64;

}

pub trait ValueToProportionOfLength {

    /**
      | Allows a user-defined mapping of value
      | to the position of the slider along its
      | length.
      | 
      | The default implementation for this
      | performs the skewing operation that
      | can be set up in the setSkewFactor()
      | method. Override it if you need some
      | kind of custom mapping instead, but
      | make sure you also implement the inverse
      | function in proportionOfLengthToValue().
      | 
      | -----------
      | @param value
      | 
      | a valid slider value, between the range
      | of values specified in setRange()
      | 
      | -----------
      | @return
      | 
      | a value 0 to 1.0 indicating the distance
      | along the slider that represents this
      | value @see proportionOfLengthToValue
      |
      */
    fn value_to_proportion_of_length(&mut self, value: f64) -> f64;
}

pub trait SnapValue {

    /**
      | This can be overridden to allow the slider
      | to snap to user-definable values.
      | 
      | If overridden, it will be called when
      | the user tries to move the slider to a
      | given position, and allows a subclass
      | to sanity-check this value, possibly
      | returning a different value to use instead.
      | 
      | -----------
      | @param attemptedValue
      | 
      | the value the user is trying to enter
      | ----------
      | @param dragMode
      | 
      | indicates whether the user is dragging
      | with the mouse; notDragging if they
      | are entering the value using the text
      | box or other non-dragging interaction
      | 
      | -----------
      | @return
      | 
      | the value to use instead
      |
      */
    fn snap_value(&mut self, 
            attempted_value: f64,
            drag_mode:       SliderDragMode) -> f64;
}

