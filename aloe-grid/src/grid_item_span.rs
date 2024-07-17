crate::ix!();

/**
  | Represents a span.
  |
  */
pub struct GridItemSpan {
    number: i32, // default = 1
    name:   String,
}

impl From<i32> for GridItemSpan {

    fn from(number_to_use: i32) -> Self {
    
        todo!();
        /*
        : number(numberToUse),

            /* GridItemSpan must be at least one and positive */
                jassert (numberToUse > 0);
        */
    }
}

impl From<&String> for GridItemSpan {
    
    fn from(name_to_use: &String) -> Self {
    
        todo!();
        /*
        : name(nameToUse),

            /* Name must not be empty */
                jassert (nameToUse.isNotEmpty());
        */
    }
}

impl GridItemSpan {

    pub fn new(
        number_to_use: i32,
        name_to_use:   &String) -> Self {
    
        todo!();
        /*
        : span(numberToUse),

            /* Name must not be empty */
                jassert (nameToUse.isNotEmpty());
                name = nameToUse;
        */
    }
}
