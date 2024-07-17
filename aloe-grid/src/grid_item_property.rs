crate::ix!();

/**
  | Represents start and end properties.
  |
  */
pub struct GridItemStartAndEndProperty { 
    start: GridItemProperty,
    end:   GridItemProperty,
}

/**
  | Represents a property.
  |
  */
pub struct GridItemProperty {

    name:    String,

    /**
      | Either an absolute line number or number
      | of lines to span across.
      |
      */
    number:  i32, // default = 1
    is_span: bool, // default = false
    is_auto: bool, // default = false
}
    
impl Default for GridItemProperty {

    fn default() -> Self {
    
        todo!();
        /*
        : is_auto(true),
        */
    }
}
    
impl GridItemProperty {

    pub fn new(
        number_to_use:    i32,
        line_name_to_use: &String) -> Self {
    
        todo!();
        /*
        : name(lineNameToUse),
        : number(numberToUse),
        */
    }

    pub fn has_span(&self) -> bool {
        
        todo!();
        /*
            return isSpan && ! isAuto;
        */
    }
    
    pub fn has_absolute(&self) -> bool {
        
        todo!();
        /*
            return ! (isSpan || isAuto);
        */
    }
    
    pub fn has_auto(&self) -> bool {
        
        todo!();
        /*
            return isAuto;
        */
    }
    
    pub fn has_name(&self) -> bool {
        
        todo!();
        /*
            return name.isNotEmpty();
        */
    }
    
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }
    
    pub fn get_number(&self) -> i32 {
        
        todo!();
        /*
            return number;
        */
    }
}
    
impl From<GridItemKeyword> for GridItemProperty {

    fn from(keyword: GridItemKeyword) -> Self {
    
        todo!();
        /*
        : isAuto (keyword == GridItem::GridItemKeyword::autoValue)
        jassert (keyword == GridItem::GridItemKeyword::autoValue);
        */
    }
}
    
impl From<*const u8> for GridItemProperty {

    fn from(line_name_to_use: *const u8) -> Self {
    
        todo!();
        /*
        : property(String (lineNameToUse)),
        */
    }
}
    
impl From<&String> for GridItemProperty {

    fn from(line_name_to_use: &String) -> Self {
    
        todo!();
        /*
        : name(lineNameToUse),
        : number(1),
        */
    }
}
    
impl From<i32> for GridItemProperty {

    fn from(number_to_use: i32) -> Self {
    
        todo!();
        /*
        : number(numberToUse),
        */
    }
}
    
impl From<GridItemSpan> for GridItemProperty {

    fn from(span_to_use: GridItemSpan) -> Self {
    
        todo!();
        /*
        : name(spanToUse.name),
        : number(spanToUse.number),
        : is_span(true),
        */
    }
}
