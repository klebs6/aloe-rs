crate::ix!();

pub trait AudioProcessorParameterGroupInterface 
: AddChild
+ AddFirstChild
+ UpdateChildParentage
+ GetID

/*
  | Returns the group's name.
  |
  */
+ GetName
+ GetSeparator
+ GetParent
+ SetName
+ AppendNewParameter
+ AppendNewSubgroup
+ GetSubgroups
+ GetParametersInGroup
+ GetGroupsForParameter
+ GetSubgroupsWithPrevious
+ GetParametersWithPrevious
+ GetGroupForParameter
{ }

pub trait ParameterOrGroup {}

pub trait AddChild {

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
    fn add_child(&mut self, child: Box<dyn ParameterOrGroup>);
}

pub trait AddFirstChildArgs {}

pub trait AddFirstChild {

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
    fn add_first_child(
        &mut self, 
        first_child:        Box<dyn ParameterOrGroup>,
        remaining_children: &dyn AddFirstChildArgs
    );
}

pub trait UpdateChildParentage {

    fn update_child_parentage(&mut self);
}

pub trait GetSeparator {

    /**
      | Returns the group's separator string.
      |
      */
    fn get_separator(&self) -> String;
}

pub trait GetParent {

    /**
      | Returns the parent of the group, or nullptr
      | if this is a top-level group.
      |
      */
    fn get_parent(&self) -> *const dyn AudioProcessorParameterGroupInterface;
}

pub trait AppendNewParameter {

    fn append_new_parameter(&mut self, new_parameter: Box<dyn AudioProcessorParameterInterface>);
}

pub trait AppendNewSubgroup {

    fn append_new_subgroup(&mut self, new_sub_group: Box<dyn AudioProcessorParameterGroupInterface>);
}

pub trait GetSubgroups {

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
    fn get_subgroups(&self, recursive: bool) -> Vec<*const dyn AudioProcessorParameterGroupInterface>;
}

pub trait GetParametersInGroup {

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
    fn get_parameters(&self, recursive: bool) -> Vec<*mut dyn AudioProcessorParameterInterface>;
}

pub trait GetGroupsForParameter {

    /**
      | Searches this group recursively for
      | a parameter and returns a depth ordered
      | list of the groups it belongs to.
      |
      */
    fn get_groups_for_parameter(&self, parameter: *mut dyn AudioProcessorParameterInterface) 
        -> Vec<*const dyn AudioProcessorParameterGroupInterface>;
}

pub trait GetSubgroupsWithPrevious {

    fn get_subgroups_with_previous(
        &self, 
        previous_groups: &mut Vec<*const dyn AudioProcessorParameterGroupInterface>,
        recursive:       bool
    );
}

pub trait GetParametersWithPrevious {

    fn get_parameters_with_previous(
        &self, 
        previous_parameters: &mut Vec<*mut dyn AudioProcessorParameterInterface>,
        recursive:           bool
    );
}

pub trait GetGroupForParameter {

    fn get_group_for_parameter(&self, parameter: *mut dyn AudioProcessorParameterInterface) 
        -> *const dyn AudioProcessorParameterGroupInterface;
}
