crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_LeakedObjectDetector.h]

#[derive(Default)]
pub struct LeakCounter {
    num_objects: AtomicI32,
}

impl Drop for LeakCounter {
    fn drop(&mut self) {
        todo!();
        /* 
                if (numObjects.value > 0)
                {
                    DBG ("*** Leaked objects detected: " << numObjects.value << " instance(s) of class " << getLeakedObjectClassName());

                    /** If you hit this, then you've leaked one or more objects of the type specified by
                        the 'OwnerClass' template parameter - the name should have been printed by the line above.

                        If you're leaking, it's probably because you're using old-fashioned, non-RAII techniques for
                        your object management. Tut, tut. Always, always use std::unique_ptrs, OwnedArrays,
                        ReferenceCountedObjects, etc, and avoid the 'delete' operator at all costs!
                    */
                    jassertfalse;
                }
             */
    }
}

/**
  | Embedding an instance of this class
  | inside another class can be used as a
  | low-overhead way of detecting leaked
  | instances.
  | 
  | This class keeps an internal static
  | count of the number of instances that
  | are active, so that when the app is shutdown
  | and the static destructors are called,
  | it can check whether there are any left-over
  | instances that may have been leaked.
  | 
  | To use it, use the ALOE_LEAK_DETECTOR
  | macro as a simple way to put one in your
  | class declaration. Have a look through
  | the aloe codebase for examples, it's
  | used in most of the classes.
  | 
  | @tags{Core}
  |
  */
pub struct LeakedObjectDetector<OwnerClass> {
    phantom: std::marker::PhantomData<OwnerClass>,
}

impl<OwnerClass> Default for LeakedObjectDetector<OwnerClass> {
    
    fn default() -> Self {
        todo!();
        /*
            ++(getCounter().numObjects)
        */
    }
}

impl<OwnerClass> Drop for LeakedObjectDetector<OwnerClass> {

    fn drop(&mut self) {
        todo!();
        /* 
            if (--(getCounter().numObjects) < 0)
            {
                DBG ("*** Dangling pointer deletion! Class: " << getLeakedObjectClassName());

                /** If you hit this, then you've managed to delete more instances of this class than you've
                    created.. That indicates that you're deleting some dangling pointers.

                    Note that although this assertion will have been triggered during a destructor, it might
                    not be this particular deletion that's at fault - the incorrect one may have happened
                    at an earlier point in the program, and simply not been detected until now.

                    Most errors like this are caused by using old-fashioned, non-RAII techniques for
                    your object management. Tut, tut. Always, always use std::unique_ptrs, OwnedArrays,
                    ReferenceCountedObjects, etc, and avoid the 'delete' operator at all costs!
                */
                jassertfalse;
            }
         */
    }
}

impl<OwnerClass> LeakedObjectDetector<OwnerClass> {

    pub fn new(_0: &LeakedObjectDetector<OwnerClass>) -> Self {
    
        todo!();
        /*
            ++(getCounter().numObjects);
        */
    }
    
    pub fn get_leaked_object_class_name() -> *const u8 {
        
        todo!();
        /*
            return OwnerClass::getLeakedObjectClassName();
        */
    }
    
    pub fn get_counter<'a>() -> &'a mut LeakCounter {
        
        todo!();
        /*
            static LeakCounter counter;
            return counter;
        */
    }
}

/**
  | This macro lets you embed a leak-detecting
  | object inside a class.
  | 
  | To use it, simply declare a ALOE_LEAK_DETECTOR(YourClassName)
  | inside a private section of the class
  | declaration. E.g.
  | 
  | 
  | -----------
  | @code
  | 
  | class MyClass
  | {
  | 
  |     MyClass();
  |     void blahBlah();
  | 
  |     ALOE_LEAK_DETECTOR (MyClass)
  | };
  | 
  | @see ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR,
  | LeakedObjectDetector
  |
  */
#[cfg(not(ALOE_LEAK_DETECTOR))]
#[cfg(ALOE_CHECK_MEMORY_LEAKS)]
macro_rules! aloe_leak_detector {
    ($OwnerClass:ident) => {
        /*
        
                friend class LeakedObjectDetector<OwnerClass>; 
                static const char* getLeakedObjectClassName()  { return #OwnerClass; } 
                LeakedObjectDetector<OwnerClass> ALOE_JOIN_MACRO (leakDetector, __LINE__);
        */
    }
}

#[cfg(not(ALOE_LEAK_DETECTOR))]
#[cfg(not(ALOE_CHECK_MEMORY_LEAKS))]
macro_rules! aloe_leak_detector {
    ($OwnerClass:ident) => { }
}
