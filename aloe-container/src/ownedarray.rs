crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_OwnedArray.h]

/**
  | An array designed for holding objects.
  | 
  | This holds a list of pointers to objects,
  | and will automatically delete the objects
  | when they are removed from the array,
  | or when the array is itself deleted.
  | 
  | Declare it in the form: OwnedArray<MyObjectClass>
  | 
  | ..and then add new objects, e.g. myOwnedArray.add
  | (new MyObjectClass());
  | 
  | After adding objects, they are 'owned'
  | by the array and will be deleted when
  | removed or replaced.
  | 
  | To make all the array's methods thread-safe,
  | pass in "CriticalSection" as the templated
  | TypeOfCriticalSectionToUse parameter,
  | instead of the default DummyCriticalSection.
  | 
  | @see Vec, ReferenceCountedArray,
  | StringArray, CriticalSection
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OwnedArray<ObjectClass,TypeOfCriticalSectionToUse = DummyCriticalSection> {
    values: ArrayBase<*mut ObjectClass,TypeOfCriticalSectionToUse>,
}

impl<ObjectClass,TypeOfCriticalSectionToUse> 
Default for OwnedArray<ObjectClass,TypeOfCriticalSectionToUse>
{
    /** Creates an empty array. */
    fn default() -> Self {
        todo!();
    }
}

impl<ObjectClass,TypeOfCriticalSectionToUse: HasScopedLockType> 

HasScopedLockType for OwnedArray<ObjectClass,TypeOfCriticalSectionToUse> {

    /**
      | Returns the type of scoped lock to use
      | for locking this array
      |
      */
    type ScopedLockType = <TypeOfCriticalSectionToUse as HasScopedLockType>::ScopedLockType;
}

//-------------------------------
impl<ObjectClass,TypeOfCriticalSectionToUse> 

Drop for OwnedArray<ObjectClass,TypeOfCriticalSectionToUse> {

    /**
      | Deletes the array and also deletes any
      | objects inside it.
      | 
      | To get rid of the array without deleting
      | its objects, use its clear (false) method
      | before deleting it.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
            deleteAllObjects();
         */
    }
}

impl<ObjectClass> Index<i32> for OwnedArray<ObjectClass> {

    type Output = ObjectClass;
    
    /**
      | Returns a pointer to the object at this
      | index in the array.
      | 
      | If the index is out-of-range, this will
      | return a null pointer, (and it could
      | be null anyway, because it's ok for the
      | array to hold null pointers as well as
      | objects).
      | 
      | @see getUnchecked
      |
      */
    fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values.getValueWithDefault (index);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_OwnedArray.cpp]
impl<ObjectClass,TypeOfCriticalSectionToUse> OwnedArray<ObjectClass,TypeOfCriticalSectionToUse> {

    /**
      | Move constructor.
      |
      */
    pub fn new(other: OwnedArray<ObjectClass>) -> Self {
    
        todo!();
        /*
        : values(std::move (other.values)),

        
        */
    }

    /**
      | Creates an array from a list of objects.
      |
      */
    pub fn new_from_slice(items: &[*mut ObjectClass]) -> Self {
    
        todo!();
        /*
            addArray (items);
        */
    }

    /**
      | Move assignment operator.
      |
      */
    pub fn assign_from(&mut self, other: OwnedArray<ObjectClass>) -> &mut OwnedArray<ObjectClass> {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            deleteAllObjects();
            values = std::move (other.values);
            return *this;
        */
    }

    /**
      | Converting move constructor.
      |
      */
    pub fn new_converting<OtherObjectClass, OtherCriticalSection>(other: OwnedArray<OtherObjectClass,OtherCriticalSection>) -> Self {
    
        todo!();
        /*


            : values (std::move (other.values))
        */
    }

    /**
      | Converting move assignment operator.
      |
      */
    pub fn assign_from_converting<OtherObjectClass, OtherCriticalSection>(&mut self, other: OwnedArray<OtherObjectClass,OtherCriticalSection>) -> &mut OwnedArray<ObjectClass> {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            deleteAllObjects();
            values = std::move (other.values);
            return *this;
        */
    }
    
    /**
      | Clears the array, optionally deleting
      | the objects inside it first.
      |
      */
    pub fn clear(&mut self, delete_objects: Option<bool>)  {

        let delete_objects: bool = delete_objects.unwrap_or(true);

        todo!();
        /*
            const ScopedLockType lock (getLock());
            clearQuick (deleteObjects);
            values.setAllocatedSize (0);
        */
    }
    
    /**
      | Clears the array, optionally deleting
      | the objects inside it first.
      |
      */
    pub fn clear_quick(&mut self, delete_objects: bool)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (deleteObjects)
                deleteAllObjects();
            else
                values.clear();
        */
    }

    /**
      | Returns the number of items currently
      | in the array. @see operator[]
      |
      */
    #[inline] pub fn size(&self) -> i32 {
        
        todo!();
        /*
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
      | Returns a pointer to the object at this
      | index in the array, without checking
      | whether the index is in-range.
      | 
      | This is a faster and less safe version
      | of operator[] which doesn't check the
      | index passed in, so it can be used when
      | you're sure the index is always going
      | to be legal.
      |
      */
    #[inline] pub fn get_unchecked(&self, index: i32) -> *mut ObjectClass {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values[index];
        */
    }

    /**
      | Returns a pointer to the first object
      | in the array.
      | 
      | This will return a null pointer if the
      | array's empty. @see getLast
      |
      */
    #[inline] pub fn get_first(&self) -> *mut ObjectClass {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values.getFirst();
        */
    }

    /**
      | Returns a pointer to the last object
      | in the array.
      | 
      | This will return a null pointer if the
      | array's empty. @see getFirst
      |
      */
    #[inline] pub fn get_last(&self) -> *mut ObjectClass {
        
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
    #[inline] pub fn get_raw_data_pointer(&mut self) -> *mut *mut ObjectClass {
        
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
    #[inline] pub fn begin_mut(&mut self) -> *mut *mut ObjectClass {
        
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
    #[inline] pub fn begin(&self) -> *mut *mut ObjectClass {
        
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
    #[inline] pub fn end_mut(&mut self) -> *mut *mut ObjectClass {
        
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
    #[inline] pub fn end(&self) -> *const *const ObjectClass {
        
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
    #[inline] pub fn data_mut(&mut self) -> *mut *mut ObjectClass {
        
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
    #[inline] pub fn data(&self) -> *const *const ObjectClass {
        
        todo!();
        /*
            return begin();
        */
    }

    /**
      | Finds the index of an object which might
      | be in the array.
      | 
      | -----------
      | @param objectToLookFor
      | 
      | the object to look for
      | 
      | -----------
      | @return
      | 
      | the index at which the object was found,
      | or -1 if it's not found
      |
      */
    pub fn index_of(&self, object_to_look_for: *const ObjectClass) -> i32 {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto* e = values.begin();

            for (; e != values.end(); ++e)
                if (objectToLookFor == *e)
                    return static_cast<int> (e - values.begin());

            return -1;
        */
    }

    /**
      | Returns true if the array contains a
      | specified object.
      | 
      | -----------
      | @param objectToLookFor
      | 
      | the object to look for
      | 
      | -----------
      | @return
      | 
      | true if the object is in the array
      |
      */
    pub fn contains(&self, object_to_look_for: *const ObjectClass) -> bool {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto* e = values.begin();

            for (; e != values.end(); ++e)
                if (objectToLookFor == *e)
                    return true;

            return false;
        */
    }

    /**
      | Appends a new object to the end of the
      | array.
      | 
      | Note that this object will be deleted
      | by the OwnedArray when it is removed,
      | so be careful not to delete it somewhere
      | else.
      | 
      | Also be careful not to add the same object
      | to the array more than once, as this will
      | obviously cause deletion of dangling
      | pointers.
      | 
      | -----------
      | @param newObject
      | 
      | the new object to add to the array
      | 
      | -----------
      | @return
      | 
      | the new object that was added @see set,
      | insert, addSorted
      |
      */
    pub fn add_raw(&mut self, new_object: *mut ObjectClass) -> *mut ObjectClass {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.add (newObject);
            return newObject;
        */
    }

    /**
      | Appends a new object to the end of the
      | array.
      | 
      | Note that this object will be deleted
      | by the OwnedArray when it is removed,
      | so be careful not to delete it somewhere
      | else.
      | 
      | Also be careful not to add the same object
      | to the array more than once, as this will
      | obviously cause deletion of dangling
      | pointers.
      | 
      | -----------
      | @param newObject
      | 
      | the new object to add to the array
      | 
      | -----------
      | @return
      | 
      | the new object that was added @see set,
      | insert, addSorted
      |
      */
    pub fn add_boxed(&mut self, new_object: Box<ObjectClass>) -> *mut ObjectClass {
        
        todo!();
        /*
            return add (newObject.release());
        */
    }

    /**
      | Inserts a new object into the array at
      | the given index.
      | 
      | Note that this object will be deleted
      | by the OwnedArray when it is removed,
      | so be careful not to delete it somewhere
      | else.
      | 
      | If the index is less than 0 or greater
      | than the size of the array, the element
      | will be added to the end of the array.
      | Otherwise, it will be inserted into
      | the array, moving all the later elements
      | along to make room.
      | 
      | Be careful not to add the same object
      | to the array more than once, as this will
      | obviously cause deletion of dangling
      | pointers.
      | 
      | -----------
      | @param indexToInsertAt
      | 
      | the index at which the new element should
      | be inserted
      | ----------
      | @param newObject
      | 
      | the new object to add to the array
      | 
      | -----------
      | @return
      | 
      | the new object that was added @see add,
      | addSorted, set
      |
      */
    pub fn insert_raw_at_index(
        &mut self, 
        index_to_insert_at: i32,
        new_object:         *mut ObjectClass) -> *mut ObjectClass {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.insert (indexToInsertAt, newObject, 1);
            return newObject;
        */
    }

    /**
      | Inserts a new object into the array at
      | the given index.
      | 
      | Note that this object will be deleted
      | by the OwnedArray when it is removed,
      | so be careful not to delete it somewhere
      | else.
      | 
      | If the index is less than 0 or greater
      | than the size of the array, the element
      | will be added to the end of the array.
      | Otherwise, it will be inserted into
      | the array, moving all the later elements
      | along to make room.
      | 
      | Be careful not to add the same object
      | to the array more than once, as this will
      | obviously cause deletion of dangling
      | pointers.
      | 
      | -----------
      | @param indexToInsertAt
      | 
      | the index at which the new element should
      | be inserted
      | ----------
      | @param newObject
      | 
      | the new object to add to the array
      | 
      | -----------
      | @return
      | 
      | the new object that was added @see add,
      | addSorted, set
      |
      */
    pub fn insert_boxed_at_index(
        &mut self, 
        index_to_insert_at: i32,
        new_object:         Box<ObjectClass>) -> *mut ObjectClass {
        
        todo!();
        /*
            return insert (indexToInsertAt, newObject.release());
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
      | @param newObjects
      | 
      | the new values to add to the array
      | ----------
      | @param numberOfElements
      | 
      | how many items are in the array @see insert,
      | add, addSorted, set
      |
      */
    pub fn insert_array(
        &mut self, 
        index_to_insert_at: i32,
        new_objects:        *const *const ObjectClass,
        number_of_elements: i32)  {
        
        todo!();
        /*
            if (numberOfElements > 0)
            {
                const ScopedLockType lock (getLock());
                values.insertArray (indexToInsertAt, newObjects, numberOfElements);
            }
        */
    }

    /**
      | Replaces an object in the array with
      | a different one.
      | 
      | If the index is less than zero, this method
      | does nothing. If the index is beyond
      | the end of the array, the new object is
      | added to the end of the array.
      | 
      | Be careful not to add the same object
      | to the array more than once, as this will
      | obviously cause deletion of dangling
      | pointers.
      | 
      | -----------
      | @param indexToChange
      | 
      | the index whose value you want to change
      | ----------
      | @param newObject
      | 
      | the new value to set for this index.
      | ----------
      | @param deleteOldElement
      | 
      | whether to delete the object that's
      | being replaced with the new one @see
      | add, insert, remove
      |
      */
    pub fn set_with_new_object_raw(
        &mut self, 
        index_to_change:    i32,
        new_object:         *mut ObjectClass,
        delete_old_element: Option<bool>

    ) -> *mut ObjectClass {

        let delete_old_element: bool = delete_old_element.unwrap_or(true);

        todo!();
        /*
            if (indexToChange >= 0)
            {
                std::unique_ptr<ObjectClass> toDelete;

                {
                    const ScopedLockType lock (getLock());

                    if (indexToChange < values.size())
                    {
                        if (deleteOldElement)
                        {
                            toDelete.reset (values[indexToChange]);

                            if (toDelete.get() == newObject)
                                toDelete.release();
                        }

                        values[indexToChange] = newObject;
                    }
                    else
                    {
                        values.add (newObject);
                    }
                }
            }
            else
            {
                jassertfalse; // you're trying to set an object at a negative index, which doesn't have
                              // any effect - but since the object is not being added, it may be leaking..
            }

            return newObject;
        */
    }

    /**
      | Replaces an object in the array with
      | a different one.
      | 
      | If the index is less than zero, this method
      | does nothing.
      | 
      | If the index is beyond the end of the array,
      | the new object is added to the end of the
      | array.
      | 
      | Be careful not to add the same object
      | to the array more than once, as this will
      | obviously cause deletion of dangling
      | pointers.
      | 
      | -----------
      | @param indexToChange
      | 
      | the index whose value you want to change
      | ----------
      | @param newObject
      | 
      | the new value to set for this index.
      | ----------
      | @param deleteOldElement
      | 
      | whether to delete the object that's
      | being replaced with the new one @see
      | add, insert, remove
      |
      */
    pub fn set_at_index_and_new_object(
        &mut self, 
        index_to_change:    i32,
        new_object:         Box<ObjectClass>,
        delete_old_element: Option<bool>

    ) -> *mut ObjectClass {

        let delete_old_element: bool = delete_old_element.unwrap_or(true);

        todo!();
        /*
            return set (indexToChange, newObject.release(), deleteOldElement);
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
    pub fn add_array<OtherArrayType>(
        &mut self, 
        array_to_add_from:   &OtherArrayType,
        start_index:         Option<i32>,
        num_elements_to_add: Option<i32>
    ) 
    {
        let start_index: i32 = start_index.unwrap_or(0);

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
      | Adds elements from another array to
      | the end of this array.
      |
      */
    pub fn add_array_from_slice<OtherArrayType>(&mut self, items: &[OtherArrayType])  {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.addArray (items);
        */
    }

    /**
      | Adds copies of the elements in another
      | array to the end of this array.
      | 
      | The other array must be either an OwnedArray
      | of a compatible type of object, or an
      | Vec containing pointers to the same
      | kind of object. The objects involved
      | must provide a copy constructor, and
      | this will be used to create new copies
      | of each element, and add them to this
      | array.
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
    pub fn add_copies_of<OtherArrayType>(
        &mut self, 
        array_to_add_from:   &OtherArrayType,
        start_index:         Option<i32>,
        num_elements_to_add: Option<i32>
    )
    {
        let start_index: i32 = start_index.unwrap_or(0);

        let num_elements_to_add: i32 =
            num_elements_to_add.unwrap_or(-1);

        todo!();
        /*
            const typename OtherArrayType::ScopedLockType lock1 (arrayToAddFrom.getLock());
            const ScopedLockType lock2 (getLock());

            if (startIndex < 0)
            {
                jassertfalse;
                startIndex = 0;
            }

            if (numElementsToAdd < 0 || startIndex + numElementsToAdd > arrayToAddFrom.size())
                numElementsToAdd = arrayToAddFrom.size() - startIndex;

            jassert (numElementsToAdd >= 0);
            values.ensureAllocatedSize (values.size() + numElementsToAdd);

            while (--numElementsToAdd >= 0)
                values.add (createCopyIfNotNull (arrayToAddFrom.getUnchecked (startIndex++)));
        */
    }

    /**
      | Inserts a new object into the array assuming
      | that the array is sorted.
      | 
      | This will use a comparator to find the
      | position at which the new object should
      | go. If the array isn't sorted, the behaviour
      | of this method will be unpredictable.
      | 
      | -----------
      | @param comparator
      | 
      | the comparator to use to compare the
      | elements - see the sort method for details
      | about this object's structure
      | ----------
      | @param newObject
      | 
      | the new object to insert to the array
      | 
      | -----------
      | @return
      | 
      | the index at which the new object was
      | added @see add, sort, indexOfSorted
      |
      */
    pub fn add_sorted<ElementComparator>(&mut self, 
        comparator: &mut ElementComparator,
        new_object: *mut ObjectClass) -> i32 {
    
        todo!();
        /*
            // If you pass in an object with a static compareElements() method, this
            // avoids getting warning messages about the parameter being unused
            ignoreUnused (comparator);

            const ScopedLockType lock (getLock());
            auto index = findInsertIndexInSortedArray (comparator, values.begin(), newObject, 0, values.size());
            insert (index, newObject);
            return index;
        */
    }

    /**
      | Finds the index of an object in the array,
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
      | @param objectToLookFor
      | 
      | the object to search for
      | 
      | -----------
      | @return
      | 
      | the index of the element, or -1 if it's
      | not found @see addSorted, sort
      |
      */
    pub fn index_of_sorted<ElementComparator>(&self, 
        comparator:         &mut ElementComparator,
        object_to_look_for: *const ObjectClass) -> i32 {
    
        todo!();
        /*
            // If you pass in an object with a static compareElements() method, this
            // avoids getting warning messages about the parameter being unused
            ignoreUnused (comparator);

            const ScopedLockType lock (getLock());
            int s = 0, e = values.size();

            while (s < e)
            {
                if (comparator.compareElements (objectToLookFor, values[s]) == 0)
                    return s;

                auto halfway = (s + e) / 2;

                if (halfway == s)
                    break;

                if (comparator.compareElements (objectToLookFor, values[halfway]) >= 0)
                    s = halfway;
                else
                    e = halfway;
            }

            return -1;
        */
    }

    /**
      | Removes an object from the array.
      | 
      | This will remove the object at a given
      | index (optionally also deleting it)
      | and move back all the subsequent objects
      | to close the gap. If the index passed
      | in is out-of-range, nothing will happen.
      | 
      | -----------
      | @param indexToRemove
      | 
      | the index of the element to remove
      | ----------
      | @param deleteObject
      | 
      | whether to delete the object that is
      | removed @see removeObject, removeRange
      |
      */
    pub fn remove(
        &mut self, 
        index_to_remove: i32,
        delete_object:   Option<bool>
    )
    {
        let delete_object: bool = delete_object.unwrap_or(true);

        todo!();
        /*
            std::unique_ptr<ObjectClass> toDelete;

            {
                const ScopedLockType lock (getLock());

                if (isPositiveAndBelow (indexToRemove, values.size()))
                {
                    auto** e = values.begin() + indexToRemove;

                    if (deleteObject)
                        toDelete.reset (*e);

                    values.removeElements (indexToRemove, 1);
                }
            }

            if ((values.size() << 1) < values.capacity())
                minimiseStorageOverheads();
        */
    }

    /**
      | Removes and returns an object from the
      | array without deleting it.
      | 
      | This will remove the object at a given
      | index and return it, moving back all
      | the subsequent objects to close the
      | gap. If the index passed in is out-of-range,
      | nothing will happen.
      | 
      | -----------
      | @param indexToRemove
      | 
      | the index of the element to remove @see
      | remove, removeObject, removeRange
      |
      */
    pub fn remove_and_return(&mut self, index_to_remove: i32) -> *mut ObjectClass {
        
        todo!();
        /*
            ObjectClass* removedItem = nullptr;
            const ScopedLockType lock (getLock());

            if (isPositiveAndBelow (indexToRemove, values.size()))
            {
               removedItem = values[indexToRemove];

                values.removeElements (indexToRemove, 1);

                if ((values.size() << 1) < values.capacity())
                    minimiseStorageOverheads();
            }

            return removedItem;
        */
    }

    /**
      | Removes a specified object from the
      | array.
      | 
      | If the item isn't found, no action is
      | taken.
      | 
      | -----------
      | @param objectToRemove
      | 
      | the object to try to remove
      | ----------
      | @param deleteObject
      | 
      | whether to delete the object (if it's
      | found) @see remove, removeRange
      |
      */
    pub fn remove_object(
        &mut self, 
        object_to_remove: *const ObjectClass,
        delete_object:    Option<bool>
    )
    {
        let delete_object: bool = delete_object.unwrap_or(true);

        todo!();
        /*
            const ScopedLockType lock (getLock());

            for (int i = 0; i < values.size(); ++i)
            {
                if (objectToRemove == values[i])
                {
                    remove (i, deleteObject);
                    break;
                }
            }
        */
    }

    /**
      | Removes a range of objects from the array.
      | 
      | This will remove a set of objects, starting
      | from the given index, and move any subsequent
      | elements down to close the gap.
      | 
      | If the range extends beyond the bounds
      | of the array, it will be safely clipped
      | to the size of the array.
      | 
      | -----------
      | @param startIndex
      | 
      | the index of the first object to remove
      | ----------
      | @param numberToRemove
      | 
      | how many objects should be removed
      | ----------
      | @param deleteObjects
      | 
      | whether to delete the objects that get
      | removed @see remove, removeObject
      |
      */
    pub fn remove_range(
        &mut self, 
        start_index:      i32,
        number_to_remove: i32,
        delete_objects:   Option<bool>)
    {
        let delete_objects: bool = delete_objects.unwrap_or(true);

        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto endIndex = jlimit (0, values.size(), startIndex + numberToRemove);
            startIndex = jlimit (0, values.size(), startIndex);
            numberToRemove = endIndex - startIndex;

            if (numberToRemove > 0)
            {
                Vec<ObjectClass*> objectsToDelete;

                if (deleteObjects)
                    objectsToDelete.addArray (values.begin() + startIndex, numberToRemove);

                values.removeElements (startIndex, numberToRemove);

                for (auto& o : objectsToDelete)
                    ContainerDeletePolicy<ObjectClass>::destroy (o);

                if ((values.size() << 1) < values.capacity())
                    minimiseStorageOverheads();
            }
        */
    }

    /**
      | Removes the last n objects from the array.
      | 
      | -----------
      | @param howManyToRemove
      | 
      | how many objects to remove from the end
      | of the array
      | ----------
      | @param deleteObjects
      | 
      | whether to also delete the objects that
      | are removed @see remove, removeObject,
      | removeRange
      |
      */
    pub fn remove_last(
        &mut self, 
        how_many_to_remove: Option<i32>,
        delete_objects:     Option<bool>
    )
    {
        let how_many_to_remove: i32 = how_many_to_remove.unwrap_or(1);

        let delete_objects: bool = delete_objects.unwrap_or(true);

        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (howManyToRemove >= values.size())
                clear (deleteObjects);
            else
                removeRange (values.size() - howManyToRemove, howManyToRemove, deleteObjects);
        */
    }

    /**
      | Swaps a pair of objects in the array.
      | 
      | If either of the indexes passed in is
      | out-of-range, nothing will happen,
      | otherwise the two objects at these positions
      | will be exchanged.
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
      | Moves one of the objects to a different
      | position.
      | 
      | This will move the object to a specified
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
      | the index of the object to be moved. If
      | this isn't a valid index, then nothing
      | will be done
      | ----------
      | @param newIndex
      | 
      | the index at which you'd like this object
      | to end up. If this is less than zero, it
      | will be moved to the end of the array
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
      | Sorts the elements in the array.
      | 
      | This will use a comparator object to
      | sort the elements into order. The object
      | passed must have a method of the form:
      | 
      | @code
      | 
      | int compareElements (ElementType*
      | first, ElementType* second);
      | 
      | @endcode
      | 
      | ..and this method must return: - a value
      | of < 0 if the first comes before the second
      | - a value of 0 if the two objects are equivalent
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
      | be rearranged. @see sortArray, indexOfSorted
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
            // If you pass in an object with a static compareElements() method, this
            // avoids getting warning messages about the parameter being unused
            ignoreUnused (comparator);

            const ScopedLockType lock (getLock());

            if (size() > 1)
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
    
    pub fn delete_all_objects(&mut self)  {
        
        todo!();
        /*
            auto i = values.size();

            while (--i >= 0)
            {
                auto* e = values[i];
                values.removeElements (i, 1);
                ContainerDeletePolicy<ObjectClass>::destroy (e);
            }
        */
    }
}

#[test] fn owned_array_test() {

    todo!();

    /*
    #if ALOE_UNIT_TESTS

    static struct OwnedArrayTest : public UnitTest
    {
        struct Base
        {
            Base() = default;
            virtual ~Base() = default;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (Base)
        };

        struct Derived : Base
        {
            Derived() = default;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (Derived)
        };

        struct DestructorObj
        {
            DestructorObj (OwnedArrayTest& p,
                           OwnedArray<DestructorObj>& arr)
                : parent (p), objectArray (arr)
            {}

            ~DestructorObj()
            {
                data = 0;

                for (auto* o : objectArray)
                {
                    parent.expect (o != nullptr);
                    parent.expect (o != this);

                    if (o != nullptr)
                        parent.expectEquals (o->data, 956);
                }
            }

            OwnedArrayTest& parent;
            OwnedArray<DestructorObj>& objectArray;
            int data = 956;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (DestructorObj)
        };

        OwnedArrayTest()
            : UnitTest ("OwnedArray", UnitTestCategories::containers)
        {}

        void runTest() override
        {
            beginTest ("After converting move construction, ownership is transferred");
            {
                OwnedArray<Derived> derived { new Derived{}, new Derived{}, new Derived{} };

                OwnedArray<Base> base  { std::move (derived) };

                expectEquals (base.size(), 3);
                expectEquals (derived.size(), 0);
            }

            beginTest ("After converting move assignment, ownership is transferred");
            {
                OwnedArray<Base> base;

                base = OwnedArray<Derived> { new Derived{}, new Derived{}, new Derived{} };

                expectEquals (base.size(), 3);
            }

            beginTest ("Iterate in destructor");
            {
                {
                    OwnedArray<DestructorObj> arr;

                    for (int i = 0; i < 2; ++i)
                        arr.add (new DestructorObj (*this, arr));
                }

                OwnedArray<DestructorObj> arr;

                for (int i = 0; i < 1025; ++i)
                    arr.add (new DestructorObj (*this, arr));

                while (! arr.isEmpty())
                    arr.remove (0);

                for (int i = 0; i < 1025; ++i)
                    arr.add (new DestructorObj (*this, arr));

                arr.removeRange (1, arr.size() - 3);

                for (int i = 0; i < 1025; ++i)
                    arr.add (new DestructorObj (*this, arr));

                arr.set (500, new DestructorObj (*this, arr));
            }
        }
    } ownedArrayTest;

    #endif
    */
}
