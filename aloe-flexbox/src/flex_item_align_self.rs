crate::ix!();

/**
  | Possible value for the alignSelf property
  |
  */
pub enum FlexItemAlignSelf 
{
    /**
      | Follows the FlexBox container's alignItems
      | property.
      |
      */
    autoAlign,       

    /**
      | Item is aligned towards the start of
      | the cross axis.
      |
      */
    flexStart,       

    /**
      | Item is aligned towards the end of the
      | cross axis.
      |
      */
    flexEnd,         

    /**
      | Item is aligned towards the center of
      | the cross axis.
      |
      */
    center,          

    /**
      | Item is stretched from start to end of
      | the cross axis.
      |
      */
    stretch,          
}
