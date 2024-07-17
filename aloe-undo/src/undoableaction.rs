crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/undomanager/aloe_UndoableAction.h]

/**
  | Used by the UndoManager class to store
  | an action which can be done and undone.
  | 
  | @see UndoManager
  | 
  | @tags{DataStructures}
  |
  */
pub trait UndoableAction /* : Default */ {
    
    /**
      | Overridden by a subclass to perform
      | the action.
      | 
      | This method is called by the UndoManager,
      | and shouldn't be used directly by applications.
      | 
      | Be careful not to make any calls in a perform()
      | method that could call recursively
      | back into the UndoManager::perform()
      | method
      | 
      | -----------
      | @return
      | 
      | true if the action could be performed.
      | @see UndoManager::perform
      |
      */
    fn perform(&mut self) -> bool;

    /**
      | Overridden by a subclass to undo the
      | action.
      | 
      | This method is called by the UndoManager,
      | and shouldn't be used directly by applications.
      | 
      | Be careful not to make any calls in an
      | undo() method that could call recursively
      | back into the UndoManager::perform()
      | method
      | 
      | -----------
      | @return
      | 
      | true if the action could be undone without
      | any errors. @see UndoManager::perform
      |
      */
    fn undo(&mut self) -> bool;
    
    /**
      | Returns a value to indicate how much
      | memory this object takes up.
      | 
      | Because the UndoManager keeps a list
      | of UndoableActions, this is used to
      | work out how much space each one will
      | take up, so that the UndoManager can
      | work out how many to keep.
      | 
      | The default value returned here is 10
      | - units are arbitrary and don't have
      | to be accurate.
      | 
      | @see UndoManager::getNumberOfUnitsTakenUpByStoredCommands,
      | UndoManager::setMaxNumberOfStoredUnits
      |
      */
    fn get_size_in_units(&mut self) -> i32 {
        10
    }

    /**
      | Allows multiple actions to be coalesced
      | into a single action object, to reduce
      | storage space.
      | 
      | If possible, this method should create
      | and return a single action that does
      | the same job as this one followed by the
      | supplied action.
      | 
      | If it's not possible to merge the two
      | actions, the method should return a
      | nullptr.
      |
      */
    fn create_coalesced_action(&mut self, next_action: *mut dyn UndoableAction) -> *mut dyn UndoableAction {
        
        todo!();
        /*
            ignoreUnused (nextAction); return nullptr;
        */
    }
}
