crate::ix!();

/**
  | Description of a Note Expression Type
  | 
  | This structure is part of the NoteExpressionTypeInfo
  | structure, it describes for given NoteExpressionTypeID
  | its default value (for example 0.5 for
  | a kTuningTypeID (kIsBipolar: centered)),
  | its minimum and maximum (for predefined
  | NoteExpressionTypeID the full range
  | is predefined too) and a stepCount when
  | the given NoteExpressionTypeID is
  | limited to discrete values (like on/off
  | state). \see NoteExpressionTypeInfo
  |
  */
pub struct NoteExpressionValueDescription
{
    /**
      | default normalized value [0,1]
      |
      */
    default_value: NoteExpressionValue,

    /**
      | minimum normalized value [0,1]
      |
      */
    minimum:       NoteExpressionValue,

    /**
      | maximum normalized value [0,1]
      |
      */
    maximum:       NoteExpressionValue,

    /**
      | number of discrete steps (0: continuous, 1:
      | toggle, discrete value otherwise - see \ref
      | vst3ParameterIntro)
      */
    step_count:    i32,
}
