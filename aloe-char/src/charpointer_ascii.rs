crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_CharPointer_ASCII.h]

/**
  | Wraps a pointer to a null-terminated
  | ASCII character string, and provides
  | various methods to operate on the data.
  | 
  | A valid ASCII string is assumed to not
  | contain any characters above 127.
  | 
  | @see CharPointer_UTF8, CharPointer_UTF16,
  | CharPointer_UTF32
  | 
  | @tags{Core}
  |
  */
pub struct CharPointer_ASCII {
    data: *mut <Self as HasCharType>::CharType,
}

impl HasCharType for CharPointer_ASCII {
    type CharType = u8;
}

impl PartialEq<CharPointer_ASCII> for CharPointer_ASCII {
    
    /**
      | This is a pointer comparison, it doesn't
      | compare the actual text.
      |
      */
    fn eq(&self, other: &CharPointer_ASCII) -> bool {
        todo!();
        /*
            return data == other.data;
        */
    }
}

impl Eq for CharPointer_ASCII {}

impl Ord for CharPointer_ASCII {
    
    fn cmp(&self, other: &CharPointer_ASCII) -> std::cmp::Ordering {
        todo!();
        /*
            return data <  other.data;
        */
    }
}

impl PartialOrd<CharPointer_ASCII> for CharPointer_ASCII {
    fn partial_cmp(&self, other: &CharPointer_ASCII) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<<Self as HasCharType>::CharType> for CharPointer_ASCII {
    
    /**
      | Returns the address that this pointer
      | is pointing to.
      |
      */
    fn into(self) -> <Self as HasCharType>::CharType {
        todo!();
        /*
            return data;
        */
    }
}

impl Deref for CharPointer_ASCII {

    type Target = wchar_t;
    
    /**
      | Returns the unicode character that
      | this pointer is pointing to.
      |
      */
    fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return (aloe_wchar) (uint8) *data;
        */
    }
}

impl AddAssign<&i32> for CharPointer_ASCII {
    
    /**
      | Moves this pointer forwards by the specified
      | number of characters.
      |
      */
    fn add_assign(&mut self, other: &i32) {
        todo!();
        /*
            data += numToSkip;
        */
    }
}

impl SubAssign<i32> for CharPointer_ASCII {
    
    fn sub_assign(&mut self, num_to_skip: i32) {
        todo!();
        /*
            data -= numToSkip;
        */
    }
}

impl Index<i32> for CharPointer_ASCII {
    type Output = wchar_t;
    
    /**
      | Returns the character at a given character
      | index from the start of the string.
      |
      */
    fn index(&self, character_index: i32) -> &Self::Output {
        todo!();
        /*
            return (aloe_wchar) (uint8) data [characterIndex];
        */
    }
}

impl Add<&CharPointer_ASCII> for CharPointer_ASCII {

    type Output = Self;
    
    /**
      | Returns a pointer which is moved forwards
      | from this one by the specified number
      | of characters.
      |
      */
    #[inline] fn add(self, other: &CharPointer_ASCII) -> Self::Output {
        todo!();
        /*
            return CharPointer_ASCII (data + numToSkip);
        */
    }
}

impl Sub<i32> for CharPointer_ASCII {

    type Output = CharPointer_ASCII;
    
    /**
      | Returns a pointer which is moved backwards
      | from this one by the specified number
      | of characters.
      |
      */
    #[inline] fn sub(self, other: i32) -> Self::Output {
        todo!();
        /*
            return CharPointer_ASCII (data - numToSkip);
        */
    }
}


impl CharPointer_ASCII {

    pub fn new(raw_pointer: *const <Self as HasCharType>::CharType) -> Self {
    
        todo!();
        /*


            : data (const_cast<CharType*> (rawPointer))
        */
    }

    pub fn new_from_str(s: &'static str) -> Self {
        todo!();
    }
    
    #[inline] pub fn assign_from_ascii(&mut self, other: CharPointer_ASCII) -> CharPointer_ASCII {
        
        todo!();
        /*
            data = other.data;
            return *this;
        */
    }
    
    #[inline] pub fn assign_from(&mut self, text: *const <Self as HasCharType>::CharType) -> CharPointer_ASCII {
        
        todo!();
        /*
            data = const_cast<CharType*> (text);
            return *this;
        */
    }

    /**
      | Returns the address that this pointer
      | is pointing to.
      |
      */
    #[inline] pub fn get_address(&self) -> *mut <Self as HasCharType>::CharType {
        
        todo!();
        /*
            return data;
        */
    }

    /**
      | Returns true if this pointer is pointing
      | to a null character.
      |
      */
    #[inline] pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return *data == 0;
        */
    }

    /**
      | Returns true if this pointer is not pointing
      | to a null character.
      |
      */
    #[inline] pub fn is_not_empty(&self) -> bool {
        
        todo!();
        /*
            return *data != 0;
        */
    }

    /**
      | Moves this pointer along to the next
      | character in the string.
      |
      */
    #[inline] pub fn prefix_increment(&mut self) -> CharPointer_ASCII {
        
        todo!();
        /*
            ++data;
            return *this;
        */
    }

    /**
      | Moves this pointer to the previous character
      | in the string.
      |
      */
    #[inline] pub fn prefix_decrement(&mut self) -> CharPointer_ASCII {
        
        todo!();
        /*
            --data;
            return *this;
        */
    }

    /**
      | Returns the character that this pointer
      | is currently pointing to, and then advances
      | the pointer to point to the next character.
      |
      */
    #[inline] pub fn get_and_advance(&mut self) -> wchar_t {
        
        todo!();
        /*
            return (aloe_wchar) (uint8) *data++;
        */
    }

    /**
      | Moves this pointer along to the next
      | character in the string.
      |
      */
    pub fn postfix_increment(&mut self) -> CharPointer_ASCII {
        
        todo!();
        /*
            auto temp (*this);
            ++data;
            return temp;
        */
    }
    
    /**
      | Writes a unicode character to this string,
      | and advances this pointer to point to
      | the next position.
      |
      */
    #[inline] pub fn write(&mut self, char_to_write: wchar_t)  {
        
        todo!();
        /*
            *data++ = (char) charToWrite;
        */
    }
    
    #[inline] pub fn replace_char(&mut self, new_char: wchar_t)  {
        
        todo!();
        /*
            *data = (char) newChar;
        */
    }

    /**
      | Writes a null character to this string
      | (leaving the pointer's position unchanged).
      |
      */
    #[inline] pub fn write_null(&self)  {
        
        todo!();
        /*
            *data = 0;
        */
    }

    /**
      | Returns the number of characters in
      | this string.
      |
      */
    pub fn length(&self) -> usize {
        
        todo!();
        /*
            return (size_t) strlen (data);
        */
    }

    /**
      | Returns the number of characters in
      | this string, or the given value, whichever
      | is lower.
      |
      */
    pub fn length_up_to_max(&self, max_chars_to_count: usize) -> usize {
        
        todo!();
        /*
            return CharacterFunctions::lengthUpTo (*this, maxCharsToCount);
        */
    }

    /**
      | Returns the number of characters in
      | this string, or up to the given end pointer,
      | whichever is lower.
      |
      */
    pub fn length_up_to(&self, end: CharPointer_ASCII) -> usize {
        
        todo!();
        /*
            return CharacterFunctions::lengthUpTo (*this, end);
        */
    }

    /**
      | Returns the number of bytes that are
      | used to represent this string.
      | 
      | This includes the terminating null
      | character.
      |
      */
    pub fn size_in_bytes(&self) -> usize {
        
        todo!();
        /*
            return length() + 1;
        */
    }

    /**
      | Returns the number of bytes that would
      | be needed to represent the given unicode
      | character in this encoding format.
      |
      */
    pub fn get_bytes_required_for_wchar(_0: aloe_wchar) -> usize {
        
        todo!();
        /*
            return 1;
        */
    }

    /**
      | Returns the number of bytes that would
      | be needed to represent the given string
      | in this encoding format.
      | 
      | The value returned does NOT include
      | the terminating null character.
      |
      */
    pub fn get_bytes_required_for_string<CharPointer>(text: CharPointer) -> usize {
    
        todo!();
        /*
            return text.length();
        */
    }

    /**
      | Returns a pointer to the null character
      | that terminates this string.
      |
      */
    pub fn find_terminating_null(&self) -> CharPointer_ASCII {
        
        todo!();
        /*
            return CharPointer_ASCII (data + length());
        */
    }

    /**
      | Copies a source string to this pointer,
      | advancing this pointer as it goes.
      |
      */
    pub fn write_all<CharPointer>(&mut self, src: CharPointer)  {
    
        todo!();
        /*
            CharacterFunctions::copyAll (*this, src);
        */
    }

    /**
      | Copies a source string to this pointer,
      | advancing this pointer as it goes.
      | 
      | The maxDestBytes parameter specifies
      | the maximum number of bytes that can
      | be written to the destination buffer
      | before stopping.
      |
      */
    pub fn write_with_dest_byte_limit<CharPointer>(&mut self, 
        src:            CharPointer,
        max_dest_bytes: usize) -> usize {
    
        todo!();
        /*
            return CharacterFunctions::copyWithDestByteLimit (*this, src, maxDestBytes);
        */
    }

    /**
      | Copies a source string to this pointer,
      | advancing this pointer as it goes.
      | 
      | The maxChars parameter specifies the
      | maximum number of characters that can
      | be written to the destination buffer
      | before stopping (including the terminating
      | null).
      |
      */
    pub fn write_with_char_limit<CharPointer>(&mut self, 
        src:       CharPointer,
        max_chars: i32)  {
    
        todo!();
        /*
            CharacterFunctions::copyWithCharLimit (*this, src, maxChars);
        */
    }

    /**
      | Compares this string with another one.
      |
      */
    pub fn compare<CharPointer>(&self, other: CharPointer) -> i32 {
    
        todo!();
        /*
            return CharacterFunctions::compare (*this, other);
        */
    }

    /**
      | Compares this string with another one.
      |
      */
    pub fn compare_ascii(&self, other: CharPointer_ASCII) -> i32 {
        
        todo!();
        /*
            return strcmp (data, other.data);
        */
    }

    /**
      | Compares this string with another one,
      | up to a specified number of characters.
      |
      */
    pub fn compare_up_to<CharPointer>(&self, 
        other:     CharPointer,
        max_chars: i32) -> i32 {
    
        todo!();
        /*
            return CharacterFunctions::compareUpTo (*this, other, maxChars);
        */
    }

    /**
      | Compares this string with another one,
      | up to a specified number of characters.
      |
      */
    pub fn compare_up_to_ascii(&self, 
        other:     CharPointer_ASCII,
        max_chars: i32) -> i32 {
        
        todo!();
        /*
            return strncmp (data, other.data, (size_t) maxChars);
        */
    }

    /**
      | Compares this string with another one.
      |
      */
    pub fn compare_ignore_case<CharPointer>(&self, other: CharPointer) -> i32 {
    
        todo!();
        /*
            return CharacterFunctions::compareIgnoreCase (*this, other);
        */
    }
    
    pub fn compare_ignore_case_with_ascii(&self, other: CharPointer_ASCII) -> i32 {
        
        todo!();
        /*
            #if ALOE_MINGW || (ALOE_WINDOWS && ALOE_CLANG)
            return CharacterFunctions::compareIgnoreCase (*this, other);
           #elif ALOE_WINDOWS
            return stricmp (data, other.data);
           #else
            return strcasecmp (data, other.data);
           #endif
        */
    }

    /**
      | Compares this string with another one,
      | up to a specified number of characters.
      |
      */
    pub fn compare_ignore_case_up_to<CharPointer>(&self, 
        other:     CharPointer,
        max_chars: i32) -> i32 {
    
        todo!();
        /*
            return CharacterFunctions::compareIgnoreCaseUpTo (*this, other, maxChars);
        */
    }

    /**
      | Returns the character index of a substring,
      | or -1 if it isn't found.
      |
      */
    pub fn index_of_string<CharPointer>(&self, string_to_find: CharPointer) -> i32 {
    
        todo!();
        /*
            return CharacterFunctions::indexOf (*this, stringToFind);
        */
    }

    /**
      | Returns the character index of a unicode
      | character, or -1 if it isn't found.
      |
      */
    pub fn index_of_char(&self, char_to_find: wchar_t) -> i32 {
        
        todo!();
        /*
            int i = 0;

            while (data[i] != 0)
            {
                if (data[i] == (char) charToFind)
                    return i;

                ++i;
            }

            return -1;
        */
    }

    /**
      | Returns the character index of a unicode
      | character, or -1 if it isn't found.
      |
      */
    pub fn index_of_char_with_ignorecase_flag(&self, 
        char_to_find: wchar_t,
        ignore_case:  bool) -> i32 {
        
        todo!();
        /*
            return ignoreCase ? CharacterFunctions::indexOfCharIgnoreCase (*this, charToFind)
                              : CharacterFunctions::indexOfChar (*this, charToFind);
        */
    }

    /**
      | Returns true if the first character
      | of this string is whitespace.
      |
      */
    pub fn is_whitespace(&self) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isWhitespace (*data) != 0;
        */
    }

    /**
      | Returns true if the first character
      | of this string is a digit.
      |
      */
    pub fn is_digit(&self) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isDigit (*data) != 0;
        */
    }

    /**
      | Returns true if the first character
      | of this string is a letter.
      |
      */
    pub fn is_letter(&self) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isLetter (*data) != 0;
        */
    }

    /**
      | Returns true if the first character
      | of this string is a letter or digit.
      |
      */
    pub fn is_letter_or_digit(&self) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isLetterOrDigit (*data) != 0;
        */
    }

    /**
      | Returns true if the first character
      | of this string is upper-case.
      |
      */
    pub fn is_upper_case(&self) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isUpperCase ((aloe_wchar) (uint8) *data) != 0;
        */
    }

    /**
      | Returns true if the first character
      | of this string is lower-case.
      |
      */
    pub fn is_lower_case(&self) -> bool {
        
        todo!();
        /*
            return CharacterFunctions::isLowerCase ((aloe_wchar) (uint8) *data) != 0;
        */
    }

    /**
      | Returns an upper-case version of the
      | first character of this string.
      |
      */
    pub fn to_upper_case(&self) -> wchar_t {
        
        todo!();
        /*
            return CharacterFunctions::toUpperCase ((aloe_wchar) (uint8) *data);
        */
    }

    /**
      | Returns a lower-case version of the
      | first character of this string.
      |
      */
    pub fn to_lower_case(&self) -> wchar_t {
        
        todo!();
        /*
            return CharacterFunctions::toLowerCase ((aloe_wchar) (uint8) *data);
        */
    }

    /**
      | Parses this string as a 32-bit integer.
      |
      */
    pub fn get_int_value32(&self) -> i32 {
        
        todo!();
        /*
            return atoi (data);
        */
    }

    /**
      | Parses this string as a 64-bit integer.
      |
      */
    pub fn get_int_value64(&self) -> i64 {
        
        todo!();
        /*
            #if ALOE_LINUX || ALOE_BSD || ALOE_ANDROID || ALOE_MINGW
            return atoll (data);
           #elif ALOE_WINDOWS
            return _atoi64 (data);
           #else
            return CharacterFunctions::getIntValue <int64, CharPointer_ASCII> (*this);
           #endif
        */
    }

    /**
      | Parses this string as a floating point
      | double.
      |
      */
    pub fn get_double_value(&self) -> f64 {
        
        todo!();
        /*
            return CharacterFunctions::getDoubleValue (*this);
        */
    }

    /**
      | Returns the first non-whitespace character
      | in the string.
      |
      */
    pub fn find_end_of_whitespace(&self) -> CharPointer_ASCII {
        
        todo!();
        /*
            return CharacterFunctions::findEndOfWhitespace (*this);
        */
    }

    /**
      | Move this pointer to the first non-whitespace
      | character in the string.
      |
      */
    pub fn increment_to_end_of_whitespace(&mut self)  {
        
        todo!();
        /*
            CharacterFunctions::incrementToEndOfWhitespace (*this);
        */
    }

    /**
      | Returns true if the given unicode character
      | can be represented in this encoding.
      |
      */
    pub fn can_represent(character: wchar_t) -> bool {
        
        todo!();
        /*
            return ((unsigned int) character) < (unsigned int) 128;
        */
    }

    /**
      | Returns true if this data contains a
      | valid string in this encoding.
      |
      */
    pub fn is_valid_string(
        data_to_test:      *const <Self as HasCharType>::CharType,
        max_bytes_to_read: i32) -> bool {
        
        todo!();
        /*
            while (--maxBytesToRead >= 0)
            {
                if (((signed char) *dataToTest) <= 0)
                    return *dataToTest == 0;

                ++dataToTest;
            }

            return true;
        */
    }
}
