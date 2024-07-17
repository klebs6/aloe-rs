crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_SortedSet.h]

/**
  | Holds a set of unique primitive objects,
  | such as ints or doubles.
  | 
  | A set can only hold one item with a given
  | value, so if for example it's a set of
  | integers, attempting to add the same
  | integer twice will do nothing the second
  | time.
  | 
  | Internally, the list of items is kept
  | sorted (which means that whatever kind
  | of primitive type is used must support
  | the ==, <, >, <= and >= operators to determine
  | the order), and searching the set for
  | known values is very fast because it
  | uses a binary-chop method.
  | 
  | Note that if you're using a class or struct
  | as the element type, it must be capable
  | of being copied or moved with a straightforward
  | memcpy, rather than needing construction
  | and destruction code.
  | 
  | To make all the set's methods thread-safe,
  | pass in "CriticalSection" as the templated
  | TypeOfCriticalSectionToUse parameter,
  | instead of the default DummyCriticalSection.
  | 
  | @see Array, OwnedArray, ReferenceCountedArray,
  | StringArray, CriticalSection
  | 
  | @tags{Core}
  |
  */
#[derive(Default)] /** Creates an empty set. */
pub struct SortedSet<ElementType,TypeOfCriticalSectionToUse = DummyCriticalSection> {
    data: Array<ElementType,TypeOfCriticalSectionToUse>,
}

impl<ElementType,TypeOfCriticalSectionToUse: HasScopedLockType> 

HasScopedLockType for SortedSet<ElementType,TypeOfCriticalSectionToUse> {

    /**
      | Returns the type of scoped lock to use
      | for locking this array
      |
      */
    type ScopedLockType = <TypeOfCriticalSectionToUse as HasScopedLockType>::ScopedLockType;
}

/**
  | Compares this set to another one. Two
  | sets are considered equal if they both
  | contain the same set of elements.
  | 
  | -----------
  | @param other
  | 
  | the other set to compare with
  |
  */
impl<ElementSet> PartialEq<SortedSet<ElementSet>> for SortedSet<ElementSet> {
    
    #[inline] fn eq(&self, other: &SortedSet<ElementSet>) -> bool {
        todo!();
        /*
            return data == other.data;
        */
    }
}

impl<ElementSet> Eq for SortedSet<ElementSet> {}

impl<ElementType> Index<i32> for SortedSet<ElementType> {

    type Output = ElementType;
    
    /**
      | Returns one of the elements in the set.
      | 
      | If the index passed in is beyond the range
      | of valid elements, this will return
      | zero.
      | 
      | If you're certain that the index will
      | always be a valid element, you can call
      | getUnchecked() instead, which is faster.
      | 
      | -----------
      | @param index
      | 
      | the index of the element being requested
      | (0 is the first element in the set) @see
      | getUnchecked, getFirst, getLast
      |
      */
    fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            return data [index];
        */
    }
}

impl<ElementType,TypeOfCriticalSectionToUse>

SortedSet<ElementType,TypeOfCriticalSectionToUse> {

    /**
      | Removes all elements from the set.
      | 
      | This will remove all the elements, and
      | free any storage that the set is using.
      | To clear it without freeing the storage,
      | use the clearQuick() method instead.
      | 
      | @see clearQuick
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            data.clear();
        */
    }

    /**
      | Removes all elements from the set without
      | freeing the array's allocated storage.
      | @see clear
      |
      */
    pub fn clear_quick(&mut self)  {
        
        todo!();
        /*
            data.clearQuick();
        */
    }
    
    /**
      | Returns the current number of elements
      | in the set.
      |
      */
    #[inline] pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return data.size();
        */
    }

    /**
      | Returns true if the set is empty, false
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
      | Returns one of the elements in the set,
      | without checking the index passed in.
      | Unlike the operator[] method, this
      | will try to return an element without
      | checking that the index is within the
      | bounds of the set, so should only be used
      | when you're confident that it will always
      | be a valid index.
      | 
      | -----------
      | @param index
      | 
      | the index of the element being requested
      | (0 is the first element in the set) @see
      | operator[], getFirst, getLast
      |
      */
    #[inline] pub fn get_unchecked(&self, index: i32) -> ElementType {
        
        todo!();
        /*
            return data.getUnchecked (index);
        */
    }

    /**
      | Returns a direct reference to one of
      | the elements in the set, without checking
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
      |
      */
    #[inline] pub fn get_reference_mut(&mut self, index: i32) -> &mut ElementType {
        
        todo!();
        /*
            return data.getReference (index);
        */
    }

    /**
      | Returns a direct reference to one of
      | the elements in the set, without checking
      | the index passed in.
      | 
      | -----------
      | @param index
      | 
      | the index of the element being requested
      | (0 is the first element in the array)
      |
      */
    #[inline] pub fn get_reference(&self, index: i32) -> &ElementType {
        
        todo!();
        /*
            return data.getReference (index);
        */
    }

    /**
      | Returns the first element in the set,
      | or 0 if the set is empty. @see operator[],
      | getUnchecked, getLast
      |
      */
    #[inline] pub fn get_first(&self) -> ElementType {
        
        todo!();
        /*
            return data.getFirst();
        */
    }

    /**
      | Returns the last element in the set,
      | or 0 if the set is empty.
      | 
      | @see operator[], getUnchecked, getFirst
      |
      */
    #[inline] pub fn get_last(&self) -> ElementType {
        
        todo!();
        /*
            return data.getLast();
        */
    }

    /**
      | Returns a pointer to the first element
      | in the set.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn begin(&self) -> *const ElementType {
        
        todo!();
        /*
            return data.begin();
        */
    }

    /**
      | Returns a pointer to the element which
      | follows the last element in the set.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn end(&self) -> *const ElementType {
        
        todo!();
        /*
            return data.end();
        */
    }

    /**
      | Finds the index of the first element
      | which matches the value passed in.
      | 
      | This will search the set for the given
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
    pub fn index_of(&self, element_to_look_for: &ElementType) -> i32 {
        
        todo!();
        /*
            const ScopedLockType lock (data.getLock());

            int s = 0;
            int e = data.size();

            for (;;)
            {
                if (s >= e)
                    return -1;

                if (elementToLookFor == data.getReference (s))
                    return s;

                auto halfway = (s + e) / 2;

                if (halfway == s)
                    return -1;

                if (elementToLookFor < data.getReference (halfway))
                    e = halfway;
                else
                    s = halfway;
            }
        */
    }

    /**
      | Returns true if the set contains at least
      | one occurrence of an object.
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
    pub fn contains(&self, element_to_look_for: &ElementType) -> bool {
        
        todo!();
        /*
            return indexOf (elementToLookFor) >= 0;
        */
    }

    /**
      | Adds a new element to the set, (as long
      | as it's not already in there).
      | 
      | Note that if a matching element already
      | exists, the new value will be assigned
      | to the existing one using operator=,
      | so that if there are any differences
      | between the objects which were not recognised
      | by the object's operator==, then the
      | set will always contain a copy of the
      | most recently added one.
      | 
      | -----------
      | @param newElement
      | 
      | the new object to add to the set
      | 
      | -----------
      | @return
      | 
      | true if the value was added, or false
      | if it already existed @see set, insert,
      | addIfNotAlreadyThere, addSorted,
      | addSet, addArray
      |
      */
    pub fn add(&mut self, new_element: &ElementType) -> bool {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            int s = 0;
            int e = data.size();

            while (s < e)
            {
                auto& elem = data.getReference (s);

                if (newElement == elem)
                {
                    elem = newElement; // force an update in case operator== permits differences.
                    return false;
                }

                auto halfway = (s + e) / 2;
                bool isBeforeHalfway = (newElement < data.getReference (halfway));

                if (halfway == s)
                {
                    if (! isBeforeHalfway)
                        ++s;

                    break;
                }

                if (isBeforeHalfway)
                    e = halfway;
                else
                    s = halfway;
            }

            data.insert (s, newElement);
            return true;
        */
    }

    /**
      | Adds elements from an array to this set.
      | 
      | -----------
      | @param elementsToAdd
      | 
      | the array of elements to add
      | ----------
      | @param numElementsToAdd
      | 
      | how many elements are in this other array
      | @see add
      |
      */
    pub fn add_array(&mut self, 
        elements_to_add:     *const ElementType,
        num_elements_to_add: i32)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            while (--numElementsToAdd >= 0)
                add (*elementsToAdd++);
        */
    }

    /**
      | Adds elements from another set to this
      | one.
      | 
      | -----------
      | @param setToAddFrom
      | 
      | the set from which to copy the elements
      | ----------
      | @param startIndex
      | 
      | the first element of the other set to
      | start copying from
      | ----------
      | @param numElementsToAdd
      | 
      | how many elements to add from the other
      | set. If this value is negative or greater
      | than the number of available elements,
      | all available elements will be copied.
      | @see add
      |
      */
    pub fn add_set<OtherSetType>(
        &mut self, 
        set_to_add_from:     &OtherSetType,
        start_index:         Option<i32>,
        num_elements_to_add: Option<i32>) 
    {
        let start_index: i32 = start_index.unwrap_or(0);

        let num_elements_to_add: i32 =
            num_elements_to_add.unwrap_or(-1);

        todo!();
        /*
            const typename OtherSetType::ScopedLockType lock1 (setToAddFrom.getLock());
            const ScopedLockType lock2 (getLock());
            jassert (this != &setToAddFrom);

            if (this != &setToAddFrom)
            {
                if (startIndex < 0)
                {
                    jassertfalse;
                    startIndex = 0;
                }

                if (numElementsToAdd < 0 || startIndex + numElementsToAdd > setToAddFrom.size())
                    numElementsToAdd = setToAddFrom.size() - startIndex;

                if (numElementsToAdd > 0)
                    addArray (&setToAddFrom.data.getReference (startIndex), numElementsToAdd);
            }
        */
    }

    /**
      | Removes an element from the set.
      | 
      | This will remove the element at a given
      | index. If the index passed in is out-of-range,
      | nothing will happen.
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
      | removeValue, removeRange
      |
      */
    pub fn remove(&mut self, index_to_remove: i32) -> ElementType {
        
        todo!();
        /*
            return data.removeAndReturn (indexToRemove);
        */
    }

    /**
      | Removes an item from the set.
      | 
      | This will remove the given element from
      | the set, if it's there.
      | 
      | -----------
      | @param valueToRemove
      | 
      | the object to try to remove @see remove,
      | removeRange
      |
      */
    pub fn remove_value(&mut self, value_to_remove: &ElementType)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            data.remove (indexOf (valueToRemove));
        */
    }

    /**
      | Removes any elements which are also
      | in another set.
      | 
      | -----------
      | @param otherSet
      | 
      | the other set in which to look for elements
      | to remove @see removeValuesNotIn,
      | remove, removeValue, removeRange
      |
      */
    pub fn remove_values_in<OtherSetType>(&mut self, other_set: &OtherSetType)  {
    
        todo!();
        /*
            const typename OtherSetType::ScopedLockType lock1 (otherSet.getLock());
            const ScopedLockType lock2 (getLock());

            if (this == &otherSet)
            {
                clear();
            }
            else if (! otherSet.isEmpty())
            {
                for (int i = data.size(); --i >= 0;)
                    if (otherSet.contains (data.getReference (i)))
                        remove (i);
            }
        */
    }

    /**
      | Removes any elements which are not found
      | in another set.
      | 
      | Only elements which occur in this other
      | set will be retained.
      | 
      | -----------
      | @param otherSet
      | 
      | the set in which to look for elements
      | NOT to remove @see removeValuesIn,
      | remove, removeValue, removeRange
      |
      */
    pub fn remove_values_not_in<OtherSetType>(&mut self, other_set: &OtherSetType)  {
    
        todo!();
        /*
            const typename OtherSetType::ScopedLockType lock1 (otherSet.getLock());
            const ScopedLockType lock2 (getLock());

            if (this != &otherSet)
            {
                if (otherSet.isEmpty())
                {
                    clear();
                }
                else
                {
                    for (int i = data.size(); --i >= 0;)
                        if (! otherSet.contains (data.getReference (i)))
                            remove (i);
                }
            }
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
    pub fn swap_with<OtherSetType>(&mut self, other_set: &mut OtherSetType)  {
    
        todo!();
        /*
            data.swapWith (otherSet.data);
        */
    }

    /**
      | Reduces the amount of storage being
      | used by the set.
      | 
      | Sets typically allocate slightly more
      | storage than they need, and after removing
      | elements, they may have quite a lot of
      | unused space allocated.
      | 
      | This method will reduce the amount of
      | allocated storage to a minimum.
      |
      */
    pub fn minimise_storage_overheads(&mut self)  {
        
        todo!();
        /*
            data.minimiseStorageOverheads();
        */
    }

    /**
      | Increases the set's internal storage
      | to hold a minimum number of elements.
      | 
      | Calling this before adding a large known
      | number of elements means that the set
      | won't have to keep dynamically resizing
      | itself as the elements are added, and
      | it'll therefore be more efficient.
      |
      */
    pub fn ensure_storage_allocated(&mut self, min_num_elements: i32)  {
        
        todo!();
        /*
            data.ensureStorageAllocated (minNumElements);
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
            return data.getLock();
        */
    }
}
