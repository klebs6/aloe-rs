crate::ix!();

/**
  | Contains status information about
  | a pen event.
  | 
  | @see MouseListener, MouseEvent
  | 
  | @tags{GUI}
  |
  */
pub struct PenDetails
{
    /**
      | The rotation of the pen device in radians.
      | Indicates the clockwise rotation,
      | or twist, of the pen. The default is 0.
      |
      */
    rotation: f32,

    /**
      | Indicates the angle of tilt of the pointer
      | in a range of -1.0 to 1.0 along the x-axis
      | where a positive value indicates a tilt
      | to the right. The default is 0.
      |
      */
    tiltx:    f32,

    /**
      | Indicates the angle of tilt of the pointer
      | in a range of -1.0 to 1.0 along the y-axis
      | where a positive value indicates a tilt
      | toward the user. The default is 0.
      |
      */
    tilty:    f32,
}

