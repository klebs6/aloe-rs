crate::ix!();

/**
  | A comparator used to sort our data when
  | the user clicks a column header
  |
  */
pub struct DemoDataSorter {
    attribute_to_sort: String,
    direction:         i32,
}

impl DemoDataSorter {

    pub fn new(
        attribute_to_sort_by: &String,
        forwards:             bool) -> Self {
    
        todo!();
        /*


            : attributeToSort (attributeToSortBy),
                  direction (forwards ? 1 : -1)
        */
    }
    
    pub fn compare_elements(&self, 
        first:  *mut XmlElement,
        second: *mut XmlElement) -> i32 {
        
        todo!();
        /*
            auto result = first->getStringAttribute (attributeToSort)
                                    .compareNatural (second->getStringAttribute (attributeToSort));

                if (result == 0)
                    result = first->getStringAttribute ("ID")
                                   .compareNatural (second->getStringAttribute ("ID"));

                return direction * result;
        */
    }
}
