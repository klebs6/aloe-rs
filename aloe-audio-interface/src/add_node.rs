crate::ix!();

pub trait AddNode {

    /**
      | Adds a node to the graph.
      | 
      | This creates a new node in the graph,
      | for the specified processor. Once you
      | have added a processor to the graph,
      | the graph owns it and will delete it later
      | when it is no longer needed.
      | 
      | The optional nodeId parameter lets
      | you specify a unique ID to use for the
      | node.
      | 
      | If the value is already in use, this method
      | will fail and return an empty node.
      | 
      | If this succeeds, it returns a pointer
      | to the newly-created node.
      |
      */
    fn add_node(
        &mut self, 
        new_processor: Box<dyn AudioProcessorInterface>,
        node_id:       AudioProcessorGraphNodeID

    ) -> AudioProcessorGraphNodePtr;
}

pub trait RemoveNodeById {

    /**
      | Deletes a node within the graph which
      | has the specified ID.
      | 
      | This will also delete any connections
      | that are attached to this node.
      |
      */
    fn remove_node_by_id(&mut self, _0: AudioProcessorGraphNodeID) -> AudioProcessorGraphNodePtr;
}

pub trait RemoveNode {

    /**
      | Deletes a node within the graph.
      | 
      | This will also delete any connections
      | that are attached to this node.
      |
      */
    fn remove_node(&mut self, _0: *mut dyn AudioProcessorGraphNodeInterface) -> AudioProcessorGraphNodePtr;
}
