crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_ThreadLocalValue.h]

#[no_copy]
pub struct ThreadLocalValueObjectHolder<Type>
{
    thread_id: Atomic<ThreadID>,
    next:      *mut ThreadLocalValueObjectHolder<Type>,
    object:    Type,
}

impl<Type> ThreadLocalValueObjectHolder<Type> {
    
    pub fn new(
        id_to_use: ThreadID,
        n:         *mut ThreadLocalValueObjectHolder<Type>

    ) -> Self {
    
        todo!();
        /*
        : thread_id(idToUse),
        : next(n),
        : object(),

        
        */
    }
}

/**
  | Provides cross-platform support for
  | thread-local objects.
  | 
  | This class holds an internal list of
  | objects of the templated type, keeping
  | an instance for each thread that requests
  | one. The first time a thread attempts
  | to access its value, an object is created
  | and added to the list for that thread.
  | 
  | Typically, you'll probably want to
  | create a static instance of a ThreadLocalValue
  | object, or hold one within a singleton.
  | 
  | The templated class for your value must
  | be a primitive type, or a simple POD struct.
  | 
  | When a thread no longer needs to use its
  | value, it can call releaseCurrentThreadStorage()
  | to allow the storage to be re-used by
  | another thread. If a thread exits without
  | calling this method, the object storage
  | will be left allocated until the ThreadLocalValue
  | object is deleted.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct ThreadLocalValue<Type> {
    first: RefCell<Atomic<*mut ThreadLocalValueObjectHolder<Type>>>,
}

impl<Type> Default for ThreadLocalValue<Type> {

    fn default() -> Self {
        todo!();
    }
}

impl<Type> Drop for ThreadLocalValue<Type> {

    /**
      | Destructor. When this object is deleted,
      | all the value objects for all threads
      | will be deleted.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
            for (auto* o = first.get(); o != nullptr;)
            {
                auto* next = o->next;
                delete o;
                o = next;
            }
         */
    }
}

impl<Type> Deref for ThreadLocalValue<Type> {

    type Target = Type;
    
    /**
      | Returns a reference to this thread's
      | instance of the value.
      | 
      | -----------
      | @note
      | 
      | the first time a thread tries to access
      | the value, an instance of the value object
      | will be created - so if your value's class
      | has a non-trivial constructor, be aware
      | that this method could invoke it.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return get();
        */
    }
}

impl<Type> ThreadLocalValue<Type> {
    
    /**
      | Assigns a new value to the thread-local
      | object.
      |
      */
    pub fn assign_from(&mut self, new_value: &Type) -> &mut ThreadLocalValue<Type> {
        
        todo!();
        /*
            get() = newValue; return *this;
        */
    }


    /**
      | Returns a reference to this thread's
      | instance of the value.
      | 
      | -----------
      | @note
      | 
      | the first time a thread tries to access
      | the value, an instance of the value object
      | will be created - so if your value's class
      | has a non-trivial constructor, be aware
      | that this method could invoke it.
      |
      */
    pub fn get(&self) -> &mut Type {
        
        todo!();
        /*
            auto threadId = Thread::getCurrentThreadId();
            ObjectHolder* o = nullptr;

            for (o = first.get(); o != nullptr; o = o->next)
                if (o->threadId.get() == threadId)
                    return o->object;

            for (o = first.get(); o != nullptr; o = o->next)
                if (o->threadId.compareAndSetBool (threadId, nullptr))
                    break;

            if (o != nullptr)
                o->object = Type();
            else
                for (o = new ObjectHolder (threadId, first.get());
                     ! first.compareAndSetBool (o, o->next);
                     o->next = first.get());

            return o->object;
        */
    }

    /**
      | Called by a thread before it terminates,
      | to allow this class to release any storage
      | associated with the thread.
      |
      */
    pub fn release_current_thread_storage(&mut self)  {
        
        todo!();
        /*
            auto threadId = Thread::getCurrentThreadId();

            for (auto* o = first.get(); o != nullptr; o = o->next)
                if (o->threadId.compareAndSetBool (nullptr, threadId))
                    return;
        */
    }
}
