crate::ix!();

pub trait AudioProcessorEditorInterface:
SetControlHighlight
+ GetControlParameterIndex
+ SupportsHostMIDIControllerPresence
+ HostMIDIControllerIsAvailable
+ SetScaleFactor {}

pub trait SetControlHighlight {

    /**
      | Some types of plugin can call this to
      | suggest that the control for a particular
      | parameter should be highlighted.
      | 
      | Currently only AAX plugins will call
      | this, and implementing it is optional.
      |
      */
    fn set_control_highlight(&mut self, _0: AudioProcessorEditorParameterControlHighlightInfo);
}

pub trait GetControlParameterIndex {

    /**
      | Called by certain plug-in wrappers
      | to find out whether a component is used
      | to control a parameter.
      | 
      | If the given component represents a
      | particular plugin parameter, then
      | this method should return the index
      | of that parameter. If not, it should
      | return -1.
      | 
      | Currently only AAX plugins will call
      | this, and implementing it is optional.
      |
      */
    fn get_control_parameter_index(&mut self, _0: &mut Component) -> i32;
}

pub trait SupportsHostMIDIControllerPresence {

    /**
      | Override this method to indicate if
      | your editor supports the presence or
      | absence of a host-provided MIDI controller.
      | 
      | Currently only AUv3 plug-ins compiled
      | for MacOS 10.13 or iOS 11.0 (or later)
      | support this functionality, and even
      | then the host may choose to ignore this
      | information.
      | 
      | The default behaviour is to report support
      | for both cases.
      |
      */
    fn supports_host_midi_controller_presence(&mut self, host_midi_controller_is_available: bool) -> bool;

}

pub trait HostMIDIControllerIsAvailable {

    /**
      | Called to indicate if a host is providing
      | a MIDI controller when the host reconfigures
      | its layout.
      | 
      | Use this as an opportunity to hide or
      | display your own onscreen keyboard
      | or other input component.
      | 
      | Currently only AUv3 plug-ins compiled
      | for MacOS 10.13 or iOS 11.0 (or later)
      | support this functionality.
      |
      */
    fn host_midi_controller_is_available(&mut self, controller_is_available: bool);
}

pub type create_unity_peer_function_type = fn(&mut Component);

