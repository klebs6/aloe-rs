crate::ix!();

/**
  | To demonstrate how sliders can have
  | custom snapping applied to their values,
  | this simple class snaps the value to
  | 50 if it comes near.
  |
  */
pub struct SnappingSlider<'a> {
    base: Slider<'a>,
}

impl<'a> SnappingSlider<'a> {

    pub fn snap_value(
        &mut self, 
        attempted_value: f64,
        drag_mode:       SliderDragMode

    ) -> f64 {
        
        todo!();
        /*
            if (dragMode == notDragging)
                return attemptedValue;  // if they're entering the value in the text-box, don't mess with it.

            if (attemptedValue > 40 && attemptedValue < 60)
                return 50.0;

            return attemptedValue;
        */
    }
}
