crate::ix!();

/**
  | VstEvent \ingroup vstEventGrp
  | 
  | Structure representing a single VstEvent
  | of different types associated to a specific
  | event (\ref kEvent) bus.
  |
  */
pub struct VstEvent
{
    /**
      | event bus index
      |
      */
    bus_index:     i32,

    /**
      | sample frames related to the current
      | block start sample position
      |
      */
    sample_offset: i32,

    /**
      | position in project
      |
      */
    ppq_position:  TQuarterNotes,

    /**
      | combination of \ref VstEventFlags
      |
      */
    flags:         u16,

    /**
      | a value from \ref VstEventTypes
      |
      */
    ty:            u16,

    u:             VstEventU,
}

/**
  | VstEvent Flags - used for VstEvent::flags
  |
  */
pub enum VstEventFlags
{
    kIsLive        = 1 << 0,  // indicates that the event is played live (directly from keyboard)
    kUserReserved1 = 1 << 14, // reserved for user (for internal use)
    kUserReserved2 = 1 << 15  // reserved for user (for internal use)
}

/**
  | VstEvent Types - used for VstEvent::type
  |
  */
pub enum VstEventTypes
{
    kNoteOnEvent              = 0,    // is \ref NoteOnEvent
    kNoteOffEvent             = 1,    // is \ref NoteOffEvent
    kDataEvent                = 2,    // is \ref DataEvent
    kPolyPressureEvent        = 3,    // is \ref PolyPressureEvent
    kNoteExpressionValueEvent = 4,    // is \ref NoteExpressionValueEvent
    kNoteExpressionTextEvent  = 5,    // is \ref NoteExpressionTextEvent
    kChordEvent               = 6,    // is \ref ChordEvent
    kScaleEvent               = 7,    // is \ref ScaleEvent
    kLegacyMIDICCOutEvent     = 65535 // is \ref LegacyMIDICCOutEvent
}

#[derive(Copy,Clone)]
pub union VstEventU
{
    /**
      | type == kNoteOnEvent
      |
      */
    note_on:               NoteOnEvent,

    /**
      | type == kNoteOffEvent
      |
      */
    note_off:              NoteOffEvent,

    /**
      | type == kDataEvent
      |
      */
    data:                  DataEvent,

    /**
      | type == kPolyPressureEvent
      |
      */
    poly_pressure:         PolyPressureEvent,

    /**
      | type == kNoteExpressionValueEvent
      |
      */
    note_expression_value: NoteExpressionValueEvent,

    /**
      | type == kNoteExpressionTextEvent
      |
      */
    note_expression_text:  NoteExpressionTextEvent,

    /**
      | type == kChordEvent
      |
      */
    chord:                 ChordEvent,

    /**
      | type == kScaleEvent
      |
      */
    scale:                 ScaleEvent,

    /**
      | type == kLegacyMIDICCOutEvent
      |
      */
    midi_cc_out:           LegacyMIDICCOutEvent,
}
