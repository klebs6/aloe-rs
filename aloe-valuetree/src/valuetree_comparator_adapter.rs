crate::ix!();

#[no_copy]
pub struct ValueTreeComparatorAdapter<'a, ElementComparator> {
    comparator: &'a mut ElementComparator,
}

impl<'a, ElementComparator> ValueTreeComparatorAdapter<'a, ElementComparator> {

    pub fn new(comp: &mut ElementComparator) -> Self {
    
        todo!();
        /*
        : comparator(comp),
        */
    }
    
    pub fn compare_elements(&mut self, 
        first:  *const ValueTree,
        second: *const ValueTree) -> i32 {
        
        todo!();
        /*
            return comparator.compareElements (*first, *second);
        */
    }
}
