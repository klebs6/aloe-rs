crate::ix!();

/**
  | Possible values for the justifySelf
  | property.
  |
  */
#[repr(i32)]
pub enum GridItemJustifySelf 
{
    /**
      | Content inside the item is justified
      | towards the left.
      |
      */
    start = 0,               

    /**
      | Content inside the item is justified
      | towards the right.
      |
      */
    end,                     

    /**
      | Content inside the item is justified
      | towards the center.
      |
      */
    center,                  

    /**
      | Content inside the item is stretched
      | from left to right.
      |
      */
    stretch,                 

    /**
      | Follows the Grid container's justifyItems
      | property.
      |
      */
    autoValue,                
}
