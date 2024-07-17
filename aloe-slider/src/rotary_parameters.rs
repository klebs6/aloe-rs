crate::ix!();

/**
  | Structure defining rotary parameters
  | for a slider
  |
  */
pub struct SliderRotaryParameters
{

    /**
      | The angle (in radians, clockwise from
      | the top) at which the slider's minimum
      | value is represented.
      |
      */
    start_angle_radians: f32,


    /**
      | The angle (in radians, clockwise from
      | the top) at which the slider's maximum
      | value is represented. This must be greater
      | than startAngleRadians.
      |
      */
    end_angle_radians:   f32,


    /**
      | Determines what happens when a circular
      | drag action rotates beyond the minimum
      | or maximum angle. If true, the value
      | will stop changing until the mouse moves
      | back the way it came; if false, the value
      | will snap back to the value nearest to
      | the mouse. Note that this has no effect
      | if the drag mode is vertical or horizontal.
      |
      */
    stop_at_end:         bool,
}
