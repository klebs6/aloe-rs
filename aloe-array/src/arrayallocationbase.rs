crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ArrayAllocationBase.h]

/**
  | Implements some basic array storage
  | allocation functions.
  | 
  | This class isn't really for public use
  | - it used to be part of the container classes
  | but has since been superseded by ArrayBase.
  | Eventually it will be removed from the
  | API.
  | 
  | @tags{Core}
  |
  */
#[derive(Default)]
#[no_copy]
pub struct ArrayAllocationBase<ElementType,TypeOfCriticalSectionToUse> {
    base:          TypeOfCriticalSectionToUse,
    elements:      HeapBlock<ElementType>,
    num_allocated: i32, // default = 0
}

impl<ElementType,TypeOfCriticalSectionToUse> ArrayAllocationBase<ElementType,TypeOfCriticalSectionToUse> {

    pub fn new(other: ArrayAllocationBase<ElementType,TypeOfCriticalSectionToUse>) -> Self {
    
        todo!();
        /*
        : elements(std::move (other.elements)),
        : num_allocated(other.numAllocated),

        
        */
    }
    
    pub fn assign_from(&mut self, other: ArrayAllocationBase<ElementType,TypeOfCriticalSectionToUse>) -> &mut ArrayAllocationBase<ElementType,TypeOfCriticalSectionToUse> {
        
        todo!();
        /*
            elements = std::move (other.elements);
            numAllocated = other.numAllocated;
            return *this;
        */
    }

    /**
      | Changes the amount of storage allocated.
      | 
      | This will retain any data currently
      | held in the array, and either add or remove
      | extra space at the end.
      | 
      | -----------
      | @param numElements
      | 
      | the number of elements that are needed
      |
      */
    pub fn set_allocated_size(&mut self, num_elements: i32)  {
        
        todo!();
        /*
            if (numAllocated != numElements)
            {
                if (numElements > 0)
                    elements.realloc ((size_t) numElements);
                else
                    elements.free();

                numAllocated = numElements;
            }
        */
    }

    /**
      | Increases the amount of storage allocated
      | if it is less than a given amount.
      | 
      | This will retain any data currently
      | held in the array, but will add extra
      | space at the end to make sure there it's
      | at least as big as the size passed in.
      | If it's already bigger, no action is
      | taken.
      | 
      | -----------
      | @param minNumElements
      | 
      | the minimum number of elements that
      | are needed
      |
      */
    pub fn ensure_allocated_size(&mut self, min_num_elements: i32)  {
        
        todo!();
        /*
            if (minNumElements > numAllocated)
                setAllocatedSize ((minNumElements + minNumElements / 2 + 8) & ~7);

            jassert (numAllocated <= 0 || elements != nullptr);
        */
    }

    /**
      | Minimises the amount of storage allocated
      | so that it's no more than the given number
      | of elements.
      |
      */
    pub fn shrink_to_no_more_than(&mut self, max_num_elements: i32)  {
        
        todo!();
        /*
            if (maxNumElements < numAllocated)
                setAllocatedSize (maxNumElements);
        */
    }

    /**
      | Swap the contents of two objects.
      |
      */
    pub fn swap_with(&mut self, other: &mut ArrayAllocationBase<ElementType,TypeOfCriticalSectionToUse>)  {
        
        todo!();
        /*
            elements.swapWith (other.elements);
            std::swap (numAllocated, other.numAllocated);
        */
    }
}

