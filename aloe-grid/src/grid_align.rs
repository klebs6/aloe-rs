crate::ix!();

/**
  | Possible values for the alignItems
  | property.
  |
  */
#[repr(i32)]
pub enum GridAlignItems 
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
}

/**
  | Possible values for the alignContent
  | property.
  |
  */
pub enum GridAlignContent
{
    /**
      | Items are aligned towards the top of
      | the container.
      |
      */
    start,                    

    /**
      | Items are aligned towards the bottom
      | of the container.
      |
      */
    end,                      

    /**
      | Items are aligned towards the center
      | of the container.
      |
      */
    center,                   

    /**
      | Items are stretched from top to bottom
      | of the container.
      |
      */
    stretch,                  

    /**
      | Items are evenly spaced along the column
      | with spaces between them.
      |
      */
    spaceAround,              

    /**
      | Items are evenly spaced along the column
      | with spaces around them.
      |
      */
    spaceBetween,             

    /**
      | Items are evenly spaced along the column
      | with even amount of spaces between them.
      |
      */
    spaceEvenly,               

}
