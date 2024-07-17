crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_ReferenceCountedObject.h]

/**
  | A base class which provides methods
  | for reference-counting.
  | 
  | To add reference-counting to a class,
  | derive it from this class, and use the
  | ReferenceCountedObjectPtr class
  | to point to it.
  | 
  | e.g.
  | 
  | -----------
  | @code
  | 
  | class MyClass : public ReferenceCountedObject
  | {
  |     void foo();
  | 
  |     // This is a neat way of declaring a typedef for a pointer class,
  |     // rather than typing out the full templated name each time..
  |     using Ptr = ReferenceCountedObjectPtr<MyClass>;
  | };
  | 
  | MyClass::Ptr p = new MyClass();
  | MyClass::Ptr p2 = p;
  | p = nullptr;
  | p2->foo();
  | 
  | Once a new ReferenceCountedObject
  | has been assigned to a pointer, be careful
  | not to delete the object manually.
  | 
  | This class uses an Atomic<int> value
  | to hold the reference count, so the reference
  | count can be updated on multiple threads.
  | Note that whilst it's thread-safe to
  | create and delete a ReferenceCountedObjectPtr
  | to a ReferenceCountedObject shared
  | between threads, it's not thread-safe
  | to modify or swap the ReferenceCountedObject.
  | 
  | For a faster but non-thread-safe version,
  | use SingleThreadedReferenceCountedObject
  | instead.
  | 
  | @see ReferenceCountedObjectPtr,
  | ReferenceCountedArray, SingleThreadedReferenceCountedObject
  | 
  | @tags{Core}
  |
  */
pub trait ReferenceCountedObjectInterface {

    fn inc_reference_count(&mut self);

    fn dec_reference_count(&mut self);

    fn dec_reference_count_without_deleting(&mut self) -> bool;

    fn get_reference_count(&self) -> i32;

    fn reset_reference_count(&mut self);
}

pub struct ReferenceCountedObject {
    ref_count: AtomicI32, // default = 0 
}

impl Default for ReferenceCountedObject {
    
    /**
      | Creates the reference-counted object
      | (with an initial ref count of zero).
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl Drop for ReferenceCountedObject {
    fn drop(&mut self) {
        todo!();
        /* 
            // it's dangerous to delete an object that's still referenced by something else!
            jassert (getReferenceCount() == 0);
         */
    }
}

impl ReferenceCountedObject {

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn new_from_ref(_0: &ReferenceCountedObject) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn new_from_other(_0: ReferenceCountedObject) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn assign_from_ref(&mut self, _0: &ReferenceCountedObject) -> &mut ReferenceCountedObject {
        
        todo!();
        /*
            return *this;
        */
    }

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn assign_from_other(&mut self, _0: ReferenceCountedObject) -> &mut ReferenceCountedObject {
        
        todo!();
        /*
            return *this;
        */
    }
}

impl ReferenceCountedObjectInterface for ReferenceCountedObject {

    /**
      | Increments the object's reference
      | count.
      | 
      | This is done automatically by the smart
      | pointer, but is public just in case it's
      | needed for nefarious purposes.
      |
      */
    fn inc_reference_count(&mut self)  {
        
        todo!();
        /*
            ++refCount;
        */
    }

    /**
      | Decreases the object's reference count.
      | 
      | If the count gets to zero, the object
      | will be deleted.
      |
      */
    fn dec_reference_count(&mut self)  {
        
        todo!();
        /*
            jassert (getReferenceCount() > 0);

            if (--refCount == 0)
                delete this;
        */
    }

    /**
      | Decreases the object's reference count.
      | 
      | If the count gets to zero, the object
      | will not be deleted, but this method
      | will return true, allowing the caller
      | to take care of deletion.
      |
      */
    fn dec_reference_count_without_deleting(&mut self) -> bool {
        
        todo!();
        /*
            jassert (getReferenceCount() > 0);
            return --refCount == 0;
        */
    }

    /**
      | Returns the object's current reference
      | count.
      |
      */
    fn get_reference_count(&self) -> i32 {
        
        todo!();
        /*
            return refCount.get();
        */
    }

    /**
      | Resets the reference count to zero without
      | deleting the object.
      | 
      | You should probably never need to use
      | this!
      |
      */
    fn reset_reference_count(&mut self)  {
        
        todo!();
        /*
            refCount = 0;
        */
    }
}

/**
  | Adds reference-counting to an object.
  | 
  | This is effectively a version of the
  | ReferenceCountedObject class, but
  | which uses a non-atomic counter, and
  | so is not thread-safe (but which will
  | be more efficient).
  | 
  | For more details on how to use it, see
  | the ReferenceCountedObject class
  | notes.
  | 
  | @see ReferenceCountedObject, ReferenceCountedObjectPtr,
  | ReferenceCountedArray
  | 
  | @tags{Core}
  |
  */
pub struct SingleThreadedReferenceCountedObject {
    ref_count: i32, // default = 0
}

impl Default for SingleThreadedReferenceCountedObject {
    
    /**
      | Creates the reference-counted object
      | (with an initial ref count of zero).
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl Drop for SingleThreadedReferenceCountedObject {
    fn drop(&mut self) {
        todo!();
        /* 
            // it's dangerous to delete an object that's still referenced by something else!
            jassert (getReferenceCount() == 0);
         */
    }
}

impl SingleThreadedReferenceCountedObject {
    
    /**
      | Increments the object's reference
      | count.
      | 
      | This is done automatically by the smart
      | pointer, but is public just in case it's
      | needed for nefarious purposes.
      |
      */
    pub fn inc_reference_count(&mut self)  {
        
        todo!();
        /*
            ++refCount;
        */
    }

    /**
      | Decreases the object's reference count.
      | 
      | If the count gets to zero, the object
      | will be deleted.
      |
      */
    pub fn dec_reference_count(&mut self)  {
        
        todo!();
        /*
            jassert (getReferenceCount() > 0);

            if (--refCount == 0)
                delete this;
        */
    }

    /**
      | Decreases the object's reference count.
      | 
      | If the count gets to zero, the object
      | will not be deleted, but this method
      | will return true, allowing the caller
      | to take care of deletion.
      |
      */
    pub fn dec_reference_count_without_deleting(&mut self) -> bool {
        
        todo!();
        /*
            jassert (getReferenceCount() > 0);
            return --refCount == 0;
        */
    }

    /**
      | Returns the object's current reference
      | count.
      |
      */
    pub fn get_reference_count(&self) -> i32 {
        
        todo!();
        /*
            return refCount;
        */
    }

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn new_from_single_threaded_rfo_ref(_0: &SingleThreadedReferenceCountedObject) -> Self {
    
        todo!();
        /*


        
        */
    }

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn new_from_single_threaded_rfo(_0: SingleThreadedReferenceCountedObject) -> Self {
    
        todo!();
        /*


        
        */
    }

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn assign_from_ref(&mut self, _0: &SingleThreadedReferenceCountedObject) -> &mut SingleThreadedReferenceCountedObject {
        
        todo!();
        /*
            return *this;
        */
    }

    /**
      | Copying from another object does not
      | affect this one's reference-count.
      |
      */
    pub fn assign_from_single_threaded_refcnt_object(&mut self, _0: SingleThreadedReferenceCountedObject) -> &mut SingleThreadedReferenceCountedObject {
        
        todo!();
        /*
            return *this;
        */
    }
}

/**
  | A smart-pointer class which points
  | to a reference-counted object.
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
  | When using this class, you'll probably
  | want to create a typedef to abbreviate
  | the full templated name - e.g.
  | 
  | -----------
  | @code
  | 
  | struct MyClass  : public ReferenceCountedObject
  | {
  |     using Ptr = ReferenceCountedObjectPtr<MyClass>;
  |     ...
  | }
  | 
  | @see ReferenceCountedObject, ReferenceCountedObjectArray
  | 
  | @tags{Core}
  |
  */
pub struct ReferenceCountedObjectPtr<ObjectType> {
    referenced_object: *mut <Self as HasReferencedType>::ReferencedType, // default = nullptr
}

pub trait HasReferencedType {
    type ReferencedType;
}

impl<ObjectType> HasReferencedType for ReferenceCountedObjectPtr<ObjectType> {

    /**
      | The class being referenced by this pointer.
      |
      */
    type ReferencedType = ObjectType;
}

impl<OwnerClass> Default for ReferenceCountedObjectPtr<OwnerClass> {
    
    /**
      | Creates a pointer to a null object.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        */
    }
}

impl<OwnerClass> Drop for ReferenceCountedObjectPtr<OwnerClass> {

    /**
      | Destructor.
      | 
      | This will decrement the object's reference-count,
      | which will cause the object to be deleted
      | when the ref-count hits zero.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
            decIfNotNull (referencedObject);
         */
    }
}

impl<OwnerClass> Deref for ReferenceCountedObjectPtr<OwnerClass> {

    type Target = <Self as HasReferencedType>::ReferencedType;

    /**
      | Dereferences the object that this pointer
      | references.
      | 
      | The pointer returned may be null, of
      | course.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            jassert (referencedObject != nullptr); return *referencedObject;
        */
    }
}

impl<ObjectType> PartialEq<Option<*mut ObjectType>> for ReferenceCountedObjectPtr<ObjectType> {

    #[inline]
    fn eq(&self, other: &Option<*mut ObjectType>) -> bool {
        match other {
            Some(other_ptr) => self.referenced_object == *other_ptr,
            None => self.referenced_object.is_null(),
        }
    }
}

impl<ObjectType> 
PartialEq<ReferenceCountedObjectPtr<ObjectType>> 
for ReferenceCountedObjectPtr<ObjectType> {
    
    /**
      | Compares two ReferenceCountedObjectPtrs.
      |
      */
    #[inline] fn eq(&self, other: &ReferenceCountedObjectPtr<ObjectType>) -> bool {
        todo!();
        /*
            return referencedObject == other.get();
        */
    }
}

#[cfg(ALOE_STRICT_REFCOUNTEDPOINTER)]
impl<ObjectType> Into<bool> for ReferenceCountedObjectPtr<ObjectType> {
    
    /**
      | Checks whether this pointer is null
      |
      */
    #[inline] fn into(self) -> bool {
        todo!();
        /*
            return referencedObject != nullptr;
        */
    }
}

/*
#[cfg(not(ALOE_STRICT_REFCOUNTEDPOINTER))]
impl<ObjectType> Into<<Self as HasReferencedType>::ReferencedType> for ReferenceCountedObjectPtr<ObjectType> {
    
    /**
      | Returns the object that this pointer
      | references.
      | 
      | The pointer returned may be null, of
      | course.
      | 
      | -----------
      | @note
      | 
      | this methods allows the compiler to
      | be very lenient with what it allows you
      | to do with the pointer, it's safer to
      | disable this by setting ALOE_STRICT_REFCOUNTEDPOINTER=1,
      | which increased type safety and can
      | prevent some common slip-ups.
      |
      */
    #[inline] fn into(self) -> <Self as HasReferencedType>::ReferencedType {
        todo!();
        /*
            return referencedObject;
        */
    }
}
*/

/*
impl<ObjectType> PartialEq<*const ObjectType> for ReferenceCountedObjectPtr<ObjectType> {
    
    /**
      | Compares two ReferenceCountedObjectPtrs.
      |
      */
    #[inline] fn eq(&self, other: &*const ObjectType) -> bool {
        todo!();
        /*
            return referencedObject == other;
        */
    }
}
*/

impl<Type> PartialEq<*const Type> for ReferenceCountedObjectPtr<Type> {

    /**
      | Compares two ReferenceCountedObjectPtrs.
      |
      */
    #[inline] fn eq(&self, other: &*const Type) -> bool {
        todo!();
        /*
            return object1 == object2.get();
        */
    }
}

impl<ObjectType> ReferenceCountedObjectPtr<ObjectType> {

    /**
      | Creates a pointer to an object.
      | 
      | This will increment the object's reference-count.
      |
      */
    pub fn new_from_ref_counted_object(ref_counted_object: *mut <Self as HasReferencedType>::ReferencedType) -> Self {
    
        todo!();
        /*
        : referenced_object(refCountedObject),

            incIfNotNull (refCountedObject);
        */
    }

    /**
      | Creates a pointer to an object.
      | 
      | This will increment the object's reference-count.
      |
      */
    pub fn new_from_referenced_type(ref_counted_object: &mut <Self as HasReferencedType>::ReferencedType) -> Self {
    
        todo!();
        /*
        : referenced_object(&refCountedObject),

            refCountedObject.incReferenceCount();
        */
    }

    /**
      | Copies another pointer.
      | 
      | This will increment the object's reference-count.
      |
      */
    pub fn new_from_refcounted_object_ptr_ref(other: &ReferenceCountedObjectPtr<ObjectType>) -> Self {
    
        todo!();
        /*
        : referenced_object(other.referencedObject),

            incIfNotNull (referencedObject);
        */
    }

    /**
      | Takes-over the object from another
      | pointer.
      |
      */
    pub fn new_from_refcounted_object_ptr(other: ReferenceCountedObjectPtr<ObjectType>) -> Self {
    
        todo!();
        /*
        : referenced_object(other.referencedObject),

            other.referencedObject = nullptr;
        */
    }

    /**
      | Copies another pointer.
      | 
      | This will increment the object's reference-count
      | (if it is non-null).
      |
      */
    pub fn new_from_convertible<Convertible>(other: &ReferenceCountedObjectPtr<Convertible>) -> Self {
    
        todo!();
        /*
        : referenced_object(other.get()),

            incIfNotNull (referencedObject);
        */
    }

    /**
      | Changes this pointer to point at a different
      | object.
      | 
      | The reference count of the old object
      | is decremented, and it might be deleted
      | if it hits zero. The new object's count
      | is incremented.
      |
      */
    pub fn assign_from_ref(&mut self, other: &ReferenceCountedObjectPtr<ObjectType>) -> &mut ReferenceCountedObjectPtr<ObjectType> {
        
        todo!();
        /*
            return operator= (other.referencedObject);
        */
    }

    /**
      | Changes this pointer to point at a different
      | object.
      | 
      | The reference count of the old object
      | is decremented, and it might be deleted
      | if it hits zero. The new object's count
      | is incremented.
      |
      */
    pub fn assign_from_from_convertible<Convertible>(&mut self, other: &ReferenceCountedObjectPtr<Convertible>) -> &mut ReferenceCountedObjectPtr<ObjectType> {
    
        todo!();
        /*
            return operator= (other.get());
        */
    }

    /**
      | Changes this pointer to point at a different
      | object.
      | 
      | The reference count of the old object
      | is decremented, and it might be deleted
      | if it hits zero. The new object's count
      | is incremented.
      |
      */
    pub fn assign_from_ptr(&mut self, new_object: *mut <Self as HasReferencedType>::ReferencedType) -> &mut ReferenceCountedObjectPtr<ObjectType> {
        
        todo!();
        /*
            if (newObject != nullptr)
                return operator= (*newObject);

            reset();
            return *this;
        */
    }

    /**
      | Changes this pointer to point at a different
      | object.
      | 
      | The reference count of the old object
      | is decremented, and it might be deleted
      | if it hits zero. The new object's count
      | is incremented.
      |
      */
    pub fn assign_from_referenced_type(&mut self, new_object: &mut <Self as HasReferencedType>::ReferencedType) -> &mut ReferenceCountedObjectPtr<ObjectType> {
        
        todo!();
        /*
            if (referencedObject != &newObject)
            {
                newObject.incReferenceCount();
                auto* oldObject = referencedObject;
                referencedObject = &newObject;
                decIfNotNull (oldObject);
            }

            return *this;
        */
    }

    /**
      | Resets this pointer to a null pointer.
      |
      */
    pub fn assign_from_none(&mut self) -> &mut ReferenceCountedObjectPtr<ObjectType> {
        
        todo!();
        /*
            reset();
            return *this;
        */
    }

    /**
      | Takes-over the object from another
      | pointer.
      |
      */
    pub fn assign_from_other(&mut self, other: ReferenceCountedObjectPtr<ObjectType>) -> &mut ReferenceCountedObjectPtr<ObjectType> {
        
        todo!();
        /*
            std::swap (referencedObject, other.referencedObject);
            return *this;
        */
    }
    
    /**
      | Returns the object that this pointer
      | references.
      | 
      | The pointer returned may be null, of
      | course.
      |
      */
    pub fn get(&self) -> *mut <Self as HasReferencedType>::ReferencedType {
        
        todo!();
        /*
            return referencedObject;
        */
    }

    /**
      | Resets this object to a null pointer.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            auto oldObject = referencedObject;  // need to null the pointer before deleting the object
            referencedObject = nullptr;         // in case this ptr is itself deleted as a side-effect
            decIfNotNull (oldObject);           // of the destructor
        */
    }
    
    pub fn inc_if_not_null(o: *mut <Self as HasReferencedType>::ReferencedType)  {
        
        todo!();
        /*
            if (o != nullptr)
                o->incReferenceCount();
        */
    }
    
    pub fn dec_if_not_null(o: *mut <Self as HasReferencedType>::ReferencedType)  {
        
        todo!();
        /*
            if (o != nullptr && o->decReferenceCountWithoutDeleting())
                ContainerDeletePolicy<ReferencedType>::destroy (o);
        */
    }
}
