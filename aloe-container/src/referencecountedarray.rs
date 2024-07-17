crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ReferenceCountedArray.h]

/**
  | Holds a list of objects derived from
  | ReferenceCountedObject, or which
  | implement basic reference-count handling
  | methods.
  | 
  | The template parameter specifies the
  | class of the object you want to point
  | to - the easiest way to make a class reference-countable
  | is to simply make it inherit from ReferenceCountedObject
  | or SingleThreadedReferenceCountedObject,
  | but if you need to, you can roll your own
  | reference-countable class by implementing
  | a set of methods called incReferenceCount(),
  | decReferenceCount(), and decReferenceCountWithoutDeleting().
  | See ReferenceCountedObject for examples
  | of how these methods should behave.
  | 
  | A ReferenceCountedArray holds objects
  | derived from ReferenceCountedObject,
  | and takes care of incrementing and decrementing
  | their ref counts when they are added
  | and removed from the array.
  | 
  | To make all the array's methods thread-safe,
  | pass in "CriticalSection" as the templated
  | TypeOfCriticalSectionToUse parameter,
  | instead of the default DummyCriticalSection.
  | 
  | @see Vec, OwnedArray, StringArray
  | 
  | @tags{Core}
  |
  */
pub struct ReferenceCountedArray<
ObjectClass,
TypeOfCriticalSectionToUse = DummyCriticalSection
> 
where ObjectClass: ?Sized
{

    values: ArrayBase<*mut ObjectClass,TypeOfCriticalSectionToUse>,
}

impl<ObjectClass,TypeOfCriticalSectionToUse: HasScopedLockType> 

HasScopedLockType for ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse> {

    /**
      | Returns the type of scoped lock to use
      | for locking this array
      |
      */
    type ScopedLockType = <TypeOfCriticalSectionToUse as HasScopedLockType>::ScopedLockType;
}

pub trait HasObjectClassPtr {

    type ObjectClassPtr;
}

impl<ObjectClass,TypeOfCriticalSectionToUse> HasObjectClassPtr for ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse> {
    type ObjectClassPtr = ReferenceCountedObjectPtr<ObjectClass>;
}

impl<ObjectClass,TypeOfCriticalSectionToUse> 

Drop for ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse> 

where ObjectClass: ?Sized
{

    /**
      | Destructor. Any objects in the array
      | will be released, and may be deleted
      | if not referenced from elsewhere.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
            releaseAllObjects();
         */
    }
}

impl<ObjectClass,TypeOfCriticalSectionToUse> 
Index<i32> for ReferenceCountedArray<ObjectClass, TypeOfCriticalSectionToUse> {

    type Output = <Self as HasObjectClassPtr>::ObjectClassPtr;
    
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
            return ObjectClassPtr (getObjectPointer (index));
        */
    }
}

impl<ObjectClass,TypeOfCriticalSectionToUse> 
PartialEq<ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse>> 
for ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse>
{
    /**
      | Compares this array to another one.
      | 
      | -----------
      | @return
      | 
      | true only if the other array contains
      | the same objects in the same order
      |
      */
    #[inline] fn eq(&self, other: &ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse>) -> bool {
        todo!();
        /*
            const ScopedLockType lock2 (other.getLock());
            const ScopedLockType lock1 (getLock());
            return values == other.values;
        */
    }
}

impl<ObjectClass,TypeOfCriticalSectionToUse> 

Eq for ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse> {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ReferenceCountedArray.cpp]
impl<ObjectClass,TypeOfCriticalSectionToUse> ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse> {

    /**
      | Creates a copy of another array
      |
      */
    pub fn new_by_copy(other: &ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse>) -> Self {
    
        todo!();
        /*


            const ScopedLockType lock (other.getLock());
            values.addArray (other.begin(), other.size());

            for (auto* o : *this)
                if (o != nullptr)
                    o->incReferenceCount();
        */
    }

    /**
      | Moves from another array
      |
      */
    pub fn new_by_move(other: ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse>) -> Self {
    
        todo!();
        /*
        : values(std::move (other.values)),
        */
    }

    /**
      | Creates a copy of another array
      |
      */
    pub fn new_by_copy_other<OtherObjectClass, OtherCriticalSection>(other: &ReferenceCountedArray<OtherObjectClass,OtherCriticalSection>) -> Self {
    
        todo!();
        /*


            const typename ReferenceCountedArray<OtherObjectClass, OtherCriticalSection>::ScopedLockType lock (other.getLock());
            values.addArray (other.begin(), other.size());

            for (auto* o : *this)
                if (o != nullptr)
                    o->incReferenceCount();
        */
    }

    /**
      | Copies another array into this one.
      | 
      | Any existing objects in this array will
      | first be released.
      |
      */
    pub fn assign_from_other_ref(
        &mut self, 
        other: &ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse>
    )
        -> &mut ReferenceCountedArray<ObjectClass,TypeOfCriticalSectionToUse> 
    {
        todo!();

        /*
            releaseAllObjects();
            auto otherCopy = other;
            swapWith (otherCopy);
            return *this;
        */
    }

    /**
      | Copies another array into this one.
      | 
      | Any existing objects in this array will
      | first be released.
      |
      */
    pub fn assign_from_other_object_class<OtherObjectClass>(
        &mut self, 
        other: &ReferenceCountedArray<OtherObjectClass,TypeOfCriticalSectionToUse>
    ) 
        -> &mut ReferenceCountedArray<ObjectClass> 
    {
    
        todo!();
        /*
            auto otherCopy = other;
            swapWith (otherCopy);
            return *this;
        */
    }

    /**
      | Moves from another array
      |
      */
    pub fn assign_from_moving(&mut self, other: ReferenceCountedArray<ObjectClass>) -> &mut ReferenceCountedArray<ObjectClass> {
        
        todo!();
        /*
            releaseAllObjects();
            values = std::move (other.values);
            return *this;
        */
    }

    /**
      | Removes all objects from the array.
      | 
      | Any objects in the array whose reference
      | counts drop to zero will be deleted.
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
      | Removes all objects from the array without
      | freeing the array's allocated storage.
      | 
      | Any objects in the array that whose reference
      | counts drop to zero will be deleted.
      | @see clear
      |
      */
    pub fn clear_quick(&mut self)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            releaseAllObjects();
        */
    }

    /**
      | Returns the current number of objects
      | in the array.
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
    #[inline] pub fn get_unchecked(&self, index: i32) 
    -> <Self as HasObjectClassPtr>::ObjectClassPtr 
    {
        todo!();
        /*
            return ObjectClassPtr (getObjectPointerUnchecked (index));
        */
    }

    /**
      | Returns a raw pointer to the object at
      | this index in the array.
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
    #[inline] pub fn get_object_pointer(&self, index: i32) -> *mut ObjectClass {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            return values.getValueWithDefault (index);
        */
    }

    /**
      | Returns a raw pointer to the object at
      | this index in the array, without checking
      | whether the index is in-range.
      |
      */
    #[inline] pub fn get_object_pointer_unchecked(&self, index: i32) -> *mut ObjectClass {
        
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
    #[inline] pub fn get_first(&self) 
    -> <Self as HasObjectClassPtr>::ObjectClassPtr 
    {
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
    #[inline] pub fn get_last(&self) 
    -> <Self as HasObjectClassPtr>::ObjectClassPtr 
    {
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
    #[inline] pub fn get_raw_data_pointer(&self) -> *mut *mut ObjectClass {
        
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
      | Finds the index of the first occurrence
      | of an object in the array.
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
    pub fn index_of_raw(&self, object_to_look_for: *const ObjectClass) -> i32 {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto* e = values.begin();
            auto* endPointer = values.end();

            while (e != endPointer)
            {
                if (objectToLookFor == *e)
                    return static_cast<int> (e - values.begin());

                ++e;
            }

            return -1;
        */
    }

    /**
      | Finds the index of the first occurrence
      | of an object in the array.
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
    pub fn index_of(&self, object_to_look_for: &<Self as HasObjectClassPtr>::ObjectClassPtr) -> i32 {
        
        todo!();
        /*
            return indexOf (objectToLookFor.get());
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
    pub fn contains_raw(&self, object_to_look_for: *const ObjectClass) -> bool {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto* e = values.begin();
            auto* endPointer = values.end();

            while (e != endPointer)
            {
                if (objectToLookFor == *e)
                    return true;

                ++e;
            }

            return false;
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
    pub fn contains(&self, object_to_look_for: &<Self as HasObjectClassPtr>::ObjectClassPtr) -> bool {
        
        todo!();
        /*
            return contains (objectToLookFor.get());
        */
    }

    /**
      | Appends a new object to the end of the
      | array.
      | 
      | This will increase the new object's
      | reference count.
      | 
      | -----------
      | @param newObject
      | 
      | the new object to add to the array @see
      | set, insert, addIfNotAlreadyThere,
      | addSorted, addArray
      |
      */
    pub fn add_raw(&mut self, new_object: *mut ObjectClass) -> *mut ObjectClass {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            values.add (newObject);

            if (newObject != nullptr)
                newObject->incReferenceCount();

            return newObject;
        */
    }

    /**
      | Appends a new object to the end of the
      | array.
      | 
      | This will increase the new object's
      | reference count.
      | 
      | -----------
      | @param newObject
      | 
      | the new object to add to the array @see
      | set, insert, addIfNotAlreadyThere,
      | addSorted, addArray
      |
      */
    pub fn add(&mut self, new_object: &<Self as HasObjectClassPtr>::ObjectClassPtr) -> *mut ObjectClass {
        
        todo!();
        /*
            return add (newObject.get());
        */
    }

    /**
      | Inserts a new object into the array at
      | the given index.
      | 
      | If the index is less than 0 or greater
      | than the size of the array, the element
      | will be added to the end of the array.
      | Otherwise, it will be inserted into
      | the array, moving all the later elements
      | along to make room.
      | 
      | This will increase the new object's
      | reference count.
      | 
      | -----------
      | @param indexToInsertAt
      | 
      | the index at which the new element should
      | be inserted
      | ----------
      | @param newObject
      | 
      | the new object to add to the array @see
      | add, addSorted, addIfNotAlreadyThere,
      | set
      |
      */
    pub fn insert_raw(&mut self, 
        index_to_insert_at: i32,
        new_object:         *mut ObjectClass) -> *mut ObjectClass {
        
        todo!();
        /*
            values.insert (indexToInsertAt, newObject, 1);

            if (newObject != nullptr)
                newObject->incReferenceCount();

            return newObject;
        */
    }

    /**
      | Inserts a new object into the array at
      | the given index.
      | 
      | If the index is less than 0 or greater
      | than the size of the array, the element
      | will be added to the end of the array.
      | Otherwise, it will be inserted into
      | the array, moving all the later elements
      | along to make room.
      | 
      | This will increase the new object's
      | reference count.
      | 
      | -----------
      | @param indexToInsertAt
      | 
      | the index at which the new element should
      | be inserted
      | ----------
      | @param newObject
      | 
      | the new object to add to the array @see
      | add, addSorted, addIfNotAlreadyThere,
      | set
      |
      */
    pub fn insert(&mut self, 
        index_to_insert_at: i32,
        new_object:         &<Self as HasObjectClassPtr>::ObjectClassPtr) -> *mut ObjectClass {
        
        todo!();
        /*
            return insert (indexToInsertAt, newObject.get());
        */
    }

    /**
      | Appends a new object at the end of the
      | array as long as the array doesn't already
      | contain it.
      | 
      | If the array already contains a matching
      | object, nothing will be done.
      | 
      | -----------
      | @param newObject
      | 
      | the new object to add to the array
      | 
      | -----------
      | @return
      | 
      | true if the object has been added, false
      | otherwise
      |
      */
    pub fn add_raw_if_not_already_there(&mut self, new_object: *mut ObjectClass) -> bool {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (contains (newObject))
                return false;

            add (newObject);
            return true;
        */
    }

    /**
      | Appends a new object at the end of the
      | array as long as the array doesn't already
      | contain it.
      | 
      | If the array already contains a matching
      | object, nothing will be done.
      | 
      | -----------
      | @param newObject
      | 
      | the new object to add to the array
      | 
      | -----------
      | @return
      | 
      | true if the object has been added, false
      | otherwise
      |
      */
    pub fn add_if_not_already_there(&mut self, new_object: &<Self as HasObjectClassPtr>::ObjectClassPtr) -> bool {
        
        todo!();
        /*
            return addIfNotAlreadyThere (newObject.get());
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
      | The object being added has its reference
      | count increased, and if it's replacing
      | another object, then that one has its
      | reference count decreased, and may
      | be deleted.
      | 
      | -----------
      | @param indexToChange
      | 
      | the index whose value you want to change
      | ----------
      | @param newObject
      | 
      | the new value to set for this index. @see
      | add, insert, remove
      |
      */
    pub fn set_raw(&mut self, 
        index_to_change: i32,
        new_object:      *mut ObjectClass)  {
        
        todo!();
        /*
            if (indexToChange >= 0)
            {
                const ScopedLockType lock (getLock());

                if (newObject != nullptr)
                    newObject->incReferenceCount();

                if (indexToChange < values.size())
                {
                    auto* e = values[indexToChange];
                    values[indexToChange] = newObject;
                    releaseObject (e);
                }
                else
                {
                    values.add (newObject);
                }
            }
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
      | The object being added has its reference
      | count increased, and if it's replacing
      | another object, then that one has its
      | reference count decreased, and may
      | be deleted.
      | 
      | -----------
      | @param indexToChange
      | 
      | the index whose value you want to change
      | ----------
      | @param newObject
      | 
      | the new value to set for this index. @see
      | add, insert, remove
      |
      */
    pub fn set(&mut self, 
        index_to_change: i32,
        new_object:      &<Self as HasObjectClassPtr>::ObjectClassPtr)  {
        
        todo!();
        /*
            set (indexToChange, newObject.get());
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
    pub fn add_array(
        &mut self, 
        array_to_add_from:   &ReferenceCountedArray<ObjectClass>,
        start_index:         Option<i32>,
        num_elements_to_add: Option<i32>
    )
    {
        let start_index:         i32 = start_index.unwrap_or(0);
        let num_elements_to_add: i32 = num_elements_to_add.unwrap_or(-1);

        todo!();
        /*
            const ScopedLockType lock1 (arrayToAddFrom.getLock());

            {
                const ScopedLockType lock2 (getLock());

                auto numElementsAdded = values.addArray (arrayToAddFrom.values, startIndex, numElementsToAdd);
                auto** e = values.end();

                for (int i = 0; i < numElementsAdded; ++i)
                    (*(--e))->incReferenceCount();
            }
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
      | the comparator object to use to compare
      | the elements - see the sort() method
      | for details about this object's form
      | ----------
      | @param newObject
      | 
      | the new object to insert to the array
      | 
      | -----------
      | @return
      | 
      | the index at which the new object was
      | added @see add, sort
      |
      */
    pub fn add_sorted<ElementComparator>(&mut self, 
        comparator: &mut ElementComparator,
        new_object: *mut ObjectClass) -> i32 {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto index = findInsertIndexInSortedArray (comparator, values.begin(), newObject, 0, values.size());
            insert (index, newObject);
            return index;
        */
    }

    /**
      | Inserts or replaces an object in the
      | array, assuming it is sorted.
      | 
      | This is similar to addSorted, but if
      | a matching element already exists,
      | then it will be replaced by the new one,
      | rather than the new one being added as
      | well.
      |
      */
    pub fn add_or_replace_sorted<ElementComparator>(&mut self, 
        comparator: &mut ElementComparator,
        new_object: *mut ObjectClass)  {
    
        todo!();
        /*
            const ScopedLockType lock (getLock());
            auto index = findInsertIndexInSortedArray (comparator, values.begin(), newObject, 0, values.size());

            if (index > 0 && comparator.compareElements (newObject, values[index - 1]) == 0)
                set (index - 1, newObject); // replace an existing object that matches
            else
                insert (index, newObject);  // no match, so insert the new one
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
      | index and move back all the subsequent
      | objects to close the gap.
      | 
      | If the index passed in is out-of-range,
      | nothing will happen.
      | 
      | The object that is removed will have
      | its reference count decreased, and
      | may be deleted if not referenced from
      | elsewhere.
      | 
      | -----------
      | @param indexToRemove
      | 
      | the index of the element to remove @see
      | removeObject, removeRange
      |
      */
    pub fn remove(&mut self, index_to_remove: i32)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (isPositiveAndBelow (indexToRemove, values.size()))
            {
                auto* e = *(values.begin() + indexToRemove);
                values.removeElements (indexToRemove, 1);
                releaseObject (e);

                if ((values.size() << 1) < values.capacity())
                    minimiseStorageOverheads();
            }
        */
    }

    /**
      | Removes and returns an object from the
      | array.
      | 
      | This will remove the object at a given
      | index and return it, moving back all
      | the subsequent objects to close the
      | gap. If the index passed in is out-of-range,
      | nothing will happen and a null pointer
      | will be returned.
      | 
      | -----------
      | @param indexToRemove
      | 
      | the index of the element to remove @see
      | remove, removeObject, removeRange
      |
      */
    pub fn remove_and_return(&mut self, index_to_remove: i32) 
        -> <Self as HasObjectClassPtr>::ObjectClassPtr 
    {
        todo!();

        /*
            ObjectClassPtr removedItem;
            const ScopedLockType lock (getLock());

            if (isPositiveAndBelow (indexToRemove, values.size()))
            {
                auto* e = *(values.begin() + indexToRemove);
                removedItem = e;
                values.removeElements (indexToRemove, 1);
                releaseObject (e);

                if ((values.size() << 1) < values.capacity())
                    minimiseStorageOverheads();
            }

            return removedItem;
        */
    }

    /**
      | Removes the first occurrence of a specified
      | object from the array.
      | 
      | If the item isn't found, no action is
      | taken. If it is found, it is removed and
      | has its reference count decreased.
      | 
      | -----------
      | @param objectToRemove
      | 
      | the object to try to remove @see remove,
      | removeRange
      |
      */
    pub fn remove_object_raw(&mut self, object_to_remove: *mut ObjectClass)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            remove (indexOf (objectToRemove));
        */
    }

    /**
      | Removes the first occurrence of a specified
      | object from the array.
      | 
      | If the item isn't found, no action is
      | taken. If it is found, it is removed and
      | has its reference count decreased.
      | 
      | -----------
      | @param objectToRemove
      | 
      | the object to try to remove @see remove,
      | removeRange
      |
      */
    pub fn remove_object(
        &mut self, 
        object_to_remove: &<Self as HasObjectClassPtr>::ObjectClassPtr
    )  {
        
        todo!();
        /*
            removeObject (objectToRemove.get());
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
      | The objects that are removed will have
      | their reference counts decreased,
      | and may be deleted if not referenced
      | from elsewhere.
      | 
      | -----------
      | @param startIndex
      | 
      | the index of the first object to remove
      | ----------
      | @param numberToRemove
      | 
      | how many objects should be removed @see
      | remove, removeObject
      |
      */
    pub fn remove_range(&mut self, 
        start_index:      i32,
        number_to_remove: i32)  {
        
        todo!();
        /*
            const ScopedLockType lock (getLock());
            startIndex    = jlimit (0, values.size(), startIndex);
            auto endIndex = jlimit (0, values.size(), startIndex + numberToRemove);
            numberToRemove = endIndex - startIndex;

            if (numberToRemove > 0)
            {
                Vec<ObjectClass*> objectsToRemove;
                objectsToRemove.addArray (values.begin() + startIndex, numberToRemove);

                values.removeElements (startIndex, numberToRemove);

                for (auto& o : objectsToRemove)
                    releaseObject (o);

                if ((values.size() << 1) < values.capacity())
                    minimiseStorageOverheads();
            }
        */
    }

    /**
      | Removes the last n objects from the array.
      | 
      | The objects that are removed will have
      | their reference counts decreased,
      | and may be deleted if not referenced
      | from elsewhere.
      | 
      | -----------
      | @param howManyToRemove
      | 
      | how many objects to remove from the end
      | of the array @see remove, removeObject,
      | removeRange
      |
      */
    pub fn remove_last(&mut self, how_many_to_remove: Option<i32>)  {

        let how_many_to_remove: i32 = how_many_to_remove.unwrap_or(1);

        todo!();
        /*
            const ScopedLockType lock (getLock());

            if (howManyToRemove > values.size())
                howManyToRemove = values.size();

            while (--howManyToRemove >= 0)
                remove (values.size() - 1);
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

            if (isPositiveAndBelow (index1, values.size())
             && isPositiveAndBelow (index2, values.size()))
            {
                std::swap (values[index1], values[index2]);
            }
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
      | @see sortArray
      |
      */
    pub fn sort<ElementComparator>(
        &mut self, 
        comparator:                       &mut ElementComparator,
        retain_order_of_equivalent_items: Option<bool>) 
    {
        let retain_order_of_equivalent_items: bool =
            retain_order_of_equivalent_items.unwrap_or(false);

        todo!();
        /*
            // If you pass in an object with a static compareElements() method, this
            // avoids getting warning messages about the parameter being unused
            ignoreUnused (comparator);

            const ScopedLockType lock (getLock());
            sortArray (comparator, values.begin(), 0, values.size() - 1, retainOrderOfEquivalentItems);
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
    
    pub fn release_all_objects(&mut self)  {
        
        todo!();
        /*
            auto i = values.size();

            while (--i >= 0)
            {
                auto* e = values[i];
                values.removeElements (i, 1);
                releaseObject (e);
            }
        */
    }
    
    pub fn release_object(o: *mut ObjectClass)  {
        
        todo!();
        /*
            if (o != nullptr && o->decReferenceCountWithoutDeleting())
                ContainerDeletePolicy<ObjectClass>::destroy (o);
        */
    }
}

#[test] fn reference_counted_array_tests() {
    /*

    #if ALOE_UNIT_TESTS

    class ReferenceCountedArrayTests   : public UnitTest
    {

        ReferenceCountedArrayTests()
            : UnitTest ("ReferenceCountedArray", UnitTestCategories::containers)
        {}

        
        void runTest() override
        {
            beginTest ("Add derived objects");
            {
                ReferenceCountedArray<TestDerivedObj> derivedArray;
                derivedArray.add (static_cast<TestDerivedObj*> (new TestBaseObj()));
                expectEquals (derivedArray.size(), 1);
                expectEquals (derivedArray.getObjectPointer (0)->getReferenceCount(), 1);
                expectEquals (derivedArray[0]->getReferenceCount(), 2);

                for (auto o : derivedArray)
                    expectEquals (o->getReferenceCount(), 1);

                ReferenceCountedArray<TestBaseObj> baseArray;
                baseArray.addArray (derivedArray);

                for (auto o : baseArray)
                    expectEquals (o->getReferenceCount(), 2);

                derivedArray.clearQuick();
                baseArray.clearQuick();

                auto* baseObject = new TestBaseObj();
                TestBaseObj::Ptr baseObjectPtr = baseObject;
                expectEquals (baseObject->getReferenceCount(), 1);

                auto* derivedObject = new TestDerivedObj();
                TestDerivedObj::Ptr derivedObjectPtr = derivedObject;
                expectEquals (derivedObject->getReferenceCount(), 1);

                baseArray.add (baseObject);
                baseArray.add (derivedObject);

                for (auto o : baseArray)
                    expectEquals (o->getReferenceCount(), 2);

                expectEquals (baseObject->getReferenceCount(),    2);
                expectEquals (derivedObject->getReferenceCount(), 2);

                derivedArray.add (derivedObject);

                for (auto o : derivedArray)
                    expectEquals (o->getReferenceCount(), 3);

                derivedArray.clearQuick();
                baseArray.clearQuick();

                expectEquals (baseObject->getReferenceCount(),    1);
                expectEquals (derivedObject->getReferenceCount(), 1);

                baseArray.add (baseObjectPtr);
                baseArray.add (derivedObjectPtr.get());

                for (auto o : baseArray)
                    expectEquals (o->getReferenceCount(), 2);

                derivedArray.add (derivedObjectPtr);

                for (auto o : derivedArray)
                    expectEquals (o->getReferenceCount(), 3);
            }

            beginTest ("Iterate in destructor");
            {
                {
                    ReferenceCountedArray<DestructorObj> arr;

                    for (int i = 0; i < 2; ++i)
                        arr.add (new DestructorObj (*this, arr));
                }

                ReferenceCountedArray<DestructorObj> arr;

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

        struct TestBaseObj : public ReferenceCountedObject
        {
            using Ptr = ReferenceCountedObjectPtr<TestBaseObj>;

            TestBaseObj() = default;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (TestBaseObj)
        };

        struct TestDerivedObj : public TestBaseObj
        {
            using Ptr = ReferenceCountedObjectPtr<TestDerivedObj>;

            TestDerivedObj() = default;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (TestDerivedObj)
        };

        struct DestructorObj : public ReferenceCountedObject
        {
            DestructorObj (ReferenceCountedArrayTests& p,
                           ReferenceCountedArray<DestructorObj>& arr)
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
                        parent.expectEquals (o->data, 374);
                }
            }

            ReferenceCountedArrayTests& parent;
            ReferenceCountedArray<DestructorObj>& objectArray;
            int data = 374;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (DestructorObj)
        };
    };

    static ReferenceCountedArrayTests referenceCountedArrayTests;

    #endif
    */
}
