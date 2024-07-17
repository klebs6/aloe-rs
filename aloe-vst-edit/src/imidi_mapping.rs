crate::ix!();

/**
  | MIDI Mapping interface: Vst::IMidiMapping
  | \ingroup vstIPlug vst301
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.0.1]
  | 
  | - [optional]
  | 
  | MIDI controllers are not transmitted
  | directly to a VST component. MIDI as
  | hardware protocol has restrictions
  | that can be avoided in software. Controller
  | data in particular come along with unclear
  | and often ignored semantics. On top
  | of this they can interfere with regular
  | parameter automation and the host is
  | unaware of what happens in the plug-in
  | when passing MIDI controllers directly.
  | 
  | So any functionality that is to be controlled
  | by MIDI controllers must be exported
  | as regular parameter.
  | 
  | The host will transform incoming MIDI
  | controller data using this interface
  | and transmit them as regular parameter
  | change. This allows the host to automate
  | them in the same way as other parameters.
  | 
  | CtrlNumber can be a typical MIDI controller
  | value extended to some others values
  | like pitchbend or aftertouch (see \ref
  | ControllerNumbers).
  | 
  | If the mapping has changed, the plug-in
  | must call IComponentHandler::restartComponent
  | (kMidiCCAssignmentChanged) to inform
  | the host about this change.
  | 
  | \section IMidiMappingExample Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | // in myeditcontroller.h
  | class MyEditController: public EditControllerEx1, public IMidiMapping
  | {
  |     //...
  |     //---IMidiMapping---------------------------
  |     tresult PLUGIN_API getMidiControllerAssignment (i32 busIndex, int16 channel, CtrlNumber midiControllerNumber, ParamID& id) SMTG_OVERRIDE;
  |     //---Interface---------
  |     OBJ_METHODS (MyEditController, EditControllerEx1)
  |     DEFINE_INTERFACES
  |         DEF_INTERFACE (IMidiMapping)
  |     END_DEFINE_INTERFACES (MyEditController)
  |     REFCOUNT_METHODS (MyEditController)
  | };
  | 
  | // in myeditcontroller.cpp
  | tresult PLUGIN_API MyEditController::getMidiControllerAssignment (i32 busIndex, int16 midiChannel, CtrlNumber midiControllerNumber, ParamID& tag)
  | {
  |     // for my first Event bus and for MIDI channel 0 and for MIDI CC Volume only
  |     if (busIndex == 0 && midiChannel == 0 && midiControllerNumber == kCtrlVolume)
  |     {
  |         tag = kGainId;
  |         return kResultTrue;
  |     }
  |     return kResultFalse;
  | }
  |
  */
pub trait IMidiMapping: FUnknown {

    /**
      | Gets an (preferred) associated ParamID
      | for a given Input Event Bus index, channel
      | and MIDI Controller.
      | 
      | -----------
      | @param[in] busIndex
      | 
      | - index of Input Event Bus
      | ----------
      | @param[in] channel
      | 
      | - channel of the bus
      | ----------
      | @param[in] midiControllerNumber
      | 
      | - see \ref ControllerNumbers for expected
      | values (could be bigger than 127)
      | ----------
      | @param[in] id
      | 
      | - return the associated ParamID to the
      | given midiControllerNumber
      |
      */
    #[PLUGIN_API]
    fn get_midi_controller_assignment(&mut self, 
            bus_index:              i32,
            channel:                i16,
            midi_controller_number: CtrlNumber,
            id:                     &mut ParamID) -> tresult;
}

lazy_static!{
    /*
    static const FUID imidi_mapping_iid;
    */
}

declare_class_iid!{
    IMidiMapping, 
    0xDF0FF9F7, 
    0x49B74669, 
    0xB63AB732, 
    0x7ADBF5E5
}
