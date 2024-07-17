crate::ix!();

pub trait SliderListener: 
    SliderValueChanged 
    + SliderDragStarted 
    + SliderDragEnded { }

/**
  | A class for receiving callbacks from
  | a Slider.
  | 
  | To be told when a slider's value changes,
  | you can register a Slider::SliderListener
  | object using Slider::addListener().
  | 
  | @see Slider::addListener, Slider::removeListener
  |
  */
pub trait SliderValueChanged {

    /**
      | Called when the slider's value is changed.
      | 
      | This may be caused by dragging it, or
      | by typing in its text entry box, or by
      | a call to Slider::setValue().
      | 
      | You can find out the new value using Slider::getValue().
      | 
      | @see Slider::valueChanged
      |
      */
    fn slider_value_changed(&mut self, slider: *mut Slider);
}

pub trait SliderDragStarted {

    /**
      | Called when the slider is about to be
      | dragged.
      | 
      | This is called when a drag begins, then
      | it's followed by multiple calls to sliderValueChanged(),
      | and then sliderDragEnded() is called
      | after the user lets go.
      | 
      | @see sliderDragEnded, Slider::startedDragging
      |
      */
    fn slider_drag_started(&mut self, _0: *mut Slider);
}

pub trait SliderDragEnded {

    /**
      | Called after a drag operation has finished.
      | @see sliderDragStarted, Slider::stoppedDragging
      |
      */
    fn slider_drag_ended(&mut self, _0: *mut Slider);
}
