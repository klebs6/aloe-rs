crate::ix!();

pub trait GetConnections {

    /**
      | Returns the list of connections in the
      | graph.
      |
      */
    fn get_connections(&self) -> Vec<AudioProcessorGraphConnection>;
}

pub trait GetNodeConnections {

    fn get_node_connections(
        &mut self, 
        node:        &mut dyn AudioProcessorGraphNodeInterface,
        connections: &mut Vec<AudioProcessorGraphConnection>
    );
}
 
pub trait IsConnected {

    /**
      | Returns true if the given connection
      | exists.
      |
      */
    fn is_connected(&self, _0: &AudioProcessorGraphConnection) -> bool;
}

pub trait IsConnectedByID {

    /**
      | Returns true if there is a direct connection
      | between any of the channels of two specified
      | nodes.
      |
      */
    fn is_connected_by_id(
        &self, 
        possible_source_nodeid: AudioProcessorGraphNodeID,
        possible_dest_nodeid:   AudioProcessorGraphNodeID
    ) -> bool;
}

pub trait CanConnect {

    /**
      | Returns true if it would be legal to connect
      | the specified points.
      |
      */
    fn can_connect(&self, _0: &AudioProcessorGraphConnection) -> bool;
}

pub trait IsConnectionLegal {

    /**
      | Returns true if the given connection's
      | channel numbers map on to valid channels
      | at each end.
      | 
      | Even if a connection is valid when created,
      | its status could change if a node changes
      | its channel config.
      |
      */
    fn is_connection_legal(&self, _0: &AudioProcessorGraphConnection) -> bool;
}

pub trait IsConnectedSrcDst {

    fn is_connected(
        &self, 
        source:         *mut dyn AudioProcessorGraphNodeInterface,
        source_channel: i32,
        dest:           *mut dyn AudioProcessorGraphNodeInterface,
        dest_channel:   i32
    ) -> bool;
}

pub trait CanConnectSrcDst {

    fn can_connect(
        &self, 
        source:         *mut dyn AudioProcessorGraphNodeInterface,
        source_channel: i32,
        dest:           *mut dyn AudioProcessorGraphNodeInterface,
        dest_channel:   i32
    ) -> bool;
}
        
pub trait IsLegal {

    fn is_legal(
        &self, 
        source:         *mut dyn AudioProcessorGraphNodeInterface,
        source_channel: i32,
        dest:           *mut dyn AudioProcessorGraphNodeInterface,
        dest_channel:   i32
    ) -> bool;
}

pub trait IsAnInputTo {

    /**
      | Does a recursive check to see if there's
      | a direct or indirect series of connections
      | between these two nodes.
      |
      */
    fn is_an_input_to(
        &self, 
        source:      &mut dyn AudioProcessorGraphNodeInterface,
        destination: &mut dyn AudioProcessorGraphNodeInterface

    ) -> bool;

    fn is_an_input_to_with_recursion_check(
        &self, 
        src:             &mut dyn AudioProcessorGraphNodeInterface,
        dst:             &mut dyn AudioProcessorGraphNodeInterface,
        recursion_check: i32
    ) -> bool;
}
