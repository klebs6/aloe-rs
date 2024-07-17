crate::ix!();

/**
  | Scale event specific data. Used in \ref
  | VstEvent (union) \ingroup vstEventGrp
  |
  */
#[derive(Copy,Clone)]
pub struct ScaleEvent
{
    /**
      | range [0, 127] = root Note/Transpose
      | Factor
      |
      */
    root:     i16,

    /**
      | Bit 0 =  C,  Bit 1 = C#, ... (0x5ab5
      | = Major Scale)
      */
    mask:     i16,

    /**
      | the number of characters (TChar) between
      | the beginning of text and the terminating null
      | character (without including the terminating
      | null character itself)
      */
    text_len: u16,

    /**
      | UTF-16, null terminated, Hosts Scale
      | Name
      |
      */
    text:     *const TChar,
}
