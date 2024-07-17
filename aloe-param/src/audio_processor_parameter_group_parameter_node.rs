crate::ix!();

/**
  | A child of an AudioProcessorParameterGroup.
  | 
  | This can contain either an AudioProcessorParameter
  | or an
  | 
  | AudioProcessorParameterGroup. You
  | can query which using the getParameter
  | and getGroup methods.
  | 
  | -----------
  | @code
  | 
  | for (auto* child : group)
  |     if (auto* parameter = node.getParameter())
  |         parameter->setValueNotifyingHost (0.5f);
  |     else
  |         node.getGroup()->AddChild (new Parameter());
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorParameterGroupAudioProcessorParameterNode {
    group:     Box<AudioProcessorParameterGroup>,
    parameter: Box<AudioProcessorParameter>,
    parent:    *mut AudioProcessorParameterGroup, // default = nullptr
}

impl AudioProcessorParameterGroupAudioProcessorParameterNode {

    pub fn new_from_node(other: AudioProcessorParameterGroupAudioProcessorParameterNode) -> Self {
    
        todo!();
        /*
        : group(std::move (other.group)),
        : parameter(std::move (other.parameter)),

            if (group != nullptr)
            group->parent = parent;
        */
    }
    
    pub fn new_from_param_and_group(
        param:        Box<AudioProcessorParameter>,
        parent_group: *mut AudioProcessorParameterGroup) -> Self {
    
        todo!();
        /*
        : parameter(std::move (param)),
        : parent(parentGroup),

        
        */
    }
    
    pub fn new_from_group_and_parent_group(
        grp:          Box<AudioProcessorParameterGroup>,
        parent_group: *mut AudioProcessorParameterGroup) -> Self {
    
        todo!();
        /*
        : group(std::move (grp)),
        : parent(parentGroup),

            group->parent = parent;
        */
    }
    
    /**
      | Returns the parent group or nullptr
      | if this is a top-level group.
      |
      */
    pub fn get_parent(&self) -> *mut AudioProcessorParameterGroup {
        
        todo!();
        /*
            return parent;
        */
    }
    
    /**
      | Returns a pointer to a parameter if this
      | node contains a parameter, nullptr
      | otherwise.
      |
      */
    pub fn get_parameter(&self) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            return parameter.get();
        */
    }
    
    /**
      | Returns a pointer to a group if this node
      | contains a group, nullptr otherwise.
      |
      */
    pub fn get_group(&self) -> *mut AudioProcessorParameterGroup {
        
        todo!();
        /*
            return group.get();
        */
    }
}
