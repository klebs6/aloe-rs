crate::ix!();

/**
  | Possible values for the justifyItems
  | property.
  |
  */
#[repr(i32)]
pub enum GridJustifyItems 
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
}

/**
  | Possible values for the justifyContent
  | property.
  |
  */
pub enum GridJustifyContent
{
    /**
      | Items are justified towards the left
      | of the container.
      |
      */
    start,                    

    /**
      | Items are justified towards the right
      | of the container.
      |
      */
    end,                      

    /**
      | Items are justified towards the center
      | of the container.
      |
      */
    center,                   

    /**
      | Items are stretched from left to right
      | of the container.
      |
      */
    stretch,                  

    /**
      | Items are evenly spaced along the row
      | with spaces between them.
      |
      */
    spaceAround,              

    /**
      | Items are evenly spaced along the row
      | with spaces around them.
      |
      */
    spaceBetween,             

    /**
      | Items are evenly spaced along the row
      | with even amount of spaces between them.
      |
      */
    spaceEvenly,               
}
