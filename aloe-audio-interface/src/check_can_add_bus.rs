crate::ix!();

pub trait CheckCanAddBus {

    /**
      | Callback to query if a bus can currently
      | be added.
      | 
      | This callback probes if a bus can currently
      | be added. You should override this callback
      | if you want to support dynamically adding/removing
      | buses by the host. This is useful for
      | mixer audio processors.
      | 
      | The default implementation will always
      | return false.
      | 
      | @see addBus
      |
      */
    fn can_add_bus(&self, is_input: bool) -> bool {
        
        todo!();
        /*
            ignoreUnused (isInput); return false;
        */
    }
}

pub trait CheckCanRemoveBus {

    /**
      | Callback to query if the last bus can
      | currently be removed.
      | 
      | This callback probes if the last bus
      | can currently be removed. You should
      | override this callback if you want to
      | support dynamically adding/removing
      | buses by the host. This is useful for
      | mixer audio processors.
      | 
      | If you return true in this callback then
      | the AudioProcessor will go ahead and
      | delete the bus.
      | 
      | The default implementation will always
      | return false.
      |
      */
    fn can_remove_bus(&self, is_input: bool) -> bool {
        
        todo!();
        /*
            ignoreUnused (isInput); return false;
        */
    }
}
