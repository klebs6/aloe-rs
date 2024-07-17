crate::ix!();

/**
  | Contains status information about
  | a mouse wheel event.
  | 
  | @see MouseListener, MouseEvent
  | 
  | @tags{GUI}
  |
  */
pub struct MouseWheelDetails {

    /**
      | The amount that the wheel has been moved
      | in the X axis.
      | 
      | If isReversed is true, then a negative
      | deltaX means that the wheel has been
      | pushed physically to the left.
      | 
      | If isReversed is false, then a negative
      | deltaX means that the wheel has been
      | pushed physically to the right.
      |
      */
    deltax:      f32,

    /**
      | The amount that the wheel has been moved
      | in the Y axis.
      | 
      | If isReversed is true, then a negative
      | deltaY means that the wheel has been
      | pushed physically upwards.
      | 
      | If isReversed is false, then a negative
      | deltaY means that the wheel has been
      | pushed physically downwards.
      |
      */
    deltay:      f32,

    /**
      | Indicates whether the user has reversed
      | the direction of the wheel.
      | 
      | See deltaX and deltaY for an explanation
      | of the effects of this value.
      |
      */
    is_reversed: bool,

    /**
      | If true, then the wheel has continuous,
      | un-stepped motion.
      |
      */
    is_smooth:   bool,

    /**
      | If true, then this event is part of the
      | inertial momentum phase that follows
      | the wheel being released.
      |
      */
    is_inertial: bool,
}


