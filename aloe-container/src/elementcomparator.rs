crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ElementComparator.h]

/**
  | This is an internal helper class which
  | converts a aloe ElementComparator
  | style class (using a "compareElements"
  | method) into a class that's compatible
  | with std::sort (i.e. using an operator()
  | to compare the elements)
  | 
  | @tags{Core}
  |
  */
pub struct SortFunctionConverter<'a, ElementComparator> {
    comparator: &'a mut ElementComparator,
}

impl<'a,ElementComparator> SortFunctionConverter<'a,ElementComparator> {

    pub fn new(e: &mut ElementComparator) -> Self {
    
        todo!();
        /*
        : comparator(e),

        
        */
    }
    
    pub fn invoke<Type>(&mut self, a: Type, b: Type) -> bool {
    
        todo!();
        /*
            return comparator.compareElements (a, b) < 0;
        */
    }
}

/**
  | Sorts a range of elements in an array.
  | 
  | The comparator object that is passed-in
  | must define a public method with the
  | following signature:
  | 
  | @code int compareElements (ElementType
  | first, ElementType second); @endcode
  | 
  | ..and this method must return:
  | 
  | - a value of < 0 if the first comes before
  | the second
  | 
  | - a value of 0 if the two objects are equivalent
  | 
  | - a value of > 0 if the second comes before
  | the first
  | 
  | To improve performance, the compareElements()
  | method can be declared as static or const.
  | 
  | -----------
  | @param comparator
  | 
  | an object which defines a compareElements()
  | method
  | ----------
  | @param array
  | 
  | the array to sort
  | ----------
  | @param firstElement
  | 
  | the index of the first element of the
  | range to be sorted
  | ----------
  | @param lastElement
  | 
  | the index of the last element in the range
  | that needs sorting (this is inclusive)
  | ----------
  | @param retainOrderOfEquivalentItems
  | 
  | if true, the order of items that the comparator
  | deems the same will be maintained - this
  | will be a slower algorithm than if they
  | are allowed to be moved around.
  | 
  | @see sortArrayRetainingOrder
  |
  */
pub fn sort_array<ElementType, ElementComparator>(
        comparator:                       &mut ElementComparator,
        array:                            *mut ElementType,
        first_element:                    i32,
        last_element:                     i32,
        retain_order_of_equivalent_items: bool)  {

    todo!();
    /*
        jassert (firstElement >= 0);

        if (lastElement > firstElement)
        {
            SortFunctionConverter<ElementComparator> converter (comparator);

            if (retainOrderOfEquivalentItems)
                std::stable_sort (array + firstElement, array + lastElement + 1, converter);
            else
                std::sort        (array + firstElement, array + lastElement + 1, converter);
        }
    */
}

/**
  | Searches a sorted array of elements,
  | looking for the index at which a specified
  | value should be inserted for it to be
  | in the correct order.
  | 
  | The comparator object that is passed-in
  | must define a public method with the
  | following signature:
  | 
  | @code int compareElements (ElementType
  | first, ElementType second); @endcode
  | 
  | ..and this method must return:
  | 
  | - a value of < 0 if the first comes before
  | the second
  | 
  | - a value of 0 if the two objects are equivalent
  | 
  | - a value of > 0 if the second comes before
  | the first
  | 
  | To improve performance, the compareElements()
  | method can be declared as static or const.
  | 
  | -----------
  | @param comparator
  | 
  | an object which defines a compareElements()
  | method
  | ----------
  | @param array
  | 
  | the array to search
  | ----------
  | @param newElement
  | 
  | the value that is going to be inserted
  | ----------
  | @param firstElement
  | 
  | the index of the first element to search
  | ----------
  | @param lastElement
  | 
  | the index of the last element in the range
  | (this is non-inclusive)
  |
  */
pub fn find_insert_index_in_sorted_array<ElementType, ElementComparator>(
        comparator:    &mut ElementComparator,
        array:         *mut ElementType,
        new_element:   ElementType,
        first_element: i32,
        last_element:  i32) -> i32 {

    todo!();
    /*
        jassert (firstElement <= lastElement);

        ignoreUnused (comparator); // if you pass in an object with a static compareElements() method, this
                                   // avoids getting warning messages about the parameter being unused

        while (firstElement < lastElement)
        {
            if (comparator.compareElements (newElement, array [firstElement]) == 0)
            {
                ++firstElement;
                break;
            }
            else
            {
                const int halfway = (firstElement + lastElement) >> 1;

                if (halfway == firstElement)
                {
                    if (comparator.compareElements (newElement, array [halfway]) >= 0)
                        ++firstElement;

                    break;
                }
                else if (comparator.compareElements (newElement, array [halfway]) >= 0)
                {
                    firstElement = halfway;
                }
                else
                {
                    lastElement = halfway;
                }
            }
        }

        return firstElement;
    */
}

/**
  | A simple ElementComparator class that
  | can be used to sort an array of objects
  | that support the '<' operator.
  | 
  | This will work for primitive types and
  | objects that implement operator<().
  | 
  | Example:
  | 
  | @code
  | 
  | Vec<int> myArray; DefaultElementComparator<int>
  | sorter; myArray.sort (sorter);
  | 
  | @endcode
  | 
  | @see ElementComparator
  | 
  | @tags{Core}
  |
  */
pub struct DefaultElementComparator<ElementType> {
    phantom: std::marker::PhantomData<ElementType>,
}

impl<ElementType> DefaultElementComparator<ElementType> {

    pub fn compare_elements<ParameterType>(
        first:  ParameterType,
        second: ParameterType) -> i32 {
        
        todo!();
        /*
            return (first < second) ? -1 : ((second < first) ? 1 : 0);
        */
    }
}
