crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_Singleton.h]

/**
  | Used by the ALOE_DECLARE_SINGLETON
  | macros to manage a static pointer to
  | a singleton instance.
  | 
  | You generally won't use this directly,
  | but see the macros ALOE_DECLARE_SINGLETON,
  | ALOE_DECLARE_SINGLETON_SINGLETHREADED,
  | ALOE_DECLARE_SINGLETON_SINGLETHREADED_MINIMAL,
  | and ALOE_IMPLEMENT_SINGLETON for
  | how it is intended to be used.
  | 
  | @tags{Core}
  |
  */
pub struct SingletonHolder<Type,MutexType,const ONLY_CREATE_ONCE_PER_RUN: bool> {
    base:     MutexType, // (inherited so we can use the empty-base-class optimisation)
    instance: *mut Type, // default = nullptr
}

impl<Type,MutexType: Default,const ONLY_CREATE_ONCE_PER_RUN: bool>
Default for SingletonHolder<Type,MutexType,ONLY_CREATE_ONCE_PER_RUN> {

    fn default() -> Self {

        Self {
            base:     MutexType::default(),
            instance: null_mut(),
        }
    }
}

impl<Type,MutexType,const ONLY_CREATE_ONCE_PER_RUN: bool> 
Drop for SingletonHolder<Type,MutexType,ONLY_CREATE_ONCE_PER_RUN> {

    fn drop(&mut self) {
        todo!();
        /* 
            /* The static singleton holder is being deleted before the object that it holds
               has been deleted. This could mean that you've forgotten to call clearSingletonInstance()
               in the class's destructor, or have failed to delete it before your app shuts down.
               If you're having trouble cleaning up your singletons, perhaps consider using the
               SharedResourcePointer class instead.
            */
            jassert (instance == nullptr);
         */
    }
}

impl<Type,MutexType,const ONLY_CREATE_ONCE_PER_RUN: bool> 
SingletonHolder<Type,MutexType,ONLY_CREATE_ONCE_PER_RUN> {

    /**
      | Returns the current instance, or creates
      | a new instance if there isn't one.
      |
      */
    pub fn get(&mut self) -> *mut Type {
        
        todo!();
        /*
            if (instance == nullptr)
            {
                typename MutexType::ScopedLockType sl (*this);

                if (instance == nullptr)
                {
                    auto once = ONLY_CREATE_ONCE_PER_RUN; // (local copy avoids VS compiler warning about this being constant)

                    if (once)
                    {
                        static bool createdOnceAlready = false;

                        if (createdOnceAlready)
                        {
                            // This means that the doNotRecreateAfterDeletion flag was set
                            // and you tried to create the singleton more than once.
                            jassertfalse;
                            return nullptr;
                        }

                        createdOnceAlready = true;
                    }

                    static bool alreadyInside = false;

                    if (alreadyInside)
                    {
                        // This means that your object's constructor has done something which has
                        // ended up causing a recursive loop of singleton creation..
                        jassertfalse;
                    }
                    else
                    {
                        alreadyInside = true;
                        getWithoutChecking();
                        alreadyInside = false;
                    }
                }
            }

            return instance;
        */
    }

    /**
      | Returns the current instance, or creates
      | a new instance if there isn't one, but
      | doesn't do any locking, or checking
      | for recursion or error conditions.
      |
      */
    pub fn get_without_checking(&mut self) -> *mut Type {
        
        todo!();
        /*
            if (instance == nullptr)
            {
                auto newObject = new Type(); // (create into a local so that instance is still null during construction)
                instance = newObject;
            }

            return instance;
        */
    }

    /**
      | Deletes and resets the current instance,
      | if there is one.
      |
      */
    pub fn delete_instance(&mut self)  {
        
        todo!();
        /*
            typename MutexType::ScopedLockType sl (*this);
            auto old = instance;
            instance = nullptr;
            delete old;
        */
    }

    /**
      | Called by the class's destructor to
      | clear the pointer if it is currently
      | set to the given object.
      |
      */
    pub fn clear(&mut self, expected_object: *mut Type)  {
        
        todo!();
        /*
            if (instance == expectedObject)
                instance = nullptr;
        */
    }
}

/**
  | Macro to generate the appropriate methods
  | and boilerplate for a singleton class.
  | 
  | To use this, add the line ALOE_DECLARE_SINGLETON(MyClass,
  | doNotRecreateAfterDeletion) to the
  | class's definition.
  | 
  | Then put a macro ALOE_IMPLEMENT_SINGLETON(MyClass)
  | along with the class's implementation
  | code.
  | 
  | It's also a very good idea to also add
  | the call clearSingletonInstance()
  | in your class's destructor, in case
  | it is deleted by other means than deleteInstance()
  | 
  | Clients can then call the static method
  | MyClass::getInstance() to get a pointer
  | to the singleton, or MyClass::getInstanceWithoutCreating()
  | which will return nullptr if no instance
  | currently exists.
  | 
  | e.g.
  | 
  | 
  | -----------
  | @code
  | 
  | struct MySingleton
  | {
  |     MySingleton() {}
  | 
  |     ~MySingleton()
  |     {
  |         // this ensures that no dangling pointers are left when the
  |         // singleton is deleted.
  |         clearSingletonInstance();
  |     }
  | 
  |     ALOE_DECLARE_SINGLETON (MySingleton, false)
  | };
  | 
  | // ..and this goes in a suitable .cpp file:
  | ALOE_IMPLEMENT_SINGLETON (MySingleton)
  | 
  | // example of usage:
  | auto* m = MySingleton::getInstance(); // creates the singleton if there isn't already one.
  | 
  | ...
  | 
  | MySingleton::deleteInstance(); // safely deletes the singleton (if it's been created).
  | 
  | If doNotRecreateAfterDeletion = true,
  | it won't allow the object to be created
  | more than once during the process's
  | lifetime - i.e. after you've created
  | and deleted the object, getInstance()
  | will refuse to create another one. This
  | can be useful to stop objects being accidentally
  | re-created during your app's shutdown
  | code.
  | 
  | If you know that your object will only
  | be created and deleted by a single thread,
  | you can use the slightly more efficient
  | ALOE_DECLARE_SINGLETON_SINGLETHREADED
  | macro instead of this one.
  | 
  | @see ALOE_IMPLEMENT_SINGLETON, ALOE_DECLARE_SINGLETON_SINGLETHREADED
  |
  */
#[macro_export]
macro_rules! aloe_declare_singleton {
    ($Classname:ty, 
     $doNotRecreateAfterDeletion:expr) => {
        /*
        

            static SingletonHolder<Classname, CriticalSection, doNotRecreateAfterDeletion> singletonHolder; 
            friend decltype (singletonHolder); 

            static Classname*  getInstance()                           { return singletonHolder.get(); } 
            static Classname*  getInstanceWithoutCreating()    { return singletonHolder.instance; } 
            static void  deleteInstance()                      { singletonHolder.deleteInstance(); } 
            void clearSingletonInstance()                                   { singletonHolder.clear (this); }
        */
    }
}

/**
  | This is a counterpart to the ALOE_DECLARE_SINGLETON
  | macros.
  | 
  | After adding the ALOE_DECLARE_SINGLETON
  | to the class definition, this macro
  | has to be used in the cpp file.
  |
  */
#[macro_export] macro_rules! aloe_implement_singleton {
    ($Classname:ty) => {
        /*
        

            decltype (Classname::singletonHolder) Classname::singletonHolder;
        */
    }
}

/**
  | Macro to declare member variables and
  | methods for a singleton class.
  | 
  | This is exactly the same as ALOE_DECLARE_SINGLETON,
  | but doesn't use a critical section to
  | make access to it thread-safe. If you
  | know that your object will only ever
  | be created or deleted by a single thread,
  | then this is a more efficient version
  | to use.
  | 
  | If doNotRecreateAfterDeletion = true,
  | it won't allow the object to be created
  | more than once during the process's
  | lifetime - i.e. after you've created
  | and deleted the object, getInstance()
  | will refuse to create another one. This
  | can be useful to stop objects being accidentally
  | re-created during your app's shutdown
  | code.
  | 
  | See the documentation for ALOE_DECLARE_SINGLETON
  | for more information about how to use
  | it. Just like ALOE_DECLARE_SINGLETON
  | you need to also have a corresponding
  | ALOE_IMPLEMENT_SINGLETON statement
  | somewhere in your code.
  | 
  | @see ALOE_IMPLEMENT_SINGLETON, ALOE_DECLARE_SINGLETON,
  | ALOE_DECLARE_SINGLETON_SINGLETHREADED_MINIMAL
  |
  */
#[macro_export] macro_rules! aloe_declare_singleton_singlethreaded {
    ($Classname:ident, $doNotRecreateAfterDeletion:ident) => {
        /*
        

            static SingletonHolder<Classname, DummyCriticalSection, doNotRecreateAfterDeletion> singletonHolder; 
            friend decltype (singletonHolder); 

            static Classname*  getInstance()                           { return singletonHolder.get(); } 
            static Classname*  getInstanceWithoutCreating()    { return singletonHolder.instance; } 
            static void  deleteInstance()                      { singletonHolder.deleteInstance(); } 
            void clearSingletonInstance()                                   { singletonHolder.clear (this); }
        */
    }
}

/**
  | Macro to declare member variables and
  | methods for a singleton class.
  | 
  | This is like ALOE_DECLARE_SINGLETON_SINGLETHREADED,
  | but doesn't do any checking for recursion
  | or repeated instantiation. It's intended
  | for use as a lightweight version of a
  | singleton, where you're using it in
  | very straightforward circumstances
  | and don't need the extra checking.
  | 
  | See the documentation for ALOE_DECLARE_SINGLETON
  | for more information about how to use
  | it. Just like ALOE_DECLARE_SINGLETON
  | you need to also have a corresponding
  | ALOE_IMPLEMENT_SINGLETON statement
  | somewhere in your code.
  | 
  | @see ALOE_IMPLEMENT_SINGLETON, ALOE_DECLARE_SINGLETON
  |
  */
#[macro_export] macro_rules! aloe_declare_singleton_singlethreaded_minimal {
    ($Classname:ty) => {
        /*
        

            static SingletonHolder<Classname, DummyCriticalSection, false> singletonHolder; 
            friend decltype (singletonHolder); 

            static Classname*  getInstance()                           { return singletonHolder.getWithoutChecking(); } 
            static Classname*  getInstanceWithoutCreating()    { return singletonHolder.instance; } 
            static void  deleteInstance()                      { singletonHolder.deleteInstance(); } 
            void clearSingletonInstance()                                   { singletonHolder.clear (this); }
        */
    }
}

/*
  | These are ancient macros, and have now been
  | updated with new names to match the Aloe style
  | guide, so please update your code to use the
  | newer versions!
  */

#[macro_export] macro_rules! aloe_declaresingleton {
    ($Classname:ty, $doNotRecreate:expr) => {
        /*
                ALOE_DECLARE_SINGLETON(Classname, doNotRecreate)
        */
    }
}

#[macro_export] macro_rules! aloe_declaresingleton_singlethreaded {
    ($Classname:ty, $doNotRecreate:expr) => {
        /*
                ALOE_DECLARE_SINGLETON_SINGLETHREADED(Classname, doNotRecreate)
        */
    }
}

#[macro_export] macro_rules! aloe_declaresingleton_singlethreaded_minimal {
    ($Classname:ty) => {
        /*
                ALOE_DECLARE_SINGLETON_SINGLETHREADED_MINIMAL(Classname)
        */
    }
}

#[macro_export] macro_rules! aloe_implementsingleton {
    ($Classname:ty) => {
        /*
                ALOE_IMPLEMENT_SINGLETON(Classname)
        */
    }
}

#[macro_export] macro_rules! aloe_implementsingleton_singlethreaded {
    ($Classname:ty) => {
        /*
                ALOE_IMPLEMENT_SINGLETON(Classname)
        */
    }
}
