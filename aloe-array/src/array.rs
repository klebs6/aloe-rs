crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_Array.h]

/**
  | Holds a resizable array of primitive
  | or copy-by-value objects.
  | 
  | Examples of arrays are: Array<int>,
  | Array<Rectangle> or Array<MyClass*>
  | 
  | The Array class can be used to hold simple,
  | non-polymorphic objects as well as
  | primitive types - to do so, the class
  | must fulfill these requirements:
  | 
  | - it must have a copy constructor and
  | assignment operator
  | 
  | - it must be able to be relocated in memory
  | by a memcpy without this causing any
  | problems - so objects whose functionality
  | relies on external pointers or references
  | to themselves can not be used.
  | 
  | You can of course have an array of pointers
  | to any kind of object, e.g. Array<MyClass*>,
  | but if you do this, the array doesn't
  | take any ownership of the objects - see
  | the OwnedArray class or the ReferenceCountedArray
  | class for more powerful ways of holding
  | lists of objects.
  | 
  | For holding lists of strings, you can
  | use Array\<String\>, but it's usually
  | better to use the specialised class
  | StringArray, which provides more useful
  | functions.
  | 
  | To make all the array's methods thread-safe,
  | pass in "CriticalSection" as the templated
  | TypeOfCriticalSectionToUse parameter,
  | instead of the default DummyCriticalSection.
  | 
  | @see OwnedArray, ReferenceCountedArray,
  | StringArray, CriticalSection
  | 
  | @tags{Core}
  |
  */
#[derive(Default)] /** Creates an empty array. */
pub struct 
Array<ElementType, TypeOfCriticalSectionToUse = DummyCriticalSection, const MINIMUM_ALLOCATED_SIZE: i32 = 0> 
{
    values: ArrayBase<ElementType,TypeOfCriticalSectionToUse>,
}

impl<
ElementType: ParameterType, 
    TypeOfCriticalSectionToUse, 
    const MINIMUM_ALLOCATED_SIZE: i32
    >

HasParameterType for Array<ElementType,TypeOfCriticalSectionToUse,MINIMUM_ALLOCATED_SIZE>
{
    type ParameterType = <ElementType as ParameterType>::Type;
}

pub trait HasParameterType {

    type ParameterType;
}

impl<ElementType, TypeOfCriticalSectionToUse: HasScopedLockType, const MINIMUM_ALLOCATED_SIZE: i32> 
HasScopedLockType for Array<ElementType,TypeOfCriticalSectionToUse,MINIMUM_ALLOCATED_SIZE>
{
    /**
      | Returns the type of scoped lock to use
      | for locking this array
      |
      */
    type ScopedLockType = <TypeOfCriticalSectionToUse as HasScopedLockType>::ScopedLockType;
}

pub trait HasScopedLockType {
    type ScopedLockType;
}

impl<ElementType,OtherArrayType> PartialEq<OtherArrayType> for Array<ElementType> {
    
    /**
      | Compares this array to another one.
      | 
      | Two arrays are considered equal if they
      | both contain the same set of elements,
      | in the same order.
      | 
      | -----------
      | @param other
      | 
      | the other array to compare with
      |
      */
    #[inline] fn eq(&self, other: &OtherArrayType) -> bool {
        todo!();
        /*
            const ScopedLockType lock (getLock());
            const typename OtherArrayType::ScopedLockType lock2 (other.getLock());
            return values == other;
        */
    }
}

impl<ElementType> Index<i32> for Array<ElementType> {

    type Output = ElementType;
    
    /**
      | Returns one of the elements in the array.
      | 
      | If the index passed in is beyond the range
      | of valid elements, this will return
      | a default value.
      | 
      | If you're certain that the index will
      | always be a valid element, you can call
      | getUnchecked() instead, which is faster.
      | 
      | -----------
      | @param index
      | 
      | the index of the element being requested
      | (0 is the first element in the array)
      | 
      | @see getUnchecked, getFirst, getLast
      |
      */
    #[inline] fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values.getValueWithDefault (index);
        */
    }
}

impl<ElementType: ParameterType, TypeOfCriticalSectionToUse, const MinimumAllocatedSize: i32> 

Array<ElementType,TypeOfCriticalSectionToUse,MinimumAllocatedSize>
{

    /**
      | Creates a copy of another array.
      | 
      | -----------
      | @param other
      | 
      | the array to copy
      |
      */
    pub fn new_from_other_ref(other: &Array<ElementType>) -> Self {
    
        todo!();
        /*

            const ScopedLockType lock (other.getLock());
            values.addArray (other.values.begin(), other.values.size());
        */
    }
    
    pub fn new_from_other(other: Array<ElementType>) -> Self {
    
        todo!();
        /*
        : values(std::move (other.values)),

        
        */
    }

    /**
      | Initalises from a null-terminated
      | raw array of values.
      | 
      | -----------
      | @param data
      | 
      | the data to copy from
      |
      */
    pub fn new_from_raw<TypeToCreateFrom>(data: *const TypeToCreateFrom) -> Self {
    
        todo!();
        /*

            while (*values != TypeToCreateFrom())
                add (*data++);
        */
    }

    /**
      | Initalises from a raw array of values.
      | 
      | -----------
      | @param data
      | 
      | the data to copy from
      | ----------
      | @param numValues
      | 
      | the number of values in the array
      |
      */
    pub fn new_from_raw_and_len<TypeToCreateFrom>(
        data:       *const TypeToCreateFrom,
        num_values: i32) -> Self {
    
        todo!();
        /*

            values.addArray (data, numValues);
        */
    }

    /**
      | Initalises an Array of size 1 containing
      | a single element.
      |
      */
    pub fn new_from_single_element_ref(single_element_to_add: &ElementType) -> Self {
    
        todo!();
        /*

            add (singleElementToAdd);
        */
    }

    /**
      | Initalises an Array of size 1 containing
      | a single element.
      |
      */
    pub fn new_from_single_element(single_element_to_add: ElementType) -> Self {
    
        todo!();
        /*

            add (std::move (singleElementToAdd));
        */
    }

    /**
      | Initalises an Array from a list of items.
      |
      */
    pub fn new_from_list_of_items_ref<OtherElements>(
        first_new_element: &ElementType,
        other_elements:    OtherElements) -> Self {
    
        todo!();
        /*

            values.add (firstNewElement, std::forward<OtherElements> (otherElements)...);
        */
    }

    /**
      | Initalises an Array from a list of items.
      |
      */
    pub fn new_from_list_of_items<OtherElements>(
        first_new_element: ElementType,
        other_elements:    OtherElements) -> Self {
    
        todo!();
        /*

            values.add (std::move (firstNewElement), std::forward<OtherElements> (otherElements)...);
        */
    }
    
    pub fn new_from_slice<TypeToCreateFrom>(items: &[TypeToCreateFrom]) -> Self {
    
        todo!();
        /*


            addArray (items);
        */
    }

    /**
      | Copies another array.
      | 
      | -----------
      | @param other
      | 
      | the array to copy
      |
      */
    pub fn assign_from_ref(&mut self, other: &Array<ElementType>) -> &mut Array<ElementType> {
        
        todo!();
        /*
            if (this != &other)
            {
                auto otherCopy (other);
                swapWith (otherCopy);
            }

            return *this;
        */
    }
    
    pub fn assign_from(&mut self, other: Array<ElementType>) -> &mut Array<ElementType> {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values = std::move (other.values);
            return *this;
        */
    }

    /**
      | Removes all elements from the array.
      | 
      | This will remove all the elements, and
      | free any storage that the array is using.
      | To clear the array without freeing the
      | storage, use the clearQuick() method
      | instead.
      | 
      | @see clearQuick
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            clearQuick();
            values.setAllocatedSize (0);
        */
    }

    /**
      | Removes all elements from the array
      | without freeing the array's allocated
      | storage. @see clear
      |
      */
    pub fn clear_quick(&mut self)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.clear();
        */
    }

    /**
      | Fills the Array with the provided value.
      |
      */
    pub fn fill(&mut self, new_value: &<Self as HasParameterType>::ParameterType)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            for (auto& e : *this)
                e = newValue;
        */
    }
    
    /**
      | Returns the current number of elements
      | in the array.
      |
      */
    #[inline] pub fn size(&self) -> i32 {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values.size();
        */
    }

    /**
      | Returns true if the array is empty, false
      | otherwise.
      |
      */
    #[inline] pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return size() == 0;
        */
    }

    /**
      | Returns one of the elements in the array,
      | without checking the index passed in.
      | 
      | Unlike the operator[] method, this
      | will try to return an element without
      | checking that the index is within the
      | bounds of the array, so should only be
      | used when you're confident that it will
      | always be a valid index.
      | 
      | -----------
      | @param index
      | 
      | the index of the element being requested
      | (0 is the first element in the array)
      | 
      | @see operator[], getFirst, getLast
      |
      */
    #[inline] pub fn get_unchecked(&self, index: i32) -> ElementType {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values[index];
        */
    }

    /**
      | Returns a direct reference to one of
      | the elements in the array, without checking
      | the index passed in.
      | 
      | This is like getUnchecked, but returns
      | a direct reference to the element, so
      | that you can alter it directly. Obviously
      | this can be dangerous, so only use it
      | when absolutely necessary.
      | 
      | -----------
      | @param index
      | 
      | the index of the element being requested
      | (0 is the first element in the array)
      | @see operator[], getFirst, getLast
      |
      */
    #[inline] pub fn get_reference_mut(&mut self, index: i32) -> &mut ElementType {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values[index];
        */
    }

    /**
      | Returns a direct reference to one of
      | the elements in the array, without checking
      | the index passed in.
      | 
      | This is like getUnchecked, but returns
      | a direct reference to the element. Obviously
      | this can be dangerous, so only use it
      | when absolutely necessary.
      | 
      | -----------
      | @param index
      | 
      | the index of the element being requested
      | (0 is the first element in the array)
      | 
      | @see operator[], getFirst, getLast
      |
      */
    #[inline] pub fn get_reference(&self, index: i32) -> &ElementType {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values[index];
        */
    }

    /**
      | Returns the first element in the array,
      | or a default value if the array is empty.
      | @see operator[], getUnchecked, getLast
      |
      */
    #[inline] pub fn get_first(&self) -> ElementType {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values.getFirst();
        */
    }

    /**
      | Returns the last element in the array,
      | or a default value if the array is empty.
      | 
      | @see operator[], getUnchecked, getFirst
      |
      */
    #[inline] pub fn get_last(&self) -> ElementType {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values.getLast();
        */
    }

    /**
      | Returns a pointer to the actual array
      | data.
      | 
      | This pointer will only be valid until
      | the next time a non-const method is called
      | on the array.
      |
      */
    #[inline] pub fn get_raw_data_pointer_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return values.begin();
        */
    }

    /**
      | Returns a pointer to the actual array
      | data.
      | 
      | This pointer will only be valid until
      | the next time a non-const method is called
      | on the array.
      |
      */
    #[inline] pub fn get_raw_data_pointer(&self) -> *const ElementType {
        
        todo!();
        /*
            return values.begin();
        */
    }

    /**
      | Returns a pointer to the first element
      | in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn begin_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return values.begin();
        */
    }

    /**
      | Returns a pointer to the first element
      | in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn begin(&self) -> *const ElementType {
        
        todo!();
        /*
            return values.begin();
        */
    }

    /**
      | Returns a pointer to the element which
      | follows the last element in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn end_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return values.end();
        */
    }

    /**
      | Returns a pointer to the element which
      | follows the last element in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn end(&self) -> *const ElementType {
        
        todo!();
        /*
            return values.end();
        */
    }

    /**
      | Returns a pointer to the first element
      | in the array.
      | 
      | This method is provided for compatibility
      | with the standard C++ containers.
      |
      */
    #[inline] pub fn data_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return begin();
        */
    }

    /**
      | Returns a pointer to the first element
      | in the array.
      | 
      | This method is provided for compatibility
      | with the standard C++ containers.
      |
      */
    #[inline] pub fn data(&self) -> *const ElementType {
        
        todo!();
        /*
            return begin();
        */
    }

    /**
      | Finds the index of the first element
      | which matches the value passed in.
      | 
      | This will search the array for the given
      | object, and return the index of its first
      | occurrence. If the object isn't found,
      | the method will return -1.
      | 
      | -----------
      | @param elementToLookFor
      | 
      | the value or object to look for
      | 
      | -----------
      | @return
      | 
      | the index of the object, or -1 if it's
      | not found
      |
      */
    pub fn index_of(&self, element_to_look_for: <Self as HasParameterType>::ParameterType) -> i32 {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto e = values.begin();
            auto endPtr = values.end();

            for (; e != endPtr; ++e)
                if (elementToLookFor == *e)
                    return static_cast<int> (e - values.begin());

            return -1;
        */
    }

    /**
      | Returns true if the array contains at
      | least one occurrence of an object.
      | 
      | -----------
      | @param elementToLookFor
      | 
      | the value or object to look for
      | 
      | -----------
      | @return
      | 
      | true if the item is found
      |
      */
    pub fn contains(&self, element_to_look_for: <Self as HasParameterType>::ParameterType) -> bool {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto e = values.begin();
            auto endPtr = values.end();

            for (; e != endPtr; ++e)
                if (elementToLookFor == *e)
                    return true;

            return false;
        */
    }

    /**
      | Appends a new element at the end of the
      | array.
      | 
      | -----------
      | @param newElement
      | 
      | the new object to add to the array
      | 
      | @see set, insert, addIfNotAlreadyThere,
      | addSorted, addUsingDefaultSort,
      | addArray
      |
      */
    pub fn add_ref(&mut self, new_element: &ElementType)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.add (newElement);
        */
    }

    /**
      | Appends a new element at the end of the
      | array.
      | 
      | -----------
      | @param newElement
      | 
      | the new object to add to the array
      | 
      | @see set, insert, addIfNotAlreadyThere,
      | addSorted, addUsingDefaultSort,
      | addArray
      |
      */
    pub fn add_new_element(&mut self, new_element: ElementType)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.add (std::move (newElement));
        */
    }
    
    pub fn add_new_element_by_ref<OtherElements>(
        &mut self, 
        first_new_element: &ElementType,
        other_elements:    OtherElements
    )  {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.add (firstNewElement, std::forward<OtherElements> (otherElements)...);
        */
    }

    /**
      | Appends multiple new elements at the
      | end of the array.
      |
      */
    pub fn add_new_element_by_first_and_others<OtherElements>(
        &mut self, 
        first_new_element: ElementType,
        other_elements:    OtherElements)  {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.add (std::move (firstNewElement), std::forward<OtherElements> (otherElements)...);
        */
    }

    /**
      | Inserts a new element into the array
      | at a given position.
      | 
      | If the index is less than 0 or greater
      | than the size of the array, the element
      | will be added to the end of the array.
      | Otherwise, it will be inserted into
      | the array, moving all the later elements
      | along to make room.
      | 
      | -----------
      | @param indexToInsertAt
      | 
      | the index at which the new element should
      | be inserted (pass in -1 to add it to the
      | end)
      | ----------
      | @param newElement
      | 
      | the new object to add to the array
      | 
      | @see add, addSorted, addUsingDefaultSort,
      | set
      |
      */
    pub fn insert(&mut self, 
        index_to_insert_at: i32,
        new_element:        <Self as HasParameterType>::ParameterType)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.insert (indexToInsertAt, newElement, 1);
        */
    }

    /**
      | Inserts multiple copies of an element
      | into the array at a given position.
      | 
      | If the index is less than 0 or greater
      | than the size of the array, the element
      | will be added to the end of the array.
      | Otherwise, it will be inserted into
      | the array, moving all the later elements
      | along to make room.
      | 
      | -----------
      | @param indexToInsertAt
      | 
      | the index at which the new element should
      | be inserted
      | ----------
      | @param newElement
      | 
      | the new object to add to the array
      | ----------
      | @param numberOfTimesToInsertIt
      | 
      | how many copies of the value to insert
      | 
      | @see insert, add, addSorted, set
      |
      */
    pub fn insert_multiple(&mut self, 
        index_to_insert_at:           i32,
        new_element:                  <Self as HasParameterType>::ParameterType,
        number_of_times_to_insert_it: i32)  {
        
        todo!();
        /*
            if (numberOfTimesToInsertIt > 0)
            {
                const ScopedLockType lock (getLock());
                values.insert (indexToInsertAt, newElement, numberOfTimesToInsertIt);
            }
        */
    }

    /**
      | Inserts an array of values into this
      | array at a given position.
      | 
      | If the index is less than 0 or greater
      | than the size of the array, the new elements
      | will be added to the end of the array.
      | Otherwise, they will be inserted into
      | the array, moving all the later elements
      | along to make room.
      | 
      | -----------
      | @param indexToInsertAt
      | 
      | the index at which the first new element
      | should be inserted
      | ----------
      | @param newElements
      | 
      | the new values to add to the array
      | ----------
      | @param numberOfElements
      | 
      | how many items are in the array
      | 
      | @see insert, add, addSorted, set
      |
      */
    pub fn insert_array(&mut self, 
        index_to_insert_at: i32,
        new_elements:       *const ElementType,
        number_of_elements: i32)  {
        
        todo!();
        /*
            if (numberOfElements > 0)
            {
                const ScopedLockType lock (getLock());
                values.insertArray (indexToInsertAt, newElements, numberOfElements);
            }
        */
    }

    /**
      | Appends a new element at the end of the
      | array as long as the array doesn't already
      | contain it.
      | 
      | If the array already contains an element
      | that matches the one passed in, nothing
      | will be done.
      | 
      | -----------
      | @param newElement
      | 
      | the new object to add to the array
      | 
      | -----------
      | @return
      | 
      | true if the element was added to the array;
      | false otherwise.
      |
      */
    pub fn add_if_not_already_there(&mut self, new_element: <Self as HasParameterType>::ParameterType) -> bool {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (contains (newElement))
                return false;

            add (newElement);
            return true;
        */
    }

    /**
      | Replaces an element with a new value.
      | 
      | If the index is less than zero, this method
      | does nothing. If the index is beyond
      | the end of the array, the item is added
      | to the end of the array.
      | 
      | -----------
      | @param indexToChange
      | 
      | the index whose value you want to change
      | ----------
      | @param newValue
      | 
      | the new value to set for this index.
      | 
      | @see add, insert
      |
      */
    pub fn set(&mut self, 
        index_to_change: i32,
        new_value:       <Self as HasParameterType>::ParameterType)  {
        
        todo!();
        /*
            if (indexToChange >= 0)
            {
                const ScopedLockType lock (getLock());

                if (indexToChange < values.size())
                    values[indexToChange] = newValue;
                else
                    values.add (newValue);
            }
            else
            {
                jassertfalse;
            }
        */
    }

    /**
      | Replaces an element with a new value
      | without doing any bounds-checking.
      | 
      | This just sets a value directly in the
      | array's internal storage, so you'd
      | better make sure it's in range!
      | 
      | -----------
      | @param indexToChange
      | 
      | the index whose value you want to change
      | ----------
      | @param newValue
      | 
      | the new value to set for this index.
      | 
      | @see set, getUnchecked
      |
      */
    pub fn set_unchecked(&mut self, 
        index_to_change: i32,
        new_value:       <Self as HasParameterType>::ParameterType)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            jassert (isPositiveAndBelow (indexToChange, values.size()));
            values[indexToChange] = newValue;
        */
    }

    /**
      | Adds elements from an array to the end
      | of this array.
      | 
      | -----------
      | @param elementsToAdd
      | 
      | an array of some kind of object from which
      | elements can be constructed.
      | ----------
      | @param numElementsToAdd
      | 
      | how many elements are in this other array
      | 
      | @see add
      |
      */
    pub fn add_elements_from_array_to_end_of_this_array<Type>(
        &mut self, 
        elements_to_add:     *const Type,
        num_elements_to_add: i32)  {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (numElementsToAdd > 0)
                values.addArray (elementsToAdd, numElementsToAdd);
        */
    }
    
    pub fn add_array_from_slice<TypeToCreateFrom>(&mut self, items: &[TypeToCreateFrom])  {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.addArray (items);
        */
    }

    /**
      | Adds elements from a null-terminated
      | array of pointers to the end of this array.
      | 
      | -----------
      | @param elementsToAdd
      | 
      | an array of pointers to some kind of object
      | from which elements can be constructed.
      | This array must be terminated by a nullptr
      | @see addArray
      |
      */
    pub fn add_null_terminated_array<Type>(&mut self, elements_to_add: *const *const Type)  {
    
        todo!();
        /*
            int num = 0;

            for (auto e = elementsToAdd; *e != nullptr; ++e)
                ++num;

            addArray (elementsToAdd, num);
        */
    }

    /**
      | This swaps the contents of this array
      | with those of another array.
      | 
      | If you need to exchange two arrays, this
      | is vastly quicker than using copy-by-value
      | because it just swaps their internal
      | pointers.
      |
      */
    pub fn swap_with<OtherArrayType>(&mut self, other_array: &mut OtherArrayType)  {
    
        todo!();
        /*
            const ScopedLockType lock1 (getLock());
            const typename OtherArrayType::ScopedLockType lock2 (otherArray.getLock());
            values.swapWith (otherArray.values);
        */
    }

    /**
      | Adds elements from another array to
      | the end of this array.
      | 
      | -----------
      | @param arrayToAddFrom
      | 
      | the array from which to copy the elements
      | 
      | @see add
      |
      */
    pub fn add_array_from_other_ref<OtherArrayType>(
        &mut self, 
        array_to_add_from: &OtherArrayType
    )
    {
        todo!();
        /*
            const typename OtherArrayType::ScopedLockType lock1 (arrayToAddFrom.getLock());
            const ScopedLockType lock2 (getLock());

            values.addArray (arrayToAddFrom);
        */
    }

    /**
      | Adds elements from another array to
      | the end of this array.
      | 
      | -----------
      | @param arrayToAddFrom
      | 
      | the array from which to copy the elements
      | ----------
      | @param startIndex
      | 
      | the first element of the other array
      | to start copying from
      | ----------
      | @param numElementsToAdd
      | 
      | how many elements to add from the other
      | array. If this value is negative or greater
      | than the number of available elements,
      | all available elements will be copied.
      | @see add
      |
      */
    pub fn add_array_from_other_with_start_and_len<OtherArrayType: NotPointer>(
        &mut self, 
        array_to_add_from:   &OtherArrayType,
        start_index:         i32,
        num_elements_to_add: Option<i32>
    ) 
    {
        let num_elements_to_add: i32 =
            num_elements_to_add.unwrap_or(-1);

        todo!();
        /*
            const typename OtherArrayType::ScopedLockType lock1 (arrayToAddFrom.getLock());
            const ScopedLockType lock2 (getLock());

            values.addArray (arrayToAddFrom, startIndex, numElementsToAdd);
        */
    }

    /**
      | This will enlarge or shrink the array
      | to the given number of elements, by adding
      | or removing items from its end.
      | 
      | If the array is smaller than the given
      | target size, empty elements will be
      | appended until its size is as specified.
      | If its size is larger than the target,
      | items will be removed from its end to
      | shorten it.
      |
      */
    pub fn resize(&mut self, target_num_items: i32)  {
        
        todo!();
        /*
            jassert (targetNumItems >= 0);
            auto numToAdd = targetNumItems - values.size();

            if (numToAdd > 0)
                insertMultiple (values.size(), ElementType(), numToAdd);
            else if (numToAdd < 0)
                removeRange (targetNumItems, -numToAdd);
        */
    }

    /**
      | Inserts a new element into the array,
      | assuming that the array is sorted.
      | 
      | This will use a comparator to find the
      | position at which the new element should
      | go. If the array isn't sorted, the behaviour
      | of this method will be unpredictable.
      | 
      | -----------
      | @param comparator
      | 
      | the comparator to use to compare the
      | elements - see the sort() method for
      | details about the form this object should
      | take
      | ----------
      | @param newElement
      | 
      | the new element to insert to the array
      | 
      | -----------
      | @return
      | 
      | the index at which the new item was added
      | @see addUsingDefaultSort, add, sort
      |
      */
    pub fn add_sorted<ElementComparator>(&mut self, 
        comparator:  &mut ElementComparator,
        new_element: <Self as HasParameterType>::ParameterType) -> i32 {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto index = findInsertIndexInSortedArray (comparator, values.begin(), newElement, 0, values.size());
            insert (index, newElement);
            return index;
        */
    }

    /**
      | Inserts a new element into the array,
      | assuming that the array is sorted.
      | 
      | This will use the DefaultElementComparator
      | class for sorting, so your ElementType
      | must be suitable for use with that class.
      | If the array isn't sorted, the behaviour
      | of this method will be unpredictable.
      | 
      | -----------
      | @param newElement
      | 
      | the new element to insert to the array
      | @see addSorted, sort
      |
      */
    pub fn add_using_default_sort(&mut self, new_element: <Self as HasParameterType>::ParameterType)  {
        
        todo!();
        /*
            DefaultElementComparator <ElementType> comparator;
            addSorted (comparator, newElement);
        */
    }

    /**
      | Finds the index of an element in the array,
      | assuming that the array is sorted.
      | 
      | This will use a comparator to do a binary-chop
      | to find the index of the given element,
      | if it exists. If the array isn't sorted,
      | the behaviour of this method will be
      | unpredictable.
      | 
      | -----------
      | @param comparator
      | 
      | the comparator to use to compare the
      | elements - see the sort() method for
      | details about the form this object should
      | take
      | ----------
      | @param elementToLookFor
      | 
      | the element to search for
      | 
      | -----------
      | @return
      | 
      | the index of the element, or -1 if it's
      | not found @see addSorted, sort
      |
      */
    pub fn index_of_sorted<ElementComparator, TargetValueType>(&self, 
        comparator:          &mut ElementComparator,
        element_to_look_for: TargetValueType) -> i32 {
    
        todo!();
        /*
            ignoreUnused (comparator); // if you pass in an object with a static compareElements() method, this
                                       // avoids getting warning messages about the parameter being unused

            const ScopedLockType lock (getLock());

            for (int s = 0, e = values.size();;)
            {
                if (s >= e)
                    return -1;

                if (comparator.compareElements (elementToLookFor, values[s]) == 0)
                    return s;

                auto halfway = (s + e) / 2;

                if (halfway == s)
                    return -1;

                if (comparator.compareElements (elementToLookFor, values[halfway]) >= 0)
                    s = halfway;
                else
                    e = halfway;
            }
        */
    }

    /**
      | Removes an element from the array.
      | 
      | This will remove the element at a given
      | index, and move back all the subsequent
      | elements to close the gap. If the index
      | passed in is out-of-range, nothing
      | will happen.
      | 
      | -----------
      | @param indexToRemove
      | 
      | the index of the element to remove @see
      | removeAndReturn, removeFirstMatchingValue,
      | removeAllInstancesOf, removeRange
      |
      */
    pub fn remove_index(&mut self, index_to_remove: i32)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (isPositiveAndBelow (indexToRemove, values.size()))
                removeInternal (indexToRemove);
        */
    }

    /**
      | Removes an element from the array.
      | 
      | This will remove the element at a given
      | index, and move back all the subsequent
      | elements to close the gap. If the index
      | passed in is out-of-range, nothing
      | will happen.
      | 
      | -----------
      | @param indexToRemove
      | 
      | the index of the element to remove
      | 
      | -----------
      | @return
      | 
      | the element that has been removed @see
      | removeFirstMatchingValue, removeAllInstancesOf,
      | removeRange
      |
      */
    pub fn remove_and_return(&mut self, index_to_remove: i32) -> ElementType {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (isPositiveAndBelow (indexToRemove, values.size()))
            {
                ElementType removed (values[indexToRemove]);
                removeInternal (indexToRemove);
                return removed;
            }

            return ElementType();
        */
    }

    /**
      | Removes an element from the array.
      | 
      | This will remove the element pointed
      | to by the given iterator, and move back
      | all the subsequent elements to close
      | the gap. If the iterator passed in does
      | not point to an element within the array,
      | behaviour is undefined.
      | 
      | -----------
      | @param elementToRemove
      | 
      | a pointer to the element to remove @see
      | removeFirstMatchingValue, removeAllInstancesOf,
      | removeRange, removeIf
      |
      */
    pub fn remove_element(&mut self, element_to_remove: *const ElementType)  {
        
        todo!();
        /*
            jassert (elementToRemove != nullptr);
            const ScopedLockType lock (getLock());

            jassert (values.begin() != nullptr);
            auto indexToRemove = (int) (elementToRemove - values.begin());

            if (! isPositiveAndBelow (indexToRemove, values.size()))
            {
                jassertfalse;
                return;
            }

            removeInternal (indexToRemove);
        */
    }

    /**
      | Removes an item from the array.
      | 
      | This will remove the first occurrence
      | of the given element from the array.
      | If the item isn't found, no action is
      | taken.
      | 
      | -----------
      | @param valueToRemove
      | 
      | the object to try to remove @see remove,
      | removeRange, removeIf
      |
      */
    pub fn remove_first_matching_value(&mut self, value_to_remove: <Self as HasParameterType>::ParameterType)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto* e = values.begin();

            for (int i = 0; i < values.size(); ++i)
            {
                if (valueToRemove == e[i])
                {
                    removeInternal (i);
                    break;
                }
            }
        */
    }

    /**
      | Removes items from the array.
      | 
      | This will remove all occurrences of
      | the given element from the array. If
      | no such items are found, no action is
      | taken.
      | 
      | -----------
      | @param valueToRemove
      | 
      | the object to try to remove
      | 
      | -----------
      | @return
      | 
      | how many objects were removed. @see
      | remove, removeRange, removeIf
      |
      */
    pub fn remove_all_instances_of(&mut self, value_to_remove: <Self as HasParameterType>::ParameterType) -> i32 {
        
        todo!();
        /*
            int numRemoved = 0;
            const ScopedLockType lock (getLock());

            for (int i = values.size(); --i >= 0;)
            {
                if (valueToRemove == values[i])
                {
                    removeInternal (i);
                    ++numRemoved;
                }
            }

            return numRemoved;
        */
    }

    /**
      | Removes items from the array.
      | 
      | This will remove all objects from the
      | array that match a condition. If no such
      | items are found, no action is taken.
      | 
      | -----------
      | @param predicate
      | 
      | the condition when to remove an item.
      | Must be a callable type that takes an
      | ElementType and returns a bool
      | 
      | -----------
      | @return
      | 
      | how many objects were removed. @see
      | remove, removeRange, removeAllInstancesOf
      |
      */
    pub fn remove_if<PredicateType>(&mut self, predicate: PredicateType) -> i32 {
    
        todo!();
        /*
            int numRemoved = 0;
            const ScopedLockType lock (getLock());

            for (int i = values.size(); --i >= 0;)
            {
                if (predicate (values[i]))
                {
                    removeInternal (i);
                    ++numRemoved;
                }
            }

            return numRemoved;
        */
    }

    /**
      | Removes a range of elements from the
      | array.
      | 
      | This will remove a set of elements, starting
      | from the given index, and move subsequent
      | elements down to close the gap.
      | 
      | If the range extends beyond the bounds
      | of the array, it will be safely clipped
      | to the size of the array.
      | 
      | -----------
      | @param startIndex
      | 
      | the index of the first element to remove
      | ----------
      | @param numberToRemove
      | 
      | how many elements should be removed
      | @see remove, removeFirstMatchingValue,
      | removeAllInstancesOf, removeIf
      |
      */
    pub fn remove_range(&mut self, 
        start_index:      i32,
        number_to_remove: i32)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            auto endIndex = jlimit (0, values.size(), startIndex + numberToRemove);
            startIndex    = jlimit (0, values.size(), startIndex);
            numberToRemove = endIndex - startIndex;

            if (numberToRemove > 0)
            {
                values.removeElements (startIndex, numberToRemove);
                minimiseStorageAfterRemoval();
            }
        */
    }

    /**
      | Removes the last n elements from the
      | array.
      | 
      | -----------
      | @param howManyToRemove
      | 
      | how many elements to remove from the
      | end of the array @see remove, removeFirstMatchingValue,
      | removeAllInstancesOf, removeRange
      |
      */
    pub fn remove_last(&mut self, how_many_to_remove: Option<i32>)  {

        let how_many_to_remove: i32 = how_many_to_remove.unwrap_or(1);

        todo!();
        /*
            jassert (howManyToRemove >= 0);

            if (howManyToRemove > 0)
            {
                const ScopedLockType lock (getLock());

                if (howManyToRemove > values.size())
                    howManyToRemove = values.size();

                values.removeElements (values.size() - howManyToRemove, howManyToRemove);
                minimiseStorageAfterRemoval();
            }
        */
    }


    /**
      | Removes any elements which are also
      | in another array.
      | 
      | -----------
      | @param otherArray
      | 
      | the other array in which to look for elements
      | to remove @see removeValuesNotIn,
      | remove, removeFirstMatchingValue,
      | removeAllInstancesOf, removeRange
      |
      */
    pub fn remove_values_in<OtherArrayType>(&mut self, other_array: &OtherArrayType)  {
    
        todo!();
        /*
            const typename OtherArrayType::ScopedLockType lock1 (otherArray.getLock());
            const ScopedLockType lock2 (getLock());

            if (this == &otherArray)
            {
                clear();
            }
            else
            {
                if (otherArray.size() > 0)
                {
                    for (int i = values.size(); --i >= 0;)
                        if (otherArray.contains (values[i]))
                            removeInternal (i);
                }
            }
        */
    }

    /**
      | Removes any elements which are not found
      | in another array.
      | 
      | Only elements which occur in this other
      | array will be retained.
      | 
      | -----------
      | @param otherArray
      | 
      | the array in which to look for elements
      | NOT to remove @see removeValuesIn,
      | remove, removeFirstMatchingValue,
      | removeAllInstancesOf, removeRange
      |
      */
    pub fn remove_values_not_in<OtherArrayType>(&mut self, other_array: &OtherArrayType)  {
    
        todo!();
        /*
            const typename OtherArrayType::ScopedLockType lock1 (otherArray.getLock());
            const ScopedLockType lock2 (getLock());

            if (this != &otherArray)
            {
                if (otherArray.size() <= 0)
                {
                    clear();
                }
                else
                {
                    for (int i = values.size(); --i >= 0;)
                        if (! otherArray.contains (values[i]))
                            removeInternal (i);
                }
            }
        */
    }

    /**
      | Swaps over two elements in the array.
      | 
      | This swaps over the elements found at
      | the two indexes passed in. If either
      | index is out-of-range, this method
      | will do nothing.
      | 
      | -----------
      | @param index1
      | 
      | index of one of the elements to swap
      | ----------
      | @param index2
      | 
      | index of the other element to swap
      |
      */
    pub fn swap(&mut self, 
        index1: i32,
        index2: i32)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.swap (index1, index2);
        */
    }

    /**
      | Moves one of the values to a different
      | position.
      | 
      | This will move the value to a specified
      | index, shuffling along any intervening
      | elements as required.
      | 
      | So for example, if you have the array
      | { 0, 1, 2, 3, 4, 5 } then calling move (2,
      | 4) would result in { 0, 1, 3, 4, 2, 5 }.
      | 
      | -----------
      | @param currentIndex
      | 
      | the index of the value to be moved. If
      | this isn't a valid index, then nothing
      | will be done
      | ----------
      | @param newIndex
      | 
      | the index at which you'd like this value
      | to end up. If this is less than zero, the
      | value will be moved to the end of the array
      |
      */
    pub fn move_(&mut self, 
        current_index: i32,
        new_index:     i32)  {
        
        todo!();
        /*
            if (currentIndex != newIndex)
            {
                const ScopedLockType lock (getLock());
                values.move (currentIndex, newIndex);
            }
        */
    }

    /**
      | Reduces the amount of storage being
      | used by the array.
      | 
      | Arrays typically allocate slightly
      | more storage than they need, and after
      | removing elements, they may have quite
      | a lot of unused space allocated. This
      | method will reduce the amount of allocated
      | storage to a minimum.
      |
      */
    pub fn minimise_storage_overheads(&mut self)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.shrinkToNoMoreThan (values.size());
        */
    }

    /**
      | Increases the array's internal storage
      | to hold a minimum number of elements.
      | 
      | Calling this before adding a large known
      | number of elements means that the array
      | won't have to keep dynamically resizing
      | itself as the elements are added, and
      | it'll therefore be more efficient.
      |
      */
    pub fn ensure_storage_allocated(&mut self, min_num_elements: i32)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.ensureAllocatedSize (minNumElements);
        */
    }

    /**
      | Sorts the array using a default comparison
      | operation.
      | 
      | If the type of your elements isn't supported
      | by the DefaultElementComparator class
      | then you may need to use the other version
      | of sort, which takes a custom comparator.
      |
      */
    pub fn sort_with_default_compare(&mut self)  {
        
        todo!();
        /*
            DefaultElementComparator<ElementType> comparator;
            sort (comparator);
        */
    }

    /**
      | Sorts the elements in the array.
      | 
      | This will use a comparator object to
      | sort the elements into order. The object
      | passed must have a method of the form:
      | 
      | @code
      | 
      | int compareElements (ElementType
      | first, ElementType second);
      | 
      | @endcode
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
      | the comparator to use for comparing
      | elements.
      | ----------
      | @param retainOrderOfEquivalentItems
      | 
      | if this is true, then items which the
      | comparator says are equivalent will
      | be kept in the order in which they currently
      | appear in the array. This is slower to
      | perform, but may be important in some
      | cases. If it's false, a faster algorithm
      | is used, but equivalent elements may
      | be rearranged.
      | 
      | @see addSorted, indexOfSorted, sortArray
      |
      */
    pub fn sort<ElementComparator>(
        &mut self, 
        comparator:                       &mut ElementComparator,
        retain_order_of_equivalent_items: Option<bool>
    ) 
    {
        let retain_order_of_equivalent_items: bool =
            retain_order_of_equivalent_items.unwrap_or(false);

        todo!();
        /*
            const ScopedLockType lock (getLock());
            ignoreUnused (comparator); // if you pass in an object with a static compareElements() method, this
                                       // avoids getting warning messages about the parameter being unused
            sortArray (comparator, values.begin(), 0, size() - 1, retainOrderOfEquivalentItems);
        */
    }
    
    /**
      | Returns the CriticalSection that locks
      | this array.
      | 
      | To lock, you can call getLock().enter()
      | and getLock().exit(), or preferably
      | use an object of ScopedLockType as an
      | RAII lock for it.
      |
      */
    #[inline] pub fn get_lock(&self) -> &TypeOfCriticalSectionToUse {
        
        todo!();
        /*
            return values;
        */
    }
    
    pub fn remove_internal(&mut self, index_to_remove: i32)  {
        
        todo!();
        /*
            values.removeElements (indexToRemove, 1);
            minimiseStorageAfterRemoval();
        */
    }
    
    pub fn minimise_storage_after_removal(&mut self)  {
        
        todo!();
        /*
            if (values.capacity() > jmax (MINIMUM_ALLOCATED_SIZE, values.size() * 2))
                values.shrinkToNoMoreThan (jmax (values.size(), jmax (MINIMUM_ALLOCATED_SIZE, 64 / (int) sizeof (ElementType))));
        */
    }
}

