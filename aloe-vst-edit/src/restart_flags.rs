crate::ix!();

/**
  | Flags used for IComponentHandler::restartComponent
  |
  */
pub enum RestartFlags
{
    /**
      | The Component should be reloaded
      | 
      | The host has to unload completely the
      | plug-in (controller/processor) and
      | reload it. [SDK 3.0.0]
      |
      */
    ReloadComponent            = 1 << 0,

    /**
      | Input / Output Bus configuration has
      | changed
      | 
      | The plug-in informs the host that either
      | the bus configuration or the bus count
      | has changed.
      | 
      | The host has to deactivate the plug-in,
      | asks the plug-in for its wanted new bus
      | configurations, adapts its processing
      | graph and reactivate the plug-in. [SDK
      | 3.0.0]
      |
      */
    IoChanged                  = 1 << 1,

    /**
      | Multiple parameter values have changed
      | (as result of a program change for example)
      | 
      | The host invalidates all caches of parameter
      | values and asks the edit controller
      | for the current values. [SDK 3.0.0]
      |
      */
    ParamValuesChanged         = 1 << 2,

    /**
      | Latency has changed
      | 
      | The plug informs the host that its latency
      | has changed, getLatencySamples should
      | return the new latency after setActive
      | (true) was called
      | 
      | The host has to deactivate and reactivate
      | the plug-in, then afterwards the host
      | could ask for the current latency (getLatencySamples)
      | see IAudioProcessor::getLatencySamples
      | [SDK 3.0.0]
      |
      */
    LatencyChanged             = 1 << 3,

    /**
      | Parameter titles, default values or
      | flags (ParameterInfoParameterFlags) have changed
      | 
      | The host invalidates all caches of parameter
      | infos and asks the edit controller for
      | the current infos. [SDK 3.0.0]
      |
      */
    ParamTitlesChanged         = 1 << 4,

    /**
      | MIDI Controllers and/or Program Changes
      | Assignments have changed
      | 
      | The plug-in informs the host that its
      | MIDI-CC mapping has changed (for example
      | after a MIDI learn or new loaded preset)
      | or if the stepCount or UnitID of a ProgramChange
      | parameter has changed.
      | 
      | The host has to rebuild the MIDI-CC =>
      | parameter mapping (getMidiControllerAssignment)
      | and reread program changes parameters
      | (stepCount and associated unitID)
      | [SDK 3.0.1]
      |
      */
    MidiCCAssignmentChanged    = 1 << 5,
    
    /**
      | Note Expression has changed (info,
      | count, PhysicalUIMapping, ...)
      | 
      | Either the note expression type info,
      | the count of note expressions or the
      | physical UI mapping has changed.
      | 
      | The host invalidates all caches of note
      | expression infos and asks the edit controller
      | for the current ones.
      | 
      | See INoteExpressionController, NoteExpressionTypeInfo
      | and INoteExpressionPhysicalUIMapping
      | [SDK 3.5.0]
      |
      */
    NoteExpressionChanged      = 1 << 6,
    
    /**
      | Input / Output bus titles have changed
      | 
      | The host invalidates all caches of bus
      | titles and asks the edit controller
      | for the current titles. [SDK 3.5.0]
      |
      */
    IoTitlesChanged            = 1 << 7,
    
    /**
      | Prefetch support has changed
      | 
      | The plug-in informs the host that its
      | PrefetchSupport has changed
      | 
      | The host has to deactivate the plug-in,
      | calls IPrefetchableSupport::getPrefetchableSupport
      | and reactivate the plug-in see IPrefetchableSupport
      | [SDK 3.6.1]
      |
      */
    PrefetchableSupportChanged = 1 << 8,

    /**
      | RoutingInfo has changed
      | 
      | The plug-in informs the host that its
      | internal routing (relation of an event-input-channel
      | to an audio-output-bus) has changed
      | 
      | The host ask the plug-in for the new routing
      | with IComponent::getRoutingInfo,
      | \ref vst3Routing see IComponent [SDK
      | 3.6.6]
      |
      */
    RoutingInfoChanged         = 1 << 9
}
