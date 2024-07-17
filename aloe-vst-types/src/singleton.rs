/*!
  | Automatic creation and destruction
  | of singleton instances.
  |
  */
crate::ix!();

pub type SingletonObjectVector = Vec<*mut *mut FObject>;

lazy_static!{
    /*
    SingletonObjectVector* singletonInstances = nullptr;
        bool singletonsTerminated = false;
        Steinberg::Base::Thread::FLock* singletonsLock;
    */
}

/**
  | Returns true when singleton instances
  | were already released.
  |
  */
pub fn singleton_is_terminated() -> bool {
    
    todo!();
        /*
            return singletonsTerminated;
        */
}

/**
  | lock and unlock the singleton registration
  | for multi-threading safety
  |
  */
pub fn singleton_lock_register()  {
    
    todo!();
        /*
            if (!singletonsLock) // assume first call not from multiple threads
                singletonsLock = NEW Steinberg::Base::Thread::FLock;
            singletonsLock->lock ();
        */
}

pub fn singleton_unlock_register()  {
    
    todo!();
        /*
            singletonsLock->unlock ();
        */
}

/**
  | registers an instance (type FObject)
  |
  */
pub fn singleton_register_instance(o: *mut *mut FObject)  {
    
    todo!();
        /*
            SMTG_ASSERT (singletonsTerminated == false)
            if (singletonsTerminated == false)
            {
                if (singletonInstances == nullptr)
                    singletonInstances = NEW std::vector<FObject**>;
                singletonInstances->push_back (o);
            }
        */
}

#[derive(Default)]
pub struct SingletonDeleter {

}

impl Drop for SingletonDeleter {

    fn drop(&mut self) {
        todo!();
        /*
            singletonsTerminated = true;
                if (singletonInstances)
                {
                    for (SingletonObjectVector::iterator it = singletonInstances->begin (),
                                                end = singletonInstances->end ();
                         it != end; ++it)
                    {
                        FObject** obj = (*it);
                        (*obj)->release ();
                        *obj = nullptr;
                        obj = nullptr;
                    }

                    delete singletonInstances;
                    singletonInstances = nullptr;
                }
                delete singletonsLock;
                singletonsLock = nullptr;
        */
    }
}

lazy_static! {
    static ref singleton_deleter: SingletonDeleter = SingletonDeleter::default();
}

#[macro_export]
macro_rules! singleton {
    ($ClassName:ty) => {
        /*
        
            static ClassName* instance (bool create = true) 
            { 
                static Steinberg::FObject* inst = nullptr; 
                if (inst == nullptr && create && Steinberg::Singleton::isTerminated () == false) 
                {   
                    Steinberg::Singleton::lockRegister (); 
                    if (inst == nullptr) 
                    { 
                        inst = NEW ClassName; 
                        Steinberg::Singleton::registerInstance (&inst); 
                    } 
                    Steinberg::Singleton::unlockRegister (); 
                }   
                return (ClassName*)inst; 
            }
        */
    }
}
