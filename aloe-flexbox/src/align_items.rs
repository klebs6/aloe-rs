crate::ix!();

/**
  | Possible values for the alignItems
  | property.
  |
  */
pub enum FlexBoxAlignItems
{
    /**
      | Items are stretched from start to end
      | of the cross axis.
      |
      */
    stretch,              

    /**
      | Items are aligned towards the start
      | of the cross axis.
      |
      */
    flexStart,            

    /**
      | Items are aligned towards the end of
      | the cross axis.
      |
      */
    flexEnd,              

    /**
      | Items are aligned towards the center
      | of the cross axis.
      |
      */
    center,                
}
