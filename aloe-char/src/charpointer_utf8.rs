crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_CharPointer_UTF8.h]

/**
  | Wraps a pointer to a null-terminated
  | UTF-8 character string, and provides
  | various methods to operate on the data.
  | @see CharPointer_UTF16, CharPointer_UTF32
  | 
  | @tags{Core}
  |
  */
pub struct CharPointer_UTF8 {
    data: *mut <Self as HasCharType>::CharType,
}

pub mod char_pointer_utf8 {

    /**
      | These values are the byte-order mark
      | (BOM) values for a UTF-8 stream.
      |
      */
    pub const byteOrderMark1: u8 = 0xef;
    pub const byteOrderMark2: u8 = 0xbb;
    pub const byteOrderMark3: u8 = 0xbf;
}

impl HasCharType for CharPointer_UTF8 {
    type CharType = u8;
}

impl PartialEq<CharPointer_UTF8> for CharPointer_UTF8 {
    
    /**
      | This is a pointer comparison, it doesn't
      | compare the actual text.
      |
      */
    #[inline] fn eq(&self, other: &CharPointer_UTF8) -> bool {
        todo!();
        /*
            return data == other.data;
        */
    }
}

impl Eq for CharPointer_UTF8 {}

impl Ord for CharPointer_UTF8 {
    
    #[inline] fn cmp(&self, other: &CharPointer_UTF8) -> std::cmp::Ordering {
        todo!();
        /*
            return data <  other.data;
        */
    }
}

impl PartialOrd<CharPointer_UTF8> for CharPointer_UTF8 {

    #[inline] fn partial_cmp(&self, other: &CharPointer_UTF8) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<<Self as HasCharType>::CharType> for CharPointer_UTF8 {
    
    /**
      | Returns the address that this pointer
      | is pointing to.
      |
      */
    #[inline] fn into(self) -> <Self as HasCharType>::CharType {
        todo!();
        /*
            return data;
        */
    }
}

impl Deref for CharPointer_UTF8 {

    type Target = wchar_t;

    /**
      | Returns the unicode character that
      | this pointer is pointing to.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            auto byte = (signed char) *data;

            if (byte >= 0)
                return (aloe_wchar) (uint8) byte;

            uint32 n = (uint32) (uint8) byte;
            uint32 mask = 0x7f;
            uint32 bit = 0x40;
            int numExtraValues = 0;

            while ((n & bit) != 0 && bit > 0x8)
            {
                mask >>= 1;
                ++numExtraValues;
                bit >>= 1;
            }

            n &= mask;

            for (int i = 1; i <= numExtraValues; ++i)
            {
                auto nextByte = (uint32) (uint8) data[i];

                if ((nextByte & 0xc0) != 0x80)
                    break;

                n <<= 6;
                n |= (nextByte & 0x3f);
            }

            return (aloe_wchar) n;
        */
    }
}

impl AddAssign<&i32> for CharPointer_UTF8 {
    
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

impl SubAssign<i32> for CharPointer_UTF8 {
    
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

impl Index<usize> for CharPointer_UTF8 {

    //type Output = wchar_t;
    type Output = char;
    
    /**
      | Returns the character at a given character
      | index from the start of the string.
      |
      */
    #[inline] fn index(&self, character_index: usize) -> &Self::Output {
        todo!();
        /*
            auto p (*this);
            p += characterIndex;
            return *p;
        */
    }
}

impl Add<&CharPointer_UTF8> for CharPointer_UTF8 {

    type Output = Self;
    
    /**
      | Returns a pointer which is moved forwards
      | from this one by the specified number
      | of characters.
      |
      */
    #[inline] fn add(self, other: &CharPointer_UTF8) -> Self::Output {
        todo!();
        /*
            auto p (*this);
            p += numToSkip;
            return p;
        */
    }
}

impl Sub<i32> for CharPointer_UTF8 {

    type Output = CharPointer_UTF8;
    
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

impl CharPointer_UTF8 {

    pub fn new(raw_pointer: *const <Self as HasCharType>::CharType) -> Self {
    
        todo!();
        /*


            : data (const_cast<CharType*> (rawPointer))
        */
    }
    
    pub fn assign_from_utf8(&mut self, other: CharPointer_UTF8) -> CharPointer_UTF8 {
        
        todo!();
        /*
            data = other.data;
            return *this;
        */
    }
    
    pub fn assign_from(&mut self, text: *const <Self as HasCharType>::CharType) -> CharPointer_UTF8 {
        
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
    pub fn get_address(&self) -> *mut <Self as HasCharType>::CharType {
        
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
    pub fn is_empty(&self) -> bool {
        
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
    pub fn is_not_empty(&self) -> bool {
        
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
    pub fn prefix_increment(&mut self) -> &mut CharPointer_UTF8 {
        
        todo!();
        /*
            jassert (*data != 0); // trying to advance past the end of the string?
            auto n = (signed char) *data++;

            if (n < 0)
            {
                uint8 bit = 0x40;

                while ((static_cast<uint8> (n) & bit) != 0 && bit > 0x8)
                {
                    ++data;
                    bit = static_cast<uint8> (bit >> 1);
                }
            }

            return *this;
        */
    }

    /**
      | Moves this pointer back to the previous
      | character in the string.
      |
      */
    pub fn prefix_decrement(&mut self) -> CharPointer_UTF8 {
        
        todo!();
        /*
            int count = 0;

            while ((*--data & 0xc0) == 0x80 && ++count < 4)
            {}

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
            auto byte = (signed char) *data++;

            if (byte >= 0)
                return (aloe_wchar) (uint8) byte;

            uint32 n = (uint32) (uint8) byte;
            uint32 mask = 0x7f;
            uint32 bit = 0x40;
            int numExtraValues = 0;

            while ((n & bit) != 0 && bit > 0x8)
            {
                mask >>= 1;
                ++numExtraValues;
                bit >>= 1;
            }

            n &= mask;

            while (--numExtraValues >= 0)
            {
                auto nextByte = (uint32) (uint8) *data;

                if ((nextByte & 0xc0) != 0x80)
                    break;

                ++data;
                n <<= 6;
                n |= (nextByte & 0x3f);
            }

            return (aloe_wchar) n;
        */
    }

    /**
      | Moves this pointer along to the next
      | character in the string.
      |
      */
    pub fn postfix_increment(&mut self) -> CharPointer_UTF8 {
        
        todo!();
        /*
            CharPointer_UTF8 temp (*this);
            ++*this;
            return temp;
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
                auto n = (uint32) (uint8) *d++;

                if ((n & 0x80) != 0)
                {
                    while ((*d & 0xc0) == 0x80)
                        ++d;
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
    pub fn length_up_to(&self, end: CharPointer_UTF8) -> usize {
        
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
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6387)
            jassert (data != nullptr);
            return strlen (data) + 1;
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }

    /**
      | Returns the number of bytes that would
      | be needed to represent the given unicode
      | character in this encoding format.
      |
      */
    pub fn get_bytes_required_for(char_to_write: wchar_t) -> usize {
        
        todo!();
        /*
            size_t num = 1;
            auto c = (uint32) charToWrite;

            if (c >= 0x80)
            {
                ++num;
                if (c >= 0x800)
                {
                    ++num;
                    if (c >= 0x10000)
                        ++num;
                }
            }

            return num;
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
    pub fn get_bytes_required_for_text<CharPointer>(text: CharPointer) -> usize {
    
        todo!();
        /*
            size_t count = 0;

            while (auto n = text.getAndAdvance())
                count += getBytesRequiredFor (n);

            return count;
        */
    }

    /**
      | Returns a pointer to the null character
      | that terminates this string.
      |
      */
    pub fn find_terminating_null(&self) -> CharPointer_UTF8 {
        
        todo!();
        /*
            return CharPointer_UTF8 (data + strlen (data));
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
            auto c = (uint32) charToWrite;

            if (c >= 0x80)
            {
                int numExtraBytes = 1;
                if (c >= 0x800)
                {
                    ++numExtraBytes;
                    if (c >= 0x10000)
                        ++numExtraBytes;
                }

                *data++ = (CharType) ((uint32) (0xff << (7 - numExtraBytes)) | (c >> (numExtraBytes * 6)));

                while (--numExtraBytes >= 0)
                    *data++ = (CharType) (0x80 | (0x3f & (c >> (numExtraBytes * 6))));
            }
            else
            {
                *data++ = (CharType) c;
            }
        */
    }

    /**
      | Writes a null character to this string
      | (leaving the pointer's position unchanged).
      |
      */
    pub fn write_null(&self)  {
        
        todo!();
        /*
            *data = 0;
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
    pub fn write_all_utf8(&mut self, src: CharPointer_UTF8)  {
        
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
      | Compares this string with another one.
      |
      */
    pub fn compare_ignore_case_utf8(&self, other: CharPointer_UTF8) -> i32 {
        
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
            return CharacterFunctions::indexOfChar (*this, charToFind);
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
            const CharType c = *data; return c == ' ' || (c <= 13 && c >= 9);
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
            const CharType c = *data; return c >= '0' && c <= '9';
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
            #if ALOE_WINDOWS && ! ALOE_MINGW
            return _atoi64 (data);
           #else
            return atoll (data);
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
    pub fn find_end_of_whitespace(&self) -> CharPointer_UTF8 {
        
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
            while (--maxBytesToRead >= 0 && *dataToTest != 0)
            {
                auto byte = (signed char) *dataToTest++;

                if (byte < 0)
                {
                    int bit = 0x40;
                    int numExtraValues = 0;

                    while ((byte & bit) != 0)
                    {
                        if (bit < 8)
                            return false;

                        ++numExtraValues;
                        bit >>= 1;

                        if (bit == 8 && (numExtraValues > maxBytesToRead
                                           || *CharPointer_UTF8 (dataToTest - 1) > 0x10ffff))
                            return false;
                    }

                    if (numExtraValues == 0)
                        return false;

                    maxBytesToRead -= numExtraValues;
                    if (maxBytesToRead < 0)
                        return false;

                    while (--numExtraValues >= 0)
                        if ((*dataToTest++ & 0xc0) != 0x80)
                            return false;
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
    pub fn atomic_swap(&mut self, new_value: CharPointer_UTF8) -> CharPointer_UTF8 {
        
        todo!();
        /*
            return CharPointer_UTF8 (reinterpret_cast<Atomic<CharType*>&> (data).exchange (newValue.data));
        */
    }

    /**
      | Returns true if the first three bytes
      | in this pointer are the UTF8 byte-order
      | mark (BOM).
      | 
      | The pointer must not be null, and must
      | point to at least 3 valid bytes.
      |
      */
    pub fn is_byte_order_mark(possible_byte_order: *const c_void) -> bool {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (28182)
            jassert (possibleByteOrder != nullptr);
            auto c = static_cast<const uint8*> (possibleByteOrder);

            return c[0] == (uint8) byteOrderMark1
                && c[1] == (uint8) byteOrderMark2
                && c[2] == (uint8) byteOrderMark3;
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }
}
