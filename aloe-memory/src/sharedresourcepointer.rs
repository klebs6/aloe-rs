crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_SharedResourcePointer.h]

/**
  | A smart-pointer that automatically
  | creates and manages the lifetime of
  | a shared static instance of a class.
  | 
  | The SharedObjectType template type
  | indicates the class to use for the shared
  | object - the only requirements on this
  | class are that it must have a public default
  | constructor and destructor.
  | 
  | The SharedResourcePointer offers
  | a pattern that differs from using a singleton
  | or static instance of an object, because
  | it uses reference-counting to make
  | sure that the underlying shared object
  | is automatically created/destroyed
  | according to the number of SharedResourcePointer
  | objects that exist. When the last one
  | is deleted, the underlying object is
  | also immediately destroyed. This allows
  | you to use scoping to manage the lifetime
  | of a shared resource.
  | 
  | -----------
  | @note
  | 
  | The construction/deletion of the shared
  | object must not involve any code that
  | makes recursive calls to a SharedResourcePointer,
  | or you'll cause a deadlock.
  | 
  | Example:
  | 
  | -----------
  | @code
  | 
  | // An example of a class that contains the shared data you want to use.
  | struct MySharedData
  | {
  |     // There's no need to ever create an instance of this class directly yourself,
  |     // but it does need a public constructor that does the initialisation.
  |     MySharedData()
  |     {
  |         sharedStuff = generateHeavyweightStuff();
  |     }
  | 
  |     Vec<SomeKindOfData> sharedStuff;
  | };
  | 
  | struct DataUserClass
  | {
  |     DataUserClass()
  |     {
  |         // Multiple instances of the DataUserClass will all have the same
  |         // shared common instance of MySharedData referenced by their sharedData
  |         // member variables.
  |         useSharedStuff (sharedData->sharedStuff);
  |     }
  | 
  |     // By keeping this pointer as a member variable, the shared resource
  |     // is guaranteed to be available for as long as the DataUserClass object.
  |     SharedResourcePointer<MySharedData> sharedData;
  | };
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct SharedResourcePointer<SharedObjectType> {
    shared_object: *mut SharedObjectType,
}

impl<SharedObjectType> Default for SharedResourcePointer<SharedObjectType> {
    
    /**
      | Creates an instance of the shared object.
      | 
      | If other SharedResourcePointer objects
      | for this type already exist, then this
      | one will simply point to the same shared
      | object that they are already using.
      | Otherwise, if this is the first SharedResourcePointer
      | to be created, then a shared object will
      | be created automatically.
      |
      */
    fn default() -> Self {
        todo!();
        /*


            initialise()
        */
    }
}

impl<SharedObjectType> Drop for SharedResourcePointer<SharedObjectType> {

    /**
      | Destructor.
      | 
      | If no other SharedResourcePointer
      | objects exist, this will also delete
      | the shared object to which it refers.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
            auto& holder = getSharedObjectHolder();
            const SpinLock::ScopedLockType sl (holder.lock);

            if (--(holder.refCount) == 0)
                holder.sharedInstance = nullptr;
         */
    }
}

/*
impl<SharedObjectType> Into<SharedObjectType> for SharedResourcePointer<SharedObjectType> {
    
    /**
      | Returns the shared object.
      |
      */
    #[inline] fn into(self) -> SharedObjectType {
        todo!();
        /*
            return sharedObject;
        */
    }
}
*/

impl<SharedObjectType> Deref for SharedResourcePointer<SharedObjectType> {

    type Target = SharedObjectType;
    
    /**
      | Returns the shared object pointer.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return sharedObject;
        */
    }
}

pub mod shared_resource_pointer {

    use super::*;

    pub struct SharedObjectHolder<SharedObjectType>
    {
        lock:            spin::Mutex<i32 /*dummy, maybe need inner*/>,
        shared_instance: Box<SharedObjectType>,
        ref_count:       i32,
    }
}

impl<SharedObjectType> SharedResourcePointer<SharedObjectType> {

    pub fn new(_0: &SharedResourcePointer<SharedObjectType>) -> Self {
    
        todo!();
        /*


            initialise();
        */
    }
    
    /**
      | Returns the shared object.
      |
      */
    pub fn get(&self) -> &mut SharedObjectType {
        
        todo!();
        /*
            return *sharedObject;
        */
    }

    /**
      | Returns the object that this pointer
      | references.
      |
      */
    pub fn get_object(&self) -> &mut SharedObjectType {
        
        todo!();
        /*
            return *sharedObject;
        */
    }

    /**
      | Returns the number of SharedResourcePointers
      | that are currently holding the shared
      | object.
      |
      */
    pub fn get_reference_count(&self) -> i32 {
        
        todo!();
        /*
            return getSharedObjectHolder().refCount;
        */
    }
    
    pub fn get_shared_object_holder<'a>() 
        -> &'a mut shared_resource_pointer::SharedObjectHolder<SharedObjectType> 
    {
        todo!();
        /*
            static void* holder [(sizeof (SharedObjectHolder) + sizeof(void*) - 1) / sizeof(void*)] = { nullptr };
            return *reinterpret_cast<SharedObjectHolder*> (holder);
        */
    }
    
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            auto& holder = getSharedObjectHolder();
            const SpinLock::ScopedLockType sl (holder.lock);

            if (++(holder.refCount) == 1)
                holder.sharedInstance.reset (new SharedObjectType());

            sharedObject = holder.sharedInstance.get();
        */
    }
}
