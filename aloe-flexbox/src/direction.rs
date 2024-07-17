crate::ix!();

/**
  | Possible values for the flexDirection
  | property.
  |
  */
pub enum FlexBoxDirection
{
    /**
      | Set the main axis direction from left
      | to right.
      |
      */
    row,                  

    /**
      | Set the main axis direction from right
      | to left.
      |
      */
    rowReverse,           

    /**
      | Set the main axis direction from top
      | to bottom.
      |
      */
    column,               

    /**
      | Set the main axis direction from bottom
      | to top.
      |
      */
    columnReverse,         
}
