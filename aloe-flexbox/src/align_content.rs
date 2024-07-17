crate::ix!();

/**
  | Possible values for the alignContent
  | property.
  |
  */
pub enum FlexBoxAlignContent
{
    /**
      | Lines of items are stretched from start
      | to end of the cross axis.
      |
      */
    stretch,              

    /**
      | Lines of items are aligned towards the
      | start of the cross axis.
      |
      */
    flexStart,            

    /**
      | Lines of items are aligned towards the
      | end of the cross axis.
      |
      */
    flexEnd,              

    /**
      | Lines of items are aligned towards the
      | center of the cross axis.
      |
      */
    center,               

    /**
      | Lines of items are evenly spaced along
      | the cross axis with spaces between them.
      |
      */
    spaceBetween,         

    /**
      | Lines of items are evenly spaced along
      | the cross axis with spaces around them.
      |
      */
    spaceAround,           
}
