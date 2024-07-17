crate::ix!();


pub enum UnicodeNormalization
{
    /**
      | Unicode normalization Form C, canonical
      | composition.
      |
      */
    kUnicodeNormC,  

    /**
      | Unicode normalization Form D, canonical
      | decomposition.
      |
      */
    kUnicodeNormD,  

    /**
      | Unicode normalization form KC, compatibility
      | composition.
      |
      */
    kUnicodeNormKC, 

    /**
      | Unicode normalization form KD, compatibility
      | decomposition.
      |
      */
    kUnicodeNormKD  
}

#[cfg(UNICODE)]
pub const WIDE_STRING_DEFAULT: bool = true;

#[cfg(not(UNICODE))]
pub const WIDE_STRING_DEFAULT: bool = false;

/**
  UTF16 Byte Order Mark
  */
pub const BOM_UTF16: u16 = 0xFEFF;

/**
  UTF8 Byte Order Mark
  */
lazy_static!{
    /*
    pub const bom_utf8: &'static str = "\xEF\xBB\xBF";
    */
}

pub const BOM_UTF_8LENGTH: i32 = 3;
