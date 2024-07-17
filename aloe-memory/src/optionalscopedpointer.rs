crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_OptionalScopedPointer.h]

/**
  | Holds a pointer to an object which can
  | optionally be deleted when this pointer
  | goes out of scope.
  | 
  | This acts in many ways like a std::unique_ptr,
  | but allows you to specify whether or
  | not the object is deleted.
  | 
  | @tags{Core}
  |
  */
pub struct OptionalScopedPointer<ObjectType: ?Sized> {
    object:        Box<ObjectType>,
    should_delete: bool, // default = false
}

impl<ObjectType: ?Sized> Default for OptionalScopedPointer<ObjectType> {
    
    /**
      | Creates an empty OptionalScopedPointer.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl<ObjectType: ?Sized> Drop for OptionalScopedPointer<ObjectType> {

    /**
      | The destructor may or may not delete
      | the object that is being held, depending
      | on the takeOwnership flag that was specified
      | when the object was first passed into
      | an OptionalScopedPointer constructor.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
            reset();
         */
    }
}

/*
impl<ObjectType> Into<ObjectType> for OptionalScopedPointer<ObjectType> {
    
    /**
      | Returns the object that this pointer
      | is managing.
      |
      */
    #[inline] fn into(self) -> ObjectType {
        todo!();
        /*
            return object.get();
        */
    }
}
*/

impl<ObjectType: ?Sized> OptionalScopedPointer<ObjectType> {
    
    /**
      | Returns the object that this pointer
      | is managing.
      |
      */
    pub fn get(&self) -> *mut ObjectType {
        
        todo!();
        /*
            return object.get();
        */
    }
}

impl<ObjectType: ?Sized> Deref for OptionalScopedPointer<ObjectType> {

    type Target = ObjectType;

    /**
      | Lets you access methods and properties
      | of the object that this pointer is holding.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return object.get();
        */
    }
}

impl<ObjectType: ?Sized> OptionalScopedPointer<ObjectType> {

    /**
      | Creates an OptionalScopedPointer
      | to point to a given object, and specifying
      | whether the OptionalScopedPointer
      | will delete it.
      | 
      | If takeOwnership is true, then the OptionalScopedPointer
      | will act like a std::unique_ptr, deleting
      | the object when it is itself deleted.
      | If this parameter is false, then the
      | OptionalScopedPointer just holds
      | a normal pointer to the object, and won't
      | delete it.
      |
      */
    pub fn new_from_object_to_hold(
        object_to_hold: *mut ObjectType,
        take_ownership: bool) -> Self {
    
        todo!();
        /*
        : object(objectToHold),
        : should_delete(takeOwnership),

        
        */
    }

    /**
      | Takes ownership of the object that another
      | OptionalScopedPointer holds.
      | 
      | Like a normal std::unique_ptr, the
      | objectToTransferFrom object will
      | become null, as ownership of the managed
      | object is transferred to this object.
      | 
      | The flag to indicate whether or not to
      | delete the managed object is also copied
      | from the source object.
      |
      */
    pub fn new_from_optional_scoped_ptr(other: OptionalScopedPointer<ObjectType>) -> Self {
    
        todo!();
        /*
        : object(std::move (other.object)),
        : should_delete(std::move (other.shouldDelete)),

        
        */
    }

    /**
      | Takes ownership of the object owned
      | by `ptr`.
      |
      */
    pub fn new_from_boxed_object(ptr: Box<ObjectType>) -> Self {
    
        todo!();
        /*
        : optional_scoped_pointer(ptr.release(), true),

        
        */
    }

    /**
      | Points to the same object as `ref`, but
      | does not take ownership.
      |
      */
    pub fn new_from_object_ref(ref_: &mut ObjectType) -> Self {
    
        todo!();
        /*
        : optional_scoped_pointer(std::addressof (ref), false),

        
        */
    }

    /**
      | Takes ownership of the object that another
      | OptionalScopedPointer holds.
      | 
      | Like a normal std::unique_ptr, the
      | objectToTransferFrom object will
      | become null, as ownership of the managed
      | object is transferred to this object.
      | 
      | The ownership flag that says whether
      | or not to delete the managed object is
      | also copied from the source object.
      |
      */
    pub fn assign_from(&mut self, other: OptionalScopedPointer<ObjectType>) -> &mut OptionalScopedPointer<ObjectType> {
        
        todo!();
        /*
            swapWith (other);
            other.reset();
            return *this;
        */
    }
    
    /**
      | Removes the current object from this
      | OptionalScopedPointer without deleting
      | it.
      | 
      | This will return the current object,
      | and set this OptionalScopedPointer
      | to a null pointer.
      |
      */
    pub fn release(&mut self) -> *mut ObjectType {
        
        todo!();
        /*
            return object.release();
        */
    }

    /**
      | Resets this pointer to null, possibly
      | deleting the object that it holds, if
      | it has ownership of it.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            if (! shouldDelete)
                object.release();
            else
                object.reset();
        */
    }

    /**
      | Does the same thing as reset().
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            reset();
        */
    }

    /**
      | Makes this OptionalScopedPointer
      | point at a new object, specifying whether
      | the OptionalScopedPointer will take
      | ownership of the object.
      | 
      | If takeOwnership is true, then the OptionalScopedPointer
      | will act like a std::unique_ptr, deleting
      | the object when it is itself deleted.
      | If this parameter is false, then the
      | OptionalScopedPointer just holds
      | a normal pointer to the object, and won't
      | delete it.
      |
      */
    pub fn set(&mut self, 
        new_object:     *mut ObjectType,
        take_ownership: bool)  {
        
        todo!();
        /*
            if (object.get() != newObject)
            {
                reset();
                object.reset (newObject);
            }

            shouldDelete = takeOwnership;
        */
    }

    /**
      | Makes this OptionalScopedPointer
      | point at a new object, and take ownership
      | of that object.
      |
      */
    pub fn set_owned(&mut self, new_object: *mut ObjectType)  {
        
        todo!();
        /*
            set (newObject, true);
        */
    }

    /**
      | Makes this OptionalScopedPointer
      | point at a new object, but will not take
      | ownership of that object.
      |
      */
    pub fn set_non_owned(&mut self, new_object: *mut ObjectType)  {
        
        todo!();
        /*
            set (newObject, false);
        */
    }

    /**
      | Returns true if the target object will
      | be deleted when this pointer object
      | is deleted.
      |
      */
    pub fn will_delete_object(&self) -> bool {
        
        todo!();
        /*
            return shouldDelete;
        */
    }
    
    /**
      | Swaps this object with another OptionalScopedPointer.
      | 
      | The two objects simply exchange their
      | states.
      |
      */
    pub fn swap_with(&mut self, other: &mut OptionalScopedPointer<ObjectType>)  {
        
        todo!();
        /*
            std::swap (other.object, object);
            std::swap (other.shouldDelete, shouldDelete);
        */
    }
}
