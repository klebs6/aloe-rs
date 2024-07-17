crate::ix!();

/**
  | NoteExpressionTypeIDs describes
  | the type of the note expression.
  | 
  | VST predefines some types like volume,
  | pan, tuning by defining their ranges
  | and curves.
  | 
  | Used by NoteExpressionEvent::typeId
  | and NoteExpressionTypeID::typeId
  | \see NoteExpressionTypeInfo
  |
  */
#[repr(u32)]
pub enum NoteExpressionTypeIDs 
{
    /**
      | Volume, plain range [0 = -oo , 0.25 = 0dB,
      | 0.5 = +6dB, 1 = +12dB]: plain = 20 * log (4
      | * norm)
      */
    VolumeTypeID = 0,      

    /**
      | Panning (L-R), plain range [0 = left, 0.5
      | = center, 1 = right]
      */
    PanTypeID,             

    /**
      | Tuning, plain range [0 = -120.0 (ten
      | octaves down), 0.5 none, 1 = +120.0 (ten
      | octaves up)] < plain = 240 * (norm - 0.5)
      | and norm = plain / 240 + 0.5 < oneOctave is
      | 12.0 / 240.0; oneHalfTune = 1.0 / 240.0;
      */
    TuningTypeID,          

    /**
      | Vibrato
      |
      */
    VibratoTypeID,         

    /**
      | Expression
      |
      */
    ExpressionTypeID,      

    /**
      | Brightness
      |
      */
    BrightnessTypeID,      

    /**
      | See NoteExpressionTextEvent
      |
      */
    TextTypeID,            

    PhonemeTypeID,         

    /**
      | start of custom note expression type
      | ids
      |
      */
    CustomStart = 100000,  

    /**
      | end of custom note expression type ids
      |
      */
    CustomEnd   = 200000,  

    /**
      | indicates an invalid note expression
      | type
      |
      */
    InvalidTypeID = 0xFFFFFFFF, 
}
