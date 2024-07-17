crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_CharPointer_UTF16.h]

/**
  | Wraps a pointer to a null-terminated
  | UTF-16 character string, and provides
  | various methods to operate on the data.
  | @see CharPointer_UTF8, CharPointer_UTF32
  | 
  | @tags{Core}
  |
  */
pub struct CharPointer_UTF16 {
    data: *mut <Self as HasCharType>::CharType,
}

pub mod char_pointer_utf16 {
    use super::*;

    /**
      | These values are the byte-order-mark
      | (BOM) values for a UTF-16 stream.
      |
      */
    pub const byteOrderMarkBE1: u8 = 0xfe;
    pub const byteOrderMarkBE2: u8 = 0xff;
    pub const byteOrderMarkLE1: u8 = 0xff;
    pub const byteOrderMarkLE2: u8 = 0xfe;
}

impl HasCharType for CharPointer_UTF16 {

    #[cfg(ALOE_NATIVE_WCHAR_IS_UTF16)]
    type CharType = libc::wchar_t;

    #[cfg(not(ALOE_NATIVE_WCHAR_IS_UTF16))]
    type CharType = i16;
}

impl PartialEq<CharPointer_UTF16> for CharPointer_UTF16 {
    
    /**
      | This is a pointer comparison, it doesn't
      | compare the actual text.
      |
      */
    fn eq(&self, other: &CharPointer_UTF16) -> bool {
        todo!();
        /*
            return data == other.data;
        */
    }
}

impl Eq for CharPointer_UTF16 {}

impl Ord for CharPointer_UTF16 {
    
    fn cmp(&self, other: &CharPointer_UTF16) -> std::cmp::Ordering {
        todo!();
        /*
            return data <  other.data;
        */
    }
}

impl PartialOrd<CharPointer_UTF16> for CharPointer_UTF16 {

    fn partial_cmp(&self, other: &CharPointer_UTF16) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<<Self as HasCharType>::CharType> for CharPointer_UTF16 {

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

impl Deref for CharPointer_UTF16 {

    type Target = wchar_t;
    
    /**
      | Returns the unicode character that
      | this pointer is pointing to.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            auto n = (uint32) (uint16) *data;

            if (n >= 0xd800 && n <= 0xdfff && ((uint32) (uint16) data[1]) >= 0xdc00)
                n = 0x10000 + (((n - 0xd800) << 10) | (((uint32) (uint16) data[1]) - 0xdc00));

            return (aloe_wchar) n;
        */
    }
}

impl AddAssign<&i32> for CharPointer_UTF16 {
    
    /**
      | Moves this pointer forwards by the specified
      | number of characters.
      |
      */
    #[inline] fn add_assign(&mut self, other: &i32) {
        todo!();
        /*
            if (numToSkip < 0)
            {
                while (++numToSkip <= 0)
                    --*this;
            }
            else
            {
                while (--numToSkip >= 0)
                    ++*this;
            }
        */
    }
}

impl SubAssign<i32> for CharPointer_UTF16 {
    
    /**
      | Moves this pointer backwards by the
      | specified number of characters.
      |
      */
    #[inline] fn sub_assign(&mut self, num_to_skip: i32) {
        todo!();
        /*
            operator+= (-numToSkip);
        */
    }
}

impl Index<i32> for CharPointer_UTF16 {
    type Output = wchar_t;
    
    /**
      | Returns the character at a given character
      | index from the start of the string.
      |
      */
    #[inline] fn index(&self, character_index: i32) -> &Self::Output {
        todo!();
        /*
            auto p (*this);
            p += characterIndex;
            return *p;
        */
    }
}

impl Add<&CharPointer_UTF16> for CharPointer_UTF16 {

    type Output = Self;

    /**
      | Returns a pointer which is moved forwards
      | from this one by the specified number
      | of characters.
      |
      */
    #[inline] fn add(self, other: &CharPointer_UTF16) -> Self::Output {
        todo!();
        /*
            auto p (*this);
            p += numToSkip;
            return p;
        */
    }
}

impl Sub<i32> for CharPointer_UTF16 {

    type Output = CharPointer_UTF16;

    /**
      | Returns a pointer which is moved backwards
      | from this one by the specified number
      | of characters.
      |
      */
    #[inline] fn sub(self, other: i32) -> Self::Output {
        todo!();
        /*
            auto p (*this);
            p += -numToSkip;
            return p;
        */
    }
}

impl CharPointer_UTF16 {

    pub fn new(raw_pointer: *const <Self as HasCharType>::CharType) -> Self {
    
        todo!();
        /*


            : data (const_cast<CharType*> (rawPointer))
        */
    }
    
    #[inline] pub fn assign_from_other(&mut self, other: CharPointer_UTF16) -> CharPointer_UTF16 {
        
        todo!();
        /*
            data = other.data;
            return *this;
        */
    }
    
    #[inline] pub fn assign_from(&mut self, text: *const <Self as HasCharType>::CharType) -> CharPointer_UTF16 {
        
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
    pub fn prefix_increment(&mut self) -> CharPointer_UTF16 {
        
        todo!();
        /*
            auto n = (uint32) (uint16) *data++;

            if (n >= 0xd800 && n <= 0xdfff && ((uint32) (uint16) *data) >= 0xdc00)
                ++data;

            return *this;
        */
    }

    /**
      | Moves this pointer back to the previous
      | character in the string.
      |
      */
    pub fn postfix_decrement(&mut self) -> CharPointer_UTF16 {
        
        todo!();
        /*
            auto n = (uint32) (uint16) (*--data);

            if (n >= 0xdc00 && n <= 0xdfff)
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
    pub fn get_and_advance(&mut self) -> wchar_t {
        
        todo!();
        /*
            auto n = (uint32) (uint16) *data++;

            if (n >= 0xd800 && n <= 0xdfff && ((uint32) (uint16) *data) >= 0xdc00)
                n = 0x10000 + ((((n - 0xd800) << 10) | (((uint32) (uint16) *data++) - 0xdc00)));

            return (aloe_wchar) n;
        */
    }

    /**
      | Moves this pointer along to the next
      | character in the string.
      |
      */
    pub fn postfix_increment(&mut self) -> CharPointer_UTF16 {
        
        todo!();
        /*
            auto temp (*this);
            ++*this;
            return temp;
        */
    }

    /**
      | Writes a unicode character to this string,
      | and advances this pointer to point to
      | the next position.
      |
      */
    pub fn write(&mut self, char_to_write: wchar_t)  {
        
        todo!();
        /*
            if (charToWrite >= 0x10000)
            {
                charToWrite -= 0x10000;
                *data++ = (CharType) (0xd800 + (charToWrite >> 10));
                *data++ = (CharType) (0xdc00 + (charToWrite & 0x3ff));
            }
            else
            {
                *data++ = (CharType) charToWrite;
            }
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
            auto* d = data;
            size_t count = 0;

            for (;;)
            {
                auto n = (uint32) (uint16) *d++;

                if (n >= 0xd800 && n <= 0xdfff)
                {
                    if (*d++ == 0)
                        break;
                }
                else if (n == 0)
                    break;

                ++count;
            }

            return count;
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
    pub fn length_up_to(&self, end: CharPointer_UTF16) -> usize {
        
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
            return sizeof (CharType) * (findNullIndex (data) + 1);
        */
    }

    /**
      | Returns the number of bytes that would
      | be needed to represent the given unicode
      | character in this encoding format.
      |
      */
    pub fn get_bytes_required_for_wchar(char_to_write: wchar_t) -> usize {
        
        todo!();
        /*
            return (charToWrite >= 0x10000) ? (sizeof (CharType) * 2) : sizeof (CharType);
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
    pub fn get_bytes_required_for<CharPointer>(text: CharPointer) -> usize {
    
        todo!();
        /*
            size_t count = 0;
            aloe_wchar n;

            while ((n = text.getAndAdvance()) != 0)
                count += getBytesRequiredFor (n);

            return count;
        */
    }

    /**
      | Returns a pointer to the null character
      | that terminates this string.
      |
      */
    pub fn find_terminating_null(&self) -> CharPointer_UTF16 {
        
        todo!();
        /*
            auto* t = data;

            while (*t != 0)
                ++t;

            return CharPointer_UTF16 (t);
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
    pub fn write_all_utf16(&mut self, src: CharPointer_UTF16)  {
        
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
    pub fn write_with_dest_byte_limit<CharPointer>(
        &mut self, 
        src:            CharPointer,
        max_dest_bytes: usize) -> usize 
    {
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
    pub fn write_with_char_limit<CharPointer>(
        &mut self, 
        src:       CharPointer,
        max_chars: i32)
    {
    
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

    #[cfg(ALOE_MSVC)]
    pub fn compare_ignore_case(&self, other: CharPointer_UTF16) -> i32 {
        
        todo!();
        /*
            return _wcsicmp (data, other.data);
        */
    }
    
    #[cfg(ALOE_MSVC)]
    pub fn compare_ignore_case_up_to(&self, 
        other:     CharPointer_UTF16,
        max_chars: i32) -> i32 {
        
        todo!();
        /*
            return _wcsnicmp (data, other.data, (size_t) maxChars);
        */
    }
    
    #[cfg(ALOE_MSVC)]
    pub fn index_of_utf16(&self, string_to_find: CharPointer_UTF16) -> i32 {
        
        todo!();
        /*
            const CharType* const t = wcsstr (data, stringToFind.getAddress());
            return t == nullptr ? -1 : (int) (t - data);
        */
    }

    /**
      | Returns the character index of a substring,
      | or -1 if it isn't found.
      |
      */
    pub fn index_of_substr<CharPointer>(&self, string_to_find: CharPointer) -> i32 {
    
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
            return CharacterFunctions::indexOfChar (*this, charToFind);
        */
    }

    /**
      | Returns the character index of a unicode
      | character, or -1 if it isn't found.
      |
      */
    pub fn index_of_char_with_ignorecase_flag(
        &self, 
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
            return CharacterFunctions::isWhitespace (operator*()) != 0;
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
            return CharacterFunctions::isDigit (operator*()) != 0;
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
            return CharacterFunctions::isLetter (operator*()) != 0;
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
            return CharacterFunctions::isLetterOrDigit (operator*()) != 0;
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
            return CharacterFunctions::isUpperCase (operator*()) != 0;
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
            return CharacterFunctions::isLowerCase (operator*()) != 0;
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
            return CharacterFunctions::toUpperCase (operator*());
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
            return CharacterFunctions::toLowerCase (operator*());
        */
    }

    /**
      | Parses this string as a 32-bit integer.
      |
      */
    pub fn get_int_value32(&self) -> i32 {
        
        todo!();
        /*
            #if ALOE_MSVC
            return _wtoi (data);
           #else
            return CharacterFunctions::getIntValue<int, CharPointer_UTF16> (*this);
           #endif
        */
    }

    /**
      | Parses this string as a 64-bit integer.
      |
      */
    pub fn get_int_value64(&self) -> i64 {
        
        todo!();
        /*
            #if ALOE_MSVC
            return _wtoi64 (data);
           #else
            return CharacterFunctions::getIntValue<int64, CharPointer_UTF16> (*this);
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
    pub fn find_end_of_whitespace(&self) -> CharPointer_UTF16 {
        
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
            auto n = (uint32) character;
            return n < 0x10ffff && (n < 0xd800 || n > 0xdfff);
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
            {
                auto n = (uint32) (uint16) *dataToTest++;

                if (n >= 0xd800)
                {
                    if (n > 0x10ffff)
                        return false;

                    if (n <= 0xdfff)
                    {
                        if (n > 0xdc00)
                            return false;

                        auto nextChar = (uint32) (uint16) *dataToTest++;

                        if (nextChar < 0xdc00 || nextChar > 0xdfff)
                            return false;
                    }
                }
            }

            return true;
        */
    }

    /**
      | Atomically swaps this pointer for a
      | new value, returning the previous value.
      |
      */
    pub fn atomic_swap(&mut self, new_value: CharPointer_UTF16) -> CharPointer_UTF16 {
        
        todo!();
        /*
            return CharPointer_UTF16 (reinterpret_cast<Atomic<CharType*>&> (data).exchange (newValue.data));
        */
    }

    /**
      | Returns true if the first pair of bytes
      | in this pointer are the UTF16 byte-order
      | mark (big endian).
      | 
      | The pointer must not be null, and must
      | contain at least two valid bytes.
      |
      */
    pub fn is_byte_order_mark_big_endian(possible_byte_order: *const c_void) -> bool {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (28182)
            jassert (possibleByteOrder != nullptr);
            auto c = static_cast<const uint8*> (possibleByteOrder);

            return c[0] == (uint8) byteOrderMarkBE1
                && c[1] == (uint8) byteOrderMarkBE2;
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    /**
      | Returns true if the first pair of bytes
      | in this pointer are the UTF16 byte-order
      | mark (little endian).
      | 
      | The pointer must not be null, and must
      | contain at least two valid bytes.
      |
      */
    pub fn is_byte_order_mark_little_endian(possible_byte_order: *const c_void) -> bool {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (28182)
            jassert (possibleByteOrder != nullptr);
            auto c = static_cast<const uint8*> (possibleByteOrder);

            return c[0] == (uint8) byteOrderMarkLE1
                && c[1] == (uint8) byteOrderMarkLE2;
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    pub fn find_null_index(t: *const <Self as HasCharType>::CharType) -> u32 {
        
        todo!();
        /*
            unsigned int n = 0;

            while (t[n] != 0)
                ++n;

            return n;
        */
    }
}
