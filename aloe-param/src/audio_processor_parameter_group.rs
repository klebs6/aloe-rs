crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorParameterGroup.h]

/**
  | A class encapsulating a group of AudioProcessorParameters
  | and nested
  | 
  | AudioProcessorParameterGroups.
  | 
  | This class is predominantly write-only;
  | there are methods for adding group members
  | but none for removing them. Ultimately
  | you will probably want to add a fully
  | constructed group to an AudioProcessor.
  | 
  | @see AudioProcessor::addParameterGroup
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorParameterGroup {
    identifier: String,
    name:       String,
    separator:  String,
    children:   Vec<Box<AudioProcessorParameterGroupAudioProcessorParameterNode>>,
    parent:     *mut AudioProcessorParameterGroup, // default = nullptr
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorParameterGroup.cpp]
impl Default for AudioProcessorParameterGroup {
    
    /**
      | Creates an empty AudioProcessorParameterGroup
      | with no name or ID.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl AudioProcessorParameterGroup {

    /**
      | Creates an AudioProcessorParameterGroup
      | with a single child.
      | 
      | -----------
      | @param groupID
      | 
      | A unique identifier for the group. Keep
      | it basic; don't use any special characters
      | like "." and avoid pure integer strings
      | which could collide with legacy parameter
      | IDs.
      | ----------
      | @param groupName
      | 
      | The group's name, which will be displayed
      | in the host.
      | ----------
      | @param subgroupSeparator
      | 
      | A separator string to use between the
      | name of this group and the name of any
      | subgroups if this group is flattened.
      | AUv3 and Vst3 plug-ins can have multiple
      | layers of nested subgroups, but AU plug-ins
      | cannot have any subgroups.
      | ----------
      | @param child
      | 
      | An AudioProcessorParameter or an AudioProcessorParameterGroup
      | to add to the group.
      |
      */
    pub fn new_with_groupinfo_and_child<ParameterOrGroup>(
        groupid:            String,
        group_name:         String,
        subgroup_separator: String,
        child:              Box<ParameterOrGroup>) -> Self {
    
        todo!();
        /*
        : audio_processor_parameter_group(groupID, groupName, subgroupSeparator),

            addChild (std::move (child));
        */
    }

    /**
      | Creates an AudioProcessorParameterGroup
      | with multiple children.
      | 
      | -----------
      | @param groupID
      | 
      | A unique identifier for the group. Keep
      | it basic; don't use any special characters
      | like "." and avoid pure integer strings
      | which could collide with legacy parameter
      | IDs.
      | ----------
      | @param groupName
      | 
      | The group's name, which will be displayed
      | in the host.
      | ----------
      | @param subgroupSeparator
      | 
      | A separator string to use between the
      | name of this group and the name of any
      | subgroups if this group is flattened.
      | AUv3 and Vst3 plug-ins can have multiple
      | layers of nested subgroups, but AU plug-ins
      | cannot have any subgroups.
      | ----------
      | @param firstChild
      | 
      | An AudioProcessorParameter or an AudioProcessorParameterGroup
      | to add to the group.
      | ----------
      | @param remainingChildren
      | 
      | A list of more AudioProcessorParameters
      | or AudioProcessorParameterGroups
      | to add to the group.
      |
      */
    pub fn new_with_group_info_and_children<ParameterOrGroup, Args>(
        groupid:            String,
        group_name:         String,
        subgroup_separator: String,
        first_child:        Box<ParameterOrGroup>,
        remaining_children: Args) -> Self {
    
        todo!();
        /*


            : AudioProcessorParameterGroup (groupID, groupName, subgroupSeparator, std::move (firstChild))

            addChild (std::forward<Args> (remainingChildren)...);
        */
    }

    /**
      | Adds a child to the group.
      | 
      | Do not add children to a group which has
      | itself already been added to the
      | 
      | AudioProcessor - the new elements will
      | be ignored.
      |
      */
    pub fn add_child<ParameterOrGroup>(&mut self, child: Box<ParameterOrGroup>)  {
    
        todo!();
        /*
            // If you hit a compiler error here then you are attempting to add a
            // child that is neither a pointer to an AudioProcessorParameterGroup
            // nor a pointer to an AudioProcessorParameter.
            append (std::move (child));
        */
    }

    /**
      | Adds multiple parameters or sub-groups
      | to this group.
      | 
      | Do not add children to a group which has
      | itself already been added to the
      | 
      | AudioProcessor - the new elements will
      | be ignored.
      |
      */
    
    pub fn add_first_child<ParameterOrGroup, Args>(
        &mut self, 
        first_child:        Box<ParameterOrGroup>,
        remaining_children: Args

    ) {
    
        todo!();
        /*
            addChild (std::move (firstChild));
            addChild (std::forward<Args> (remainingChildren)...);
        */
    }
    
    /**
      | Creates an empty AudioProcessorParameterGroup.
      | 
      | -----------
      | @param groupID
      | 
      | A unique identifier for the group. Keep
      | it basic; don't use any special characters
      | like "." and avoid pure integer strings
      | which could collide with legacy parameter
      | IDs.
      | ----------
      | @param groupName
      | 
      | The group's name, which will be displayed
      | in the host.
      | ----------
      | @param subgroupSeparator
      | 
      | A separator string to use between the
      | name of this group and the name of any
      | subgroups if this group is flattened.
      | AUv3 and Vst3 plug-ins can have multiple
      | layers of nested subgroups, but AU plug-ins
      | cannot have any subgroups.
      |
      */
    pub fn new_with_groupinfo_and_subgroup_separator(
        groupid:            String,
        group_name:         String,
        subgroup_separator: String) -> Self {
    
        todo!();
        /*
        : identifier(std::move (groupID)),
        : name(std::move (groupName)),
        : separator(std::move (subgroupSeparator)),

        
        */
    }
    
    /**
      | Once a group has been added to an AudioProcessor
      | don't try to mutate it by moving or swapping
      | it - this will crash most hosts.
      |
      */
    pub fn new_from_parameter_group(other: AudioProcessorParameterGroup) -> Self {
    
        todo!();
        /*
        : identifier(std::move (other.identifier)),
        : name(std::move (other.name)),
        : separator(std::move (other.separator)),
        : children(std::move (other.children)),

            updateChildParentage();
        */
    }
    
    /**
      | Once a group has been added to an AudioProcessor
      | don't try to mutate it by moving or swapping
      | it - this will crash most hosts.
      |
      */
    pub fn assign_from(&mut self, other: AudioProcessorParameterGroup) -> &mut AudioProcessorParameterGroup {
        
        todo!();
        /*
            identifier = std::move (other.identifier);
        name = std::move (other.name);
        separator = std::move (other.separator);
        children = std::move (other.children);
        updateChildParentage();
        return *this;
        */
    }
    
    pub fn update_child_parentage(&mut self)  {
        
        todo!();
        /*
            for (auto* child : children)
        {
            child->parent = this;

            if (auto* group = child->getGroup())
                group->parent = this;
        }
        */
    }
    
    /**
      | Returns the group's ID.
      |
      */
    pub fn getid(&self) -> String {
        
        todo!();
        /*
            return identifier;
        */
    }
    
    /**
      | Returns the group's name.
      |
      */
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return name;
        */
    }
    
    /**
      | Returns the group's separator string.
      |
      */
    pub fn get_separator(&self) -> String {
        
        todo!();
        /*
            return separator;
        */
    }
    
    /**
      | Returns the parent of the group, or nullptr
      | if this is a top-level group.
      |
      */
    pub fn get_parent(&self) -> *const AudioProcessorParameterGroup {
        
        todo!();
        /*
            return parent;
        */
    }
    
    /**
      | Changes the name of the group. If you
      | do this after the group has been added
      | to an AudioProcessor, call updateHostDisplay()
      | to inform the host of the change. Not
      | all hosts support dynamic group name
      | changes.
      |
      */
    pub fn set_name(&mut self, new_name: String)  {
        
        todo!();
        /*
            name = std::move (newName);
        */
    }
    
    pub fn begin(&self) 
        -> *const *const AudioProcessorParameterGroupAudioProcessorParameterNode 
    {
        todo!();
        /*
            return const_cast<const AudioProcessorParameterGroupAudioProcessorParameterNode**> (children.begin());
        */
    }
    
    pub fn end(&self) -> *const *const AudioProcessorParameterGroupAudioProcessorParameterNode {
        
        todo!();
        /*
            return const_cast<const AudioProcessorParameterGroupAudioProcessorParameterNode**> (children.end());
        */
    }
    
    pub fn append_new_parameter(&mut self, new_parameter: Box<AudioProcessorParameter>)  {
        
        todo!();
        /*
            children.add (new AudioProcessorParameterGroupAudioProcessorParameterNode (std::move (newParameter), this));
        */
    }
    
    pub fn append_new_subgroup(&mut self, new_sub_group: Box<AudioProcessorParameterGroup>)  {
        
        todo!();
        /*
            children.add (new AudioProcessorParameterGroupAudioProcessorParameterNode (std::move (newSubGroup), this));
        */
    }
    
    /**
      | Returns all subgroups of this group.
      | 
      | -----------
      | @param recursive
      | 
      | If this is true then this method will
      | fetch all nested subgroups using a depth
      | first search.
      |
      */
    pub fn get_subgroups(&self, recursive: bool) -> Vec<*const AudioProcessorParameterGroup> {
        
        todo!();
        /*
            Vec<const AudioProcessorParameterGroup*> groups;
        getSubgroups (groups, recursive);
        return groups;
        */
    }
    
    /**
      | Returns all the parameters in this group.
      | 
      | -----------
      | @param recursive
      | 
      | If this is true then this method will
      | fetch all nested parameters using a
      | depth first search.
      |
      */
    pub fn get_parameters(&self, recursive: bool) -> Vec<*mut AudioProcessorParameter> {
        
        todo!();
        /*
            Vec<AudioProcessorParameter*> parameters;
        getParameters (parameters, recursive);
        return parameters;
        */
    }
    
    /**
      | Searches this group recursively for
      | a parameter and returns a depth ordered
      | list of the groups it belongs to.
      |
      */
    pub fn get_groups_for_parameter(&self, parameter: *mut AudioProcessorParameter) -> Vec<*const AudioProcessorParameterGroup> {
        
        todo!();
        /*
            Vec<const AudioProcessorParameterGroup*> groups;

        if (auto* group = getGroupForParameter (parameter))
        {
            while (group != nullptr && group != this)
            {
                groups.insert (0, group);
                group = group->getParent();
            }
        }

        return groups;
        */
    }
    
    pub fn get_subgroups_with_previous(
        &self, 
        previous_groups: &mut Vec<*const AudioProcessorParameterGroup>,
        recursive:       bool

    )  {
        
        todo!();
        /*
            for (auto* child : children)
        {
            if (auto* group = child->getGroup())
            {
                previousGroups.add (group);

                if (recursive)
                    group->getSubgroups (previousGroups, true);
            }
        }
        */
    }
    
    pub fn get_parameters_with_previous(
        &self, 
        previous_parameters: &mut Vec<*mut AudioProcessorParameter>,
        recursive:           bool

    ) {
        
        todo!();
        /*
            for (auto* child : children)
        {
            if (auto* parameter = child->getParameter())
                previousParameters.add (parameter);
            else if (recursive)
                child->getGroup()->getParameters (previousParameters, true);
        }
        */
    }
    
    pub fn get_group_for_parameter(&self, parameter: *mut AudioProcessorParameter) -> *const AudioProcessorParameterGroup {
        
        todo!();
        /*
            for (auto* child : children)
        {
            if (child->getParameter() == parameter)
                return this;

            if (auto* group = child->getGroup())
                if (auto* foundGroup = group->getGroupForParameter (parameter))
                    return foundGroup;
        }

        return nullptr;
        */
    }
}
