crate::ix!();

/**
  | A class to keep an eye on a component and
  | check for it being deleted.
  | 
  | This is designed for use with the ListenerList::callChecked()
  | methods, to allow the list iterator
  | to stop cleanly if the component is deleted
  | by a listener callback while the list
  | is still being iterated.
  |
  */
#[no_copy]
pub struct ComponentBailOutChecker<'a> {
    safe_pointer: WeakReference<Component<'a>>,
}

impl<'a> ComponentBailOutChecker<'a> {
    
    /**
      | Creates a checker that watches one component.
      |
      */
    pub fn new(component: *mut Component<'a>) -> Self {
    
        todo!();
        /*
        : safe_pointer(component),

            jassert (component != nullptr);
        */
    }
    
    /**
      | Returns true if either of the two components
      | have been deleted since this object
      | was created.
      |
      */
    pub fn should_bail_out(&self) -> bool {
        
        todo!();
        /*
            return safePointer == nullptr;
        */
    }
}

