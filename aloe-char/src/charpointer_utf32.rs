crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_CharPointer_UTF32.h]

/**
  | Wraps a pointer to a null-terminated
  | UTF-32 character string, and provides
  | various methods to operate on the data.
  | 
  | @see CharPointer_UTF8, CharPointer_UTF16
  | 
  | @tags{Core}
  |
  */
pub struct CharPointer_UTF32 {
    data: *mut <Self as HasCharType>::CharType,
}

impl HasCharType for CharPointer_UTF32 {
    type CharType = wchar_t;
}

impl PartialEq<CharPointer_UTF32> for CharPointer_UTF32 {
    
    /**
      | This is a pointer comparison, it doesn't
      | compare the actual text.
      |
      */
    fn eq(&self, other: &CharPointer_UTF32) -> bool {
        todo!();
        /*
            return data == other.data;
        */
    }
}

impl Eq for CharPointer_UTF32 {}

impl Ord for CharPointer_UTF32 {
    
    fn cmp(&self, other: &CharPointer_UTF32) -> std::cmp::Ordering {
        todo!();
        /*
            return data <  other.data;
        */
    }
}

impl PartialOrd<CharPointer_UTF32> for CharPointer_UTF32 {

    fn partial_cmp(&self, other: &CharPointer_UTF32) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<<Self as HasCharType>::CharType> for CharPointer_UTF32 {
    
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

impl Deref for CharPointer_UTF32 {

    type Target = wchar_t;
    
    /**
      | Returns the unicode character that
      | this pointer is pointing to.
      |
      */
    fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return *data;
        */
    }
}

impl AddAssign<&i32> for CharPointer_UTF32 {
    
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

impl SubAssign<i32> for CharPointer_UTF32 {
    
    fn sub_assign(&mut self, num_to_skip: i32) {
        todo!();
        /*
            data -= numToSkip;
        */
    }
}


impl Index<i32> for CharPointer_UTF32 {
    type Output = wchar_t;
    
    /**
      | Returns the character at a given character
      | index from the start of the string.
      |
      */
    fn index(&self, character_index: i32) -> &Self::Output {
        todo!();
        /*
            return data [characterIndex];
        */
    }
}

impl Add<&CharPointer_UTF32> for CharPointer_UTF32 {

    type Output = Self;
    
    /**
      | Returns a pointer which is moved forwards
      | from this one by the specified number
      | of characters.
      |
      */
    #[inline] fn add(self, other: &CharPointer_UTF32) -> Self::Output {
        todo!();
        /*
            return CharPointer_UTF32 (data + numToSkip);
        */
    }
}

impl Sub<i32> for CharPointer_UTF32 {

    type Output = CharPointer_UTF32;

    /**
      | Returns a pointer which is moved backwards
      | from this one by the specified number
      | of characters.
      |
      */
    #[inline] fn sub(self, other: i32) -> Self::Output {
        todo!();
        /*
            return CharPointer_UTF32 (data - numToSkip);
        */
    }
}

impl CharPointer_UTF32 {

    pub fn new(raw_pointer: *const <Self as HasCharType>::CharType) -> Self {
    
        todo!();
        /*


            : data (const_cast<CharType*> (rawPointer))
        */
    }
    
    #[inline] pub fn assign_from_utf32(&mut self, other: CharPointer_UTF32) -> CharPointer_UTF32 {
        
        todo!();
        /*
            data = other.data;
            return *this;
        */
    }
    
    #[inline] pub fn assign_from(&mut self, text: *const <Self as HasCharType>::CharType) -> CharPointer_UTF32 {
        
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
    #[inline] pub fn prefix_increment(&mut self) -> CharPointer_UTF32 {
        
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
    #[inline] pub fn postfix_decrement(&mut self) -> CharPointer_UTF32 {
        
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
            return *data++;
        */
    }

    /**
      | Moves this pointer along to the next
      | character in the string.
      |
      */
    pub fn postfix_increment(&mut self) -> CharPointer_UTF32 {
        
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
            *data++ = charToWrite;
        */
    }
    
    #[inline] pub fn replace_char(&mut self, new_char: wchar_t)  {
        
        todo!();
        /*
            *data = newChar;
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
            #if ALOE_NATIVE_WCHAR_IS_UTF32 && ! ALOE_ANDROID
            return wcslen (data);
           #else
            size_t n = 0;
            while (data[n] != 0)
                ++n;
            return n;
           #endif
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
    pub fn length_up_to(&self, end: CharPointer_UTF32) -> usize {
        
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
            return sizeof (CharType) * (length() + 1);
        */
    }

    /**
      | Returns the number of bytes that would
      | be needed to represent the given unicode
      | character in this encoding format.
      |
      */
    pub fn get_bytes_required_for_wchar(_0: wchar_t) -> usize {
        
        todo!();
        /*
            return sizeof (CharType);
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
            return sizeof (CharType) * text.length();
        */
    }

    /**
      | Returns a pointer to the null character
      | that terminates this string.
      |
      */
    pub fn find_terminating_null(&self) -> CharPointer_UTF32 {
        
        todo!();
        /*
            return CharPointer_UTF32 (data + length());
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
      */
    pub fn write_all_utf32(&mut self, src: CharPointer_UTF32)  {
        
        todo!();
        /*
            auto* s = src.data;

            while ((*data = *s) != 0)
            {
                ++data;
                ++s;
            }
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
    #[cfg(all(ALOE_NATIVE_WCHAR_IS_UTF32,not(target_os="android")))]
    pub fn compare(&self, other: CharPointer_UTF32) -> i32 {
        
        todo!();
        /*
            return wcscmp (data, other.data);
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
      | Compares this string with another one.
      |
      */
    pub fn compare_ignore_case<CharPointer>(&self, other: CharPointer) -> i32 {
    
        todo!();
        /*
            return CharacterFunctions::compareIgnoreCase (*this, other);
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
    pub fn index_of_substring<CharPointer>(&self, string_to_find: CharPointer) -> i32 {
    
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
    pub fn index_of_wchar(&self, char_to_find: wchar_t) -> i32 {
        
        todo!();
        /*
            int i = 0;

            while (data[i] != 0)
            {
                if (data[i] == charToFind)
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
    pub fn index_of_wchar_with_ignorecase_flag(&self, 
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
            return CharacterFunctions::isUpperCase (*data) != 0;
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
            return CharacterFunctions::isLowerCase (*data) != 0;
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
            return CharacterFunctions::toUpperCase (*data);
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
            return CharacterFunctions::toLowerCase (*data);
        */
    }

    /**
      | Parses this string as a 32-bit integer.
      |
      */
    pub fn get_int_value32(&self) -> i32 {
        
        todo!();
        /*
            return CharacterFunctions::getIntValue <int, CharPointer_UTF32> (*this);
        */
    }

    /**
      | Parses this string as a 64-bit integer.
      |
      */
    pub fn get_int_value64(&self) -> i64 {
        
        todo!();
        /*
            return CharacterFunctions::getIntValue <int64, CharPointer_UTF32> (*this);
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
    pub fn find_end_of_whitespace(&self) -> CharPointer_UTF32 {
        
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
            return ((uint32) character) < (uint32) 0x10ffff;
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
            maxBytesToRead /= (int) sizeof (CharType);

            while (--maxBytesToRead >= 0 && *dataToTest != 0)
                if (! canRepresent (*dataToTest++))
                    return false;

            return true;
        */
    }

    /**
      | Atomically swaps this pointer for a
      | new value, returning the previous value.
      |
      */
    pub fn atomic_swap(&mut self, new_value: CharPointer_UTF32) -> CharPointer_UTF32 {
        
        todo!();
        /*
            return CharPointer_UTF32 (reinterpret_cast<Atomic<CharType*>&> (data).exchange (newValue.data));
        */
    }
}
