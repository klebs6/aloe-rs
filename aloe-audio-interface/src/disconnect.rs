crate::ix!();

pub trait RemoveConnection {

    /**
      | Deletes the given connection.
      |
      */
    fn remove_connection(&mut self, _0: &AudioProcessorGraphConnection) -> bool;
}

pub trait DisconnectNode {

    /**
      | Removes all connections from the specified
      | node.
      |
      */
    fn disconnect_node(&mut self, _0: AudioProcessorGraphNodeID) -> bool;
}

pub trait RemoveIllegalConnections {

    /**
      | Performs a sanity checks of all the connections.
      | 
      | This might be useful if some of the processors
      | are doing things like changing their
      | channel counts, which could render
      | some connections obsolete.
      |
      */
    fn remove_illegal_connections(&mut self) -> bool;
}
