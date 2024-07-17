crate::ix!();

pub trait GetNodes {

    /**
      | Returns the array of nodes in the graph.
      |
      */
    fn get_nodes(&self) -> &Vec<AudioProcessorGraphNodePtr>;
}

pub trait GetNumNodes {

    /**
      | Returns the number of nodes in the graph.
      |
      */
    fn get_num_nodes(&self) -> i32;
}

pub trait GetNode {

    /**
      | Returns a pointer to one of the nodes
      | in the graph.
      | 
      | This will return nullptr if the index
      | is out of range. @see getNodeForId
      |
      */
    fn get_node(&self, index: i32) -> AudioProcessorGraphNodePtr;
}

pub trait GetNodeForId {

    /**
      | Searches the graph for a node with the
      | given ID number and returns it.
      | 
      | If no such node was found, this returns
      | nullptr. @see getNode
      |
      */
    fn get_node_for_id(&self, _0: AudioProcessorGraphNodeID) 
        -> Rc<RefCell<dyn AudioProcessorGraphNodeInterface>>;
}
