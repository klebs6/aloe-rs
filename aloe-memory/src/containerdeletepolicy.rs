crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_ContainerDeletePolicy.h]

/**
  | Used by container classes as an indirect
  | way to delete an object of a particular
  | type.
  | 
  | The generic implementation of this
  | class simply calls 'delete', but you
  | can create a specialised version of
  | it for a particular class if you need
  | to delete that type of object in a more
  | appropriate way.
  | 
  | @see OwnedArray
  | 
  | @tags{Core}
  |
  */
pub struct ContainerDeletePolicy<ObjectType> {
    phantom: std::marker::PhantomData<ObjectType>,
}

impl<ObjectType> ContainerDeletePolicy<ObjectType> {

    pub fn destroy(object: *mut ObjectType)  {
        
        todo!();
        /*
            // If the line below triggers a compiler error, it means that you are using
            // an incomplete type for ObjectType (for example, a type that is declared
            // but not defined). This is a problem because then the following delete is
            // undefined behaviour. The purpose of the sizeof is to capture this situation.
            // If this was caused by a OwnedArray of a forward-declared type, move the
            // implementation of all methods trying to use the OwnedArray (e.g. the destructor
            // of the class owning it) into cpp files where they can see to the definition
            // of ObjectType. This should fix the error.
            ignoreUnused (sizeof (ObjectType));

            delete object;
        */
    }
}
