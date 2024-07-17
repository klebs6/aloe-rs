crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ArrayBase.h]

pub type NonTriviallyCopyableVoid<T: Clone> = T;
pub type TriviallyCopyableVoid<T: Copy> = T;

/**
  | A basic object container.
  | 
  | This class isn't really for public use
  | - it's used by the other array classes,
  | but might come in handy for some purposes.
  | 
  | It inherits from a critical section
  | class to allow the arrays to use the "empty
  | base class optimisation" pattern to
  | reduce their footprint.
  | 
  | @see Array, OwnedArray, ReferenceCountedArray
  | 
  | @tags{Core}
  |
  */
#[derive(Default)]
#[no_copy]
pub struct ArrayBase<ElementType,TypeOfCriticalSectionToUse> {
    base:          TypeOfCriticalSectionToUse,
    elements:      HeapBlock<ElementType>,
    num_allocated: i32, // default = 0
    num_used:      i32, // default = 0
}

impl<ElementType,TypeOfCriticalSectionToUse> Drop for ArrayBase<ElementType,TypeOfCriticalSectionToUse> {

    fn drop(&mut self) {
        todo!();
        /* 
            clear();
         */
    }
}

/*TODO
impl<ElementType,TypeOfCriticalSectionToUse> HasParameterType for ArrayBase<ElementType,TypeOfCriticalSectionToUse> {
    type ParameterType = <Self as HasParameterType>::ParameterType<ElementType>;
}

impl<ElementType,TypeOfCriticalSectionToUse> HasAllowConversion for ArrayBase<ElementType,TypeOfCriticalSectionToUse> {

    lazy_static!{
        /*
        template <class OtherElementType, class OtherCriticalSection>
            using AllowConversion = typename std::enable_if<! std::is_same<std::tuple<ElementType, TypeOfCriticalSectionToUse>,
                                                                           std::tuple<OtherElementType, OtherCriticalSection>>::value>::type;
        */
    }
}
*/

pub trait NotPointer {}

impl<ElementType, TypeOfCriticalSectionToUse, OtherArrayType> 

PartialEq<OtherArrayType> for ArrayBase<ElementType,TypeOfCriticalSectionToUse> {
    
    #[inline] fn eq(&self, other: &OtherArrayType) -> bool {
        todo!();
        /*
            if (size() != (int) other.size())
                return false;

            auto* e = begin();

            for (auto& o : other)
                if (! (*e++ == o))
                    return false;

            return true;
        */
    }
}

impl<ElementType,TypeOfCriticalSectionToUse> IndexMut<i32> for ArrayBase<ElementType,TypeOfCriticalSectionToUse> {
    
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        todo!();
        /*
            jassert (elements != nullptr);
            jassert (isPositiveAndBelow (index, numUsed));
            return elements[index];
        */
    }
}

impl<ElementType,TypeOfCriticalSectionToUse> Index<i32> for ArrayBase<ElementType,TypeOfCriticalSectionToUse> {
    type Output = ElementType;
    
    fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            jassert (elements != nullptr);
            jassert (isPositiveAndBelow (index, numUsed));
            return elements[index];
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ArrayBase.cpp]
impl<ElementType: ParameterType,TypeOfCriticalSectionToUse> ArrayBase<ElementType,TypeOfCriticalSectionToUse> {

    pub fn new_from_other(other: ArrayBase<ElementType,TypeOfCriticalSectionToUse>) -> Self {
    
        todo!();
        /*
        : elements(std::move (other.elements)),
        : num_allocated(other.numAllocated),
        : num_used(other.numUsed),

            other.numAllocated = 0;
            other.numUsed = 0;
        */
    }
    
    pub fn assign_from_array_base(&mut self, other: ArrayBase<ElementType,TypeOfCriticalSectionToUse>) -> &mut ArrayBase<ElementType,TypeOfCriticalSectionToUse> {
        
        todo!();
        /*
            if (this != &other)
            {
                auto tmp (std::move (other));
                swapWith (tmp);
            }

            return *this;
        */
    }

    /**
      | Converting move constructor.
      | 
      | Only enabled when the other array has
      | a different type to this one.
      | 
      | If you see a compile error here, it's
      | probably because you're attempting
      | a conversion that HeapBlock won't allow.
      |
      */
    pub fn new_converting<OtherElementType, OtherCriticalSection>(other: ArrayBase<OtherElementType,OtherCriticalSection>) -> Self {
    
        todo!();
        /*
        : elements(std::move (other.elements)),
        : num_allocated(other.numAllocated),
        : num_used(other.numUsed),

            other.numAllocated = 0;
            other.numUsed = 0;
        */
    }

    /**
      | Converting move assignment operator.
      | 
      | Only enabled when the other array has
      | a different type to this one.
      | 
      | If you see a compile error here, it's
      | probably because you're attempting
      | a conversion that HeapBlock won't allow.
      |
      */
    pub fn assign_from_converting<
        OtherElementType, 
        OtherCriticalSection 
        /*, AllowConversion<OtherElementType,OtherCriticalSection> */ 
    >
        (&mut self, other: ArrayBase<OtherElementType,OtherCriticalSection>) 
        -> &mut ArrayBase<ElementType,TypeOfCriticalSectionToUse> 
    {
        todo!();
        /*
            // No need to worry about assignment to *this, because 'other' must be of a different type.
            elements = std::move (other.elements);
            numAllocated = other.numAllocated;
            numUsed = other.numUsed;

            other.numAllocated = 0;
            other.numUsed = 0;

            return *this;
        */
    }
    
    #[inline] pub fn get_value_with_default(&self, index: i32) -> ElementType {
        
        todo!();
        /*
            return isPositiveAndBelow (index, numUsed) ? elements[index] : ElementType();
        */
    }

    #[inline] pub fn get_first(&self) -> ElementType {
        
        todo!();
        /*
            return numUsed > 0 ? elements[0] : ElementType();
        */
    }

    #[inline] pub fn get_last(&self) -> ElementType {
        
        todo!();
        /*
            return numUsed > 0 ? elements[numUsed - 1] : ElementType();
        */
    }

    #[inline] pub fn begin_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return elements;
        */
    }

    #[inline] pub fn begin(&self) -> *const ElementType {
        
        todo!();
        /*
            return elements;
        */
    }

    #[inline] pub fn end_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return elements + numUsed;
        */
    }

    #[inline] pub fn end(&self) -> *const ElementType {
        
        todo!();
        /*
            return elements + numUsed;
        */
    }

    #[inline] pub fn data_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return elements;
        */
    }

    #[inline] pub fn data(&self) -> *const ElementType {
        
        todo!();
        /*
            return elements;
        */
    }

    #[inline] pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return numUsed;
        */
    }

    #[inline] pub fn capacity(&self) -> i32 {
        
        todo!();
        /*
            return numAllocated;
        */
    }

    pub fn set_allocated_size(&mut self, num_elements: i32)  {
        
        todo!();
        /*
            jassert (numElements >= numUsed);

            if (numAllocated != numElements)
            {
                if (numElements > 0)
                    setAllocatedSizeInternal (numElements);
                else
                    elements.free();
            }

            numAllocated = numElements;
        */
    }

    pub fn ensure_allocated_size(&mut self, min_num_elements: i32)  {
        
        todo!();
        /*
            if (minNumElements > numAllocated)
                setAllocatedSize ((minNumElements + minNumElements / 2 + 8) & ~7);

            jassert (numAllocated <= 0 || elements != nullptr);
        */
    }

    pub fn shrink_to_no_more_than(&mut self, max_num_elements: i32)  {
        
        todo!();
        /*
            if (maxNumElements < numAllocated)
                setAllocatedSize (maxNumElements);
        */
    }

    pub fn clear(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < numUsed; ++i)
                elements[i].~ElementType();

            numUsed = 0;
        */
    }

    pub fn swap_with(&mut self, other: &mut ArrayBase<ElementType,TypeOfCriticalSectionToUse>)  {
        
        todo!();
        /*
            elements.swapWith (other.elements);
            std::swap (numAllocated, other.numAllocated);
            std::swap (numUsed,      other.numUsed);
        */
    }

    pub fn add(&mut self, new_element: &ElementType)  {
        
        todo!();
        /*
            addImpl (newElement);
        */
    }

    pub fn add_new_element(&mut self, new_element: ElementType)  {
        
        todo!();
        /*
            addImpl (std::move (newElement));
        */
    }

    pub fn add_first_ref_and_others<OtherElements>(
        &mut self, 
        first_new_element: &ElementType,
        other_elements:    OtherElements
    )
    {
    
        todo!();
        /*
            addImpl (firstNewElement, std::forward<OtherElements> (otherElements)...);
        */
    }

    pub fn add_first_and_other_elements<OtherElements>(
        &mut self, 
        first_new_element: ElementType,
        other_elements:    OtherElements)  {
    
        todo!();
        /*
            addImpl (std::move (firstNewElement), std::forward<OtherElements> (otherElements)...);
        */
    }

    pub fn add_array_from_raw_array<Type>(
        &mut self, 
        elements_to_add:     *const Type,
        num_elements_to_add: i32)  {
    
        todo!();
        /*
            ensureAllocatedSize (numUsed + numElementsToAdd);
            addArrayInternal (elementsToAdd, numElementsToAdd);
            numUsed += numElementsToAdd;
        */
    }

    pub fn add_array_from_slice<TypeToCreateFrom>(&mut self, items: &[TypeToCreateFrom])  {
    
        todo!();
        /*
            ensureAllocatedSize (numUsed + (int) items.size());

            for (auto& item : items)
                new (elements + numUsed++) ElementType (item);
        */
    }

    pub fn add_array_from_other_ref<OtherArrayType>(&mut self, array_to_add_from: &OtherArrayType)  {
    
        todo!();
        /*
            jassert ((const void*) this != (const void*) &arrayToAddFrom); // can't add from our own elements!
            ensureAllocatedSize (numUsed + (int) arrayToAddFrom.size());

            for (auto& e : arrayToAddFrom)
                addAssumingCapacityIsReady (e);
        */
    }
    /**
      typename std::enable_if<! std::is_pointer<OtherArrayType>::value, int>::type
      */
    pub fn new_from_other_by_start_and_len<OtherArrayType: NotPointer>(
        array_to_add_from:   &OtherArrayType,
        start_index:         i32,
        num_elements_to_add: Option<i32>
    ) -> Self {

        let num_elements_to_add: i32 =
            num_elements_to_add.unwrap_or(-1);

        todo!();
        /*

            jassert ((const void*) this != (const void*) &arrayToAddFrom); // can't add from our own elements!

            if (startIndex < 0)
            {
                jassertfalse;
                startIndex = 0;
            }

            if (numElementsToAdd < 0 || startIndex + numElementsToAdd > (int) arrayToAddFrom.size())
                numElementsToAdd = (int) arrayToAddFrom.size() - startIndex;

            addArray (arrayToAddFrom.data() + startIndex, numElementsToAdd);

            return numElementsToAdd;
        */
    }

    pub fn insert(
        &mut self, 
        index_to_insert_at:           i32,
        new_element:                  <ElementType as ParameterType>::Type,
        number_of_times_to_insert_it: i32)  {
        
        todo!();
        /*
            checkSourceIsNotAMember (newElement);
            auto* space = createInsertSpace (indexToInsertAt, numberOfTimesToInsertIt);

            for (int i = 0; i < numberOfTimesToInsertIt; ++i)
                new (space++) ElementType (newElement);

            numUsed += numberOfTimesToInsertIt;
        */
    }

    pub fn insert_array(&mut self, 
        index_to_insert_at: i32,
        new_elements:       *const ElementType,
        number_of_elements: i32)  {
        
        todo!();
        /*
            auto* space = createInsertSpace (indexToInsertAt, numberOfElements);

            for (int i = 0; i < numberOfElements; ++i)
                new (space++) ElementType (*(newElements++));

            numUsed += numberOfElements;
        */
    }

    pub fn remove_elements(&mut self, 
        index_to_remove_at:     i32,
        num_elements_to_remove: i32)  {
        
        todo!();
        /*
            jassert (indexToRemoveAt >= 0);
            jassert (numElementsToRemove >= 0);
            jassert (indexToRemoveAt + numElementsToRemove <= numUsed);

            if (numElementsToRemove > 0)
            {
                removeElementsInternal (indexToRemoveAt, numElementsToRemove);
                numUsed -= numElementsToRemove;
            }
        */
    }

    pub fn swap(&mut self, 
        index1: i32,
        index2: i32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (index1, numUsed)
             && isPositiveAndBelow (index2, numUsed))
            {
                std::swap (elements[index1],
                           elements[index2]);
            }
        */
    }

    pub fn move_(&mut self, 
        current_index: i32,
        new_index:     i32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (currentIndex, numUsed))
            {
                if (! isPositiveAndBelow (newIndex, numUsed))
                    newIndex = numUsed - 1;

                moveInternal (currentIndex, newIndex);
            }
        */
    }

    lazy_static!{
        /*
        template <typename T>
           #if defined(__GNUC__) && __GNUC__ < 5 && ! defined(__clang__)
            using IsTriviallyCopyable = std::is_scalar<T>;
           #else
            using IsTriviallyCopyable = std::is_trivially_copyable<T>;
           #endif

            template <typename T>
            using TriviallyCopyableVoid = typename std::enable_if<IsTriviallyCopyable<T>::value, void>::type;

            template <typename T>
            using NonTriviallyCopyableVoid = typename std::enable_if<! IsTriviallyCopyable<T>::value, void>::type;
        */
    }
    
    pub fn add_array_internal_with_element_array<T /* = ElementType */>(
        &mut self, 
        other_elements: *const ElementType,
        num_elements:   i32) -> TriviallyCopyableVoid<T> {
    
        todo!();
        /*
            if (numElements > 0)
                memcpy (elements + numUsed, otherElements, (size_t) numElements * sizeof (ElementType));
        */
    }
    
    pub fn add_array_internal_with_type_array<Type, T /* = ElementType */>(
        &mut self, 
        other_elements: *const Type,
        num_elements:   i32) -> TriviallyCopyableVoid<T> {
    
        todo!();
        /*
            auto* start = elements + numUsed;

            while (--numElements >= 0)
                new (start++) ElementType (*(otherElements++));
        */
    }
    
    pub fn add_array_internal_with_type_array_into_nontrivially_copyable<Type, T /* = ElementType */>(
        &mut self, 
        other_elements: *const Type,
        num_elements:   i32
    ) -> NonTriviallyCopyableVoid<T> 
    {
        todo!();

        /*
            auto* start = elements + numUsed;

            while (--numElements >= 0)
                new (start++) ElementType (*(otherElements++));
        */
    }
    
    pub fn set_allocated_size_internal_into_trivially_copyable<T /* = ElementType */>(&mut self, num_elements: i32) -> TriviallyCopyableVoid<T> {
    
        todo!();
        /*
            elements.realloc ((size_t) numElements);
        */
    }

    pub fn set_allocated_size_internal_into_nontrivially_copyable<T /*= ElementType*/>(&mut self, num_elements: i32) -> NonTriviallyCopyableVoid<T> {
    
        todo!();
        /*
            HeapBlock<ElementType> newElements (numElements);

            for (int i = 0; i < numUsed; ++i)
            {
                new (newElements + i) ElementType (std::move (elements[i]));
                elements[i].~ElementType();
            }

            elements = std::move (newElements);
        */
    }

    pub fn create_insert_space(&mut self, 
        index_to_insert_at: i32,
        num_elements:       i32) -> *mut ElementType {
        
        todo!();
        /*
            ensureAllocatedSize (numUsed + numElements);

            if (! isPositiveAndBelow (indexToInsertAt, numUsed))
                return elements + numUsed;

            createInsertSpaceInternal (indexToInsertAt, numElements);

            return elements + indexToInsertAt;
        */
    }

    pub fn create_insert_space_internal_into_trivially_copyable<T /*= ElementType*/>(
        &mut self, 
        index_to_insert_at: i32,
        num_elements:       i32
    ) -> TriviallyCopyableVoid<T> {
    
        todo!();
        /*
            auto* start = elements + indexToInsertAt;
            auto numElementsToShift = numUsed - indexToInsertAt;
            memmove (start + numElements, start, (size_t) numElementsToShift * sizeof (ElementType));
        */
    }

    pub fn create_insert_space_internal_into_nontrivially_copyable<T /*= ElementType*/>(
        &mut self, 
        index_to_insert_at: i32,
        num_elements:       i32
    ) -> NonTriviallyCopyableVoid<T> {
    
        todo!();
        /*
            auto* end = elements + numUsed;
            auto* newEnd = end + numElements;
            auto numElementsToShift = numUsed - indexToInsertAt;

            for (int i = 0; i < numElementsToShift; ++i)
            {
                new (--newEnd) ElementType (std::move (*(--end)));
                end->~ElementType();
            }
        */
    }

    pub fn remove_elements_internal_into_trivially_copyable<T /*= ElementType*/>(
        &mut self, 
        index_to_remove_at:     i32,
        num_elements_to_remove: i32
    ) -> TriviallyCopyableVoid<T> {
    
        todo!();
        /*
            auto* start = elements + indexToRemoveAt;
            auto numElementsToShift = numUsed - (indexToRemoveAt + numElementsToRemove);
            memmove (start, start + numElementsToRemove, (size_t) numElementsToShift * sizeof (ElementType));
        */
    }

    pub fn remove_elements_internal_into_nontrivially_copyable<T /*= ElementType*/>(
        &mut self, 
        index_to_remove_at:     i32,
        num_elements_to_remove: i32
    ) -> NonTriviallyCopyableVoid<T> {
    
        todo!();
        /*
            auto numElementsToShift = numUsed - (indexToRemoveAt + numElementsToRemove);
            auto* destination = elements + indexToRemoveAt;
            auto* source = destination + numElementsToRemove;

            for (int i = 0; i < numElementsToShift; ++i)
                moveAssignElement (destination++, std::move (*(source++)));

            for (int i = 0; i < numElementsToRemove; ++i)
                (destination++)->~ElementType();
        */
    }

    pub fn move_internal_into_trivially_copyable<T /*= ElementType*/>(
        &mut self, 
        current_index: i32,
        new_index:     i32) -> TriviallyCopyableVoid<T> {
    
        todo!();
        /*
            char tempCopy[sizeof (ElementType)];
            memcpy (tempCopy, elements + currentIndex, sizeof (ElementType));

            if (newIndex > currentIndex)
            {
                memmove (elements + currentIndex,
                         elements + currentIndex + 1,
                         (size_t) (newIndex - currentIndex) * sizeof (ElementType));
            }
            else
            {
                memmove (elements + newIndex + 1,
                         elements + newIndex,
                         (size_t) (currentIndex - newIndex) * sizeof (ElementType));
            }

            memcpy (elements + newIndex, tempCopy, sizeof (ElementType));
        */
    }

    pub fn move_internal_into_nontrivially_copyable<T /*= ElementType*/>(
        &mut self, 
        current_index: i32,
        new_index:     i32) -> NonTriviallyCopyableVoid<T> {
    
        todo!();
        /*
            auto* e = elements + currentIndex;
            ElementType tempCopy (std::move (*e));
            auto delta = newIndex - currentIndex;

            if (delta > 0)
            {
                for (int i = 0; i < delta; ++i)
                {
                    moveAssignElement (e, std::move (*(e + 1)));
                    ++e;
                }
            }
            else
            {
                for (int i = 0; i < -delta; ++i)
                {
                    moveAssignElement (e, std::move (*(e - 1)));
                    --e;
                }
            }

            moveAssignElement (e, std::move (tempCopy));
        */
    }

    pub fn add_impl<Elements>(&mut self, to_add: Elements)  {
    
        todo!();
        /*
            ignoreUnused (std::initializer_list<int> { (((void) checkSourceIsNotAMember (toAdd)), 0)... });
            ensureAllocatedSize (numUsed + (int) sizeof... (toAdd));
            addAssumingCapacityIsReady (std::forward<Elements> (toAdd)...);
        */
    }

    pub fn add_assuming_capacity_is_ready<Elements>(&mut self, to_add: Elements)  {
    
        todo!();
        /*
            ignoreUnused (std::initializer_list<int> { ((void) (new (elements + numUsed++) ElementType (std::forward<Elements> (toAdd))), 0)... });
        */
    }

    pub fn new_from_move_assignable<T: MoveAssignable>(
        destination: *mut ElementType,
        source:      ElementType) -> Self {
    
        todo!();
        /*

            *destination = std::move (source);
        */
    }

    pub fn new_from_not_move_assignable<T: NotMoveAssignable>(
        destination: *mut ElementType,
        source:      ElementType) -> Self {
    
        todo!();
        /*

            destination->~ElementType();
            new (destination) ElementType (std::move (source));
        */
    }

    pub fn check_source_is_nota_member(&mut self, element: &ElementType)  {
        
        todo!();
        /*
            // when you pass a reference to an existing element into a method like add() which
            // may need to reallocate the array to make more space, the incoming reference may
            // be deleted indirectly during the reallocation operation! To work around this,
            // make a local copy of the item you're trying to add (and maybe use std::move to
            // move it into the add() method to avoid any extra overhead)
            jassertquiet (std::addressof (element) < begin() || end() <= std::addressof (element));
        */
    }
}

pub trait NotMoveAssignable:  {}
pub trait MoveAssignable:  {}

#[cfg(ALOE_UNIT_TESTS)]
pub mod array_base_tests_helpers {
    use super::*;

    #[derive(Default)]
    pub struct TriviallyCopyableType {
        value: i32, // default = -1111 
    }

    impl PartialEq<TriviallyCopyableType> for TriviallyCopyableType {
        
        #[inline] fn eq(&self, other: &TriviallyCopyableType) -> bool {
            todo!();
            /*
                return getValue() == other.getValue();
            */
        }
    }

    impl Eq for TriviallyCopyableType {}

    impl TriviallyCopyableType {

        pub fn new_from_i32(v: i32) -> Self {
        
            todo!();
            /*
            : value(v),

            
            */
        }
        
        pub fn new_from_f32(v: f32) -> Self {
        
            todo!();
            /*

                : value ((int) v)
            */
        }
        
        pub fn get_value(&self) -> i32 {
            
            todo!();
            /*
                return value;
            */
        }
    }

    ///-------------------
    #[derive(Default)]
    pub struct NonTriviallyCopyableType {

        value: i32, // default = -1111 
        ptr:   *mut i32, // = &value;
    }

    impl PartialEq<NonTriviallyCopyableType> for NonTriviallyCopyableType {
        
        #[inline] fn eq(&self, other: &NonTriviallyCopyableType) -> bool {
            todo!();
            /*
                return getValue() == other.getValue();
            */
        }
    }

    impl Eq for NonTriviallyCopyableType {}

    impl NonTriviallyCopyableType {

        pub fn new_from_i32(v: i32) -> Self {
        
            todo!();
            /*
            : value(v),

            
            */
        }
        
        pub fn new_from_f32(v: f32) -> Self {
        
            todo!();
            /*


                : value ((int) v)
            */
        }
        
        pub fn new_from_non_trivially_copyable_ref(other: &NonTriviallyCopyableType) -> Self {
        
            todo!();
            /*


                : value (other.value)
            */
        }
        
        pub fn assign_from_nontrivially_copyable_ref(&mut self, other: &NonTriviallyCopyableType) -> &mut NonTriviallyCopyableType {
            
            todo!();
            /*
                value = other.value;
                    return *this;
            */
        }
        
        pub fn get_value(&self) -> i32 {
            
            todo!();
            /*
                return *ptr;
            */
        }
    }

    impl PartialEq<TriviallyCopyableType> for TriviallyCopyableType {
        
        #[inline] fn eq(&self, other: &TriviallyCopyableType) -> bool {
            todo!();
            /*
                return tct.getValue() == ntct.getValue();
            */
        }
    }

    impl Eq for TriviallyCopyableType {}

    impl PartialEq<NonTriviallyCopyableType> for NonTriviallyCopyableType {
        
        #[inline] fn eq(&self, other: &NonTriviallyCopyableType) -> bool {
            todo!();
            /*
                return tct == ntct;
            */
        }
    }

    impl Eq for NonTriviallyCopyableType {}
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct ArrayBaseTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod array_base_tests {
    use super::*;

    pub type CopyableType    = TriviallyCopyableType;
    pub type NoncopyableType = NonTriviallyCopyableType;

    lazy_static!{
        /*
        #if ! (defined(__GNUC__) && __GNUC__ < 5 && ! defined(__clang__))
            static_assert (std::is_trivially_copyable<CopyableType>::value,
                           "Test TriviallyCopyableType is not trivially copyable");
            static_assert (! std::is_trivially_copyable<NoncopyableType>::value,
                           "Test NonTriviallyCopyableType is trivially copyable");
           #endif

            struct Base
            {
                virtual ~Base() = default;
            };

            struct Derived : Base
            {
            };
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for ArrayBaseTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("ArrayBase", UnitTestCategories::containers
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl ArrayBaseTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("grow capacity");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                int originalCapacity = 4;
                referenceContainer.reserve ((size_t) originalCapacity);
                expectEquals ((int) referenceContainer.capacity(), originalCapacity);
                copyableContainer.setAllocatedSize (originalCapacity);
                expectEquals (copyableContainer.capacity(), originalCapacity);
                noncopyableContainer.setAllocatedSize (originalCapacity);
                expectEquals (noncopyableContainer.capacity(), originalCapacity);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                addData (referenceContainer, copyableContainer, noncopyableContainer, 33);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                expect ((int) referenceContainer.capacity() != originalCapacity);
                expect (copyableContainer.capacity()        != originalCapacity);
                expect (noncopyableContainer.capacity()     != originalCapacity);
            }

            beginTest ("shrink capacity");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                int numElements = 45;
                addData (referenceContainer, copyableContainer, noncopyableContainer, numElements);

                copyableContainer.shrinkToNoMoreThan (numElements);
                noncopyableContainer.setAllocatedSize (numElements + 1);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                referenceContainer.clear();
                copyableContainer.removeElements    (0, numElements);
                noncopyableContainer.removeElements (0, numElements);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                copyableContainer.setAllocatedSize    (0);
                noncopyableContainer.setAllocatedSize (0);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                addData (referenceContainer, copyableContainer, noncopyableContainer, numElements);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("equality");
            {
                std::vector<int> referenceContainer = { 1, 2, 3 };
                ArrayBase<int, DummyCriticalSection> testContainer1, testContainer2;

                for (auto i : referenceContainer)
                {
                    testContainer1.add (i);
                    testContainer2.add (i);
                }

                expect (testContainer1 == referenceContainer);
                expect (testContainer2 == testContainer1);

                testContainer1.ensureAllocatedSize (257);
                referenceContainer.shrink_to_fit();

                expect (testContainer1 == referenceContainer);
                expect (testContainer2 == testContainer1);

                testContainer1.removeElements (0, 1);

                expect (testContainer1 != referenceContainer);
                expect (testContainer2 != testContainer1);
            }

            beginTest ("accessors");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                addData (referenceContainer, copyableContainer, noncopyableContainer, 3);

                int testValue = -123;
                referenceContainer[0]   = testValue;
                copyableContainer[0]    = testValue;
                noncopyableContainer[0] = testValue;

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                expect (copyableContainer   .getFirst().getValue() == testValue);
                expect (noncopyableContainer.getFirst().getValue() == testValue);

                auto last = referenceContainer.back().getValue();

                expectEquals (copyableContainer   .getLast().getValue(), last);
                expectEquals (noncopyableContainer.getLast().getValue(), last);

                ArrayBase<CopyableType,    DummyCriticalSection> copyableEmpty;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableEmpty;

                auto defualtValue = CopyableType().getValue();
                expectEquals (defualtValue, NoncopyableType().getValue());

                expectEquals (copyableEmpty   .getFirst().getValue(), defualtValue);
                expectEquals (noncopyableEmpty.getFirst().getValue(), defualtValue);
                expectEquals (copyableEmpty   .getLast() .getValue(), defualtValue);
                expectEquals (noncopyableEmpty.getLast() .getValue(), defualtValue);

                ArrayBase<float*, DummyCriticalSection> floatPointers;
                expect (floatPointers.getValueWithDefault (-3) == nullptr);
            }

            beginTest ("add moved");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                for (int i = 0; i < 5; ++i)
                {
                    CopyableType ref    (-i);
                    CopyableType ct     (-i);
                    NoncopyableType nct (-i);
                    referenceContainer.push_back (std::move (ref));
                    copyableContainer.add (std::move (ct));
                    noncopyableContainer.add (std::move (nct));
                }

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("add multiple");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                for (int i = 4; i < 7; ++i)
                    referenceContainer.push_back ({ -i });

                copyableContainer.add    (CopyableType    (-4), CopyableType    (-5), CopyableType    (-6));
                noncopyableContainer.add (NoncopyableType (-4), NoncopyableType (-5), NoncopyableType (-6));

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("add array from a pointer");
            {
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                std::vector<CopyableType>    copyableData    = { 3, 4, 5 };
                std::vector<NoncopyableType> noncopyableData = { 3, 4, 5 };

                copyableContainer.addArray    (copyableData.data(),    (int) copyableData.size());
                noncopyableContainer.addArray (noncopyableData.data(), (int) noncopyableData.size());

                checkEqual (copyableContainer, noncopyableContainer, copyableData);
            }

            beginTest ("add array from a pointer of a different type");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                std::vector<float> floatData = { 1.4f, 2.5f, 3.6f };

                for (auto f : floatData)
                    referenceContainer.push_back ({ f });

                copyableContainer.addArray    (floatData.data(), (int) floatData.size());
                noncopyableContainer.addArray (floatData.data(), (int) floatData.size());

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("add array from initializer_list");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                std::initializer_list<CopyableType>    ilct  { { 3 }, { 4 }, { 5 } };
                std::initializer_list<NoncopyableType> ilnct { { 3 }, { 4 }, { 5 } };

                for (auto v : ilct)
                    referenceContainer.push_back ({ v });

                copyableContainer.addArray    (ilct);
                noncopyableContainer.addArray (ilnct);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("add array from containers");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                addData (referenceContainer, copyableContainer, noncopyableContainer, 5);

                std::vector<CopyableType> referenceContainerCopy (referenceContainer);
                std::vector<NoncopyableType> noncopyableReferenceContainerCopy;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainerCopy;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainerCopy;

                for (auto& v : referenceContainerCopy)
                    noncopyableReferenceContainerCopy.push_back ({ v.getValue() });

                for (size_t i = 0; i < referenceContainerCopy.size(); ++i)
                {
                    auto value = referenceContainerCopy[i].getValue();
                    copyableContainerCopy.add    ({ value });
                    noncopyableContainerCopy.add ({ value });
                }

                // From self-types
                copyableContainer.addArray    (copyableContainerCopy);
                noncopyableContainer.addArray (noncopyableContainerCopy);

                for (auto v : referenceContainerCopy)
                    referenceContainer.push_back (v);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                // From std containers
                copyableContainer.addArray    (referenceContainerCopy);
                noncopyableContainer.addArray (noncopyableReferenceContainerCopy);

                for (auto v : referenceContainerCopy)
                    referenceContainer.push_back (v);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                // From std containers with offset
                int offset = 5;
                copyableContainer.addArray    (referenceContainerCopy,            offset);
                noncopyableContainer.addArray (noncopyableReferenceContainerCopy, offset);

                for (size_t i = 5; i < referenceContainerCopy.size(); ++i)
                    referenceContainer.push_back (referenceContainerCopy[i]);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("insert");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                addData (referenceContainer, copyableContainer, noncopyableContainer, 8);

                referenceContainer.insert (referenceContainer.begin(), -4);
                copyableContainer.insert    (0, -4, 1);
                noncopyableContainer.insert (0, -4, 1);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                for (int i = 0; i < 3; ++i)
                    referenceContainer.insert (referenceContainer.begin() + 1, -3);

                copyableContainer.insert    (1, -3, 3);
                noncopyableContainer.insert (1, -3, 3);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                for (int i = 0; i < 50; ++i)
                    referenceContainer.insert (referenceContainer.end() - 1, -9);

                copyableContainer.insert    (copyableContainer.size()    - 2, -9, 50);
                noncopyableContainer.insert (noncopyableContainer.size() - 2, -9, 50);
            }

            beginTest ("insert array");
            {
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                std::vector<CopyableType>    copyableData    = { 3, 4, 5, 6, 7, 8 };
                std::vector<NoncopyableType> noncopyableData = { 3, 4, 5, 6, 7, 8 };

                std::vector<CopyableType> referenceContainer { copyableData };

                copyableContainer.insertArray    (0, copyableData.data(),    (int) copyableData.size());
                noncopyableContainer.insertArray (0, noncopyableData.data(), (int) noncopyableData.size());

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                int insertPos = copyableContainer.size() - 1;

                for (auto it = copyableData.end(); it != copyableData.begin(); --it)
                    referenceContainer.insert (referenceContainer.begin() + insertPos, CopyableType (*(it - 1)));

                copyableContainer.insertArray    (insertPos, copyableData.data(),    (int) copyableData.size());
                noncopyableContainer.insertArray (insertPos, noncopyableData.data(), (int) noncopyableData.size());

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("remove");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                addData (referenceContainer, copyableContainer, noncopyableContainer, 17);

                for (int i = 0; i < 4; ++i)
                {
                    referenceContainer.erase (referenceContainer.begin() + i);
                    copyableContainer.removeElements (i, 1);
                    noncopyableContainer.removeElements (i, 1);
                }

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                addData (referenceContainer, copyableContainer, noncopyableContainer, 17);
                int blockSize = 3;

                for (int i = 0; i < 4; ++i)
                {
                    for (int j = 0; j < blockSize; ++j)
                        referenceContainer.erase (referenceContainer.begin() + i);

                    copyableContainer.removeElements (i, blockSize);
                    noncopyableContainer.removeElements (i, blockSize);
                }

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);

                auto numToRemove = copyableContainer.size() - 2;

                for (int i = 0; i < numToRemove; ++i)
                    referenceContainer.erase (referenceContainer.begin() + 1);

                copyableContainer.removeElements    (1, numToRemove);
                noncopyableContainer.removeElements (1, numToRemove);

                checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
            }

            beginTest ("move");
            {
                std::vector<CopyableType> referenceContainer;
                ArrayBase<CopyableType,    DummyCriticalSection> copyableContainer;
                ArrayBase<NoncopyableType, DummyCriticalSection> noncopyableContainer;

                addData (referenceContainer, copyableContainer, noncopyableContainer, 6);

                std::vector<std::pair<int, int>> testValues;
                testValues.emplace_back (2, 4);
                testValues.emplace_back (0, 5);
                testValues.emplace_back (4, 1);
                testValues.emplace_back (5, 0);

                for (auto p : testValues)
                {
                    if (p.second > p.first)
                        std::rotate (referenceContainer.begin() + p.first,
                                     referenceContainer.begin() + p.first + 1,
                                     referenceContainer.begin() + p.second + 1);
                    else
                        std::rotate (referenceContainer.begin() + p.second,
                                     referenceContainer.begin() + p.first,
                                     referenceContainer.begin() + p.first + 1);

                    copyableContainer.move    (p.first, p.second);
                    noncopyableContainer.move (p.first, p.second);

                    checkEqual (copyableContainer, noncopyableContainer, referenceContainer);
                }
            }

            beginTest ("After converting move construction, ownership is transferred");
            {
                Derived obj;
                ArrayBase<Derived*, DummyCriticalSection> derived;
                derived.setAllocatedSize (5);
                derived.add (&obj);

                ArrayBase<Base*, DummyCriticalSection> base { std::move (derived) };

                expectEquals (base.capacity(), 5);
                expectEquals (base.size(), 1);
                expect (base.getFirst() == &obj);
                expectEquals (derived.capacity(), 0);
                expectEquals (derived.size(), 0);
                expect (derived.data() == nullptr);
            }

            beginTest ("After converting move assignment, ownership is transferred");
            {
                Derived obj;
                ArrayBase<Derived*, DummyCriticalSection> derived;
                derived.setAllocatedSize (5);
                derived.add (&obj);
                ArrayBase<Base*, DummyCriticalSection> base;

                base = std::move (derived);

                expectEquals (base.capacity(), 5);
                expectEquals (base.size(), 1);
                expect (base.getFirst() == &obj);
                expectEquals (derived.capacity(), 0);
                expectEquals (derived.size(), 0);
                expect (derived.data() == nullptr);
            }
        */
    }
    
    pub fn add_data(
        reference_container:   &mut Vec<CopyableType>,
        copyable_container:    &mut ArrayBase<CopyableType,DummyCriticalSection>,
        noncopyable_container: &mut ArrayBase<NoncopyableType,DummyCriticalSection>,
        num_values:            i32)  {
        
        todo!();
        /*
            for (int i = 0; i < numValues; ++i)
            {
                referenceContainer.push_back ({ i });
                copyableContainer.add ({ i });
                NoncopyableContainer.add ({ i });
            }
        */
    }
    
    pub fn check_equal<A, B>(&mut self, 
        a: &ArrayBase<A,DummyCriticalSection>,
        b: &ArrayBase<B,DummyCriticalSection>)  {
    
        todo!();
        /*
            expectEquals ((int) a.size(), (int) b.size());

            for (int i = 0; i < (int) a.size(); ++i)
                expect (a[i] == b[i]);
        */
    }
    
    pub fn check_equal<A, B>(&mut self, 
        a: &mut ArrayBase<A,DummyCriticalSection>,
        b: &mut Vec<B>)  {
    
        todo!();
        /*
            expectEquals ((int) a.size(), (int) b.size());

            for (int i = 0; i < (int) a.size(); ++i)
                expect (a[i] == b[(size_t) i]);
        */
    }
    
    pub fn check_equal<A, B, C>(&mut self, 
        a: &mut ArrayBase<A,DummyCriticalSection>,
        b: &mut ArrayBase<B,DummyCriticalSection>,
        c: &mut Vec<C>)  {
    
        todo!();
        /*
            checkEqual (a, b);
            checkEqual (a, c);
            checkEqual (b, c);
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static ArrayBaseTests arrayBaseTests;
    */
}

