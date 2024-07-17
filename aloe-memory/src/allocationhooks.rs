crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_AllocationHooks.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_AllocationHooks.cpp]

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub struct AllocationHooks {
    listener_list: ListenerList<Listener>,
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub mod allocation_hooks {
    use super::*;

    pub trait Listener
    {
        fn new_or_delete_called(&mut self);
    }
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
impl AllocationHooks {

    pub fn add_listener(&mut self, l: *mut Listener)  {
        
        todo!();
        /*
            listenerList.add (l);
        */
    }
    
    pub fn remove_listener(&mut self, l: *mut Listener)  {
        
        todo!();
        /*
            listenerList.remove (l);
        */
    }
}

/**
  | Scoped checker which will cause a unit
  | test failure if any new/delete calls
  | are made during the lifetime of the UnitTestAllocationChecker.
  |
  */
#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub struct UnitTestAllocationChecker {
    base:      AllocationHooks::Listener,
    unit_test: &mut UnitTest,
    calls:     usize, // default = 0
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
impl Drop for UnitTestAllocationChecker {

    /**
      | Will add a failure to the test if the number
      | of new/delete calls during this object's
      | lifetime was greater than zero.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
impl UnitTestAllocationChecker {

    /**
      | Create a checker which will log a failure
      | to the passed test if any calls to new/delete
      | are made.
      | 
      | Remember to call `UnitTest::beginTest`
      | before constructing this checker!
      |
      */
    pub fn new(test: &mut UnitTest) -> Self {
    
        todo!();
        /*
        
        */
    }
    
    pub fn new_or_delete_called(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn get_allocation_hooks_for_thread() -> &mut AllocationHooks {
    
    todo!();
    /*
        thread_local AllocationHooks hooks;
        return hooks;
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn notify_allocation_hooks_for_thread()  {
    
    todo!();
    /*
        getAllocationHooksForThread().listenerList.call ([] (AllocationHooks::Listener& l)
        {
            l.newOrDeleteCalled();
        });
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn operator_new(s: usize)  {
    
    todo!();
    /*
        notifyAllocationHooksForThread();
        return std::malloc (s);
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn operator_new_arr(s: usize)  {
    
    todo!();
    /*
        notifyAllocationHooksForThread();
        return std::malloc (s);
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn operator_delete(p: *mut c_void)  {
    
    todo!();
    /*
        notifyAllocationHooksForThread();
        std::free (p);
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn operator_delete_arr(p: *mut c_void)  {
    
    todo!();
    /*
        notifyAllocationHooksForThread();
        std::free (p);
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn operator_delete(
        p:  *mut c_void,
        _1: usize)  {
    
    todo!();
    /*
        notifyAllocationHooksForThread();
        std::free (p);
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
pub fn operator_delete_arr(
        p:  *mut c_void,
        _1: usize)  {
    
    todo!();
    /*
        notifyAllocationHooksForThread();
        std::free (p);
    */
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
impl Drop for UnitTestAllocationChecker {
    fn drop(&mut self) {
        todo!();
        /* 
        getAllocationHooksForThread().removeListener (this);
        unitTest.expectEquals ((int) calls, 0, "new or delete was incorrectly called while allocation checker was active");
 */
    }
}

#[cfg(ALOE_ENABLE_ALLOCATION_HOOKS)]
impl UnitTestAllocationChecker {
    
    pub fn new(test: &mut UnitTest) -> Self {
    
        todo!();
        /*
        : unit_test(test),

            getAllocationHooksForThread().addListener (this);
        */
    }

    pub fn new_or_delete_called(&mut self)  {
        
        todo!();
        /*
            ++calls;
        */
    }
}
