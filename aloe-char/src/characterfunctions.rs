crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_CharacterFunctions.h]

/*
#[feature(unboxed_closures, fn_traits)]
overloadable::overloadable! {
    pub make_unsigned as
    #[inline]
    fn(x: isize) -> usize { x as _ },
    #[inline] 
    fn(x: i8) -> u8 { x as _ },
    #[inline]
    fn(x: i16) -> u16 { x as _ },
    #[inline]
    fn(x: i32) -> i32 { x as _ },
    #[inline]
    fn(x: i64) -> u64 { x as _ },
    #[inline]
    fn(x: i128) -> u128 { x as _ }
}
*/

/**
  | A collection of functions for manipulating
  | characters and character strings.
  | 
  | Most of these methods are designed for
  | internal use by the String and CharPointer
  | classes, but some of them may be useful
  | to call directly.
  | 
  | @see String, CharPointer_UTF8, CharPointer_UTF16,
  | CharPointer_UTF32
  | 
  | @tags{Core}
  |
  */
pub struct CharacterFunctions {

}

pub mod character_functions {

    use super::*;

    /**
      | Parses a character string, to read a
      | hexadecimal value.
      |
      */
    pub struct HexParser {

    }

    impl HexParser {

        pub fn parse<CharPointerType,ResultType>(t: CharPointerType) -> ResultType {
        
            todo!();
            /*
                ResultType result = 0;

                    while (! t.isEmpty())
                    {
                        auto hexValue = CharacterFunctions::getHexDigitValue (t.getAndAdvance());

                        if (hexValue >= 0)
                            result = (result << 4) | hexValue;
                    }

                    return result;
            */
        }
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_CharacterFunctions.cpp]
impl CharacterFunctions {

    /**
      | Parses a character string to read a floating-point
      | number.
      | 
      | -----------
      | @note
      | 
      | this will advance the pointer that is
      | passed in, leaving it at the end of the
      | number.
      |
      */
    pub fn read_double_value<CharPointerType>(text: &mut CharPointerType) -> f64 {
    
        todo!();
        /*
            constexpr auto inf = std::numeric_limits<double>::infinity();

            bool isNegative = false;
           #if ! ALOE_MINGW
            constexpr const int maxSignificantDigits = 17 + 1; // An additional digit for rounding
            constexpr const int bufferSize = maxSignificantDigits + 7 + 1; // -.E-XXX and a trailing null-terminator
            char buffer[(size_t) bufferSize] = {};
            char* writePtr = &(buffer[0]);
           #endif

            const auto endOfWhitspace = text.findEndOfWhitespace();
            text = endOfWhitspace;

            auto c = *text;

            switch (c)
            {
                case '-':
                    isNegative = true;
                   #if ! ALOE_MINGW
                    *writePtr++ = '-';
                   #endif
                    ALOE_FALLTHROUGH
                case '+':
                    c = *++text;
                    break;
                default:
                    break;
            }

            switch (c)
            {
                case 'n':
                case 'N':
                {
                    if ((text[1] == 'a' || text[1] == 'A') && (text[2] == 'n' || text[2] == 'N'))
                    {
                        text += 3;
                        return std::numeric_limits<double>::quiet_NaN();
                    }

                    text = endOfWhitspace;
                    return 0.0;
                }

                case 'i':
                case 'I':
                {
                    if ((text[1] == 'n' || text[1] == 'N') && (text[2] == 'f' || text[2] == 'F'))
                    {
                        text += 3;
                        return isNegative ? -inf : inf;
                    }

                    text = endOfWhitspace;
                    return 0.0;
                }

                default:
                    break;
            }

           #if ALOE_MINGW
            // MinGW does not have access to the locale functions required for strtold, so we parse the doubles
            // ourselves. There are some edge cases where the least significant digit will be wrong!
            double result[3] = { 0 }, accumulator[2] = { 0 };
            int exponentAdjustment[2] = { 0 }, exponentAccumulator[2] = { -1, -1 };
            int exponent = 0, decPointIndex = 0, digit = 0;
            int lastDigit = 0, numSignificantDigits = 0;
            bool digitsFound = false;
            constexpr const int maxSignificantDigits = 17 + 1;

            for (;;)
            {
                if (text.isDigit())
                {
                    lastDigit = digit;
                    digit = (int) text.getAndAdvance() - '0';
                    digitsFound = true;

                    if (decPointIndex != 0)
                        exponentAdjustment[1]++;

                    if (numSignificantDigits == 0 && digit == 0)
                        continue;

                    if (++numSignificantDigits > maxSignificantDigits)
                    {
                        if (digit > 5)
                            ++accumulator [decPointIndex];
                        else if (digit == 5 && (lastDigit & 1) != 0)
                            ++accumulator [decPointIndex];

                        if (decPointIndex > 0)
                            exponentAdjustment[1]--;
                        else
                            exponentAdjustment[0]++;

                        while (text.isDigit())
                        {
                            ++text;
                            if (decPointIndex == 0)
                                exponentAdjustment[0]++;
                        }
                    }
                    else
                    {
                        const auto maxAccumulatorValue = (double) ((std::numeric_limits<unsigned int>::max() - 9) / 10);
                        if (accumulator [decPointIndex] > maxAccumulatorValue)
                        {
                            result [decPointIndex] = mulexp10 (result [decPointIndex], exponentAccumulator [decPointIndex])
                                                     + accumulator [decPointIndex];
                            accumulator [decPointIndex] = 0;
                            exponentAccumulator [decPointIndex] = 0;
                        }

                        accumulator [decPointIndex] = accumulator[decPointIndex] * 10 + digit;
                        exponentAccumulator [decPointIndex]++;
                    }
                }
                else if (decPointIndex == 0 && *text == '.')
                {
                    ++text;
                    decPointIndex = 1;

                    if (numSignificantDigits > maxSignificantDigits)
                    {
                        while (text.isDigit())
                            ++text;
                        break;
                    }
                }
                else
                {
                    break;
                }
            }

            result[0] = mulexp10 (result[0], exponentAccumulator[0]) + accumulator[0];

            if (decPointIndex != 0)
                result[1] = mulexp10 (result[1], exponentAccumulator[1]) + accumulator[1];

            c = *text;
            if ((c == 'e' || c == 'E') && digitsFound)
            {
                auto negativeExponent = false;

                switch (*++text)
                {
                    case '-':   negativeExponent = true; ALOE_FALLTHROUGH
                    case '+':   ++text;
                }

                while (text.isDigit())
                    exponent = (exponent * 10) + ((int) text.getAndAdvance() - '0');

                if (negativeExponent)
                    exponent = -exponent;
            }

            auto r = mulexp10 (result[0], exponent + exponentAdjustment[0]);
            if (decPointIndex != 0)
                r += mulexp10 (result[1], exponent - exponentAdjustment[1]);

            return isNegative ? -r : r;

           #else   // ! ALOE_MINGW

            int numSigFigs = 0, extraExponent = 0;
            bool decimalPointFound = false, leadingZeros = false;

            for (;;)
            {
                if (text.isDigit())
                {
                    auto digit = (int) text.getAndAdvance() - '0';

                    if (decimalPointFound)
                    {
                        if (numSigFigs >= maxSignificantDigits)
                            continue;
                    }
                    else
                    {
                        if (numSigFigs >= maxSignificantDigits)
                        {
                            ++extraExponent;
                            continue;
                        }

                        if (numSigFigs == 0 && digit == 0)
                        {
                            leadingZeros = true;
                            continue;
                        }
                    }

                    *writePtr++ = (char) ('0' + (char) digit);
                    numSigFigs++;
                }
                else if ((! decimalPointFound) && *text == '.')
                {
                    ++text;
                    *writePtr++ = '.';
                    decimalPointFound = true;
                }
                else
                {
                    break;
                }
            }

            if ((! leadingZeros) && (numSigFigs == 0))
            {
                text = endOfWhitspace;
                return 0.0;
            }

            auto writeExponentDigits = [] (int exponent, char* destination)
            {
                auto exponentDivisor = 100;

                while (exponentDivisor > 1)
                {
                    auto digit = exponent / exponentDivisor;
                    *destination++ = (char) ('0' + (char) digit);
                    exponent -= digit * exponentDivisor;
                    exponentDivisor /= 10;
                }

                *destination++ = (char) ('0' + (char) exponent);
            };

            c = *text;

            if (c == 'e' || c == 'E')
            {
                const auto startOfExponent = text;
                *writePtr++ = 'e';
                bool parsedExponentIsPositive = true;

                switch (*++text)
                {
                    case '-':
                        parsedExponentIsPositive = false;
                        ALOE_FALLTHROUGH
                    case '+':
                        ++text;
                        break;
                    default:
                        break;
                }

                int exponent = 0;
                const auto startOfExponentDigits = text;

                while (text.isDigit())
                {
                    auto digit = (int) text.getAndAdvance() - '0';

                    if (digit != 0 || exponent != 0)
                        exponent = (exponent * 10) + digit;
                }

                if (text == startOfExponentDigits)
                    text = startOfExponent;

                exponent = extraExponent + (parsedExponentIsPositive ? exponent : -exponent);

                if (exponent < 0)
                {
                    if (exponent < std::numeric_limits<double>::min_exponent10 - 1)
                        return isNegative ? -0.0 : 0.0;

                    *writePtr++ = '-';
                    exponent = -exponent;
                }
                else if (exponent > std::numeric_limits<double>::max_exponent10 + 1)
                {
                    return isNegative ? -inf : inf;
                }

                writeExponentDigits (exponent, writePtr);
            }
            else if (extraExponent > 0)
            {
                *writePtr++ = 'e';
                writeExponentDigits (extraExponent, writePtr);
            }

           #if ALOE_WINDOWS
            static _locale_t locale = _create_locale (LC_ALL, "C");
            return _strtod_l (&buffer[0], nullptr, locale);
           #else
            static locale_t locale = newlocale (LC_ALL_MASK, "C", nullptr);
            #if ALOE_ANDROID
            return (double) strtold_l (&buffer[0], nullptr, locale);
            #else
            return strtod_l (&buffer[0], nullptr, locale);
            #endif
           #endif

           #endif   // ALOE_MINGW
        */
    }

    /**
      | Parses a character string, to read a
      | floating-point value.
      |
      */
    pub fn get_double_value<CharPointerType>(text: CharPointerType) -> f64 {
    
        todo!();
        /*
            return readDoubleValue (text);
        */
    }

    /**
      | Parses a character string, to read an
      | integer value.
      |
      */
    pub fn get_int_value<IntType, CharPointerType>(text: CharPointerType) -> IntType {
    
        todo!();
        /*
            using UIntType = typename internal::make_unsigned<IntType>::type;

            UIntType v = 0;
            auto s = text.findEndOfWhitespace();
            const bool isNeg = *s == '-';

            if (isNeg)
                ++s;

            for (;;)
            {
                auto c = s.getAndAdvance();

                if (c >= '0' && c <= '9')
                    v = v * 10 + (UIntType) (c - '0');
                else
                    break;
            }

            return isNeg ? - (IntType) v : (IntType) v;
        */
    }
    
    /**
      | Counts the number of characters in a
      | given string, stopping if the count
      | exceeds a specified limit.
      |
      */
    pub fn length_up_to_max<CharPointerType>(
        text:               CharPointerType,
        max_chars_to_count: usize) -> usize {
    
        todo!();
        /*
            size_t len = 0;

            while (len < maxCharsToCount && text.getAndAdvance() != 0)
                ++len;

            return len;
        */
    }

    /**
      | Counts the number of characters in a
      | given string, stopping if the count
      | exceeds a specified end-pointer.
      |
      */
    pub fn length_up_to<CharPointerType>(
        start: CharPointerType,
        end:   CharPointerType) -> usize {
    
        todo!();
        /*
            size_t len = 0;

            while (start < end && start.getAndAdvance() != 0)
                ++len;

            return len;
        */
    }

    /**
      | Copies null-terminated characters
      | from one string to another.
      |
      */
    pub fn copy_all<DestCharPointerType, SrcCharPointerType>(
        dest: &mut DestCharPointerType,
        src:  SrcCharPointerType)  {
    
        todo!();
        /*
            while (auto c = src.getAndAdvance())
                dest.write (c);

            dest.writeNull();
        */
    }

    /**
      | Copies characters from one string to
      | another, up to a null terminator or a
      | given byte size limit.
      |
      */
    pub fn copy_with_dest_byte_limit<DestCharPointerType, SrcCharPointerType>(
        dest:               &mut DestCharPointerType,
        src:                SrcCharPointerType,
        max_bytes_to_write: usize) -> usize {
    
        todo!();
        /*
            auto startAddress = dest.getAddress();
            auto maxBytes = (ssize_t) maxBytesToWrite;
            maxBytes -= (ssize_t) sizeof (typename DestCharPointerType::CharType); // (allow for a terminating null)

            for (;;)
            {
                auto c = src.getAndAdvance();
                auto bytesNeeded = (ssize_t) DestCharPointerType::getBytesRequiredFor (c);
                maxBytes -= bytesNeeded;

                if (c == 0 || maxBytes < 0)
                    break;

                dest.write (c);
            }

            dest.writeNull();

            return (size_t) getAddressDifference (dest.getAddress(), startAddress)
                     + sizeof (typename DestCharPointerType::CharType);
        */
    }

    /**
      | Copies characters from one string to
      | another, up to a null terminator or a
      | given maximum number of characters.
      |
      */
    pub fn copy_with_char_limit<DestCharPointerType, SrcCharPointerType>(
        dest:      &mut DestCharPointerType,
        src:       SrcCharPointerType,
        max_chars: i32)  {
    
        todo!();
        /*
            while (--maxChars > 0)
            {
                auto c = src.getAndAdvance();

                if (c == 0)
                    break;

                dest.write (c);
            }

            dest.writeNull();
        */
    }

    /**
      | Compares two characters.
      |
      */
    pub fn compare_characters(
        char1: wchar_t,
        char2: wchar_t) -> i32 {
        
        todo!();
        /*
            if (auto diff = static_cast<int> (char1) - static_cast<int> (char2))
                return diff < 0 ? -1 : 1;

            return 0;
        */
    }

    /**
      | Compares two null-terminated character
      | strings.
      |
      */
    pub fn compare_strings<CharPointerType1, CharPointerType2>(
        s1: CharPointerType1,
        s2: CharPointerType2) -> i32 {
    
        todo!();
        /*
            for (;;)
            {
                auto c1 = s1.getAndAdvance();

                if (auto diff = compare (c1, s2.getAndAdvance()))
                    return diff;

                if (c1 == 0)
                    break;
            }

            return 0;
        */
    }

    /**
      | Compares two null-terminated character
      | strings, up to a given number of characters.
      |
      */
    pub fn compare_up_to<CharPointerType1, CharPointerType2>(
        s1:        CharPointerType1,
        s2:        CharPointerType2,
        max_chars: i32) -> i32 {
    
        todo!();
        /*
            while (--maxChars >= 0)
            {
                auto c1 = s1.getAndAdvance();

                if (auto diff = compare (c1, s2.getAndAdvance()))
                    return diff;

                if (c1 == 0)
                    break;
            }

            return 0;
        */
    }

    /**
      | Compares two characters, using a case-independant
      | match.
      |
      */
    pub fn compare_wchar_ignore_case(
        char1: wchar_t,
        char2: wchar_t) -> i32 {
        
        todo!();
        /*
            return char1 != char2 ? compare (toUpperCase (char1), toUpperCase (char2)) : 0;
        */
    }

    /**
      | Compares two null-terminated character
      | strings, using a case-independant
      | match.
      |
      */
    pub fn compare_strings_ignore_case<CharPointerType1, CharPointerType2>(
        s1: CharPointerType1,
        s2: CharPointerType2) -> i32 {
    
        todo!();
        /*
            for (;;)
            {
                auto c1 = s1.getAndAdvance();

                if (auto diff = compareIgnoreCase (c1, s2.getAndAdvance()))
                    return diff;

                if (c1 == 0)
                    break;
            }

            return 0;
        */
    }

    /**
      | Compares two null-terminated character
      | strings, using a case-independent
      | match.
      |
      */
    pub fn compare_ignore_case_up_to<CharPointerType1, CharPointerType2>(
        s1:        CharPointerType1,
        s2:        CharPointerType2,
        max_chars: i32) -> i32 {
    
        todo!();
        /*
            while (--maxChars >= 0)
            {
                auto c1 = s1.getAndAdvance();

                if (auto diff = compareIgnoreCase (c1, s2.getAndAdvance()))
                    return diff;

                if (c1 == 0)
                    break;
            }

            return 0;
        */
    }

    /**
      | Finds the character index of a given
      | substring in another string.
      | 
      | Returns -1 if the substring is not found.
      |
      */
    pub fn index_of<CharPointerType1, CharPointerType2>(
        text_to_search:        CharPointerType1,
        substring_to_look_for: CharPointerType2) -> i32 {
    
        todo!();
        /*
            int index = 0;
            auto substringLength = (int) substringToLookFor.length();

            for (;;)
            {
                if (textToSearch.compareUpTo (substringToLookFor, substringLength) == 0)
                    return index;

                if (textToSearch.getAndAdvance() == 0)
                    return -1;

                ++index;
            }
        */
    }

    /**
      | Returns a pointer to the first occurrence
      | of a substring in a string.
      | 
      | If the substring is not found, this will
      | return a pointer to the string's null
      | terminator.
      |
      */
    pub fn find_substr<CharPointerType1, CharPointerType2>(
        text_to_search:        CharPointerType1,
        substring_to_look_for: CharPointerType2) -> CharPointerType1 {
    
        todo!();
        /*
            auto substringLength = (int) substringToLookFor.length();

            while (textToSearch.compareUpTo (substringToLookFor, substringLength) != 0
                     && ! textToSearch.isEmpty())
                ++textToSearch;

            return textToSearch;
        */
    }

    /**
      | Returns a pointer to the first occurrence
      | of a substring in a string.
      | 
      | If the substring is not found, this will
      | return a pointer to the string's null
      | terminator.
      |
      */
    pub fn find_char<CharPointerType>(
        text_to_search:   &CharPointerType,
        char_to_look_for: char

    ) -> CharPointerType {
    
        todo!();
        /*
            for (;; ++textToSearch)
            {
                auto c = *textToSearch;

                if (c == charToLookFor || c == 0)
                    break;
            }

            return textToSearch;
        */
    }

    /**
      | Finds the character index of a given
      | substring in another string, using
      | a case-independent match.
      | 
      | Returns -1 if the substring is not found.
      |
      */
    pub fn index_of_ignore_case<CharPointerType1, CharPointerType2>(
        haystack: CharPointerType1,
        needle:   CharPointerType2) -> i32 {
    
        todo!();
        /*
            int index = 0;
            auto needleLength = (int) needle.length();

            for (;;)
            {
                if (haystack.compareIgnoreCaseUpTo (needle, needleLength) == 0)
                    return index;

                if (haystack.getAndAdvance() == 0)
                    return -1;

                ++index;
            }
        */
    }

    /**
      | Finds the character index of a given
      | character in another string.
      | 
      | Returns -1 if the character is not found.
      |
      */
    pub fn index_of_char<Type>(
        text:         Type,
        char_to_find: wchar_t) -> i32 {
    
        todo!();
        /*
            int i = 0;

            while (! text.isEmpty())
            {
                if (text.getAndAdvance() == charToFind)
                    return i;

                ++i;
            }

            return -1;
        */
    }

    /**
      | Finds the character index of a given
      | character in another string, using
      | a case-independent match.
      | 
      | Returns -1 if the character is not found.
      |
      */
    pub fn index_of_char_ignore_case<Type>(
        text:         Type,
        char_to_find: wchar_t) -> i32 {
    
        todo!();
        /*
            charToFind = CharacterFunctions::toLowerCase (charToFind);
            int i = 0;

            while (! text.isEmpty())
            {
                if (text.toLowerCase() == charToFind)
                    return i;

                ++text;
                ++i;
            }

            return -1;
        */
    }

    /**
      | Increments a pointer until it points
      | to the first non-whitespace character
      | in a string.
      | 
      | If the string contains only whitespace,
      | the pointer will point to the string's
      | null terminator.
      |
      */
    pub fn increment_to_end_of_whitespace<Type>(text: &mut Type)  {
    
        todo!();
        /*
            while (text.isWhitespace())
                ++text;
        */
    }

    /**
      | Returns a pointer to the first non-whitespace
      | character in a string.
      | 
      | If the string contains only whitespace,
      | this will return a pointer to its null
      | terminator.
      |
      */
    pub fn find_end_of_whitespace<Type>(text: Type) -> Type {
    
        todo!();
        /*
            incrementToEndOfWhitespace (text);
            return text;
        */
    }

    /**
      | Returns a pointer to the first character
      | in the string which is found in the breakCharacters
      | string.
      |
      */
    pub fn find_end_of_token<Type, BreakType>(
        text:             Type,
        break_characters: BreakType,
        quote_characters: Type) -> Type {
    
        todo!();
        /*
            aloe_wchar currentQuoteChar = 0;

            while (! text.isEmpty())
            {
                auto c = text.getAndAdvance();

                if (currentQuoteChar == 0 && breakCharacters.indexOf (c) >= 0)
                {
                    --text;
                    break;
                }

                if (quoteCharacters.indexOf (c) >= 0)
                {
                    if (currentQuoteChar == 0)
                        currentQuoteChar = c;
                    else if (currentQuoteChar == c)
                        currentQuoteChar = 0;
                }
            }

            return text;
        */
    }
    
    /**
      | Converts a character to upper-case.
      |
      */
    pub fn to_upper_case(&mut self, character: wchar_t) -> wchar_t {
        
        todo!();
        /*
            return (aloe_wchar) towupper ((wint_t) character);
        */
    }
    
    /**
      | Converts a character to lower-case.
      |
      */
    pub fn to_lower_case(&mut self, character: wchar_t) -> wchar_t {
        
        todo!();
        /*
            return (aloe_wchar) towlower ((wint_t) character);
        */
    }
    
    /**
      | Checks whether a unicode character
      | is upper-case.
      |
      */
    pub fn is_upper_case(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            #if ALOE_WINDOWS
        return iswupper ((wint_t) character) != 0;
       #else
        return toLowerCase (character) != character;
       #endif
        */
    }

    /**
      | Checks whether a unicode character
      | is lower-case.
      |
      */
    pub fn is_lower_case(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            #if ALOE_WINDOWS
        return iswlower ((wint_t) character) != 0;
       #else
        return toUpperCase (character) != character;
       #endif
        */
    }

    /**
      | Checks whether a character is whitespace.
      |
      */
    pub fn is_u8_whitespace(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return character == ' ' || (character <= 13 && character >= 9);
        */
    }

    /**
      | Checks whether a character is whitespace.
      |
      */
    pub fn is_wchar_whitespace(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            return iswspace ((wint_t) character) != 0;
        */
    }

    /**
      | Checks whether a character is a digit.
      |
      */
    pub fn u8_is_digit(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return (character >= '0' && character <= '9');
        */
    }

    /**
      | Checks whether a character is a digit.
      |
      */
    pub fn char_is_digit(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            return iswdigit ((wint_t) character) != 0;
        */
    }

    /**
      | Checks whether a character is alphabetic.
      |
      */
    pub fn is_u8_letter(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return (character >= 'a' && character <= 'z')
            || (character >= 'A' && character <= 'Z');
        */
    }

    /**
      | Checks whether a character is alphabetic.
      |
      */
    pub fn is_wchar_letter(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            return iswalpha ((wint_t) character) != 0;
        */
    }

    /**
      | Checks whether a character is alphabetic
      | or numeric.
      |
      */
    pub fn is_u8_letter_or_digit(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return (character >= 'a' && character <= 'z')
            || (character >= 'A' && character <= 'Z')
            || (character >= '0' && character <= '9');
        */
    }

    /**
      | Checks whether a character is alphabetic
      | or numeric.
      |
      */
    pub fn is_wchar_letter_or_digit(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            return iswalnum ((wint_t) character) != 0;
        */
    }

    /**
      | Checks whether a character is a printable
      | character, i.e. alphabetic, numeric,
      | a punctuation character or a space.
      |
      */
    pub fn is_u8_printable(&mut self, character: u8) -> bool {
        
        todo!();
        /*
            return (character >= ' ' && character <= '~');
        */
    }

    /**
      | Checks whether a character is a printable
      | character, i.e. alphabetic, numeric,
      | a punctuation character or a space.
      |
      */
    pub fn is_wchar_printable(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            return iswprint ((wint_t) character) != 0;
        */
    }

    /**
      | Returns 0 to 16 for '0' to 'F", or -1 for
      | characters that aren't a legal hex digit.
      |
      */
    pub fn get_hex_digit_value(&mut self, digit: wchar_t) -> i32 {
        
        todo!();
        /*
            auto d = (unsigned int) (digit - '0');

        if (d < (unsigned int) 10)
            return (int) d;

        d += (unsigned int) ('0' - 'a');

        if (d < (unsigned int) 6)
            return (int) d + 10;

        d += (unsigned int) ('a' - 'A');

        if (d < (unsigned int) 6)
            return (int) d + 10;

        return -1;
        */
    }

    pub fn mulexp10(&mut self, 
        value:    f64,
        exponent: i32) -> f64 {
        
        todo!();
        /*
            if (exponent == 0)
            return value;

        if (value == 0.0)
            return 0;

        const bool negative = (exponent < 0);

        if (negative)
            exponent = -exponent;

        double result = 1.0, power = 10.0;

        for (int bit = 1; exponent != 0; bit <<= 1)
        {
            if ((exponent & bit) != 0)
            {
                exponent ^= bit;
                result *= power;

                if (exponent == 0)
                    break;
            }

            power *= power;
        }

        return negative ? (value / result) : (value * result);
        */
    }

    /**
      | Converts a byte of Windows 1252 codepage
      | to unicode.
      |
      */
    pub fn get_unicode_char_from_windows_1252codepage(&mut self, c: u8) -> wchar_t {
        
        todo!();
        /*
            if (c < 0x80 || c >= 0xa0)
            return (aloe_wchar) c;

        static const uint16 lookup[] = { 0x20AC, 0x0007, 0x201A, 0x0192, 0x201E, 0x2026, 0x2020, 0x2021,
                                         0x02C6, 0x2030, 0x0160, 0x2039, 0x0152, 0x0007, 0x017D, 0x0007,
                                         0x0007, 0x2018, 0x2019, 0x201C, 0x201D, 0x2022, 0x2013, 0x2014,
                                         0x02DC, 0x2122, 0x0161, 0x203A, 0x0153, 0x0007, 0x017E, 0x0178 };

        return (aloe_wchar) lookup[c - 0x80];
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod character_functions_tests {
    use super::*;

    macro_rules! quote {
        ($x:ident) => {
            /*
                    #x
            */
        }
    }

    macro_rules! str {
        ($value:ident) => {
            /*
                    QUOTE(value)
            */
        }
    }

    macro_rules! asym_charptr_double_pair {
        ($str:ident, $value:ident) => {
            /*
                    std::pair<const char*, double> (STR(str), value)
            */
        }
    }

    macro_rules! charptr_double_pair {
        ($value:ident) => {
            /*
                    ASYM_CHARPTR_DOUBLE_PAIR(value, value)
            */
        }
    }

    macro_rules! charptr_double_pair_combos {
        ($value:ident) => {
            /*
            
                CHARPTR_DOUBLE_PAIR(value), 
                CHARPTR_DOUBLE_PAIR(-value), 
                ASYM_CHARPTR_DOUBLE_PAIR(+value, value), 
                ASYM_CHARPTR_DOUBLE_PAIR(000000 ## value, value), 
                ASYM_CHARPTR_DOUBLE_PAIR(+000 ## value, value), 
                ASYM_CHARPTR_DOUBLE_PAIR(-0 ## value, -value)
            */
        }
    }

    pub fn memory_block_from_char_ptr<CharPointerType>(char_ptr: *const CharPointerType::CharType) -> MemoryBlock {

        todo!();
        /*
            using CharType = typename CharPointerType::CharType;

            MemoryBlock result;
            CharPointerType source (charPtr);

            result.setSize (CharPointerType::getBytesRequiredFor (source) + sizeof (CharType));
            CharPointerType dest { (CharType*) result.getData() };
            dest.writeAll (source);
            return result;
        */
    }

    pub fn convert<FromCharPointerType, ToCharPointerType>(
            source:                 &MemoryBlock,
            remove_null_terminator: bool) -> MemoryBlock {

        let remove_null_terminator: bool =
                 remove_null_terminator.unwrap_or(false);

        todo!();
        /*
            using ToCharType   = typename ToCharPointerType  ::CharType;
            using FromCharType = typename FromCharPointerType::CharType;

            FromCharPointerType sourcePtr { (FromCharType*) source.getData() };

            std::vector<aloe_wchar> sourceChars;
            size_t requiredSize = 0;
            aloe_wchar c;

            while ((c = sourcePtr.getAndAdvance()) != '\0')
            {
                requiredSize += ToCharPointerType::getBytesRequiredFor (c);
                sourceChars.push_back (c);
            }

            if (! removeNullTerminator)
                requiredSize += sizeof (ToCharType);

            MemoryBlock result;
            result.setSize (requiredSize);

            ToCharPointerType dest { (ToCharType*) result.getData() };

            for (auto wc : sourceChars)
                dest.write (wc);

            if (! removeNullTerminator)
                dest.writeNull();

            return result;
        */
    }

    pub struct SeparatorStrings {
        terminals: Vec<MemoryBlock>,
        nulls:     Vec<MemoryBlock>,
    }

    pub fn get_separators<CharPointerType>() -> SeparatorStrings {

        todo!();
        /*
            jassertfalse;
            return {};
        */
    }

    pub fn get_separators_char_pointer_ascii() -> SeparatorStrings {
        
        todo!();
        /*
            SeparatorStrings result;

            const CharPointer_ASCII::CharType* terminalCharPtrs[] = {
                "", "-", "+", "e", "e+", "E-", "f", " ", ",", ";", "<", "'", "\"", "_", "k",
                " +", " -", " -e", "-In ", " +n", "n", "  r"
            };

            for (auto ptr : terminalCharPtrs)
                result.terminals.push_back (memoryBlockFromCharPtr<CharPointer_ASCII> (ptr));

            const CharPointer_ASCII::CharType* nullCharPtrs[] = { "." };

            result.nulls = result.terminals;

            for (auto ptr : nullCharPtrs)
                result.nulls.push_back (memoryBlockFromCharPtr<CharPointer_ASCII> (ptr));

            return result;
        */
    }

    pub fn get_separators_char_pointer_utf8() -> SeparatorStrings {
        
        todo!();
        /*
            auto result = getSeparators<CharPointer_ASCII>();

            const CharPointer_UTF8::CharType* terminalCharPtrs[] = {
                "\xe2\x82\xac",                      // €
                "\xf0\x90\x90\xB7",                  // 𐐷
                "\xf0\x9f\x98\x83",                  // 😃
                "\xf0\x9f\x8f\x81\xF0\x9F\x9A\x97"   // 🏁🚗
            };

            for (auto ptr : terminalCharPtrs)
            {
                auto block = memoryBlockFromCharPtr<CharPointer_UTF8> (ptr);

                for (auto vec : { &result.terminals, &result.nulls })
                    vec->push_back (block);
            }

            return result;
        */
    }

    pub fn get_separators_char_pointer_utf16() -> SeparatorStrings {
        
        todo!();
        /*
            const std::vector<std::vector<char16_t>> terminalCharPtrs {
                { 0x0                                 },
                { 0x0076, 0x0                         },   // v
                { 0x20ac, 0x0                         },   // €
                { 0xd801, 0xdc37, 0x0                 },   // 𐐷
                { 0x0065, 0xd83d, 0xde03, 0x0         },   // e😃
                { 0xd83c, 0xdfc1, 0xd83d, 0xde97, 0x0 }    // 🏁🚗
            };

            return prefixWithAsciiSeparators<CharPointer_UTF16> (terminalCharPtrs);
        */
    }

    pub fn get_separators_char_pointer_utf32() -> SeparatorStrings {
        
        todo!();
        /*
            const std::vector<std::vector<char32_t>> terminalCharPtrs = {
                { 0x00000076, 0x0             },   // v
                { 0x000020aC, 0x0             },   // €
                { 0x00010437, 0x0             },   // 𐐷
                { 0x00000065, 0x0001f603, 0x0 },   // e😃
                { 0x0001f3c1, 0x0001f697, 0x0 }    // 🏁🚗
            };

            return prefixWithAsciiSeparators<CharPointer_UTF32> (terminalCharPtrs);
        */
    }

    pub fn prefix_with_ascii_separators<CharPointerType, StorageType>(terminal_char_ptrs: &Vec<Vec<StorageType>>) -> SeparatorStrings {

        todo!();
        /*
            auto asciiSeparators = getSeparators<CharPointer_ASCII>();

            SeparatorStrings result;

            for (const auto& block : asciiSeparators.terminals)
                result.terminals.push_back (convert<CharPointer_ASCII, CharPointerType> (block));

            for (const auto& block : asciiSeparators.nulls)
                result.nulls.push_back (convert<CharPointer_ASCII, CharPointerType> (block));

            for (auto& t : terminalCharPtrs)
            {
                const auto block = memoryBlockFromCharPtr<CharPointerType> ((typename CharPointerType::CharType*) t.data());

                for (auto vec : { &result.terminals, &result.nulls })
                    vec->push_back (block);
            }

            return result;
        */
    }

    pub fn with_all_prefixes_and_suffixes<TestFunction>(
            prefixes:    &Vec<MemoryBlock>,
            suffixes:    &Vec<MemoryBlock>,
            test_values: &Vec<MemoryBlock>,
            test:        TestFunction)  {

        todo!();
        /*
            for (const auto& prefix : prefixes)
            {
                for (const auto& testValue : testValues)
                {
                    MemoryBlock testBlock = prefix;
                    testBlock.append (testValue.getData(), testValue.getSize());

                    for (const auto& suffix : suffixes)
                    {
                        MemoryBlock data = testBlock;
                        data.append (suffix.getData(), suffix.getSize());

                        test (data, suffix);
                    }
                }
            }
        */
    }

    pub struct CharacterFunctionsTests<CharPointerType> {
        base: UnitTest,
    }

    impl<CharPointerType> HasCharType for CharacterFunctionsTests<CharPointerType> {
        type CharType = CharPointerType::CharType;
    }

    impl Default for CharacterFunctionsTests {
        
        fn default() -> Self {
            todo!();
            /*


                : UnitTest ("CharacterFunctions", UnitTestCategories::text
            */
        }
    }

    impl CharacterFunctionsTests<CharPointerType> {

        pub fn run_test(&mut self)  {
            
            todo!();
            /*
                beginTest ("readDoubleValue");

                const std::pair<const char*, double> trials[] =
                {
                    // Integers
                    CHARPTR_DOUBLE_PAIR_COMBOS (0),
                    CHARPTR_DOUBLE_PAIR_COMBOS (3),
                    CHARPTR_DOUBLE_PAIR_COMBOS (4931),
                    CHARPTR_DOUBLE_PAIR_COMBOS (5000),
                    CHARPTR_DOUBLE_PAIR_COMBOS (9862097),

                    // Floating point numbers
                    CHARPTR_DOUBLE_PAIR_COMBOS (0.),
                    CHARPTR_DOUBLE_PAIR_COMBOS (9.),
                    CHARPTR_DOUBLE_PAIR_COMBOS (7.000),
                    CHARPTR_DOUBLE_PAIR_COMBOS (0.2),
                    CHARPTR_DOUBLE_PAIR_COMBOS (.298630),
                    CHARPTR_DOUBLE_PAIR_COMBOS (1.118),
                    CHARPTR_DOUBLE_PAIR_COMBOS (0.9000),
                    CHARPTR_DOUBLE_PAIR_COMBOS (0.0000001),
                    CHARPTR_DOUBLE_PAIR_COMBOS (500.0000001),
                    CHARPTR_DOUBLE_PAIR_COMBOS (9862098.2398604),

                    // Exponents
                    CHARPTR_DOUBLE_PAIR_COMBOS (0e0),
                    CHARPTR_DOUBLE_PAIR_COMBOS (0.e0),
                    CHARPTR_DOUBLE_PAIR_COMBOS (0.00000e0),
                    CHARPTR_DOUBLE_PAIR_COMBOS (.0e7),
                    CHARPTR_DOUBLE_PAIR_COMBOS (0e-5),
                    CHARPTR_DOUBLE_PAIR_COMBOS (2E0),
                    CHARPTR_DOUBLE_PAIR_COMBOS (4.E0),
                    CHARPTR_DOUBLE_PAIR_COMBOS (1.2000000E0),
                    CHARPTR_DOUBLE_PAIR_COMBOS (1.2000000E6),
                    CHARPTR_DOUBLE_PAIR_COMBOS (.398e3),
                    CHARPTR_DOUBLE_PAIR_COMBOS (10e10),
                    CHARPTR_DOUBLE_PAIR_COMBOS (1.4962e+2),
                    CHARPTR_DOUBLE_PAIR_COMBOS (3198693.0973e4),
                    CHARPTR_DOUBLE_PAIR_COMBOS (10973097.2087E-4),
                    CHARPTR_DOUBLE_PAIR_COMBOS (1.3986e00006),
                    CHARPTR_DOUBLE_PAIR_COMBOS (2087.3087e+00006),
                    CHARPTR_DOUBLE_PAIR_COMBOS (6.0872e-00006),

                    CHARPTR_DOUBLE_PAIR_COMBOS (1.7976931348623157e+308),
                    CHARPTR_DOUBLE_PAIR_COMBOS (2.2250738585072014e-308),

                    // Too many sig figs. The parsing routine on MinGW gets the last
                    // significant figure wrong.
                    CHARPTR_DOUBLE_PAIR_COMBOS (17654321098765432.9),
                    CHARPTR_DOUBLE_PAIR_COMBOS (183456789012345678.9),
                    CHARPTR_DOUBLE_PAIR_COMBOS (1934567890123456789.9),
                    CHARPTR_DOUBLE_PAIR_COMBOS (20345678901234567891.9),
                    CHARPTR_DOUBLE_PAIR_COMBOS (10000000000000000303786028427003666890752.000000),
                    CHARPTR_DOUBLE_PAIR_COMBOS (10000000000000000303786028427003666890752e3),
                    CHARPTR_DOUBLE_PAIR_COMBOS (10000000000000000303786028427003666890752e100),
                    CHARPTR_DOUBLE_PAIR_COMBOS (10000000000000000303786028427003666890752.000000e-5),
                    CHARPTR_DOUBLE_PAIR_COMBOS (10000000000000000303786028427003666890752.000005e-40),

                    CHARPTR_DOUBLE_PAIR_COMBOS (1.23456789012345678901234567890),
                    CHARPTR_DOUBLE_PAIR_COMBOS (1.23456789012345678901234567890e-111),
                };

                auto asciiToMemoryBlock = [] (const char* asciiPtr, bool removeNullTerminator)
                {
                    auto block = memoryBlockFromCharPtr<CharPointer_ASCII> (asciiPtr);
                    return convert<CharPointer_ASCII, CharPointerType> (block, removeNullTerminator);
                };

                const auto separators = getSeparators<CharPointerType>();

                for (const auto& trial : trials)
                {
                    for (const auto& terminal : separators.terminals)
                    {
                        MemoryBlock data { asciiToMemoryBlock (trial.first, true) };
                        data.append (terminal.getData(), terminal.getSize());

                        CharPointerType charPtr { (CharType*) data.getData() };
                        expectEquals (CharacterFunctions::readDoubleValue (charPtr), trial.second);
                        expect (*charPtr == *(CharPointerType ((CharType*) terminal.getData())));
                    }
                }

                auto asciiToMemoryBlocks = [&] (const std::vector<const char*>& asciiPtrs, bool removeNullTerminator)
                {
                    std::vector<MemoryBlock> result;

                    for (auto* ptr : asciiPtrs)
                        result.push_back (asciiToMemoryBlock (ptr, removeNullTerminator));

                    return result;
                };

                std::vector<const char*> prefixCharPtrs = { "" , "+", "-" };
                const auto prefixes = asciiToMemoryBlocks (prefixCharPtrs, true);

                {
                    std::vector<const char*> nanCharPtrs = { "NaN", "nan", "NAN", "naN" };
                    auto nans = asciiToMemoryBlocks (nanCharPtrs, true);

                    withAllPrefixesAndSuffixes (prefixes, separators.terminals, nans, [this] (const MemoryBlock& data,
                                                                                              const MemoryBlock& suffix)
                    {
                        CharPointerType charPtr { (CharType*) data.getData() };
                        expect (std::isnan (CharacterFunctions::readDoubleValue (charPtr)));
                        expect (*charPtr == *(CharPointerType ((CharType*) suffix.getData())));
                    });
                }

                {
                    std::vector<const char*> infCharPtrs = { "Inf", "inf", "INF", "InF", "1.0E1024", "1.23456789012345678901234567890e123456789" };
                    auto infs = asciiToMemoryBlocks (infCharPtrs, true);

                    withAllPrefixesAndSuffixes (prefixes, separators.terminals, infs, [this] (const MemoryBlock& data,
                                                                                              const MemoryBlock& suffix)
                    {
                        CharPointerType charPtr { (CharType*) data.getData() };
                        auto expected = charPtr[0] == '-' ? -std::numeric_limits<double>::infinity()
                                                          :  std::numeric_limits<double>::infinity();
                        expectEquals (CharacterFunctions::readDoubleValue (charPtr), expected);
                        expect (*charPtr == *(CharPointerType ((CharType*) suffix.getData())));
                    });
                }

                {
                    std::vector<const char*> zeroCharPtrs = { "1.0E-400", "1.23456789012345678901234567890e-123456789" };
                    auto zeros = asciiToMemoryBlocks (zeroCharPtrs, true);

                    withAllPrefixesAndSuffixes (prefixes, separators.terminals, zeros, [this] (const MemoryBlock& data,
                                                                                               const MemoryBlock& suffix)
                    {
                        CharPointerType charPtr { (CharType*) data.getData() };
                        auto expected = charPtr[0] == '-' ? -0.0 : 0.0;
                        expectEquals (CharacterFunctions::readDoubleValue (charPtr), expected);
                        expect (*charPtr == *(CharPointerType ((CharType*) suffix.getData())));
                    });
                }

                {
                    for (const auto& n : separators.nulls)
                    {
                        MemoryBlock data { n.getData(), n.getSize() };
                        CharPointerType charPtr { (CharType*) data.getData() };
                        expectEquals (CharacterFunctions::readDoubleValue (charPtr), 0.0);
                        expect (charPtr == CharPointerType { (CharType*) data.getData() }.findEndOfWhitespace());
                    }
                }
            */
        }
    }
}
