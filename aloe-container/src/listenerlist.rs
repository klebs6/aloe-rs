crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ListenerList.h]

/**
  | Holds a set of objects and can invoke a member
  | function callback on each object in the set
  | with a single call.
  |
  | Use a ListenerList to manage a set of objects
  | which need a callback, and you can invoke
  | a member function by simply calling call() or
  | callChecked().
  |
  | E.g.
  | @code
  | class MyListenerType
  | {
  |
  |     void myCallbackMethod (int foo, bool bar);
  | };
  |
  | ListenerList<MyListenerType> listeners;
  | listeners.add (someCallbackObjects...);
  |
  | // This will invoke myCallbackMethod 
  | // (1234, true) on each of the objects 
  | // in the list...
  | listeners.call ([] (MyListenerType& l) { l.myCallbackMethod (1234, true); });
  | @endcode
  |
  | If you add or remove listeners from the list
  | during one of the callbacks - i.e. while it's
  | in the middle of iterating the listeners, then
  | it's guaranteed that no listeners will be
  | mistakenly called after they've been removed,
  | but it may mean that some of the listeners
  | could be called more than once, or not at all,
  | depending on the list's order.
  |
  | Sometimes, there's a chance that invoking one
  | of the callbacks might result in the list
  | itself being deleted while it's still
  | iterating - to survive this situation, you can
  | use callChecked() instead of call(), passing
  | it a local object to act as
  | a "BailOutChecker".  The BailOutChecker must
  | implement a method of the form "bool
  | shouldBailOut()", and the list will check this
  | after each callback to determine whether it
  | should abort the operation. For an example of
  | a bail-out checker, see the
  | Component::BailOutChecker class, which can be
  | used to check when a Component has been
  | deleted. See also
  | ListenerList::DummyBailOutChecker, which is
  | a dummy checker that always returns false.
  |
  | @tags{Core}
  */
#[derive(Default)] /** Creates an empty list. */
#[no_copy]
pub struct ListenerList<ListenerClass,ArrayType = Vec<*mut ListenerClass>> {
    listeners: ArrayType,
    phantom:   std::marker::PhantomData<ListenerClass>,
}

pub mod listener_list {

    use super::*;
    
    /**
      | A dummy bail-out checker that always
      | returns false.
      | 
      | See the ListenerList notes for more
      | info about bail-out checkers.
      |
      */
    pub struct DummyBailOutChecker { }

    impl DummyBailOutChecker {
        
        pub fn should_bail_out(&self) -> bool {
            
            todo!();
            /*
                return false;
            */
        }
    }

    pub type ThisType<ListenerClass,ArrayType> = ListenerList<ListenerClass,ArrayType>;
    pub type ListenerType<ListenerClass>       = ListenerClass;

    /**
      | Iterates the listeners in a ListenerList.
      |
      */
    #[no_copy]
    pub struct Iterator<'a, BailOutCheckerType,ListType: HasListenerType> {
        list:    &'a ListType,
        index:   i32,
        phantom: std::marker::PhantomData<BailOutCheckerType>,
    }

    impl<'a,BailOutCheckerType,ListType: HasListenerType> Iterator<'a, BailOutCheckerType,ListType> {

        pub fn new(list_to_iterate: &ListType) -> Self {
        
            todo!();
            /*
            : list(listToIterate),
            : index(listToIterate.size()),

            
            */
        }
        
        pub fn next(&mut self) -> bool {
            
            todo!();
            /*
                if (index <= 0)
                        return false;

                    auto listSize = list.size();

                    if (--index < listSize)
                        return true;

                    index = listSize - 1;
                    return index >= 0;
            */
        }
        
        pub fn next_with_bailout_checker(&mut self, bail_out_checker: &BailOutCheckerType) -> bool {
            
            todo!();
            /*
                return (! bailOutChecker.shouldBailOut()) && next();
            */
        }
        
        pub fn get_listener(&self) -> *mut <ListType as HasListenerType>::ListenerType {
            
            todo!();
            /*
                return list.getListeners().getUnchecked (index);
            */
        }
    }
}

pub trait HasListenerType {
    type ListenerType;
}

impl<ListenerClass,ArrayType> ListenerList<ListenerClass,ArrayType> {

    /**
      | Adds a listener to the list.
      | 
      | A listener can only be added once, so
      | if the listener is already in the list,
      | this method has no effect. @see remove
      |
      */
    pub fn add(&mut self, listener_to_add: *mut ListenerClass)  {
        
        todo!();
        /*
            if (listenerToAdd != nullptr)
                listeners.addIfNotAlreadyThere (listenerToAdd);
            else
                jassertfalse;  // Listeners can't be null pointers!
        */
    }

    /**
      | Removes a listener from the list.
      | 
      | If the listener wasn't in the list, this
      | has no effect.
      |
      */
    pub fn remove(&mut self, listener_to_remove: *mut ListenerClass)  {
        
        todo!();
        /*
            jassert (listenerToRemove != nullptr); // Listeners can't be null pointers!
            listeners.removeFirstMatchingValue (listenerToRemove);
        */
    }

    /**
      | Returns the number of registered listeners.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return listeners.size();
        */
    }

    /**
      | Returns true if no listeners are registered,
      | false otherwise.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return listeners.isEmpty();
        */
    }

    /**
      | Clears the list.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            listeners.clear();
        */
    }

    /**
      | Returns true if the specified listener
      | has been added to the list.
      |
      */
    pub fn contains(&self, listener: *mut ListenerClass) -> bool {
        
        todo!();
        /*
            return listeners.contains (listener);
        */
    }

    /**
      | Returns the raw array of listeners.
      |
      */
    pub fn get_listeners(&self) -> &ArrayType {
        
        todo!();
        /*
            return listeners;
        */
    }

    /**
      | Calls a member function on each listener
      | in the list, with multiple parameters.
      |
      */
    pub fn call_callback<Callback>(&mut self, callback: Callback)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<DummyBailOutChecker, ThisType> iter (*this); iter.next();)
                callback (*iter.getListener());
        */
    }

    /**
      | Calls a member function with 1 parameter,
      | on all but the specified listener in
      | the list.
      | 
      | This can be useful if the caller is also
      | a listener and needs to exclude itself.
      |
      */
    pub fn call_excluding_with_listener_and_callback<Callback>(
        &mut self, 
        listener_to_exclude: *mut ListenerClass,
        callback:            Callback)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<DummyBailOutChecker, ThisType> iter (*this); iter.next();)
            {
                auto* l = iter.getListener();

                if (l != listenerToExclude)
                    callback (*l);
            }
        */
    }

    /**
      | Calls a member function on each listener
      | in the list, with 1 parameter and a bail-out-checker.
      | 
      | See the class description for info about
      | writing a bail-out checker.
      |
      */
    pub fn call_checked_with_bailout_checker_and_callback<Callback, BailOutCheckerType>(
        &mut self, 
        bail_out_checker: &BailOutCheckerType,
        callback:         Callback)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<BailOutCheckerType, ThisType> iter (*this); iter.next (bailOutChecker);)
                callback (*iter.getListener());
        */
    }

    /**
      | Calls a member function, with 1 parameter,
      | on all but the specified listener in
      | the list with a bail-out-checker. This
      | can be useful if the caller is also a listener
      | and needs to exclude itself. See the
      | class description for info about writing
      | a bail-out checker.
      |
      */
    pub fn call_checked_excluding<Callback, BailOutCheckerType>(&mut self, 
        listener_to_exclude: *mut ListenerClass,
        bail_out_checker:    &BailOutCheckerType,
        callback:            Callback)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<BailOutCheckerType, ThisType> iter (*this); iter.next (bailOutChecker);)
            {
                auto* l = iter.getListener();

                if (l != listenerToExclude)
                    callback (*l);
            }
        */
    }
    
    /**
      | There are now lambda-based call functions
      | that can be used to replace these old
      | method-based versions.
      |
      | We'll eventually deprecate these old ones,
      | so please begin moving your code to use
      | lambdas!
      */
    pub fn call_with_callback_function_ptr(&mut self, callback_function: fn() -> c_void)
    {
        todo!();
        /*
            call ([=] (ListenerClass& l) { (l.*callbackFunction)(); });
        */
    }
    
    pub fn call_excluding_with_listener_and_callback_function_ptr(
        &mut self, 
        listener_to_exclude: *mut ListenerClass,
        callback_function:   fn() -> c_void)
    {
        todo!();
        /*
            callExcluding (listenerToExclude, [=] (ListenerClass& l) { (l.*callbackFunction)(); });
        */
    }
    
    pub fn call_checked_with_bailout_checker_and_callback_function_ptr<BailOutCheckerType>(
        &mut self, 
        bail_out_checker:  &BailOutCheckerType,
        callback_function: fn() -> c_void)  {
    
        todo!();
        /*
            callChecked (bailOutChecker, [=] (ListenerClass& l) { (l.*callbackFunction)(); });
        */
    }
    
    pub fn call_checked_excluding_with_function_ptr<BailOutCheckerType>(&mut self, 
        listener_to_exclude: *mut ListenerClass,
        bail_out_checker:    &BailOutCheckerType,
        callback_function:   fn() -> c_void)  {
    
        todo!();
        /*
            callCheckedExcluding (listenerToExclude, bailOutChecker, [=] (ListenerClass& l) { (l.*callbackFunction)(); });
        */
    }
    
    pub fn call_with_callback_function_and_args<MethodArgs, Args>(
        &mut self, 
        callback_function: fn(_0: MethodArgs) -> c_void,
        args:              Args)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<DummyBailOutChecker, ThisType> iter (*this); iter.next();)
                (iter.getListener()->*callbackFunction) (static_cast<typename TypeHelpers::ParameterType<Args>::type> (args)...);
        */
    }
    
    pub fn call_excluding_with_listener_and_callback_function_ptr_with_args<MethodArgs, Args>(
        &mut self, 
        listener_to_exclude: *mut ListenerClass,
        callback_function:   fn(_0: MethodArgs) -> c_void,
        args:                Args)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<DummyBailOutChecker, ThisType> iter (*this); iter.next();)
                if (iter.getListener() != listenerToExclude)
                    (iter.getListener()->*callbackFunction) (static_cast<typename TypeHelpers::ParameterType<Args>::type> (args)...);
        */
    }
    
    pub fn call_checked_with_callback_and_args<BailOutCheckerType, MethodArgs, Args>(
        &mut self, 
        bail_out_checker:  &BailOutCheckerType,
        callback_function: fn(_0: MethodArgs) -> c_void,
        args:              Args)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<BailOutCheckerType, ThisType> iter (*this); iter.next (bailOutChecker);)
                (iter.getListener()->*callbackFunction) (static_cast<typename TypeHelpers::ParameterType<Args>::type> (args)...);
        */
    }
    
    pub fn call_checked_excluding_with_callback_and_args<BailOutCheckerType, MethodArgs, Args>(&mut self, 
        listener_to_exclude: *mut ListenerClass,
        bail_out_checker:    &BailOutCheckerType,
        callback_function:   fn(_0: MethodArgs) -> c_void,
        args:                Args)  {
    
        todo!();
        /*
            typename ArrayType::ScopedLockType lock (listeners.getLock());

            for (Iterator<BailOutCheckerType, ThisType> iter (*this); iter.next (bailOutChecker);)
                if (iter.getListener() != listenerToExclude)
                    (iter.getListener()->*callbackFunction) (static_cast<typename TypeHelpers::ParameterType<Args>::type> (args)...);
        */
    }
}
