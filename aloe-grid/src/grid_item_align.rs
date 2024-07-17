crate::ix!();

/**
  | Possible values for the alignSelf property.
  |
  */
#[repr(i32)]
pub enum GridItemAlignSelf
{
    /**
      | Content inside the item is aligned towards
      | the top.
      |
      */
    start = 0,               

    /**
      | Content inside the item is aligned towards
      | the bottom.
      |
      */
    end,                     

    /**
      | Content inside the item is aligned towards
      | the center.
      |
      */
    center,                  

    /**
      | Content inside the item is stretched
      | from top to bottom.
      |
      */
    stretch,                 

    /**
      | Follows the Grid container's alignItems
      | property.
      |
      */
    autoValue,                
}
