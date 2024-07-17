crate::ix!();

/**
  | Possible values for the autoFlow property.
  |
  */
pub enum GridAutoFlow
{
    /**
      | Fills the grid by adding rows of items.
      |
      */
    row,                      

    /**
      | Fills the grid by adding columns of items.
      |
      */
    column,                   

    /**
      | Fills the grid by adding rows of items
      | and attempts to fill in gaps.
      |
      */
    rowDense,                 

    /**
      | Fills the grid by adding columns of items
      | and attempts to fill in gaps.
      |
      */
    columnDense,               
}
