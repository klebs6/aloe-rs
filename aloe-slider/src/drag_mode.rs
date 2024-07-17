crate::ix!();

/**
  | Describes the type of mouse-dragging
  | that is happening when a value is being
  | changed. @see snapValue
  |
  */
pub enum SliderDragMode
{
    /**
      | Dragging is not active.
      |
      */
    notDragging,            

    /**
      | The dragging corresponds directly
      | to the value that is displayed.
      |
      */
    absoluteDrag,           

    /**
      | The dragging value change is relative
      | to the velocity of the mouse movement.
      |
      */
    velocityDrag,            
}
