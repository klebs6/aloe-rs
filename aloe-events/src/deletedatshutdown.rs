crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_DeletedAtShutdown.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_DeletedAtShutdown.cpp]

/**
  | Classes derived from this will be automatically
  | deleted when the application exits.
  | 
  | After ALOEApplicationBase::shutdown()
  | has been called, any objects derived
  | from DeletedAtShutdown which are still
  | in existence will be deleted in the reverse
  | order to that in which they were created.
  | 
  | So if you've got a singleton and don't
  | want to have to explicitly delete it,
  | just inherit from this and it'll be taken
  | care of.
  | 
  | @tags{Events}
  |
  */
#[no_copy]
pub struct DeletedAtShutdown {


}

pub fn get_deleted_at_shutdown_objects<'a>() -> &'a mut Vec<*mut DeletedAtShutdown> {
    
    todo!();
    /*
        static Vec<DeletedAtShutdown*> objects;
        return objects;
    */
}

lazy_static!{
    /*
    static SpinLock deletedAtShutdownLock; // use a spin lock because it can be statically initialised
    */
}

impl Drop for DeletedAtShutdown {

    /**
      | Destructor.
      | 
      | It's ok to delete these objects explicitly
      | - it's only the ones left dangling at
      | the end that will be deleted automatically.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        const SpinLock::ScopedLockType sl (deletedAtShutdownLock);
        getDeletedAtShutdownObjects().removeFirstMatchingValue (this);
 */
    }
}

impl Default for DeletedAtShutdown {
    
    fn default() -> Self {
    
        todo!();
        /*


            const SpinLock::ScopedLockType sl (deletedAtShutdownLock);
        getDeletedAtShutdownObjects().add (this);
        */
    }
}

impl DeletedAtShutdown {

    /**
      | Deletes all extant objects.
      | 
      | This shouldn't be used by applications,
      | as it's called automatically in the
      | shutdown code of the ALOEApplicationBase
      | class.
      |
      */
    pub fn delete_all(&mut self)  {
        
        todo!();
        /*
            // make a local copy of the array, so it can't get into a loop if something
        // creates another DeletedAtShutdown object during its destructor.
        Vec<DeletedAtShutdown*> localCopy;

        {
            const SpinLock::ScopedLockType sl (deletedAtShutdownLock);
            localCopy = getDeletedAtShutdownObjects();
        }

        for (int i = localCopy.size(); --i >= 0;)
        {
            ALOE_TRY
            {
                auto* deletee = localCopy.getUnchecked(i);

                // double-check that it's not already been deleted during another object's destructor.
                {
                    const SpinLock::ScopedLockType sl (deletedAtShutdownLock);

                    if (! getDeletedAtShutdownObjects().contains (deletee))
                        deletee = nullptr;
                }

                delete deletee;
            }
            ALOE_CATCH_EXCEPTION
        }

        // if this fails, then it's likely that some new DeletedAtShutdown objects were
        // created while executing the destructors of the other ones.
        jassert (getDeletedAtShutdownObjects().isEmpty());

        getDeletedAtShutdownObjects().clear(); // just to make sure the array doesn't have any memory still allocated
        */
    }
}
