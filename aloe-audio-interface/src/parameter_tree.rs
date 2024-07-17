crate::ix!();

pub trait GetParameterTree {

    /**
      | Returns the group of parameters managed
      | by this AudioProcessor.
      |
      */
    fn get_parameter_tree(&self) -> &dyn AudioProcessorParameterGroupInterface;
}

pub trait SetParameterTree {

    /**
      | Sets the group of parameters managed
      | by this AudioProcessor.
      | 
      | Replacing the tree after your AudioProcessor
      | has been constructed will crash many
      | hosts, so don't do it! You may, however,
      | change parameter and group names by
      | iterating the tree returned by getParameterTree().
      | 
      | Afterwards, call updateHostDisplay()
      | to inform the host of the changes.
      | 
      | Not all hosts support dynamic changes
      | to parameters and group names.
      |
      */
    fn set_parameter_tree(&mut self, new_tree: &dyn AudioProcessorParameterGroupInterface);
}
