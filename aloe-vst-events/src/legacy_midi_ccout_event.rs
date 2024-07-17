crate::ix!();

/**
  | Legacy MIDI CC Out event specific data.
  | Used in \ref VstEvent (union) \ingroup
  | vstEventGrp
  | 
  | - [released: 3.6.12]
  | 
  | This kind of event is reserved for generating
  | MIDI CC as output event for kEvent Bus
  | during the process call.
  |
  */
#[derive(Copy,Clone)]
pub struct LegacyMIDICCOutEvent
{
    /**
      | see enum ControllerNumbers [0, 255]
      |
      */
    control_number: u8,

    /**
      | channel index in event bus [0, 15]
      |
      */
    channel:        i8,

    /**
      | value of Controller [0, 127]
      |
      */
    value:          i8,

    /**
       | [0, 127] used for pitch bend (kPitchBend)
       | and polyPressure (kCtrlPolyPressure)
      */
    value2:         i8,
}
