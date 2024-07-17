crate::ix!();

pub trait ValueSourceInterface {

    /**
      | Returns the current value of this object.
      |
      */
    fn get_value(&self) -> Var;

    /**
      | Changes the current value. This must
      | also trigger a change message if the
      | value actually changes.
      |
      */

    fn set_value(&mut self, new_value: &Var);
}

/**
  | Used internally by the Value class as
  | the base class for its shared value objects.
  | 
  | The Value class is essentially a reference-counted
  | pointer to a shared instance of a ValueSource
  | object. If you're feeling adventurous,
  | you can create your own custom ValueSource
  | classes to allow Value objects to represent
  | your own custom data items.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ValueSource<'a> {
    base:                  ReferenceCountedObject,
    base2:                 AsyncUpdater<'a>,
    values_with_listeners: SortedSet<*mut Value<'a>>,
}

impl<'a> Drop for ValueSource<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
        cancelPendingUpdate();
        */
    }
}

impl<'a> ValueSource<'a> {

    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            sendChangeMessage (true);
        */
    }
    
    /**
      | Delivers a change message to all the
      | listeners that are registered with
      | this value.
      | 
      | If dispatchSynchronously is true,
      | the method will call all the listeners
      | before returning; otherwise it'll
      | dispatch a message and make the call
      | later.
      |
      */
    pub fn send_change_message(&mut self, synchronous: bool)  {
        
        todo!();
        /*
            const int numListeners = valuesWithListeners.size();

        if (numListeners > 0)
        {
            if (synchronous)
            {
                const ReferenceCountedObjectPtr<ValueSource> localRef (this);

                cancelPendingUpdate();

                for (int i = numListeners; --i >= 0;)
                    if (Value* const v = valuesWithListeners[i])
                        v->callListeners();
            }
            else
            {
                triggerAsyncUpdate();
            }
        }
        */
    }
}
