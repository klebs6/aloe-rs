crate::ix!();

/**
  | Possible values for the justifyContent
  | property.
  |
  */
pub enum FlexBoxJustifyContent
{
    /**
      | Items are justified towards the start
      | of the main axis.
      |
      */
    flexStart,            

    /**
      | Items are justified towards the end
      | of the main axis.
      |
      */
    flexEnd,              

    /**
      | Items are justified towards the center
      | of the main axis.
      |
      */
    center,               

    /**
      | Items are evenly spaced along the main
      | axis with spaces between them.
      |
      */
    spaceBetween,         

    /**
      | Items are evenly spaced along the main
      | axis with spaces around them.
      |
      */
    spaceAround,           
}
