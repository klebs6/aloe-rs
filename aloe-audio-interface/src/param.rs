crate::ix!();

pub trait AudioProcessorParameterInterface
: SetValueNotifyingHost
+ BeginChangeGesture
+ EndChangeGesture
+ GetParameterIndex
+ AddAudioProcessorParameterListener
+ RemoveAudioProcessorParameterListener
+ SendValueChangedMessageToListeners
{}

pub trait GetParameters {

    /**
      | Returns a flat list of the parameters
      | in the current tree.
      |
      */
    fn get_parameters(&self) -> &[*mut dyn AudioProcessorParameterInterface];
}

pub trait GetParameterIndex {

    /**
      | Returns the index of this parameter
      | in its parent processor's parameter
      | list.
      |
      */
    fn get_parameter_index(&self) -> i32;
}
