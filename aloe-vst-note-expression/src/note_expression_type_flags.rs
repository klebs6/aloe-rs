crate::ix!();

/**
  | Note Expression Types
  |
  */
pub type NoteExpressionTypeID = u32;

/**
  | Note Expression Value
  |
  */
pub type NoteExpressionValue = f64;

/**
  | NoteExpressionTypeInfo is the structure
  | describing a note expression supported
  | by the plug-in.
  | 
  | This structure is used by the method
  | \ref INoteExpressionController::getNoteExpressionInfo.
  | \see INoteExpressionController
  |
  */
pub struct NoteExpressionTypeInfo
{
    /**
      | unique identifier of this note Expression
      | type
      |
      */
    type_id:                 NoteExpressionTypeID,

    /**
      | note Expression type title (e.g. "Volume")
      |
      */
    title:                   String128,

    /**
      | note Expression type short title (e.g.
      | "Vol")
      |
      */
    short_title:             String128,

    /**
      | note Expression type unit (e.g. "dB")
      |
      */
    units:                   String128,

    /**
      | id of unit this NoteExpression belongs to
      | (see \ref vst3Units), in order to sort the
      | note expression, it is possible to use unitId
      | like for parameters. -1 means no unit used.
      */
    unit_id:                 i32,

    /**
      | value description see \ref
      | NoteExpressionValueDescription
      |
      */
    value_desc:              NoteExpressionValueDescription,

    /**
      | optional associated parameter ID (for
      | mapping from note expression to global
      | (using the parameter automation for example)
      | and back). Only used when
      | kAssociatedParameterIDValid is set in flags.
      */
    associated_parameter_id: ParamID,

    /**
      | NoteExpressionTypeFlags (see below)
      |
      */
    flags:                   i32,
}

pub enum NoteExpressionTypeFlags
{
    /**
      | event is bipolar (centered), otherwise
      | unipolar
      |
      */
    IsBipolar                  = 1 << 0, 

    /**
      | event occurs only one time for its associated
      | note (at begin of the noteOn)
      |
      */
    IsOneShot                  = 1 << 1, 

    /**
      | This note expression will apply an absolute
      | change to the sound (not relative (offset))
      |
      */
    IsAbsolute                 = 1 << 2, 

    /**
      | indicates that the associatedParameterID
      | is valid and could be used
      |
      */
    AssociatedParameterIDValid = 1 << 3, 
}
