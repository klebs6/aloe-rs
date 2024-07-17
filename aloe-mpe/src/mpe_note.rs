crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPENote.h]

/**
  | Possible values for the note key state.
  |
  */
pub enum MpeNoteKeyState
{
    /**
      | The key is up (off)
      |
      */
    Off                  = 0, 

    /**
      | The note key is currently down (pressed).
      |
      */
    KeyDown              = 1, 

    /**
      | The note is sustained (by a sustain or
      | sostenuto pedal).
      |
      */
    Sustained            = 2, 

    /**
      | The note key is down and sustained (by
      | a sustain or sostenuto pedal)
      |
      */
    KeyDownAndSustained  = 3  
}

pub fn generate_noteid(
        midi_channel:     i32,
        midi_note_number: i32) -> u16 {
    
    todo!();
    /*
        jassert (midiChannel > 0 && midiChannel <= 16);
            jassert (midiNoteNumber >= 0 && midiNoteNumber < 128);

            return uint16 ((midiChannel << 7) + midiNoteNumber);
    */
}

/**
  | This struct represents a playing MPE note.
  |
  | A note is identified by a unique ID, or
  | alternatively, by a MIDI channel and an
  | initial note. It is characterised by five
  | dimensions of continuous expressive
  | control. Their current values are represented
  | as MPEValue objects.
  |
  | @see MPEValue
  |
  | @tags{Audio}
  */
pub struct MPENote {

    /**
      | A unique ID. Useful to distinguish the note from
      | other simultaneously sounding notes that may use
      | the same note number or MIDI channel. This should
      | never change during the lifetime of a note object.
      */
    noteid:                       u16, // default = 0

    /**
      | The MIDI channel which this note uses. This should
      | never change during the lifetime of an MPENote
      | object.
      */
    midi_channel:                 u8, // default = 0

    /**
      | The MIDI note number that was sent when the note
      | was triggered. This should never change during
      | the lifetime of an MPENote object.
      */
    initial_note:                 u8, // default = 0

    /* The five dimensions of continuous expressive control */

    /**
      | The velocity ("strike") of the note-on.
      | This dimension will stay constant after
      | the note has been turned on.
      |
      */
    note_on_velocity:             MPEValue, //= MPEValue::minValue() ;

    /**
      | Current per-note pitchbend of the note (in units
      | of MIDI pitchwheel position). This dimension can
      | be modulated while the note sounds. Note: This
      | value is not aware of the currently used pitchbend
      | range, or an additional master pitchbend that may
      | be simultaneously applied. To compute the actual
      | effective pitchbend of an MPENote, you should probably
      | use the member totalPitchbendInSemitones instead.
      | @see totalPitchbendInSemitones, getFrequencyInHertz
      */
    pitchbend:                    MPEValue,

    /**
      | Current pressure with which the note is held down.
      | This dimension can be modulated while the note
      | sounds.
      */
    pressure:                     MPEValue,

    /**
      | Initial value of timbre when the note was triggered.
      | This should never change during the lifetime of
      | an MPENote object.
      */
    initial_timbre:               MPEValue,

    /**
      | Current value of the note's third expressive dimension,
      | typically encoding some kind of timbre parameter.
      | This dimension can be modulated while the note
      | sounds.
      */
    timbre:                       MPEValue,

    /**
      | The release velocity ("lift") of the note after
      | a note-off has been received. This dimension will
      | only have a meaningful value after a note-off has
      | been received for the note (and keyState is set
      | to MPENote::off or MPENote::sustained). Initially,
      | the value is undefined.
      */
    note_off_velocity:            MPEValue,

    /**
      | Current effective pitchbend of the note in units
      | of semitones, relative to initialNote. You should
      | use this to compute the actual effective pitch
      | of the note. This value is computed and set by
      | an MPEInstrument to the sum of the per-note pitchbend
      | value (stored in MPEValue::pitchbend) and the master
      | pitchbend of the MPE zone, weighted with the per-note
      | pitchbend range and master pitchbend range of the
      | zone, respectively. @see getFrequencyInHertz
      */
    total_pitchbend_in_semitones: f64,

    /**
      | Current key state. Indicates whether the note key
      | is currently down (pressed) and/or the note is
      | sustained (by a sustain or sostenuto pedal).
      */
    key_state:                    MpeNoteKeyState, // default = MPENote::off 
}

impl PartialEq<MPENote> for MPENote {
    
    #[inline] fn eq(&self, other: &MPENote) -> bool {
        todo!();
        /*
            jassert (isValid() && other.isValid());
        return noteID == other.noteID;
        */
    }
}

impl Eq for MPENote {}

impl Default for MPENote {
    
    /**
      | Default constructor.
      | 
      | Constructs an invalid MPE note (a note
      | with the key state MPENote::off and
      | an invalid MIDI channel. The only allowed
      | use for such a note is to call isValid()
      | on it; everything else is undefined
      | behaviour.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPENote.cpp]
impl MPENote {

    /* ------- Invariants that define the note.   ------- */

    /**
      | Constructor.
      | 
      | -----------
      | @param midiChannel
      | 
      | The MIDI channel of the note, between
      | 2 and 15. (Channel 1 and channel 16 can
      | never be note channels in MPE).
      | ----------
      | @param initialNote
      | 
      | The MIDI note number, between 0 and 127.
      | ----------
      | @param velocity
      | 
      | The note-on velocity of the note.
      | ----------
      | @param pitchbend
      | 
      | The initial per-note pitchbend of the
      | note.
      | ----------
      | @param pressure
      | 
      | The initial pressure of the note.
      | ----------
      | @param timbre
      | 
      | The timbre value of the note.
      | ----------
      | @param keyState
      | 
      | The key state of the note (whether the
      | key is down and/or the note is sustained).
      | This value must not be MPENote::off,
      | since you are triggering a new note.
      | (If not specified, the default value
      | will be MPENote::keyDown.)
      |
      */
    pub fn new(
        midi_channel:     i32,
        initial_note:     i32,
        note_on_velocity: MPEValue,
        pitchbend:        MPEValue,
        pressure:         MPEValue,
        timbre:           MPEValue,
        key_state:        Option<MpeNoteKeyState>

    ) -> Self {

        let key_state: MpeNoteKeyState =
            key_state.unwrap_or(MpeNoteKeyState::KeyDown);
    
        todo!();
        /*
        : noteid(generateNoteID (midiChannel_, initialNote_)),
        : midi_channel(uint8 (midiChannel_)),
        : initial_note(uint8 (initialNote_)),
        : note_on_velocity(noteOnVelocity_),
        : pitchbend(pitchbend_),
        : pressure(pressure_),
        : initial_timbre(timbre_),
        : timbre(timbre_),
        : key_state(keyState_),

            jassert (keyState != MPENote::off);
        jassert (isValid());
        */
    }
    
    /**
      | Checks whether the MPE note is valid
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return midiChannel > 0 && midiChannel <= 16 && initialNote < 128;
        */
    }
    
    /**
      | Returns the current frequency of the
      | note in Hertz. This is the sum of the initialNote
      | and the totalPitchbendInSemitones,
      | converted to Hertz.
      |
      */
    pub fn get_frequency_in_hertz(&self, frequency_ofa: Option<f64>) -> f64 {

        let frequency_ofa: f64 = frequency_ofa.unwrap_or(440.0);
        
        todo!();
        /*
            auto pitchInSemitones = double (initialNote) + totalPitchbendInSemitones;
        return frequencyOfA * std::pow (2.0, (pitchInSemitones - 69.0) / 12.0);
        */
    }
}
