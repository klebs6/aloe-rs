crate::ix!();

/**
  | This is the character encoding type
  | used internally to store the string.
  | 
  | By setting the value of ALOE_STRING_UTF_TYPE
  | to 8, 16, or 32, you can change the internal
  | storage format of the String class.
  | UTF-8 uses the least space (if your strings
  | contain few extended characters),
  | but call operator[] involves iterating
  | the string to find the required index.
  | UTF-32 provides instant random access
  | to its characters, but uses 4 bytes per
  | character to store them. UTF-16 uses
  | more space than UTF-8 and is also slow
  | to index, but is the native wchar_t format
  | used in Windows.
  | 
  | It doesn't matter too much which format
  | you pick, because the toUTF8(), toUTF16()
  | and toUTF32() methods let you access
  | the string's content in any of the other
  | formats.
  |
  */
#[cfg(ALOE_STRING_UTF_TYPE_EQ_32)]
pub type CharPointerType = CharPointer_UTF32;

#[cfg(ALOE_STRING_UTF_TYPE_EQ_16)]
pub type CharPointerType = CharPointer_UTF16;

#[cfg(ALOE_STRING_UTF_TYPE_EQ_8)]
pub type CharPointerType = CharPointer_UTF8;

#[cfg(not(any(
            ALOE_STRING_UTF_TYPE_EQ_8,
            ALOE_STRING_UTF_TYPE_EQ_16,
            ALOE_STRING_UTF_TYPE_EQ_32,
)))]
pub type CharPointerType = CharPointer_UTF8;

pub trait HasCharType {

    type CharType;
}
