crate::ix!();

/**
  | An RAII class for sending slider listener
  | drag messages.
  | 
  | This is useful if you are programatically
  | updating the slider's value and want
  | to imitate a mouse event, for example
  | in a custom AccessibilityHandler.
  | 
  | @see Slider::SliderListener
  |
  */
#[no_copy]
#[no_move]
pub struct SliderScopedDragNotification<'a> {
    slider_being_dragged: &'a mut Slider<'a>,
}

impl<'a> Drop for SliderScopedDragNotification<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            sliderBeingDragged.impl->sendDragEnd();
        */
    }
}

impl<'a> SliderScopedDragNotification<'a> {

    pub fn new(s: &mut Slider) -> Self {
    
        todo!();
        /*
        : slider_being_dragged(s),

            sliderBeingDragged.impl->sendDragStart();
        */
    }
}
