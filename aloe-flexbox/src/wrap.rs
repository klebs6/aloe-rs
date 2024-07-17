crate::ix!();

/**
  | Possible values for the flexWrap property.
  |
  */
pub enum FlexBoxWrap
{
    /**
      | Items are forced into a single line.
      |
      */
    noWrap,               

    /**
      | Items are wrapped onto multiple lines
      | from top to bottom.
      |
      */
    wrap,                 

    /**
      | Items are wrapped onto multiple lines
      | from bottom to top.
      |
      */
    wrapReverse,           
}
