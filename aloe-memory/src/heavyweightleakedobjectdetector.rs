crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_HeavyweightLeakedObjectDetector.h]

#[derive(Default)]
pub struct BacktraceMapHolder<OwnerClass> {
    map: <HeavyweightLeakedObjectDetector<OwnerClass> as HasBacktraceMap>::BacktraceMap,
}

impl<OwnerClass> Drop for BacktraceMapHolder<OwnerClass> {

    fn drop(&mut self) {
        todo!();
        /* 
                if (map.size() > 0)
                {
                    DBG ("*** Leaked objects detected: " << map.size() << " instance(s) of class " << getLeakedObjectClassName());
                    DBG (getFormattedBacktracesString());

                    /** If you hit this, then you've leaked one or more objects of the type specified by
                        the 'OwnerClass' template parameter - the name and stack trace of its creation should
                        have been printed by the lines above.

                        If you're leaking, it's probably because you're using old-fashioned, non-RAII techniques for
                        your object management. Tut, tut. Always, always use std::unique_ptrs, OwnedArrays,
                        ReferenceCountedObjects, etc, and avoid the 'delete' operator at all costs!
                    */
                    jassertfalse;
                }
             */
    }
}

impl<OwnerClass> BacktraceMapHolder<OwnerClass> {

    pub fn get_formatted_backtraces_string(&self) -> String {
        
        todo!();
        /*
            String str;

                int counter = 1;
                for (auto& bt : map)
                {
                    str << "\nBacktrace " << String (counter++)                                << "\n"
                        << "-----------------------------------------------------------------" << "\n"
                        << bt.second;
                }

                return str;
        */
    }
}

/**
  | This class is a useful way of tracking
  | down hard to find memory leaks when the
  | regular LeakedObjectDetector isn't
  | enough.
  | 
  | As well as firing when any instances
  | of the OwnerClass type are leaked, it
  | will print out a stack trace showing
  | where the leaked object was created.
  | This is obviously quite a heavyweight
  | task so, unlike the LeakedObjectDetector
  | which should be always be added to your
  | classes, you should only use this object
  | temporarily when you are debugging
  | and remove it when finished.
  | 
  | To use it, use the ALOE_HEAVYWEIGHT_LEAK_DETECTOR
  | macro as a simple way to put one in your
  | class declaration.
  | 
  | @tags{Core}
  |
  */
pub struct HeavyweightLeakedObjectDetector<OwnerClass> {
    phantom: std::marker::PhantomData<OwnerClass>,
}

impl<OwnerClass> Default for HeavyweightLeakedObjectDetector<OwnerClass> {
    
    fn default() -> Self {
        todo!();
        /*


            getBacktraceMap()[this] = SystemStats::getStackBacktrace()
        */
    }
}

impl<OwnerClass> Drop for HeavyweightLeakedObjectDetector<OwnerClass> {
    fn drop(&mut self) {
        todo!();
        /*      getBacktraceMap().erase (this);  */
    }
}

pub trait HasBacktraceMap {
    type BacktraceMap;
}

impl<OwnerClass> HasBacktraceMap for HeavyweightLeakedObjectDetector<OwnerClass> {
    type BacktraceMap = HashMap<*mut HeavyweightLeakedObjectDetector<OwnerClass>,String>;
}

impl<OwnerClass> HeavyweightLeakedObjectDetector<OwnerClass> {

    pub fn new(_0: &HeavyweightLeakedObjectDetector<OwnerClass>) -> Self {
    
        todo!();
        /*


            getBacktraceMap()[this] = SystemStats::getStackBacktrace();
        */
    }
    
    pub fn get_backtrace_map<'a>() -> &'a mut <Self as HasBacktraceMap>::BacktraceMap {
        
        todo!();
        /*
            static BacktraceMapHolder holder;
            return holder.map;
        */
    }
    
    pub fn get_leaked_object_class_name() -> *const u8 {
        
        todo!();
        /*
            return OwnerClass::getLeakedObjectClassName();
        */
    }
}

/**
  | This macro lets you embed a heavyweight
  | leak-detecting object inside a class.
  | 
  | To use it, simply declare a ALOE_HEAVYWEIGHT_LEAK_DETECTOR
  | (YourClassName) inside a private section
  | of the class declaration. E.g.
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
  |     ALOE_HEAVYWEIGHT_LEAK_DETECTOR (MyClass)
  | };
  | 
  | NB: you should only use this when you
  | really need to track down a tricky memory
  | leak, and should never leave one of these
  | inside a class!
  | 
  | @see HeavyweightLeakedObjectDetector,
  | ALOE_LEAK_DETECTOR, LeakedObjectDetector
  |
  */
#[cfg(not(ALOE_HEAVYWEIGHT_LEAK_DETECTOR))]
#[cfg(ALOE_CHECK_MEMORY_LEAKS)]
macro_rules! aloe_heavyweight_leak_detector {
    ($OwnerClass:ident) => {
        /*
        
                friend class HeavyweightLeakedObjectDetector<OwnerClass>; 
                static const char* getLeakedObjectClassName()  { return #OwnerClass; } 
                HeavyweightLeakedObjectDetector<OwnerClass> ALOE_JOIN_MACRO (leakDetector, __LINE__);
        */
    }
}

#[cfg(not(ALOE_HEAVYWEIGHT_LEAK_DETECTOR))]
#[cfg(not(ALOE_CHECK_MEMORY_LEAKS))]
macro_rules! aloe_heavyweight_leak_detector {
    ($OwnerClass:ident) => { }
}
