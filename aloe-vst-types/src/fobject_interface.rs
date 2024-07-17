crate::ix!();

pub trait FobjectInterface {

    /**
      | a local alternative to getFClassID()
      |
      */
    fn isa(&self) -> FClassID;
        
    /**
      | evaluates if the passed ID is of the FObject
      | type
      |
      */
    fn isa_with(&self, s: FClassID) -> bool;

    /**
      | evaluates if the passed ID is of the FObject
      | type
      |
      */
    fn is_type_of(
        &self, 
        s:              FClassID,
        ask_base_class: Option<bool>

    ) -> bool;

    /**
      | Inform all dependents, that the object
      | has changed.
      |
      */
    fn changed(&mut self, msg: Option<IDependentChangeMessage>);

    /**
      | Similar to triggerUpdates, except only
      | delivered in idle (usefull in collecting
      | updates).
      */
    fn defer_update(&mut self, msg: Option<IDependentChangeMessage>);

    /**
      | empty virtual method that should be
      | overridden by derived classes
      |
      */
    fn update_done(&mut self, msg: i32);
        
    fn is_equal_instance(&mut self, d: *mut dyn FUnknown) -> bool;
}
