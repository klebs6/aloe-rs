crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_String.h]

pub struct PreallocationBytes
{
    num_bytes: usize,
}

impl PreallocationBytes {
    
    pub fn new(num: usize) -> Self {
    
        todo!();
        /*
        : num_bytes(num),

        
        */
    }
}

lazy_static!{
    /*
    #if ! DOXYGEN && (ALOE_MAC || ALOE_IOS)
     // Annoyingly we can only forward-declare a typedef by forward-declaring the
     // aliased type
     #if __has_attribute(objc_bridge)
      #define ALOE_CF_BRIDGED_TYPE(T) __attribute__((objc_bridge(T)))
     #else
      #define ALOE_CF_BRIDGED_TYPE(T)
     #endif

     typedef const struct ALOE_CF_BRIDGED_TYPE(NSString) __CFString * CFStringRef;

     #undef ALOE_CF_BRIDGED_TYPE
    #endif
    */
}

/**
  | The Aloe AloeString class!
  | 
  | Using a reference-counted internal
  | representation, these strings are
  | fast and efficient, and there are methods
  | to do just about any operation you'll
  | ever dream of.
  | 
  | @see StringArray, StringPairArray
  | 
  | @tags{Core}
  |
  */
pub struct AloeString {
    text: CharPointerType,
}

impl fmt::Debug for AloeString {

    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        todo!() 
    }
}
    
impl Into<bool> for AloeString {
    
    /**
      | This private cast operator should prevent
      | strings being accidentally cast to bools
      | (this is possible because the compiler can
      | add an implicit cast via a const char*)
      */
    #[inline] fn into(self) -> bool {
        todo!();
        /*
            return false;
        */
    }
}

impl AloeString {
    
    /**
      | Creates a string from a UTF-8 character
      | string
      |
      */
    pub fn new_from_utf8_ptr(text: CharPointer_UTF8) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates a string from a UTF-8 character
      | string
      |
      */
    pub fn new_from_utf8_ptr_with_max(
        text:      CharPointer_UTF8,
        max_chars: usize) -> Self {
    
        todo!();
        /*

        */
    }

    /**
      | Creates a string from a UTF-16 character
      | string
      |
      */
    pub fn new_from_utf16_ptr(text: CharPointer_UTF16) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates a string from a UTF-16 character
      | string
      |
      */
    pub fn new_from_utf16_ptr_with_max(
        text:      CharPointer_UTF16,
        max_chars: usize) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates a string from a UTF-32 character
      | string
      |
      */
    pub fn new_from_utf32_text(text: CharPointer_UTF32) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates a string from a UTF-32 character
      | string
      |
      */
    pub fn new_from_utf32_ptr_with_max(
        text:      CharPointer_UTF32,
        max_chars: usize) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates a string from an ASCII character
      | string
      |
      */
    pub fn new_from_ascii(text: CharPointer_ASCII) -> Self {
    
        todo!();
        /*
        
        */
    }

    /* ---- Assignment and concatenation operators..  ---- */

    /**
      | Appends a string to the end of this one.
      | 
      | -----------
      | @param startOfTextToAppend
      | 
      | the start of the string to add. This must
      | not be a nullptr
      | ----------
      | @param endOfTextToAppend
      | 
      | the end of the string to add. This must
      | not be a nullptr
      |
      */
    pub fn append_char_pointer_with_range2<CharPointer>(&mut self, 
        start_of_text_to_append: CharPointer,
        end_of_text_to_append:   CharPointer)  {
    
        todo!();
        /*
            jassert (startOfTextToAppend.getAddress() != nullptr && endOfTextToAppend.getAddress() != nullptr);

            size_t extraBytesNeeded = 0, numChars = 1;

            for (auto t = startOfTextToAppend; t != endOfTextToAppend && ! t.isEmpty(); ++numChars)
                extraBytesNeeded += CharPointerType::getBytesRequiredFor (t.getAndAdvance());

            if (extraBytesNeeded > 0)
            {
                auto byteOffsetOfNull = getByteOffsetOfEnd();

                preallocateBytes (byteOffsetOfNull + extraBytesNeeded);
                CharPointerType (addBytesToPointer (text.getAddress(), (int) byteOffsetOfNull))
                    .writeWithCharLimit (startOfTextToAppend, (int) numChars);
            }
        */
    }

    /**
      | Appends a string to the end of this one.
      | 
      | -----------
      | @param textToAppend
      | 
      | the string to add
      | ----------
      | @param maxCharsToTake
      | 
      | the maximum number of characters to
      | take from the string passed in
      |
      */
    pub fn append_char_pointer_with_text_and_max<CharPointer>(&mut self, 
        text_to_append:    CharPointer,
        max_chars_to_take: usize)  {
    
        todo!();
        /*
            if (textToAppend.getAddress() != nullptr)
            {
                size_t extraBytesNeeded = 0, numChars = 1;

                for (auto t = textToAppend; numChars <= maxCharsToTake && ! t.isEmpty(); ++numChars)
                    extraBytesNeeded += CharPointerType::getBytesRequiredFor (t.getAndAdvance());

                if (extraBytesNeeded > 0)
                {
                    auto byteOffsetOfNull = getByteOffsetOfEnd();

                    preallocateBytes (byteOffsetOfNull + extraBytesNeeded);
                    CharPointerType (addBytesToPointer (text.getAddress(), (int) byteOffsetOfNull))
                        .writeWithCharLimit (textToAppend, (int) numChars);
                }
            }
        */
    }

    /**
      | Appends a string to the end of this one.
      |
      */
    pub fn append_char_pointer_with_text(&mut self, text_to_append: CharPointerType)  {
        
        todo!();
        /*
            appendCharPointer (textToAppend, textToAppend.findTerminatingNull());
        */
    }
    
    /**
      | Appends a string to the end of this one.
      |
      */
    pub fn append_char_pointer_with_text_generic<CharPointer>(&mut self, text_to_append: CharPointer)  {
    
        todo!();
        /*
            appendCharPointer (textToAppend, std::numeric_limits<size_t>::max());
        */
    }

    /* ------------- Comparison methods..   ------------- */
    
    /**
      | Returns true if the string contains
      | no characters.
      | 
      | -----------
      | @note
      | 
      | there's also an isNotEmpty() method
      | to help write readable code. @see containsNonWhitespaceChars()
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return text.isEmpty();
        */
    }

    /**
      | Returns true if the string contains
      | at least one character.
      | 
      | -----------
      | @note
      | 
      | there's also an isEmpty() method to
      | help write readable code. @see containsNonWhitespaceChars()
      |
      */
    pub fn is_not_empty(&self) -> bool {
        
        todo!();
        /*
            return ! text.isEmpty();
        */
    }

    /* --------- Substring location methods..   --------- */
    
    /* Substring extraction and manipulation methods */

    /**
      | Creates a AloeString from a printf-style
      | parameter list.
      | 
      | I don't like this method. I don't use
      | it myself, and I recommend avoiding
      | it and using the operator<< methods
      | or pretty much anything else instead.
      | It's only provided here because of the
      | popular unrest that was stirred-up
      | when I tried to remove it...
      | 
      | If you're really determined to use it,
      | at least make sure that you never, ever,
      | pass any AloeString objects to it as parameters.
      | And bear in mind that internally, depending
      | on the platform, it may be using wchar_t
      | or char character types, so that even
      | string literals can't be safely used
      | as parameters if you're writing portable
      | code.
      |
      */
    pub fn formatted<Args>(
        format_str: &AloeString,
        args:       Args) -> AloeString {
    
        todo!();
        /*
            return formattedRaw (formatStr.toRawUTF8(), args...);
        */
    }

    /**
      | Returns an iterator pointing at the
      | beginning of the string.
      |
      */
    pub fn begin(&self) -> CharPointerType {
        
        todo!();
        /*
            return getCharPointer();
        */
    }

    /**
      | Returns an iterator pointing at the
      | terminating null of the string.
      | 
      | -----------
      | @note
      | 
      | this has to find the terminating null
      | before returning it, so prefer to call
      | this once before looping and then reuse
      | the result, rather than calling 'end()'
      | each time through the loop.
      | 
      | @code AloeString str = ...;
      | 
      | // BEST for (auto c : str) DBG (c);
      | 
      | // GOOD for (auto ptr = str.begin(),
      | end = str.end(); ptr != end; ++ptr) DBG
      | (*ptr);
      | 
      | std::for_each (str.begin(), str.end(),
      | [] (aloe_wchar c) { DBG (c); });
      | 
      | // BAD for (auto ptr = str.begin(); ptr
      | != str.end(); ++ptr) DBG (*ptr); @endcode
      |
      */
    pub fn end(&self) -> CharPointerType {
        
        todo!();
        /*
            return begin().findTerminatingNull();
        */
    }

    /* ------------ Numeric conversions..   ------------ */

    /**
      | Returns a string representing this
      | numeric value in hexadecimal.
      |
      */
    pub fn to_hex_string_from_integer<IntegerType>(number: IntegerType) -> AloeString {
    
        todo!();
        /*
            return createHex (number);
        */
    }

    /**
      | Returns a string containing a decimal
      | with a set number of significant figures.
      | 
      | -----------
      | @param number
      | 
      | the input number
      | ----------
      | @param numberOfSignificantFigures
      | 
      | the number of significant figures to
      | use
      |
      */
    pub fn to_decimal_string_with_significant_figures<DecimalType>(
        number:                        DecimalType,
        number_of_significant_figures: i32) -> AloeString {
    
        todo!();
        /*
            jassert (numberOfSignificantFigures > 0);

            if (number == 0)
            {
                if (numberOfSignificantFigures > 1)
                {
                    AloeString result ("0.0");

                    for (int i = 2; i < numberOfSignificantFigures; ++i)
                        result += "0";

                    return result;
                }

                return "0";
            }

            auto numDigitsBeforePoint = (int) std::ceil (std::log10 (number < 0 ? -number : number));

            auto shift = numberOfSignificantFigures - numDigitsBeforePoint;
            auto factor = std::pow (10.0, shift);
            auto rounded = std::round (number * factor) / factor;

            std::stringstream ss;
            ss << std::fixed << std::setprecision (std::max (shift, 0)) << rounded;
            return ss.str();
        */
    }

    /**
      | Returns the character pointer currently
      | being used to store this string.
      | 
      | Because it returns a reference to the
      | string's internal data, the pointer
      | that is returned must not be stored anywhere,
      | as it can be deleted whenever the string
      | changes.
      |
      */
    pub fn get_char_pointer(&self) -> CharPointerType {
        
        todo!();
        /*
            return text;
        */
    }

    /**
      | OSX ONLY - Creates a AloeString from an OSX
      | CFString.
      |
      */
    #[cfg(any(any(target_os="macos",target_os="ios"),DOXYGEN))]
    pub fn from_cf_string(cf_string: CFStringRef) -> AloeString {
        
        todo!();
        /*
        
        */
    }

    /**
      | OSX ONLY - Converts this string to a CFString.
      | 
      | Remember that you must use CFRelease()
      | to free the returned string when you're
      | finished with it.
      |
      */
    #[cfg(any(any(target_os="macos",target_os="ios"),DOXYGEN))]
    pub fn to_cf_string(&self) -> CFStringRef {
        
        todo!();
        /*
        
        */
    }

    /**
      | OSX ONLY - Returns a copy of this string
      | in which any decomposed unicode characters
      | have been converted to their precomposed
      | equivalents.
      |
      */
    #[cfg(any(any(target_os="macos",target_os="ios"),DOXYGEN))]
    pub fn convert_to_precomposed_unicode(&self) -> AloeString {
        
        todo!();
        /*
        
        */
    }

    /**
      | This constructor preallocates a certain
      | amount of memory
      |
      */
    pub fn new_preallocated(_0: &PreallocationBytes) -> Self {
    
        todo!();
        /*
        
        */
    }
    
    pub fn create_hex<Type>(n: Type) -> AloeString {
    
        todo!();
        /*
            return createHex (static_cast<typename TypeHelpers::UnsignedTypeWithSize<(int) sizeof (n)>::type> (n));
        */
    }
}

lazy_static!{
    /*
    /** Concatenates two strings. */
     AloeString  operator+ (const char* string1,     const AloeString& string2);
    /** Concatenates two strings. */
     AloeString  operator+ (const wchar_t* string1,  const AloeString& string2);
    /** Concatenates two strings. */
     AloeString  operator+ (char string1,            const AloeString& string2);
    /** Concatenates two strings. */
     AloeString  operator+ (wchar_t string1,         const AloeString& string2);
    #if ! ALOE_NATIVE_WCHAR_IS_UTF32
    /** Concatenates two strings. */
     AloeString  operator+ (aloe_wchar string1,      const AloeString& string2);
    #endif

    /** Concatenates two strings. */
     AloeString  operator+ (AloeString string1, const AloeString& string2);
    /** Concatenates two strings. */
     AloeString  operator+ (AloeString string1, const char* string2);
    /** Concatenates two strings. */
     AloeString  operator+ (AloeString string1, const wchar_t* string2);
    /** Concatenates two strings. */
     AloeString  operator+ (AloeString string1, const std::string& string2);
    /** Concatenates two strings. */
     AloeString  operator+ (AloeString string1, char characterToAppend);
    /** Concatenates two strings. */
     AloeString  operator+ (AloeString string1, wchar_t characterToAppend);
    #if ! ALOE_NATIVE_WCHAR_IS_UTF32
    /** Concatenates two strings. */
     AloeString  operator+ (AloeString string1, aloe_wchar characterToAppend);
    #endif

    /** Appends a character at the end of a string. */
     AloeString&  operator<< (AloeString& string1, char characterToAppend);
    /** Appends a character at the end of a string. */
     AloeString&  operator<< (AloeString& string1, wchar_t characterToAppend);
    #if ! ALOE_NATIVE_WCHAR_IS_UTF32
    /** Appends a character at the end of a string. */
     AloeString&  operator<< (AloeString& string1, aloe_wchar characterToAppend);
    #endif

    /** Appends a string to the end of the first one. */
     AloeString&  operator<< (AloeString& string1, const char* string2);
    /** Appends a string to the end of the first one. */
     AloeString&  operator<< (AloeString& string1, const wchar_t* string2);
    /** Appends a string to the end of the first one. */
     AloeString&  operator<< (AloeString& string1, const AloeString& string2);
    /** Appends a string to the end of the first one. */
     AloeString&  operator<< (AloeString& string1, &str string2);
    /** Appends a string to the end of the first one. */
     AloeString&  operator<< (AloeString& string1, const std::string& string2);

    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, uint8 number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, short number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, int number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, long number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, unsigned long number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, int64 number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, uint64 number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, float number);
    /** Appends a decimal number to the end of a string. */
     AloeString&  operator<< (AloeString& string1, double number);

    #ifndef DOXYGEN
    // Automatically creating a AloeString from a bool opens up lots of nasty type conversion edge cases.
    // If you want a AloeString representation of a bool you can cast the bool to an int first.
    AloeString&  operator<< (AloeString&, bool) = delete;
    #endif

    /** Case-sensitive comparison of two strings. */
     bool  operator== (const AloeString& string1, const AloeString& string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator== (const AloeString& string1, const char* string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator== (const AloeString& string1, const wchar_t* string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator== (const AloeString& string1, CharPointer_UTF8 string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator== (const AloeString& string1, CharPointer_UTF16 string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator== (const AloeString& string1, CharPointer_UTF32 string2) ;

    /** Case-sensitive comparison of two strings. */
     bool  operator!= (const AloeString& string1, const AloeString& string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator!= (const AloeString& string1, const char* string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator!= (const AloeString& string1, const wchar_t* string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator!= (const AloeString& string1, CharPointer_UTF8 string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator!= (const AloeString& string1, CharPointer_UTF16 string2) ;
    /** Case-sensitive comparison of two strings. */
     bool  operator!= (const AloeString& string1, CharPointer_UTF32 string2) ;
    */
}

/**
  | This operator allows you to write a aloe
  | AloeString directly to std output streams.
  | 
  | This is handy for writing strings to
  | std::cout, std::cerr, etc.
  |
  */
pub fn write_aloe_string_to_stream<W: Write>(stream: &mut W, aloe_string: &AloeString) -> std::io::Result<()> {
    // Assuming `to_raw_utf8` is a method of `AloeString` which returns a UTF-8 representation
    // Replace with appropriate method or conversion if needed
    let utf8_str = aloe_string.to_rawutf8(); 

    let slice: &[u8] = unsafe {
        std::slice::from_raw_parts(utf8_str, aloe_string.length())
    };
    
    stream.write_all(slice)
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_String.cpp]

lazy_static!{
    /*
    NewLine newLine;
    */
}

#[cfg(ALOE_NATIVE_WCHAR_IS_UTF8)]
pub type CharPointer_wchar_t = CharPointer_UTF8;

#[cfg(ALOE_NATIVE_WCHAR_IS_UTF16)]
pub type CharPointer_wchar_t = CharPointer_UTF16;

#[cfg(not(any(ALOE_NATIVE_WCHAR_IS_UTF8,ALOE_NATIVE_WCHAR_IS_UTF16)))]
pub type CharPointer_wchar_t = CharPointer_UTF32;

pub fn cast_to_char_pointer_wchar_t(t: *const c_void) -> CharPointer_wchar_t {
    
    todo!();
    /*
        return CharPointer_wchar_t (static_cast<const CharPointer_wchar_t::CharType*> (t));
    */
}

/**
   (Mirrors the structure of StringHolder, but
   without the atomic member, so can be statically
   constructed)
  */
pub struct EmptyString
{
    ref_count:       i32,
    allocated_bytes: usize,
    text:            <CharPointerType as HasCharType>::CharType,
}

lazy_static!{
    /*
    static const EmptyString emptyString { 0x3fffffff, sizeof (CharPointerType::CharType), 0 };
    */
}

pub struct StringHolder<CharPointerType: HasCharType> {
    ref_count:           AtomicI32,
    allocated_num_bytes: usize,
    text:                [<CharPointerType as HasCharType>::CharType; 1],
}

impl<CharPointerType: HasCharType> HasCharType for StringHolder<CharPointerType> {

    type CharType = <CharPointerType as HasCharType>::CharType;
}

impl<CharPointerType: HasCharType> StringHolder<CharPointerType> {

    pub fn create_uninitialised_bytes(num_bytes: usize) -> CharPointerType {
        
        todo!();
        /*
            numBytes = (numBytes + 3) & ~(size_t) 3;
            auto s = unalignedPointerCast<StringHolder*> (new char [sizeof (StringHolder) - sizeof (CharType) + numBytes]);
            s->refCount.value = 0;
            s->allocatedNumBytes = numBytes;
            return CharPointerType (s->text);
        */
    }
    
    pub fn create_from_char_pointer_from_text<CharPointer>(text: CharPointer) -> CharPointerType {
    
        todo!();
        /*
            if (text.getAddress() == nullptr || text.isEmpty())
                return CharPointerType (&(emptyString.text));

            auto bytesNeeded = sizeof (CharType) + CharPointerType::getBytesRequiredFor (text);
            auto dest = createUninitialisedBytes (bytesNeeded);
            CharPointerType (dest).writeAll (text);
            return dest;
        */
    }
    
    pub fn create_from_char_pointer_from_text_with_max<CharPointer>(
        text:      CharPointer,
        max_chars: usize) -> CharPointerType {
    
        todo!();
        /*
            if (text.getAddress() == nullptr || text.isEmpty() || maxChars == 0)
                return CharPointerType (&(emptyString.text));

            auto end = text;
            size_t numChars = 0;
            size_t bytesNeeded = sizeof (CharType);

            while (numChars < maxChars && ! end.isEmpty())
            {
                bytesNeeded += CharPointerType::getBytesRequiredFor (end.getAndAdvance());
                ++numChars;
            }

            auto dest = createUninitialisedBytes (bytesNeeded);
            CharPointerType (dest).writeWithCharLimit (text, (int) numChars + 1);
            return dest;
        */
    }
    
    pub fn create_from_char_pointer_from_range<CharPointer>(
        start: CharPointer,
        end:   CharPointer) -> CharPointerType {
    
        todo!();
        /*
            if (start.getAddress() == nullptr || start.isEmpty())
                return CharPointerType (&(emptyString.text));

            auto e = start;
            int numChars = 0;
            auto bytesNeeded = sizeof (CharType);

            while (e < end && ! e.isEmpty())
            {
                bytesNeeded += CharPointerType::getBytesRequiredFor (e.getAndAdvance());
                ++numChars;
            }

            auto dest = createUninitialisedBytes (bytesNeeded);
            CharPointerType (dest).writeWithCharLimit (start, numChars + 1);
            return dest;
        */
    }
    
    pub fn create_from_char_pointer_from_range2(
        start: CharPointerType,
        end:   CharPointerType) -> CharPointerType {
        
        todo!();
        /*
            if (start.getAddress() == nullptr || start.isEmpty())
                return CharPointerType (&(emptyString.text));

            auto numBytes = (size_t) (reinterpret_cast<const char*> (end.getAddress())
                                       - reinterpret_cast<const char*> (start.getAddress()));
            auto dest = createUninitialisedBytes (numBytes + sizeof (CharType));
            memcpy (dest.getAddress(), start, numBytes);
            dest.getAddress()[numBytes / sizeof (CharType)] = 0;
            return dest;
        */
    }
    
    pub fn create_from_fixed_length(
        src:       *const u8,
        num_chars: usize) -> CharPointerType {
        
        todo!();
        /*
            auto dest = createUninitialisedBytes (numChars * sizeof (CharType) + sizeof (CharType));
            CharPointerType (dest).writeWithCharLimit (CharPointer_UTF8 (src), (int) (numChars + 1));
            return dest;
        */
    }
    
    pub fn retain(text: CharPointerType)  {
        
        todo!();
        /*
            auto* b = bufferFromText (text);

            if (! isEmptyString (b))
                ++(b->refCount);
        */
    }
    
    pub fn release_string_holder(b: *mut StringHolder<CharPointerType>)  {
        
        todo!();
        /*
            if (! isEmptyString (b))
                if (--(b->refCount) == -1)
                    delete[] reinterpret_cast<char*> (b);
        */
    }
    
    pub fn release(text: CharPointerType)  {
        
        todo!();
        /*
            release (bufferFromText (text));
        */
    }
    
    pub fn get_reference_count(text: CharPointerType) -> i32 {
        
        todo!();
        /*
            return bufferFromText (text)->refCount.get() + 1;
        */
    }
    
    pub fn make_unique_with_byte_size(
        text:      CharPointerType,
        num_bytes: usize) -> CharPointerType {
        
        todo!();
        /*
            auto* b = bufferFromText (text);

            if (isEmptyString (b))
            {
                auto newText = createUninitialisedBytes (numBytes);
                newText.writeNull();
                return newText;
            }

            if (b->allocatedNumBytes >= numBytes && b->refCount.get() <= 0)
                return text;

            auto newText = createUninitialisedBytes (jmax (b->allocatedNumBytes, numBytes));
            memcpy (newText.getAddress(), text.getAddress(), b->allocatedNumBytes);
            release (b);

            return newText;
        */
    }
    
    pub fn get_allocated_num_bytes(text: CharPointerType) -> usize {
        
        todo!();
        /*
            return bufferFromText (text)->allocatedNumBytes;
        */
    }
    
    pub fn buffer_from_text(text: CharPointerType) -> *mut StringHolder<CharPointerType> {
        
        todo!();
        /*
            // (Can't use offsetof() here because of warnings about this not being a POD)
            return unalignedPointerCast<StringHolder*> (reinterpret_cast<char*> (text.getAddress())
                        - (reinterpret_cast<size_t> (reinterpret_cast<StringHolder*> (128)->text) - 128));
        */
    }
    
    pub fn is_empty_string(other: *mut StringHolder<CharPointerType>) -> bool {
        
        todo!();
        /*
            return (other->refCount.get() & 0x30000000) != 0;
        */
    }
    
    pub fn compile_time_checks(&mut self)  {
        
        todo!();
        /*
            // Let me know if any of these assertions fail on your system!
           #if ALOE_NATIVE_WCHAR_IS_UTF8
            static_assert (sizeof (wchar_t) == 1, "ALOE_NATIVE_WCHAR_IS_* macro has incorrect value");
           #elif ALOE_NATIVE_WCHAR_IS_UTF16
            static_assert (sizeof (wchar_t) == 2, "ALOE_NATIVE_WCHAR_IS_* macro has incorrect value");
           #elif ALOE_NATIVE_WCHAR_IS_UTF32
            static_assert (sizeof (wchar_t) == 4, "ALOE_NATIVE_WCHAR_IS_* macro has incorrect value");
           #else
            #error "native wchar_t size is unknown"
           #endif

            static_assert (sizeof (EmptyString) == sizeof (StringHolder),
                           "StringHolder is not large enough to hold an empty AloeString");
        */
    }
}

impl Default for AloeString {
    
    /**
      | Creates an empty string. @see empty
      |
      */
    fn default() -> Self {
        todo!();
        /*
            : text (&(emptyString.text)
        */
    }
}

impl Drop for AloeString {
    fn drop(&mut self) {
        todo!();
        /* 
        StringHolder::release (text);
 */
    }
}

impl AloeString {

    pub fn new(other: &str) -> Self {
    
        todo!();
        /*
        : text(other.text),

            StringHolder::retain (text);
        */
    }
    
    /**
      | Swaps the contents of this string with
      | another one.
      | 
      | This is a very fast operation, as no allocation
      | or copying needs to be done.
      |
      */
    pub fn swap_with(&mut self, other: &mut AloeString)  {
        
        todo!();
        /*
            std::swap (text, other.text);
        */
    }
    
    /**
      | Resets this string to be empty.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            StringHolder::release (text);
        text = &(emptyString.text);
        */
    }
    
    /**
      | Replaces this string's contents with
      | another string.
      |
      */
    pub fn assign_from_other_ref(&mut self, other: &AloeString) -> &mut AloeString {
        
        todo!();
        /*
            StringHolder::retain (other.text);
        StringHolder::release (text.atomicSwap (other.text));
        return *this;
        */
    }
    
    pub fn new_from_other(other: AloeString) -> Self {
    
        todo!();
        /*
        : text(other.text),

            other.text = &(emptyString.text);
        */
    }
    
    /**
      | Moves the contents of another string
      | to the receiver
      |
      */
    pub fn assign_from_other(&mut self, other: AloeString) -> &mut AloeString {
        
        todo!();
        /*
            std::swap (text, other.text);
        return *this;
        */
    }
    
    pub fn new_with_preallocation(preallocation_size: &PreallocationBytes) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createUninitialisedBytes (preallocationSize.numBytes + sizeof (CharPointerType::CharType)))
        */
    }
    
    /**
      | Increases the string's internally
      | allocated storage.
      | 
      | Although the string's contents won't
      | be affected by this call, it will increase
      | the amount of memory allocated internally
      | for the string to grow into.
      | 
      | If you're about to make a large number
      | of calls to methods such as += or <<, it's
      | more efficient to preallocate enough
      | extra space beforehand, so that these
      | methods won't have to keep resizing
      | the string to append the extra characters.
      | 
      | -----------
      | @param numBytesNeeded
      | 
      | the number of bytes to allocate storage
      | for. If this value is less than the currently
      | allocated size, it will have no effect.
      |
      */
    pub fn preallocate_bytes(&mut self, num_bytes_needed: usize)  {
        
        todo!();
        /*
            text = StringHolder::makeUniqueWithByteSize (text, numBytesNeeded + sizeof (CharPointerType::CharType));
        */
    }
    
    /**
      | Returns the number of AloeString objects
      | which are currently sharing the same
      | internal data as this one.
      |
      */
    pub fn get_reference_count(&self) -> i32 {
        
        todo!();
        /*
            return StringHolder::getReferenceCount (text);
        */
    }
    
    /**
      | Creates a string from a zero-terminated
      | ascii text string.
      | 
      | The string passed-in must not contain
      | any characters with a value above 127,
      | because these can't be converted to
      | unicode without knowing the original
      | encoding that was used to create the
      | string. If you attempt to pass-in values
      | above 127, you'll get an assertion.
      | 
      | To create strings with extended characters
      | from UTF-8, you should explicitly call
      | AloeString (CharPointer_UTF8 ("my utf8
      | string..")). It's *highly* recommended
      | that you use UTF-8 with escape characters
      | in your source code to represent extended
      | characters, because there's no other
      | way to represent unicode strings in
      | a way that isn't dependent on the compiler,
      | source code editor and platform.
      |
      */
    pub fn new_from_u8ptr(t: *const u8) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (CharPointer_ASCII (t)))

        /*  If you get an assertion here, then you're trying to create a string from 8-bit data
            that contains values greater than 127. These can NOT be correctly converted to unicode
            because there's no way for the AloeString class to know what encoding was used to
            create them. The source data could be UTF-8, ASCII or one of many local code-pages.

            To get around this problem, you must be more explicit when you pass an ambiguous 8-bit
            string to the AloeString class - so for example if your source data is actually UTF-8,
            you'd call AloeString (CharPointer_UTF8 ("my utf8 string..")), and it would be able to
            correctly convert the multi-byte characters to unicode. It's *highly* recommended that
            you use UTF-8 with escape characters in your source code to represent extended characters,
            because there's no other way to represent these strings in a way that isn't dependent on
            the compiler, source code editor and platform.

            Note that the Proaloer has a handy string literal generator utility that will convert
            any unicode string to a valid C++ string literal, creating ascii escape sequences that will
            work in any compiler.
        */
        jassert (t == nullptr || CharPointer_ASCII::isValidString (t, std::numeric_limits<int>::max()));
        */
    }
    
    /**
      | Creates a string from a string of 8-bit
      | ascii characters.
      | 
      | The string passed-in must not contain
      | any characters with a value above 127,
      | because these can't be converted to
      | unicode without knowing the original
      | encoding that was used to create the
      | string. If you attempt to pass-in values
      | above 127, you'll get an assertion.
      | 
      | To create strings with extended characters
      | from UTF-8, you should explicitly call
      | AloeString (CharPointer_UTF8 ("my utf8
      | string..")). It's *highly* recommended
      | that you use UTF-8 with escape characters
      | in your source code to represent extended
      | characters, because there's no other
      | way to represent unicode strings in
      | a way that isn't dependent on the compiler,
      | source code editor and platform.
      | 
      | This will use up to the first maxChars
      | characters of the string (or less if
      | the string is actually shorter).
      |
      */
    pub fn new_from_u8ptr_with_max(
        t:         *const u8,
        max_chars: usize) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (CharPointer_ASCII (t), maxChars))

        /*  If you get an assertion here, then you're trying to create a string from 8-bit data
            that contains values greater than 127. These can NOT be correctly converted to unicode
            because there's no way for the AloeString class to know what encoding was used to
            create them. The source data could be UTF-8, ASCII or one of many local code-pages.

            To get around this problem, you must be more explicit when you pass an ambiguous 8-bit
            string to the AloeString class - so for example if your source data is actually UTF-8,
            you'd call AloeString (CharPointer_UTF8 ("my utf8 string..")), and it would be able to
            correctly convert the multi-byte characters to unicode. It's *highly* recommended that
            you use UTF-8 with escape characters in your source code to represent extended characters,
            because there's no other way to represent these strings in a way that isn't dependent on
            the compiler, source code editor and platform.

            Note that the Proaloer has a handy string literal generator utility that will convert
            any unicode string to a valid C++ string literal, creating ascii escape sequences that will
            work in any compiler.
        */
        jassert (t == nullptr || CharPointer_ASCII::isValidString (t, (int) maxChars));
        */
    }
    
    /**
      | Creates a string from a wchar_t character
      | string.
      | 
      | Depending on the platform, this may
      | be treated as either UTF-32 or UTF-16.
      |
      */
    pub fn new_from_wchar_ptr(t: *const wchar_t) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (castToCharPointer_wchar_t (t)))
        */
    }
    
    pub fn new_from_utf8ptr(t: CharPointer_UTF8) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (t))
        */
    }
    
    pub fn new_from_utf16ptr(t: CharPointer_UTF16) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (t))
        */
    }
    
    pub fn new_from_utf32ptr(t: CharPointer_UTF32) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (t))
        */
    }
    
    pub fn new_from_asciiptr(t: CharPointer_ASCII) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (t))
        */
    }
    
    pub fn new_from_utf8ptr_with_max(
        t:         CharPointer_UTF8,
        max_chars: usize) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (t, maxChars))
        */
    }
    
    pub fn new_from_utf16ptr_with_max(
        t:         CharPointer_UTF16,
        max_chars: usize) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (t, maxChars))
        */
    }
    
    pub fn new_from_utf32ptr_with_max(
        t:         CharPointer_UTF32,
        max_chars: usize) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (t, maxChars))
        */
    }
    
    /**
      | Creates a string from a wchar_t character
      | string.
      | 
      | Depending on the platform, this may
      | be treated as either UTF-32 or UTF-16.
      |
      */
    pub fn new_from_wchar_ptr_with_max(
        t:         *const wchar_t,
        max_chars: usize) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (castToCharPointer_wchar_t (t), maxChars))
        */
    }
    
    /**
      | Creates a string from a UTF-8 character
      | string
      |
      */
    pub fn new_from_utf8_range(
        start: CharPointer_UTF8,
        end:   CharPointer_UTF8) -> Self {
    
        todo!();
        /*
            : text (StringHolder::createFromCharPointer (start, end))
        */
    }
    
    /**
      | Creates a string from a UTF-16 character
      | string
      |
      */
    pub fn new_from_utf16_range(
        start: CharPointer_UTF16,
        end:   CharPointer_UTF16) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (start, end))
        */
    }
    
    /**
      | Creates a string from a UTF-32 character
      | string
      |
      */
    pub fn new_from_utf32_range(
        start: CharPointer_UTF32,
        end:   CharPointer_UTF32) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (start, end))
        */
    }
    
    /**
      | Creates a copy of another string.
      |
      */
    pub fn new_from_ref_string(s: &AloeString) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromFixedLength (s.data(), s.size()))
        */
    }
    
    /**
      | Creates a string from a &str
      |
      */
    pub fn new_from_stringref(s: &str) -> Self {
    
        todo!();
        /*


            : text (StringHolder::createFromCharPointer (s.text))
        */
    }
    
    /**
      | Creates a string from a single character.
      |
      */
    pub fn char_to_string(&mut self, character: wchar_t) -> AloeString {
        
        todo!();
        /*
            AloeString result (PreallocationBytes (CharPointerType::getBytesRequiredFor (character)));
        CharPointerType t (result.text);
        t.write (character);
        t.writeNull();
        return result;
        */
    }
}

pub mod number_to_string_converters {
    use super::*;

    pub const CHARS_NEEDED_FOR_INT:    usize = 32;
    pub const CHARS_NEEDED_FOR_DOUBLE: usize = 48;

    pub fn print_digits<Type: std::fmt::Display>(v: Type) -> String {
        v.to_string()
    }

    pub struct StackArrayStream {
        buffer: Vec<u8>,
    }

    impl StackArrayStream {
        pub fn new() -> Self {
            Self {
                buffer: Vec::with_capacity(CHARS_NEEDED_FOR_DOUBLE),
            }
        }

        pub fn write_double(
            &mut self, 
            n:                       f64, 
            num_dec_places:          usize, 
            use_scientific_notation: bool) -> usize 
        {
            let formatted = if use_scientific_notation {
                format!("{:.*e}", num_dec_places, n)
            } else {
                format!("{:.*}", num_dec_places, n)
            };

            self.buffer.extend_from_slice(formatted.as_bytes());
            formatted.len()
        }
    }

    pub fn double_to_string(
        n:                       f64,
        num_dec_places:          usize,
        use_scientific_notation: bool,
    ) -> String 
    {
        let mut stream = StackArrayStream::new();

        stream.write_double(n, num_dec_places, use_scientific_notation);

        String::from_utf8(stream.buffer).unwrap()
    }

    pub fn create_from_integer(number: isize) -> String {
        number.to_string()
    }

    pub fn create_from_double(
        number:                   f64,
        number_of_decimal_places: usize,
        use_scientific_notation:  bool,
    ) 
        -> String 
    {
        double_to_string(
            number, 
            number_of_decimal_places, 
            use_scientific_notation
        )
    }
}

impl AloeString {
    
    /**
      | Creates a string containing this signed
      | 32-bit integer as a decimal number.
      | @see getIntValue, getFloatValue,
      | getDoubleValue, toHexString
      |
      */
    pub fn new_from_i32(number: i32) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromInteger (number))
        */
    }
    
    /**
      | Creates a string containing this unsigned
      | 32-bit integer as a decimal number.
      | 
      | @see getIntValue, getFloatValue,
      | getDoubleValue, toHexString
      |
      */
    pub fn new_from_u32(number: u32) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromInteger (number))
        */
    }
    
    /**
      | Creates a string containing this signed
      | 16-bit integer as a decimal number.
      | 
      | @see getIntValue, getFloatValue,
      | getDoubleValue, toHexString
      |
      */
    pub fn new_from_i16(number: i16) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromInteger ((int) number))
        */
    }
    
    /**
      | Creates a string containing this unsigned
      | 16-bit integer as a decimal number.
      | 
      | @see getIntValue, getFloatValue,
      | getDoubleValue, toHexString
      |
      */
    pub fn new_from_u16(number: u16) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromInteger ((unsigned int) number))
        */
    }
    
    /**
      | Creates a string containing this unsigned
      | 64-bit integer as a decimal number.
      | 
      | @see getLargeIntValue, getFloatValue,
      | getDoubleValue, toHexString
      |
      */
    pub fn new_from_u64(number: u64) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromInteger (number))
        */
    }
    
    /**
      | Creates a string containing this signed
      | 64-bit integer as a decimal number.
      | 
      | @see getLargeIntValue, getFloatValue,
      | getDoubleValue, toHexString
      |
      */
    pub fn new_from_i64(number: i64) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromInteger (number))
        */
    }
    
    /**
      | Creates a string representing this
      | floating-point number.
      | 
      | -----------
      | @param floatValue
      | 
      | the value to convert to a string @see
      | getDoubleValue, getIntValue
      |
      */
    pub fn new_from_f32(number: f32) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromDouble ((double) number, 0, false))
        */
    }
    
    /**
      | Creates a string representing this
      | floating-point number.
      | 
      | -----------
      | @param doubleValue
      | 
      | the value to convert to a string @see
      | getFloatValue, getIntValue
      |
      */
    pub fn new_from_f64(number: f64) -> Self {
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromDouble (         number, 0, false))
        */
    }
    
    /**
      | Creates a string representing this
      | floating-point number.
      | 
      | -----------
      | @param floatValue
      | 
      | the value to convert to a string
      | ----------
      | @param numberOfDecimalPlaces
      | 
      | if this is > 0 the number will be formatted
      | using that many decimal places, adding
      | trailing zeros as required. If 0 or less
      | the number will be formatted using the
      | C++ standard library default format,
      | which uses scientific notation for
      | large and small numbers.
      | ----------
      | @param useScientificNotation
      | 
      | if the number should be formatted using
      | scientific notation @see getDoubleValue,
      | getIntValue
      |
      */
    pub fn new_from_scientific_f32(
        number:                   f32,
        number_of_decimal_places: i32,
        use_scientific_notation:  Option<bool>) -> Self {
    
        let use_scientific_notation: bool =
            use_scientific_notation.unwrap_or(false);

        todo!();
        /*


            : text (NumberToStringConverters::createFromDouble ((double) number, numberOfDecimalPlaces, useScientificNotation))
        */
    }
    
    /**
      | Creates a string representing this
      | floating-point number.
      | 
      | -----------
      | @param doubleValue
      | 
      | the value to convert to a string
      | ----------
      | @param numberOfDecimalPlaces
      | 
      | if this is > 0, it will format the number
      | using that many decimal places, adding
      | trailing zeros as required, and will
      | not use exponent notation. If 0 or less,
      | it will use exponent notation if necessary.
      | ----------
      | @param useScientificNotation
      | 
      | if the number should be formatted using
      | scientific notation @see getFloatValue,
      | getIntValue
      |
      */
    pub fn new_from_scientific_f64(
        number:                   f64,
        number_of_decimal_places: i32,
        use_scientific_notation:  Option<bool>) -> Self {

        let use_scientific_notation: bool =
            use_scientific_notation.unwrap_or(false);
    
        todo!();
        /*


            : text (NumberToStringConverters::createFromDouble (         number, numberOfDecimalPlaces, useScientificNotation))
        */
    }
    
    /**
      | Returns the number of characters in
      | the string.
      |
      */
    pub fn length(&self) -> usize {
        
        todo!();
        /*
            return (int) text.length();
        */
    }
    
    pub fn get_byte_offset_of_end(&self) -> usize {
        
        todo!();
        /*
            return findByteOffsetOfEnd (text);
        */
    }
}

pub fn find_byte_offset_of_end(text: &str) -> usize {
    text.len()
}

impl Index<i32> for AloeString {
    type Output = wchar_t;
    
    /**
      | Returns the character at this index
      | in the string. In a release build, no
      | checks are made to see if the index is
      | within a valid range, so be careful!
      | In a debug build, the index is checked
      | and an assertion fires if it's out-of-range.
      | 
      | Also beware that depending on the encoding
      | format that the string is using internally,
      | this method may execute in either O(1)
      | or O(n) time, so be careful when using
      | it in your algorithms. If you're scanning
      | through a string to inspect its characters,
      | you should never use this operator for
      | random access, it's far more efficient
      | to call getCharPointer() to return
      | a pointer, and then to use that to iterate
      | the string. @see getCharPointer
      |
      */
    #[inline] fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            jassert (index == 0 || (index > 0 && index <= (int) text.lengthUpTo ((size_t) index + 1)));
        return text [index];
        */
    }
}

pub struct HashGenerator<Type> {
    phantom: std::marker::PhantomData<Type>,
}

pub trait HasMultiplier {
    const MULTIPLIER: usize;
}

lazy_static!{
    /*
    impl<T> HasMultiplier for HashGenerator<T> 
    where {size_of::<T>() > 4}
    {
        const MULTIPLIER: usize = 101;
    }

    impl<T> HasMultiplier for HashGenerator<T> 
    where {size_of::<T>() <= 4}
    {
        const MULTIPLIER: usize = 31;
    }
    */
}

impl<Type> HashGenerator<Type> {

    pub fn calculate<CharPointer>(t: CharPointer) -> Type {
    
        todo!();
        /*
            Type result = {};

            while (! t.isEmpty())
                result = ((Type) multiplier) * result + (Type) t.getAndAdvance();

            return result;
        */
    }
}

impl AloeString {
    
    /**
      | Generates a probably-unique 32-bit
      | hashcode from this string.
      |
      */
    pub fn hash_code(&self) -> i32 {
        
        todo!();
        /*
            return (int) HashGenerator<uint32>    ::calculate (text);
        */
    }
    
    /**
      | Generates a probably-unique 64-bit
      | hashcode from this string.
      |
      */
    pub fn hash_code64(&self) -> i64 {
        
        todo!();
        /*
            return (int64) HashGenerator<uint64>  ::calculate (text);
        */
    }
    
    /**
      | Generates a probably-unique hashcode
      | from this string.
      |
      */
    pub fn hash(&self) -> usize {
        
        todo!();
        /*
            return HashGenerator<size_t>          ::calculate (text);
        */
    }
}


impl PartialEq<AloeString> for AloeString {
    
    #[inline] fn eq(&self, other: &AloeString) -> bool {
        todo!();
        /*
            return s1.compare (s2) == 0;
        */
    }
}

impl Eq for AloeString {}

impl PartialEq<AloeString> for *mut u8 {
    
    #[inline] fn eq(&self, other: &AloeString) -> bool {
        todo!();
        /*
            return s1.compare (s2) == 0;
        */
    }
}

impl PartialEq<AloeString> for *mut wchar_t {
    
    #[inline] fn eq(&self, other: &AloeString) -> bool {
        todo!();
        /*
            return s1.compare (s2) == 0;
        */
    }
}

impl PartialEq<AloeString> for &str {
    
    #[inline] fn eq(&self, other: &AloeString) -> bool {
        todo!();
        /*
            return s1.getCharPointer().compare (s2.text) == 0;
        */
    }
}

impl PartialEq<AloeString> for CharPointer_UTF8 {
    
    #[inline] fn eq(&self, other: &AloeString) -> bool {
        todo!();
        /*
            return s1.getCharPointer().compare (s2) == 0;
        */
    }
}

impl PartialEq<AloeString> for CharPointer_UTF16 {
    
    #[inline] fn eq(&self, other: &AloeString) -> bool {
        todo!();
        /*
            return s1.getCharPointer().compare (s2) == 0;
        */
    }
}

impl PartialEq<AloeString> for CharPointer_UTF32 {
    
    #[inline] fn eq(&self, other: &AloeString) -> bool {
        todo!();
        /*
            return s1.getCharPointer().compare (s2) == 0;
        */
    }
}

impl AloeString {
    
    /**
      | Case-insensitive comparison with
      | another string.
      |
      */
    pub fn equals_wchar_string_ignore_case(&self, t: *const wchar_t) -> bool {
        
        todo!();
        /*
            return t != nullptr ? text.compareIgnoreCase (castToCharPointer_wchar_t (t)) == 0
                            : isEmpty();
        */
    }
    
    /**
      | Case-insensitive comparison with
      | another string.
      |
      */
    pub fn equals_raw_string_ignore_case(&self, t: *const u8) -> bool {
        
        todo!();
        /*
            return t != nullptr ? text.compareIgnoreCase (CharPointer_UTF8 (t)) == 0
                            : isEmpty();
        */
    }
    
    /**
      | Case-insensitive comparison with
      | another string.
      |
      */
    pub fn equals_str_ignore_case(&self, t: &str) -> bool {
        
        todo!();
        /*
            return text.compareIgnoreCase (t.text) == 0;
        */
    }
    
    /**
      | Case-insensitive comparison with
      | another string.
      |
      */
    pub fn equals_string_ignore_case(&self, other: &AloeString) -> bool {
        
        todo!();
        /*
            return text == other.text
                || text.compareIgnoreCase (other.text) == 0;
        */
    }
    
    /**
      | Case-sensitive comparison with another
      | string.
      | 
      | -----------
      | @return
      | 
      | 0 if the two strings are identical; negative
      | if this string comes before the other
      | one alphabetically, or positive if
      | it comes after it.
      |
      */
    pub fn compare_with_other(&self, other: &AloeString) -> i32 {
        
        todo!();
        /*
            return (text == other.text) ? 0 : text.compare (other.text);
        */
    }
    
    /**
      | Case-sensitive comparison with another
      | string.
      | 
      | -----------
      | @return
      | 
      | 0 if the two strings are identical; negative
      | if this string comes before the other
      | one alphabetically, or positive if
      | it comes after it.
      |
      */
    pub fn compare_with_raw_string(&self, other: *const u8) -> i32 {
        
        todo!();
        /*
            return text.compare (CharPointer_UTF8 (other));
        */
    }
    
    /**
      | Case-sensitive comparison with another
      | string.
      | 
      | -----------
      | @return
      | 
      | 0 if the two strings are identical; negative
      | if this string comes before the other
      | one alphabetically, or positive if
      | it comes after it.
      |
      */
    pub fn compare_with_raw_wchar_string(&self, other: *const wchar_t) -> i32 {
        
        todo!();
        /*
            return text.compare (castToCharPointer_wchar_t (other));
        */
    }
    
    /**
      | Case-insensitive comparison with
      | another string.
      | 
      | -----------
      | @return
      | 
      | 0 if the two strings are identical; negative
      | if this string comes before the other
      | one alphabetically, or positive if
      | it comes after it.
      |
      */
    pub fn compare_ignore_case(&self, other: &AloeString) -> i32 {
        
        todo!();
        /*
            return (text == other.text) ? 0 : text.compareIgnoreCase (other.text);
        */
    }
}

pub fn string_compare_right(
        s1: CharPointerType,
        s2: CharPointerType) -> i32 {
    
    todo!();
    /*
        for (int bias = 0;;)
        {
            auto c1 = s1.getAndAdvance();
            bool isDigit1 = CharacterFunctions::isDigit (c1);

            auto c2 = s2.getAndAdvance();
            bool isDigit2 = CharacterFunctions::isDigit (c2);

            if (! (isDigit1 || isDigit2))   return bias;
            if (! isDigit1)                 return -1;
            if (! isDigit2)                 return 1;

            if (c1 != c2 && bias == 0)
                bias = c1 < c2 ? -1 : 1;

            jassert (c1 != 0 && c2 != 0);
        }
    */
}

pub fn string_compare_left(
        s1: CharPointerType,
        s2: CharPointerType) -> i32 {
    
    todo!();
    /*
        for (;;)
        {
            auto c1 = s1.getAndAdvance();
            bool isDigit1 = CharacterFunctions::isDigit (c1);

            auto c2 = s2.getAndAdvance();
            bool isDigit2 = CharacterFunctions::isDigit (c2);

            if (! (isDigit1 || isDigit2))   return 0;
            if (! isDigit1)                 return -1;
            if (! isDigit2)                 return 1;
            if (c1 < c2)                    return -1;
            if (c1 > c2)                    return 1;
        }
    */
}

pub fn natural_string_compare(
    s1:                CharPointerType,
    s2:                CharPointerType,
    is_case_sensitive: bool) -> i32 {

    todo!();
    /*
        bool firstLoop = true;

        for (;;)
        {
            const bool hasSpace1 = s1.isWhitespace();
            const bool hasSpace2 = s2.isWhitespace();

            if ((! firstLoop) && (hasSpace1 ^ hasSpace2))
            {
                if (s1.isEmpty())  return -1;
                if (s2.isEmpty())  return 1;

                return hasSpace2 ? 1 : -1;
            }

            firstLoop = false;

            if (hasSpace1)  s1 = s1.findEndOfWhitespace();
            if (hasSpace2)  s2 = s2.findEndOfWhitespace();

            if (s1.isDigit() && s2.isDigit())
            {
                auto result = (*s1 == '0' || *s2 == '0') ? stringCompareLeft  (s1, s2)
                                                         : stringCompareRight (s1, s2);

                if (result != 0)
                    return result;
            }

            auto c1 = s1.getAndAdvance();
            auto c2 = s2.getAndAdvance();

            if (c1 != c2 && ! isCaseSensitive)
            {
                c1 = CharacterFunctions::toUpperCase (c1);
                c2 = CharacterFunctions::toUpperCase (c2);
            }

            if (c1 == c2)
            {
                if (c1 == 0)
                    return 0;
            }
            else
            {
                const bool isAlphaNum1 = CharacterFunctions::isLetterOrDigit (c1);
                const bool isAlphaNum2 = CharacterFunctions::isLetterOrDigit (c2);

                if (isAlphaNum2 && ! isAlphaNum1) return -1;
                if (isAlphaNum1 && ! isAlphaNum2) return 1;

                return c1 < c2 ? -1 : 1;
            }

            jassert (c1 != 0 && c2 != 0);
        }
    */
}

impl AloeString {
    
    /**
      | Compares two strings, taking into account
      | textual characteristics like numbers
      | and spaces.
      | 
      | This comparison is case-insensitive
      | and can detect words and embedded numbers
      | in the strings, making it good for sorting
      | human-readable lists of things like
      | filenames.
      | 
      | -----------
      | @return
      | 
      | 0 if the two strings are identical; negative
      | if this string comes before the other
      | one alphabetically, or positive if
      | it comes after it.
      |
      */
    pub fn compare_natural(
        &self, 
        other:             &str,
        is_case_sensitive: Option<bool>) -> i32 
    {
        let is_case_sensitive: bool = is_case_sensitive.unwrap_or(false);
        
        todo!();
        /*
            return naturalStringCompare (getCharPointer(), other.text, isCaseSensitive);
        */
    }
    
    /**
      | Appends a string to the end of this one.
      | 
      | -----------
      | @param textToAppend
      | 
      | the string to add
      | ----------
      | @param maxCharsToTake
      | 
      | the maximum number of characters to
      | take from the string passed in
      |
      */
    pub fn append(&mut self, 
        text_to_append:    &AloeString,
        max_chars_to_take: usize)  {
        
        todo!();
        /*
            appendCharPointer (this == &textToAppend ? AloeString (textToAppend).text
                                                 : textToAppend.text, maxCharsToTake);
        */
    }
    
    /**
      | Appends a string to the end of this one.
      | 
      | -----------
      | @param startOfTextToAppend
      | 
      | the start of the string to add. This must
      | not be a nullptr
      | ----------
      | @param endOfTextToAppend
      | 
      | the end of the string to add. This must
      | not be a nullptr
      |
      */
    pub fn append_char_pointer_with_range(&mut self, 
        start_of_text_to_append: CharPointerType,
        end_of_text_to_append:   CharPointerType)  {
        
        todo!();
        /*
            jassert (startOfTextToAppend.getAddress() != nullptr && endOfTextToAppend.getAddress() != nullptr);

        auto extraBytesNeeded = getAddressDifference (endOfTextToAppend.getAddress(),
                                                      startOfTextToAppend.getAddress());
        jassert (extraBytesNeeded >= 0);

        if (extraBytesNeeded > 0)
        {
            auto byteOffsetOfNull = getByteOffsetOfEnd();
            preallocateBytes ((size_t) extraBytesNeeded + byteOffsetOfNull);

            auto* newStringStart = addBytesToPointer (text.getAddress(), (int) byteOffsetOfNull);
            memcpy (newStringStart, startOfTextToAppend.getAddress(), (size_t) extraBytesNeeded);
            CharPointerType (addBytesToPointer (newStringStart, extraBytesNeeded)).writeNull();
        }
        */
    }
}

/*
impl AddAssign<&wchar_t> for AloeString {
    
    #[inline] fn add_assign(&mut self, other: &wchar_t) {
        todo!();
        /*
        appendCharPointer (castToCharPointer_wchar_t (t));
        return *this;
        */
    }
}
*/

impl AddAssign<*const u8> for AloeString {
    
    #[inline] fn add_assign(&mut self, other: *const u8) {
        todo!();
        /*
        appendCharPointer (CharPointer_UTF8 (t)); // (using UTF8 here triggers a faster code-path than ascii)
        return *this;
        */
    }
}

impl AddAssign<&AloeString> for AloeString {
    
    /**
      | Appends another string at the end of
      | this one.
      |
      */
    #[inline] fn add_assign(&mut self, other: &AloeString) {
        todo!();
        /*
            if (isEmpty())
            return operator= (other);

        if (this == &other)
            return operator+= (AloeString (*this));

        appendCharPointer (other.text);
        return *this;
        */
    }
}

impl AddAssign<&str> for AloeString {
    
    #[inline] fn add_assign(&mut self, other: &str) {
        todo!();
        /*
            return operator+= (AloeString (other));
        */
    }
}

impl AddAssign<&u8> for AloeString {
    
    #[inline] fn add_assign(&mut self, other: &u8) {
        todo!();
        /*
            const char asString[] = { ch, 0 };
        return operator+= (asString);
        */
    }
}

/*
impl AddAssign<wchar_t> for AloeString {
    
    #[cfg(ALOE_NATIVE_WCHAR_IS_UTF32)]
    #[inline] fn add_assign(&mut self, other: &wchar_t) {
        todo!();
        /*
            const wchar_t asString[] = { ch, 0 };
        return operator+= (asString);
        */
    }
}

impl AddAssign<wchar_t> for AloeString {
    
    #[cfg(not(ALOE_NATIVE_WCHAR_IS_UTF32))]
    #[inline] fn add_assign(&mut self, other: &wchar_t) {
        todo!();
        /*
            const aloe_wchar asString[] = { ch, 0 };
        appendCharPointer (CharPointer_UTF32 (asString));
        return *this;
        */
    }
}
*/

pub mod string_helpers {
    use super::*;

    #[inline] pub fn operation_add_assign<T>(
            str_:   &mut AloeString,
            number: T) -> &mut AloeString {

        todo!();
        /*
            char buffer [(sizeof(T) * 8) / 2];
                auto* end = buffer + numElementsInArray (buffer);
                auto* start = NumberToStringConverters::numberToString (end, number);

               #if ALOE_STRING_UTF_TYPE == 8
                str.appendCharPointer (CharPointerType (start), CharPointerType (end));
               #else
                str.appendCharPointer (CharPointer_ASCII (start), CharPointer_ASCII (end));
               #endif

                return str;
        */
    }
}

impl AddAssign<&i32> for AloeString {
    
    #[inline] fn add_assign(&mut self, other: &i32) {
        todo!();
        /*
            return StringHelpers::operationAddAssign<int>    (*this, number);
        */
    }
}

impl AddAssign<&i64> for AloeString {
    
    #[inline] fn add_assign(&mut self, other: &i64) {
        todo!();
        /*
            return StringHelpers::operationAddAssign<int64>  (*this, number);
        */
    }
}

impl AddAssign<&u64> for AloeString {
    
    #[inline] fn add_assign(&mut self, other: &u64) {
        todo!();
        /*
            return StringHelpers::operationAddAssign<uint64> (*this, number);
        */
    }
}

lazy_static!{
    /*
    AloeString  operator+ (const char* s1, const AloeString& s2)    { AloeString s (s1); return s += s2; }
     AloeString  operator+ (const wchar_t* s1, const AloeString& s2) { AloeString s (s1); return s += s2; }

     AloeString  operator+ (char s1, const AloeString& s2)           { return AloeString::charToString ((aloe_wchar) (uint8) s1) + s2; }
     AloeString  operator+ (wchar_t s1, const AloeString& s2)        { return AloeString::charToString (s1) + s2; }

     AloeString  operator+ (AloeString s1, const AloeString& s2)         { return s1 += s2; }
     AloeString  operator+ (AloeString s1, const char* s2)           { return s1 += s2; }
     AloeString  operator+ (AloeString s1, const wchar_t* s2)        { return s1 += s2; }
     AloeString  operator+ (AloeString s1, const std::string& s2)    { return s1 += s2.c_str(); }

     AloeString  operator+ (AloeString s1, char s2)                  { return s1 += s2; }
     AloeString  operator+ (AloeString s1, wchar_t s2)               { return s1 += s2; }

    #if ! ALOE_NATIVE_WCHAR_IS_UTF32
     AloeString  operator+ (aloe_wchar s1, const AloeString& s2)     { return AloeString::charToString (s1) + s2; }
     AloeString  operator+ (AloeString s1, aloe_wchar s2)            { return s1 += s2; }
     AloeString&  operator<< (AloeString& s1, aloe_wchar s2)         { return s1 += s2; }
    #endif

     AloeString&  operator<< (AloeString& s1, char s2)               { return s1 += s2; }
     AloeString&  operator<< (AloeString& s1, wchar_t s2)            { return s1 += s2; }

     AloeString&  operator<< (AloeString& s1, const char* s2)        { return s1 += s2; }
     AloeString&  operator<< (AloeString& s1, const wchar_t* s2)     { return s1 += s2; }
     AloeString&  operator<< (AloeString& s1, const AloeString& s2)      { return s1 += s2; }
     AloeString&  operator<< (AloeString& s1, &str s2)          { return s1 += s2; }
     AloeString&  operator<< (AloeString& s1, const std::string& s2) { return s1 += s2.c_str(); }

     AloeString&  operator<< (AloeString& s1, uint8  number)         { return s1 += (int) number; }
     AloeString&  operator<< (AloeString& s1, short  number)         { return s1 += (int) number; }
     AloeString&  operator<< (AloeString& s1, int    number)         { return s1 += number; }
     AloeString&  operator<< (AloeString& s1, long   number)         { return s1 += AloeString (number); }
     AloeString&  operator<< (AloeString& s1, unsigned long number)  { return s1 += AloeString (number); }
     AloeString&  operator<< (AloeString& s1, int64  number)         { return s1 += AloeString (number); }
     AloeString&  operator<< (AloeString& s1, uint64 number)         { return s1 += AloeString (number); }
     AloeString&  operator<< (AloeString& s1, float  number)         { return s1 += AloeString (number); }
     AloeString&  operator<< (AloeString& s1, double number)         { return s1 += AloeString (number); }
    */
}

impl AloeString {
    
    /**
      | Searches for a character inside this
      | string. Uses a case-sensitive comparison.
      | 
      | -----------
      | @return
      | 
      | the index of the first occurrence of
      | the character in this string, or -1 if
      | it's not found.
      |
      */
    pub fn index_of_char(&self, character: wchar_t) -> i32 {
        
        todo!();
        /*
            return text.indexOf (character);
        */
    }
    
    /**
      | Searches for a character inside this
      | string.
      | 
      | Uses a case-sensitive comparison.
      | 
      | -----------
      | @param startIndex
      | 
      | the index from which the search should
      | proceed
      | ----------
      | @param characterToLookFor
      | 
      | the character to look for
      | 
      | -----------
      | @return
      | 
      | the index of the first occurrence of
      | the character in this string, or -1 if
      | it's not found.
      |
      */
    pub fn index_of_char_from_start(&self, 
        start_index: i32,
        character:   wchar_t) -> i32 {
        
        todo!();
        /*
            auto t = text;

        for (int i = 0; ! t.isEmpty(); ++i)
        {
            if (i >= startIndex)
            {
                if (t.getAndAdvance() == character)
                    return i;
            }
            else
            {
                ++t;
            }
        }

        return -1;
        */
    }
    
    /**
      | Searches for a character inside this
      | string (working backwards from the
      | end of the string).
      | 
      | Uses a case-sensitive comparison.
      | 
      | -----------
      | @return
      | 
      | the index of the last occurrence of the
      | character in this string, or -1 if it's
      | not found.
      |
      */
    pub fn last_index_of_char(&self, character: wchar_t) -> i32 {
        
        todo!();
        /*
            auto t = text;
        int last = -1;

        for (int i = 0; ! t.isEmpty(); ++i)
            if (t.getAndAdvance() == character)
                last = i;

        return last;
        */
    }
    
    /**
      | Returns the index of the first character
      | that matches one of the characters passed-in
      | to this method.
      | 
      | This scans the string, beginning from
      | the startIndex supplied, and if it finds
      | a character that appears in the string
      | charactersToLookFor, it returns its
      | index.
      | 
      | If none of these characters are found,
      | it returns -1.
      | 
      | If ignoreCase is true, the comparison
      | will be case-insensitive.
      | 
      | @see indexOfChar, lastIndexOfAnyOf
      |
      */
    pub fn index_of_any_of(
        &self, 
        characters_to_look_for: &str,
        start_index:            Option<i32>,
        ignore_case:            Option<bool>
    ) -> i32 
    {
        let start_index: i32 = start_index.unwrap_or(0);
        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            auto t = text;

        for (int i = 0; ! t.isEmpty(); ++i)
        {
            if (i >= startIndex)
            {
                if (charactersToLookFor.text.indexOf (t.getAndAdvance(), ignoreCase) >= 0)
                    return i;
            }
            else
            {
                ++t;
            }
        }

        return -1;
        */
    }
    
    /**
      | Searches for a substring within this
      | string.
      | 
      | Uses a case-sensitive comparison.
      | 
      | -----------
      | @return
      | 
      | the index of the first occurrence of
      | this substring, or -1 if it's not found.
      | If textToLookFor is an empty string,
      | this will always return 0.
      |
      */
    pub fn index_of(&self, other: &str) -> i32 {
        
        todo!();
        /*
            return other.isEmpty() ? 0 : text.indexOf (other.text);
        */
    }
    
    /**
      | Searches for a substring within this
      | string.
      | 
      | Uses a case-insensitive comparison.
      | 
      | -----------
      | @return
      | 
      | the index of the first occurrence of
      | this substring, or -1 if it's not found.
      | If textToLookFor is an empty string,
      | this will always return 0.
      |
      */
    pub fn index_of_ignore_case(&self, other: &str) -> i32 {
        
        todo!();
        /*
            return other.isEmpty() ? 0 : CharacterFunctions::indexOfIgnoreCase (text, other.text);
        */
    }
    
    /**
      | Searches for a substring within this
      | string.
      | 
      | Uses a case-sensitive comparison.
      | 
      | -----------
      | @param startIndex
      | 
      | the index from which the search should
      | proceed
      | ----------
      | @param textToLookFor
      | 
      | the string to search for
      | 
      | -----------
      | @return
      | 
      | the index of the first occurrence of
      | this substring, or -1 if it's not found.
      | If textToLookFor is an empty string,
      | this will always return -1.
      |
      */
    pub fn index_of_other_from_start(&self, 
        start_index: i32,
        other:       &str) -> i32 {
        
        todo!();
        /*
            if (other.isEmpty())
            return -1;

        auto t = text;

        for (int i = startIndex; --i >= 0;)
        {
            if (t.isEmpty())
                return -1;

            ++t;
        }

        auto found = t.indexOf (other.text);
        return found >= 0 ? found + startIndex : found;
        */
    }
    
    /**
      | Searches for a substring within this
      | string.
      | 
      | Uses a case-insensitive comparison.
      | 
      | -----------
      | @param startIndex
      | 
      | the index from which the search should
      | proceed
      | ----------
      | @param textToLookFor
      | 
      | the string to search for
      | 
      | -----------
      | @return
      | 
      | the index of the first occurrence of
      | this substring, or -1 if it's not found.
      | If textToLookFor is an empty string,
      | this will always return -1.
      |
      */
    pub fn index_of_other_from_start_ignore_case(&self, 
        start_index: i32,
        other:       &str) -> i32 {
        
        todo!();
        /*
            if (other.isEmpty())
            return -1;

        auto t = text;

        for (int i = startIndex; --i >= 0;)
        {
            if (t.isEmpty())
                return -1;

            ++t;
        }

        auto found = CharacterFunctions::indexOfIgnoreCase (t, other.text);
        return found >= 0 ? found + startIndex : found;
        */
    }
    
    /**
      | Searches for a substring inside this
      | string (working backwards from the
      | end of the string).
      | 
      | Uses a case-sensitive comparison.
      | 
      | -----------
      | @return
      | 
      | the index of the start of the last occurrence
      | of the substring within this string,
      | or -1 if it's not found. If textToLookFor
      | is an empty string, this will always
      | return -1.
      |
      */
    pub fn last_index_of(&self, other: &str) -> i32 {
        
        todo!();
        /*
            if (other.isNotEmpty())
        {
            auto len = other.length();
            int i = length() - len;

            if (i >= 0)
            {
                for (auto n = text + i; i >= 0; --i)
                {
                    if (n.compareUpTo (other.text, len) == 0)
                        return i;

                    --n;
                }
            }
        }

        return -1;
        */
    }
    
    /**
      | Searches for a substring inside this
      | string (working backwards from the
      | end of the string).
      | 
      | Uses a case-insensitive comparison.
      | 
      | -----------
      | @return
      | 
      | the index of the start of the last occurrence
      | of the substring within this string,
      | or -1 if it's not found. If textToLookFor
      | is an empty string, this will always
      | return -1.
      |
      */
    pub fn last_index_of_ignore_case(&self, other: &str) -> i32 {
        
        todo!();
        /*
            if (other.isNotEmpty())
        {
            auto len = other.length();
            int i = length() - len;

            if (i >= 0)
            {
                for (auto n = text + i; i >= 0; --i)
                {
                    if (n.compareIgnoreCaseUpTo (other.text, len) == 0)
                        return i;

                    --n;
                }
            }
        }

        return -1;
        */
    }
    
    /**
      | Returns the index of the last character
      | in this string that matches one of the
      | characters passed-in to this method.
      | 
      | This scans the string backwards, starting
      | from its end, and if it finds a character
      | that appears in the string charactersToLookFor,
      | it returns its index.
      | 
      | If none of these characters are found,
      | it returns -1.
      | 
      | If ignoreCase is true, the comparison
      | will be case-insensitive.
      | 
      | @see lastIndexOf, indexOfAnyOf
      |
      */
    pub fn last_index_of_any_of(
        &self, 
        characters_to_look_for: &str,
        ignore_case:            Option<bool>) -> i32 
    {
        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            auto t = text;
        int last = -1;

        for (int i = 0; ! t.isEmpty(); ++i)
            if (charactersToLookFor.text.indexOf (t.getAndAdvance(), ignoreCase) >= 0)
                last = i;

        return last;
        */
    }
    
    /**
      | Tests whether the string contains another
      | substring.
      | 
      | If the parameter is an empty string,
      | this will always return true.
      | 
      | Uses a case-sensitive comparison.
      |
      */
    pub fn contains(&self, other: &str) -> bool {
        
        todo!();
        /*
            return indexOf (other) >= 0;
        */
    }
    
    /**
      | Tests whether the string contains a
      | particular character.
      | 
      | Uses a case-sensitive comparison.
      |
      */
    pub fn contains_char(&self, character: wchar_t) -> bool {
        
        todo!();
        /*
            return text.indexOf (character) >= 0;
        */
    }
    
    /**
      | Tests whether the string contains another
      | substring.
      | 
      | Uses a case-insensitive comparison.
      |
      */
    pub fn contains_ignore_case(&self, t: &str) -> bool {
        
        todo!();
        /*
            return indexOfIgnoreCase (t) >= 0;
        */
    }
    
    /**
      | Finds an instance of another substring
      | if it exists as a distinct word.
      | 
      | -----------
      | @return
      | 
      | if the string contains this word, surrounded
      | by non-alphanumeric characters, then
      | this will return the index of the start
      | of the substring. If it isn't found,
      | then it will return -1 @see indexOfWholeWordIgnoreCase,
      | containsWholeWord
      |
      */
    pub fn index_of_whole_word(&self, word: &str) -> i32 {
        
        todo!();
        /*
            if (word.isNotEmpty())
        {
            auto t = text;
            auto wordLen = word.length();
            auto end = (int) t.length() - wordLen;

            for (int i = 0; i <= end; ++i)
            {
                if (t.compareUpTo (word.text, wordLen) == 0
                      && (i == 0 || ! (t - 1).isLetterOrDigit())
                      && ! (t + wordLen).isLetterOrDigit())
                    return i;

                ++t;
            }
        }

        return -1;
        */
    }
    
    /**
      | Finds an instance of another substring
      | if it exists as a distinct word.
      | 
      | -----------
      | @return
      | 
      | if the string contains this word, surrounded
      | by non-alphanumeric characters, then
      | this will return the index of the start
      | of the substring. If it isn't found,
      | then it will return -1 @see indexOfWholeWord,
      | containsWholeWordIgnoreCase
      |
      */
    pub fn index_of_whole_word_ignore_case(&self, word: &str) -> i32 {
        
        todo!();
        /*
            if (word.isNotEmpty())
        {
            auto t = text;
            auto wordLen = word.length();
            auto end = (int) t.length() - wordLen;

            for (int i = 0; i <= end; ++i)
            {
                if (t.compareIgnoreCaseUpTo (word.text, wordLen) == 0
                      && (i == 0 || ! (t - 1).isLetterOrDigit())
                      && ! (t + wordLen).isLetterOrDigit())
                    return i;

                ++t;
            }
        }

        return -1;
        */
    }
    
    /**
      | Tests whether the string contains another
      | substring as a distinct word.
      | 
      | -----------
      | @return
      | 
      | true if the string contains this word,
      | surrounded by non-alphanumeric characters
      | @see indexOfWholeWord, containsWholeWordIgnoreCase
      |
      */
    pub fn contains_whole_word(&self, word_to_look_for: &str) -> bool {
        
        todo!();
        /*
            return indexOfWholeWord (wordToLookFor) >= 0;
        */
    }
    
    /**
      | Tests whether the string contains another
      | substring as a distinct word.
      | 
      | -----------
      | @return
      | 
      | true if the string contains this word,
      | surrounded by non-alphanumeric characters
      | @see indexOfWholeWordIgnoreCase,
      | containsWholeWord
      |
      */
    pub fn contains_whole_word_ignore_case(&self, word_to_look_for: &str) -> bool {
        
        todo!();
        /*
            return indexOfWholeWordIgnoreCase (wordToLookFor) >= 0;
        */
    }
}

///-----------------------
pub struct WildCardMatcher<CharPointer> {
    phantom: std::marker::PhantomData<CharPointer>,
}

impl<CharPointer> WildCardMatcher<CharPointer> {

    pub fn matches(
        wildcard:    CharPointer,
        test:        CharPointer,
        ignore_case: bool) -> bool {
        
        todo!();
        /*
            for (;;)
            {
                auto wc = wildcard.getAndAdvance();

                if (wc == '*')
                    return wildcard.isEmpty() || matchesAnywhere (wildcard, test, ignoreCase);

                if (! characterMatches (wc, test.getAndAdvance(), ignoreCase))
                    return false;

                if (wc == 0)
                    return true;
            }
        */
    }
    
    pub fn character_matches(
        wc:          wchar_t,
        tc:          wchar_t,
        ignore_case: bool) -> bool {
        
        todo!();
        /*
            return (wc == tc) || (wc == '?' && tc != 0)
                    || (ignoreCase && CharacterFunctions::toLowerCase (wc) == CharacterFunctions::toLowerCase (tc));
        */
    }
    
    pub fn matches_anywhere(
        wildcard:    CharPointer,
        test:        CharPointer,
        ignore_case: bool) -> bool {
        
        todo!();
        /*
            for (; ! test.isEmpty(); ++test)
                if (matches (wildcard, test, ignoreCase))
                    return true;

            return false;
        */
    }
}

impl AloeString {
    
    /**
      | Returns true if the string matches this
      | simple wildcard expression.
      | 
      | So for example AloeString ("abcdef").matchesWildcard
      | ("*DEF", true) would return true.
      | 
      | This isn't a full-blown regex though!
      | The only wildcard characters supported
      | are "*" and "?". It's mainly intended
      | for filename pattern matching.
      |
      */
    pub fn matches_wildcard(&self, 
        wildcard:    &str,
        ignore_case: bool) -> bool {
        
        todo!();
        /*
            return WildCardMatcher<CharPointerType>::matches (wildcard.text, text, ignoreCase);
        */
    }
    
    /**
      | Creates a string which is a version of
      | a string repeated and joined together.
      | 
      | -----------
      | @param stringToRepeat
      | 
      | the string to repeat
      | ----------
      | @param numberOfTimesToRepeat
      | 
      | how many times to repeat it
      |
      */
    pub fn repeated_string(&mut self, 
        string_to_repeat:          &str,
        number_of_times_to_repeat: i32) -> AloeString {
        
        todo!();
        /*
            if (numberOfTimesToRepeat <= 0)
            return {};

        AloeString result (PreallocationBytes (findByteOffsetOfEnd (stringToRepeat) * (size_t) numberOfTimesToRepeat));
        auto n = result.text;

        while (--numberOfTimesToRepeat >= 0)
            n.writeAll (stringToRepeat.text);

        return result;
        */
    }
    
    /**
      | Returns a copy of this string with the
      | specified character repeatedly added
      | to its beginning until the total length
      | is at least the minimum length specified.
      |
      */
    pub fn padded_left(&self, 
        pad_character:  wchar_t,
        minimum_length: i32) -> AloeString {
        
        todo!();
        /*
            jassert (padCharacter != 0);

        auto extraChars = minimumLength;
        auto end = text;

        while (! end.isEmpty())
        {
            --extraChars;
            ++end;
        }

        if (extraChars <= 0 || padCharacter == 0)
            return *this;

        auto currentByteSize = (size_t) (((char*) end.getAddress()) - (char*) text.getAddress());
        AloeString result (PreallocationBytes (currentByteSize + (size_t) extraChars * CharPointerType::getBytesRequiredFor (padCharacter)));
        auto n = result.text;

        while (--extraChars >= 0)
            n.write (padCharacter);

        n.writeAll (text);
        return result;
        */
    }
    
    /**
      | Returns a copy of this string with the
      | specified character repeatedly added
      | to its end until the total length is at
      | least the minimum length specified.
      |
      */
    pub fn padded_right(&self, 
        pad_character:  wchar_t,
        minimum_length: i32) -> AloeString {
        
        todo!();
        /*
            jassert (padCharacter != 0);

        auto extraChars = minimumLength;
        CharPointerType end (text);

        while (! end.isEmpty())
        {
            --extraChars;
            ++end;
        }

        if (extraChars <= 0 || padCharacter == 0)
            return *this;

        auto currentByteSize = (size_t) (((char*) end.getAddress()) - (char*) text.getAddress());
        AloeString result (PreallocationBytes (currentByteSize + (size_t) extraChars * CharPointerType::getBytesRequiredFor (padCharacter)));
        auto n = result.text;

        n.writeAll (text);

        while (--extraChars >= 0)
            n.write (padCharacter);

        n.writeNull();
        return result;
        */
    }
    
    /**
      | Replaces a sub-section of the string
      | with another string.
      | 
      | This will return a copy of this string,
      | with a set of characters from startIndex
      | to startIndex + numCharsToReplace
      | removed, and with a new string inserted
      | in their place.
      | 
      | Note that this is a const method, and
      | won't alter the string itself.
      | 
      | -----------
      | @param startIndex
      | 
      | the first character to remove. If this
      | is beyond the bounds of the string, it
      | will be constrained to a valid range.
      | ----------
      | @param numCharactersToReplace
      | 
      | the number of characters to remove.
      | If zero or less, no characters will be
      | taken out.
      | ----------
      | @param stringToInsert
      | 
      | the new string to insert at startIndex
      | after the characters have been removed.
      |
      */
    pub fn replace_section(&self, 
        index:                i32,
        num_chars_to_replace: i32,
        string_to_insert:     &str) -> AloeString {
        
        todo!();
        /*
            if (index < 0)
        {
            // a negative index to replace from?
            jassertfalse;
            index = 0;
        }

        if (numCharsToReplace < 0)
        {
            // replacing a negative number of characters?
            numCharsToReplace = 0;
            jassertfalse;
        }

        auto insertPoint = text;

        for (int i = 0; i < index; ++i)
        {
            if (insertPoint.isEmpty())
            {
                // replacing beyond the end of the string?
                jassertfalse;
                return *this + stringToInsert;
            }

            ++insertPoint;
        }

        auto startOfRemainder = insertPoint;

        for (int i = 0; i < numCharsToReplace && ! startOfRemainder.isEmpty(); ++i)
            ++startOfRemainder;

        if (insertPoint == text && startOfRemainder.isEmpty())
            return stringToInsert.text;

        auto initialBytes = (size_t) (((char*) insertPoint.getAddress()) - (char*) text.getAddress());
        auto newStringBytes = findByteOffsetOfEnd (stringToInsert);
        auto remainderBytes = (size_t) (((char*) startOfRemainder.findTerminatingNull().getAddress()) - (char*) startOfRemainder.getAddress());

        auto newTotalBytes = initialBytes + newStringBytes + remainderBytes;

        if (newTotalBytes <= 0)
            return {};

        AloeString result (PreallocationBytes ((size_t) newTotalBytes));

        auto* dest = (char*) result.text.getAddress();
        memcpy (dest, text.getAddress(), initialBytes);
        dest += initialBytes;
        memcpy (dest, stringToInsert.text.getAddress(), newStringBytes);
        dest += newStringBytes;
        memcpy (dest, startOfRemainder.getAddress(), remainderBytes);
        dest += remainderBytes;
        CharPointerType ((CharPointerType::CharType*) dest).writeNull();

        return result;
        */
    }
    
    /**
      | Replaces all occurrences of a substring
      | with another string.
      | 
      | Returns a copy of this string, with any
      | occurrences of stringToReplace swapped
      | for stringToInsertInstead.
      | 
      | Note that this is a const method, and
      | won't alter the string itself.
      |
      */
    pub fn replace(
        &self, 
        string_to_replace: &str,
        string_to_insert:  &str,
        ignore_case:       Option<bool>) -> AloeString 
    {
        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            auto stringToReplaceLen = stringToReplace.length();
        auto stringToInsertLen = stringToInsert.length();

        int i = 0;
        AloeString result (*this);

        while ((i = (ignoreCase ? result.indexOfIgnoreCase (i, stringToReplace)
                                : result.indexOf (i, stringToReplace))) >= 0)
        {
            result = result.replaceSection (i, stringToReplaceLen, stringToInsert);
            i += stringToInsertLen;
        }

        return result;
        */
    }
    
    /**
      | Replaces the first occurrence of a substring
      | with another string.
      | 
      | Returns a copy of this string, with the
      | first occurrence of stringToReplace
      | swapped for stringToInsertInstead.
      | 
      | Note that this is a const method, and
      | won't alter the string itself.
      |
      */
    pub fn replace_first_occurrence_of(
        &self, 
        string_to_replace: &str,
        string_to_insert:  &str,
        ignore_case:       Option<bool>) -> AloeString 
    {
        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            auto stringToReplaceLen = stringToReplace.length();
        auto index = ignoreCase ? indexOfIgnoreCase (stringToReplace)
                                : indexOf (stringToReplace);

        if (index >= 0)
            return replaceSection (index, stringToReplaceLen, stringToInsert);

        return *this;
        */
    }
}

///-------------------------
pub struct StringCreationHelper {
    result:          AloeString,
    source:          CharPointerType, // default = nullptr 
    dest:            CharPointerType, // default = nullptr 
    allocated_bytes: usize,
    bytes_written:   usize, // default = 0
}

impl StringCreationHelper {

    pub fn new_from_initial_bytes(initial_bytes: usize) -> Self {
    
        todo!();
        /*
        : allocated_bytes(initialBytes),

            result.preallocateBytes (allocatedBytes);
            dest = result.getCharPointer();
        */
    }
    
    pub fn new_from_char_ptr(s: CharPointerType) -> Self {
    
        todo!();
        /*


            : source (s), allocatedBytes (StringHolder::getAllocatedNumBytes (s))

            result.preallocateBytes (allocatedBytes);
            dest = result.getCharPointer();
        */
    }
    
    pub fn write(&mut self, c: wchar_t)  {
        
        todo!();
        /*
            bytesWritten += CharPointerType::getBytesRequiredFor (c);

            if (bytesWritten > allocatedBytes)
            {
                allocatedBytes += jmax ((size_t) 8, allocatedBytes / 16);
                auto destOffset = (size_t) (((char*) dest.getAddress()) - (char*) result.getCharPointer().getAddress());
                result.preallocateBytes (allocatedBytes);
                dest = addBytesToPointer (result.getCharPointer().getAddress(), (int) destOffset);
            }

            dest.write (c);
        */
    }
}

pub fn is_quote_character(c: wchar_t) -> bool {
    
    todo!();
    /*
        return c == '"' || c == '\'';
    */
}

impl AloeString {
    
    /**
      | Returns a string with all occurrences
      | of a character replaced with a different
      | one.
      |
      */
    pub fn replace_character(&self, 
        char_to_replace: wchar_t,
        char_to_insert:  wchar_t) -> AloeString {
        
        todo!();
        /*
            if (! containsChar (charToReplace))
            return *this;

        StringCreationHelper builder (text);

        for (;;)
        {
            auto c = builder.source.getAndAdvance();

            if (c == charToReplace)
                c = charToInsert;

            builder.write (c);

            if (c == 0)
                break;
        }

        return std::move (builder.result);
        */
    }
    
    /**
      | Replaces a set of characters with another
      | set.
      | 
      | Returns a string in which each character
      | from charactersToReplace has been
      | replaced by the character at the equivalent
      | position in newCharacters (so the two
      | strings passed in must be the same length).
      | 
      | e.g. replaceCharacters ("abc", "def")
      | replaces 'a' with 'd', 'b' with 'e',
      | etc.
      | 
      | Note that this is a const method, and
      | won't affect the string itself.
      |
      */
    pub fn replace_characters(&self, 
        characters_to_replace:        &str,
        characters_to_insert_instead: &str) -> AloeString {
        
        todo!();
        /*
            // Each character in the first string must have a matching one in the
        // second, so the two strings must be the same length.
        jassert (charactersToReplace.length() == charactersToInsertInstead.length());

        StringCreationHelper builder (text);

        for (;;)
        {
            auto c = builder.source.getAndAdvance();
            auto index = charactersToReplace.text.indexOf (c);

            if (index >= 0)
                c = charactersToInsertInstead [index];

            builder.write (c);

            if (c == 0)
                break;
        }

        return std::move (builder.result);
        */
    }
    
    /**
      | Tests whether the string begins with
      | another string.
      | 
      | If the parameter is an empty string,
      | this will always return true.
      | 
      | Uses a case-sensitive comparison.
      |
      */
    pub fn starts_with(&self, other: &str) -> bool {
        
        todo!();
        /*
            return text.compareUpTo (other.text, other.length()) == 0;
        */
    }
    
    /**
      | Tests whether the string begins with
      | another string.
      | 
      | If the parameter is an empty string,
      | this will always return true.
      | 
      | Uses a case-insensitive comparison.
      |
      */
    pub fn starts_with_ignore_case(&self, other: &str) -> bool {
        
        todo!();
        /*
            return text.compareIgnoreCaseUpTo (other.text, other.length()) == 0;
        */
    }
    
    /**
      | Tests whether the string begins with
      | a particular character.
      | 
      | If the character is 0, this will always
      | return false.
      | 
      | Uses a case-sensitive comparison.
      |
      */
    pub fn starts_with_char(&self, character: wchar_t) -> bool {
        
        todo!();
        /*
            jassert (character != 0); // strings can't contain a null character!

        return *text == character;
        */
    }
    
    /**
      | Tests whether the string ends with a
      | particular character.
      | 
      | If the character is 0, this will always
      | return false.
      | 
      | Uses a case-sensitive comparison.
      |
      */
    pub fn ends_with_char(&self, character: wchar_t) -> bool {
        
        todo!();
        /*
            jassert (character != 0); // strings can't contain a null character!

        if (text.isEmpty())
            return false;

        auto t = text.findTerminatingNull();
        return *--t == character;
        */
    }
    
    /**
      | Tests whether the string ends with another
      | string.
      | 
      | If the parameter is an empty string,
      | this will always return true.
      | 
      | Uses a case-sensitive comparison.
      |
      */
    pub fn ends_with(&self, other: &str) -> bool {
        
        todo!();
        /*
            auto end = text.findTerminatingNull();
        auto otherEnd = other.text.findTerminatingNull();

        while (end > text && otherEnd > other.text)
        {
            --end;
            --otherEnd;

            if (*end != *otherEnd)
                return false;
        }

        return otherEnd == other.text;
        */
    }
    
    /**
      | Tests whether the string ends with another
      | string.
      | 
      | If the parameter is an empty string,
      | this will always return true.
      | 
      | Uses a case-insensitive comparison.
      |
      */
    pub fn ends_with_ignore_case(&self, other: &str) -> bool {
        
        todo!();
        /*
            auto end = text.findTerminatingNull();
        auto otherEnd = other.text.findTerminatingNull();

        while (end > text && otherEnd > other.text)
        {
            --end;
            --otherEnd;

            if (end.toLowerCase() != otherEnd.toLowerCase())
                return false;
        }

        return otherEnd == other.text;
        */
    }
    
    /**
      | Returns an upper-case version of this
      | string.
      |
      */
    pub fn to_upper_case(&self) -> AloeString {
        
        todo!();
        /*
            StringCreationHelper builder (text);

        for (;;)
        {
            auto c = builder.source.toUpperCase();
            builder.write (c);

            if (c == 0)
                break;

            ++(builder.source);
        }

        return std::move (builder.result);
        */
    }
    
    /**
      | Returns an lower-case version of this
      | string.
      |
      */
    pub fn to_lower_case(&self) -> AloeString {
        
        todo!();
        /*
            StringCreationHelper builder (text);

        for (;;)
        {
            auto c = builder.source.toLowerCase();
            builder.write (c);

            if (c == 0)
                break;

            ++(builder.source);
        }

        return std::move (builder.result);
        */
    }
    
    /**
      | Returns the final character of the string.
      | 
      | If the string is empty this will return
      | 0.
      |
      */
    pub fn get_last_character(&self) -> wchar_t {
        
        todo!();
        /*
            return isEmpty() ? aloe_wchar() : text [length() - 1];
        */
    }
    
    /**
      | Returns a section of the string, starting
      | from a given position.
      | 
      | -----------
      | @param startIndex
      | 
      | the first character to include. If this
      | is beyond the end of the string, an empty
      | string is returned. If it is zero or less,
      | the whole string is returned.
      | 
      | -----------
      | @return
      | 
      | the substring from startIndex up to
      | the end of the string @see dropLastCharacters,
      | getLastCharacters, fromFirstOccurrenceOf,
      | upToFirstOccurrenceOf, fromLastOccurrenceOf
      |
      */
    pub fn substring_from_offset_len(&self, 
        start: i32,
        end:   i32) -> AloeString {
        
        todo!();
        /*
            if (start < 0)
            start = 0;

        if (end <= start)
            return {};

        int i = 0;
        auto t1 = text;

        while (i < start)
        {
            if (t1.isEmpty())
                return {};

            ++i;
            ++t1;
        }

        auto t2 = t1;

        while (i < end)
        {
            if (t2.isEmpty())
            {
                if (start == 0)
                    return *this;

                break;
            }

            ++i;
            ++t2;
        }

        return AloeString (t1, t2);
        */
    }
    
    /**
      | Returns a subsection of the string.
      | 
      | If the range specified is beyond the
      | limits of the string, as much as possible
      | is returned.
      | 
      | -----------
      | @param startIndex
      | 
      | the index of the start of the substring
      | needed
      | ----------
      | @param endIndex
      | 
      | all characters from startIndex up to
      | (but not including) this index are returned
      | @see fromFirstOccurrenceOf, dropLastCharacters,
      | getLastCharacters, upToFirstOccurrenceOf
      |
      */
    pub fn substring_from_offset(&self, start: i32) -> AloeString {
        
        todo!();
        /*
            if (start <= 0)
            return *this;

        auto t = text;

        while (--start >= 0)
        {
            if (t.isEmpty())
                return {};

            ++t;
        }

        return AloeString (t);
        */
    }
    
    /**
      | Returns a version of this string with
      | a number of characters removed from
      | the end.
      | 
      | -----------
      | @param numberToDrop
      | 
      | the number of characters to drop from
      | the end of the string. If this is greater
      | than the length of the string, an empty
      | string will be returned. If zero or less,
      | the original string will be returned.
      | @see substring, fromFirstOccurrenceOf,
      | upToFirstOccurrenceOf, fromLastOccurrenceOf,
      | getLastCharacter
      |
      */
    pub fn drop_last_characters(&self, number_to_drop: i32) -> AloeString {
        
        todo!();
        /*
            return AloeString (text, (size_t) jmax (0, length() - numberToDrop));
        */
    }
    
    /**
      | Returns a number of characters from
      | the end of the string.
      | 
      | This returns the last numCharacters
      | characters from the end of the string.
      | If the string is shorter than numCharacters,
      | the whole string is returned.
      | 
      | @see substring, dropLastCharacters,
      | getLastCharacter
      |
      */
    pub fn get_last_characters(&self, num_characters: i32) -> AloeString {
        
        todo!();
        /*
            return AloeString (text + jmax (0, length() - jmax (0, numCharacters)));
        */
    }
    
    /**
      | Returns a section of the string starting
      | from a given substring.
      | 
      | This will search for the first occurrence
      | of the given substring, and return the
      | section of the string starting from
      | the point where this is found (optionally
      | not including the substring itself).
      | 
      | e.g. for the string "123456",
      | 
      | fromFirstOccurrenceOf ("34", true)
      | would return "3456",
      | 
      | and
      | 
      | fromFirstOccurrenceOf ("34", false)
      | would return "56".
      | 
      | If the substring isn't found, the method
      | will return an empty string.
      | 
      | If ignoreCase is true, the comparison
      | will be case-insensitive.
      | 
      | @see upToFirstOccurrenceOf, fromLastOccurrenceOf
      |
      */
    pub fn from_first_occurrence_of(&self, 
        sub:                &str,
        include_sub_string: bool,
        ignore_case:        bool) -> AloeString {
        
        todo!();
        /*
            auto i = ignoreCase ? indexOfIgnoreCase (sub)
                            : indexOf (sub);
        if (i < 0)
            return {};

        return substring (includeSubString ? i : i + sub.length());
        */
    }
    
    /**
      | Returns a section of the string starting
      | from the last occurrence of a given substring.
      | 
      | Similar to fromFirstOccurrenceOf(),
      | but using the last occurrence of the
      | substring, and unlike fromFirstOccurrenceOf(),
      | if the substring isn't found, this method
      | will return the whole of the original
      | string.
      | 
      | @see fromFirstOccurrenceOf, upToLastOccurrenceOf
      |
      */
    pub fn from_last_occurrence_of(&self, 
        sub:                &str,
        include_sub_string: bool,
        ignore_case:        bool) -> AloeString {
        
        todo!();
        /*
            auto i = ignoreCase ? lastIndexOfIgnoreCase (sub)
                            : lastIndexOf (sub);
        if (i < 0)
            return *this;

        return substring (includeSubString ? i : i + sub.length());
        */
    }
    
    /**
      | Returns the start of this string, up
      | to the first occurrence of a substring.
      | 
      | This will search for the first occurrence
      | of a given substring, and then return
      | a copy of the string, up to the position
      | of this substring, optionally including
      | or excluding the substring itself in
      | the result.
      | 
      | e.g. for the string "123456",
      | 
      | upTo ("34", false) would return "12",
      | 
      | and
      | 
      | upTo ("34", true) would return "1234".
      | 
      | If the substring isn't found, this will
      | return the whole of the original string.
      | 
      | @see upToLastOccurrenceOf, fromFirstOccurrenceOf
      |
      */
    pub fn up_to_first_occurrence_of(&self, 
        sub:                &str,
        include_sub_string: bool,
        ignore_case:        bool) -> AloeString {
        
        todo!();
        /*
            auto i = ignoreCase ? indexOfIgnoreCase (sub)
                            : indexOf (sub);
        if (i < 0)
            return *this;

        return substring (0, includeSubString ? i + sub.length() : i);
        */
    }
    
    /**
      | Returns the start of this string, up
      | to the last occurrence of a substring.
      | 
      | Similar to upToFirstOccurrenceOf(),
      | but this finds the last occurrence rather
      | than the first. If the substring isn't
      | found, this will return the whole of
      | the original string.
      | 
      | @see upToFirstOccurrenceOf, fromFirstOccurrenceOf
      |
      */
    pub fn up_to_last_occurrence_of(&self, 
        sub:                &str,
        include_sub_string: bool,
        ignore_case:        bool) -> AloeString {
        
        todo!();
        /*
            auto i = ignoreCase ? lastIndexOfIgnoreCase (sub)
                            : lastIndexOf (sub);
        if (i < 0)
            return *this;

        return substring (0, includeSubString ? i + sub.length() : i);
        */
    }
    
    /**
      | Checks whether the string might be in
      | quotation marks.
      | 
      | -----------
      | @return
      | 
      | true if the string begins with a quote
      | character (either a double or single
      | quote). It is also true if there is whitespace
      | before the quote, but it doesn't check
      | the end of the string. @see unquoted,
      | quoted
      |
      */
    pub fn is_quoted_string(&self) -> bool {
        
        todo!();
        /*
            return isQuoteCharacter (*text.findEndOfWhitespace());
        */
    }
    
    /**
      | Removes quotation marks from around
      | the string, (if there are any).
      | 
      | Returns a copy of this string with any
      | quotes removed from its ends. Quotes
      | that aren't at the ends of the string
      | are not affected. If there aren't any
      | quotes, the original string is returned.
      | 
      | Note that this is a const method, and
      | won't alter the string itself.
      | 
      | @see isQuotedString, quoted
      |
      */
    pub fn unquoted(&self) -> AloeString {
        
        todo!();
        /*
            if (! isQuoteCharacter (*text))
            return *this;

        auto len = length();
        return substring (1, len - (isQuoteCharacter (text[len - 1]) ? 1 : 0));
        */
    }
    
    /**
      | Adds quotation marks around a string.
      | 
      | This will return a copy of the string
      | with a quote at the start and end, (but
      | won't add the quote if there's already
      | one there, so it's safe to call this on
      | strings that may already have quotes
      | around them).
      | 
      | Note that this is a const method, and
      | won't alter the string itself.
      | 
      | -----------
      | @param quoteCharacter
      | 
      | the character to add at the start and
      | end @see isQuotedString, unquoted
      |
      */
    pub fn quoted(&self, quote_character: Option<char>) -> AloeString {

        let quote_character = quote_character.unwrap_or('"');
        
        todo!();
        /*
            if (isEmpty())
            return charToString (quoteCharacter) + quoteCharacter;

        AloeString t (*this);

        if (! t.startsWithChar (quoteCharacter))
            t = charToString (quoteCharacter) + t;

        if (! t.endsWithChar (quoteCharacter))
            t += quoteCharacter;

        return t;
        */
    }
    
    pub fn find_trimmed_end(
        start: CharPointerType,
        end:   CharPointerType) -> CharPointerType {
        
        todo!();
        /*
            while (end > start)
        {
            if (! (--end).isWhitespace())
            {
                ++end;
                break;
            }
        }

        return end;
        */
    }
    
    /**
      | Returns a copy of this string with any
      | whitespace characters removed from
      | the start and end.
      |
      */
    pub fn trim(&self) -> AloeString {
        
        todo!();
        /*
            if (isNotEmpty())
        {
            auto start = text.findEndOfWhitespace();
            auto end = start.findTerminatingNull();
            auto trimmedEnd = findTrimmedEnd (start, end);

            if (trimmedEnd <= start)
                return {};

            if (text < start || trimmedEnd < end)
                return AloeString (start, trimmedEnd);
        }

        return *this;
        */
    }
    
    /**
      | Returns a copy of this string with any
      | whitespace characters removed from
      | the start.
      |
      */
    pub fn trim_start(&self) -> AloeString {
        
        todo!();
        /*
            if (isNotEmpty())
        {
            auto t = text.findEndOfWhitespace();

            if (t != text)
                return AloeString (t);
        }

        return *this;
        */
    }
    
    /**
      | Returns a copy of this string with any
      | whitespace characters removed from
      | the end.
      |
      */
    pub fn trim_end(&self) -> AloeString {
        
        todo!();
        /*
            if (isNotEmpty())
        {
            auto end = text.findTerminatingNull();
            auto trimmedEnd = findTrimmedEnd (text, end);

            if (trimmedEnd < end)
                return AloeString (text, trimmedEnd);
        }

        return *this;
        */
    }
    
    /**
      | Returns a copy of this string, having
      | removed a specified set of characters
      | from its start.
      | 
      | Characters are removed from the start
      | of the string until it finds one that
      | is not in the specified set, and then
      | it stops.
      | 
      | -----------
      | @param charactersToTrim
      | 
      | the set of characters to remove. @see
      | trim, trimStart, trimCharactersAtEnd
      |
      */
    pub fn trim_characters_at_start(&self, characters_to_trim: &str) -> AloeString {
        
        todo!();
        /*
            auto t = text;

        while (charactersToTrim.text.indexOf (*t) >= 0)
            ++t;

        return t == text ? *this : AloeString (t);
        */
    }
    
    /**
      | Returns a copy of this string, having
      | removed a specified set of characters
      | from its end.
      | 
      | Characters are removed from the end
      | of the string until it finds one that
      | is not in the specified set, and then
      | it stops.
      | 
      | -----------
      | @param charactersToTrim
      | 
      | the set of characters to remove. @see
      | trim, trimEnd, trimCharactersAtStart
      |
      */
    pub fn trim_characters_at_end(&self, characters_to_trim: &str) -> AloeString {
        
        todo!();
        /*
            if (isNotEmpty())
        {
            auto end = text.findTerminatingNull();
            auto trimmedEnd = end;

            while (trimmedEnd > text)
            {
                if (charactersToTrim.text.indexOf (*--trimmedEnd) < 0)
                {
                    ++trimmedEnd;
                    break;
                }
            }

            if (trimmedEnd < end)
                return AloeString (text, trimmedEnd);
        }

        return *this;
        */
    }
    
    /**
      | Returns a version of this string that
      | only retains a fixed set of characters.
      | 
      | This will return a copy of this string,
      | omitting any characters which are not
      | found in the string passed-in.
      | 
      | e.g. for "1122334455", retainCharacters
      | ("432") would return "223344"
      | 
      | Note that this is a const method, and
      | won't alter the string itself.
      |
      */
    pub fn retain_characters(&self, characters_to_retain: &str) -> AloeString {
        
        todo!();
        /*
            if (isEmpty())
            return {};

        StringCreationHelper builder (text);

        for (;;)
        {
            auto c = builder.source.getAndAdvance();

            if (charactersToRetain.text.indexOf (c) >= 0)
                builder.write (c);

            if (c == 0)
                break;
        }

        builder.write (0);
        return std::move (builder.result);
        */
    }
    
    /**
      | Returns a version of this string with
      | a set of characters removed.
      | 
      | This will return a copy of this string,
      | omitting any characters which are found
      | in the string passed-in.
      | 
      | e.g. for "1122334455", removeCharacters
      | ("432") would return "1155"
      | 
      | Note that this is a const method, and
      | won't alter the string itself.
      |
      */
    pub fn remove_characters(&self, characters_to_remove: &str) -> AloeString {
        
        todo!();
        /*
            if (isEmpty())
            return {};

        StringCreationHelper builder (text);

        for (;;)
        {
            auto c = builder.source.getAndAdvance();

            if (charactersToRemove.text.indexOf (c) < 0)
                builder.write (c);

            if (c == 0)
                break;
        }

        return std::move (builder.result);
        */
    }
    
    /**
      | Returns a section from the start of the
      | string that only contains a certain
      | set of characters.
      | 
      | This returns the leftmost section of
      | the string, up to (and not including)
      | the first character that doesn't appear
      | in the string passed in.
      |
      */
    pub fn initial_section_containing_only(&self, permitted_characters: &str) -> AloeString {
        
        todo!();
        /*
            for (auto t = text; ! t.isEmpty(); ++t)
            if (permittedCharacters.text.indexOf (*t) < 0)
                return AloeString (text, t);

        return *this;
        */
    }
    
    /**
      | Returns a section from the start of the
      | string that only contains a certain
      | set of characters.
      | 
      | This returns the leftmost section of
      | the string, up to (and not including)
      | the first character that occurs in the
      | string passed in. (If none of the specified
      | characters are found in the string,
      | the return value will just be the original
      | string).
      |
      */
    pub fn initial_section_not_containing(&self, characters_to_stop_at: &str) -> AloeString {
        
        todo!();
        /*
            for (auto t = text; ! t.isEmpty(); ++t)
            if (charactersToStopAt.text.indexOf (*t) >= 0)
                return AloeString (text, t);

        return *this;
        */
    }
    
    /**
      | Looks for a set of characters in the string.
      | 
      | Uses a case-sensitive comparison.
      | 
      | -----------
      | @return
      | 
      | Returns false if any of the characters
      | in this string do not occur in the parameter
      | string. If this string is empty, the
      | return value will always be true.
      |
      */
    pub fn contains_only(&self, chars: &str) -> bool {
        
        todo!();
        /*
            for (auto t = text; ! t.isEmpty();)
            if (chars.text.indexOf (t.getAndAdvance()) < 0)
                return false;

        return true;
        */
    }
    
    /**
      | Looks for any of a set of characters in
      | the string.
      | 
      | Uses a case-sensitive comparison.
      | 
      | -----------
      | @return
      | 
      | true if the string contains any of the
      | characters from the string that is passed
      | in.
      |
      */
    pub fn contains_any_of(&self, chars: &str) -> bool {
        
        todo!();
        /*
            for (auto t = text; ! t.isEmpty();)
            if (chars.text.indexOf (t.getAndAdvance()) >= 0)
                return true;

        return false;
        */
    }
    
    /**
      | Returns true if this string contains
      | any non-whitespace characters.
      | 
      | This will return false if the string
      | contains only whitespace characters,
      | or if it's empty.
      | 
      | It is equivalent to calling "myString.trim().isNotEmpty()".
      |
      */
    pub fn contains_non_whitespace_chars(&self) -> bool {
        
        todo!();
        /*
            for (auto t = text; ! t.isEmpty(); ++t)
            if (! t.isWhitespace())
                return true;

        return false;
        */
    }
    
    pub fn formatted_raw(&mut self, 
        pf:   *const u8,
        args: &[&str]) -> AloeString {
        
        todo!();
        /*
            size_t bufferSize = 256;

        for (;;)
        {
            va_list args;
            va_start (args, pf);

           #if ALOE_WINDOWS
            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wdeprecated-declarations")
           #endif

          #if ALOE_ANDROID
            HeapBlock<char> temp (bufferSize);
            int num = (int) vsnprintf (temp.get(), bufferSize - 1, pf, args);
            if (num >= static_cast<int> (bufferSize))
                num = -1;
          #else
            AloeString wideCharVersion (pf);
            HeapBlock<wchar_t> temp (bufferSize);
            const int num = (int)
           #if ALOE_WINDOWS
                _vsnwprintf
           #else
                vswprintf
           #endif
                    (temp.get(), bufferSize - 1, wideCharVersion.toWideCharPointer(), args);
          #endif

           #if ALOE_WINDOWS
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE
           #endif
            va_end (args);

            if (num > 0)
                return AloeString (temp.get());

            bufferSize += 256;

            if (num == 0 || bufferSize > 65536) // the upper limit is a sanity check to avoid situations where vprintf repeatedly
                break;                          // returns -1 because of an error rather than because it needs more space.
        }

        return {};
        */
    }
    
    /**
      | Reads the value of the string as a decimal
      | number (up to 32 bits in size).
      | 
      | -----------
      | @return
      | 
      | the value of the string as a 32 bit signed
      | base-10 integer. @see getTrailingIntValue,
      | getHexValue32, getHexValue64
      |
      */
    pub fn get_int_value(&self) -> i32 {
        
        todo!();
        /*
            return text.getIntValue32();
        */
    }
    
    /**
      | Reads the value of the string as a decimal
      | number (up to 64 bits in size).
      | 
      | -----------
      | @return
      | 
      | the value of the string as a 64 bit signed
      | base-10 integer.
      |
      */
    pub fn get_large_int_value(&self) -> i64 {
        
        todo!();
        /*
            return text.getIntValue64();
        */
    }
    
    /**
      | Parses this string as a floating point
      | number.
      | 
      | -----------
      | @return
      | 
      | the value of the string as a 32-bit floating
      | point value. @see getDoubleValue
      |
      */
    pub fn get_float_value(&self) -> f32 {
        
        todo!();
        /*
            return (float) getDoubleValue();
        */
    }
    
    /**
      | Parses this string as a floating point
      | number.
      | 
      | -----------
      | @return
      | 
      | the value of the string as a 64-bit floating
      | point value. @see getFloatValue
      |
      */
    pub fn get_double_value(&self) -> f64 {
        
        todo!();
        /*
            return text.getDoubleValue();
        */
    }
    
    /**
      | Parses a decimal number from the end
      | of the string.
      | 
      | This will look for a value at the end of
      | the string. e.g. for "321 xyz654" it
      | will return 654; for "2 3 4" it'll return
      | 4.
      | 
      | If the string ends with a hyphen followed
      | by numeric characters, the return value
      | will be negative.
      | 
      | @see getIntValue
      |
      */
    pub fn get_trailing_int_value(&self) -> i32 {
        
        todo!();
        /*
            int n = 0;
        int mult = 1;
        auto t = text.findTerminatingNull();

        while (--t >= text)
        {
            if (! t.isDigit())
            {
                if (*t == '-')
                    n = -n;

                break;
            }

            n += (int) (((aloe_wchar) mult) * (*t - '0'));
            mult *= 10;
        }

        return n;
        */
    }
}

pub const hexDigits: &'static str = "0123456789abcdef";

pub fn hex_to_string<Type>(v: Type) -> AloeString {

    todo!();
    /*
        CharPointerType::CharType buffer[32];
        auto* end = buffer + numElementsInArray (buffer) - 1;
        auto* t = end;
        *t = 0;

        do
        {
            *--t = hexDigits [(int) (v & 15)];
            v = static_cast<Type> (v >> 4);

        } while (v != 0);

        return AloeString (CharPointerType (t),
                       CharPointerType (end));
    */
}

impl AloeString {
    
    pub fn create_hex_from_u8(&mut self, n: u8) -> AloeString {
        
        todo!();
        /*
            return hexToString (n);
        */
    }
    
    pub fn create_hex_from_u16(&mut self, n: u16) -> AloeString {
        
        todo!();
        /*
            return hexToString (n);
        */
    }
    
    pub fn create_hex_from_u32(&mut self, n: u32) -> AloeString {
        
        todo!();
        /*
            return hexToString (n);
        */
    }
    
    pub fn create_hex_from_u64(&mut self, n: u64) -> AloeString {
        
        todo!();
        /*
            return hexToString (n);
        */
    }
    
    /**
      | Returns a string containing a hex dump
      | of a block of binary data.
      | 
      | -----------
      | @param data
      | 
      | the binary data to use as input
      | ----------
      | @param size
      | 
      | how many bytes of data to use
      | ----------
      | @param groupSize
      | 
      | how many bytes are grouped together
      | before inserting a space into the output.
      | e.g. group size 0 has no spaces, group
      | size 1 looks like: "be a1 c2 ff", group
      | size 2 looks like "bea1 c2ff".
      |
      */
    pub fn to_hex_string(
        &mut self, 
        d:          *const c_void,
        size:       i32,
        group_size: Option<i32>
    ) -> AloeString {

        let group_size: i32 = group_size.unwrap_or(1);
        
        todo!();
        /*
            if (size <= 0)
            return {};

        int numChars = (size * 2) + 2;
        if (groupSize > 0)
            numChars += size / groupSize;

        AloeString s (PreallocationBytes ((size_t) numChars * sizeof (CharPointerType::CharType)));

        auto* data = static_cast<const unsigned char*> (d);
        auto dest = s.text;

        for (int i = 0; i < size; ++i)
        {
            const unsigned char nextByte = *data++;
            dest.write ((aloe_wchar) hexDigits [nextByte >> 4]);
            dest.write ((aloe_wchar) hexDigits [nextByte & 0xf]);

            if (groupSize > 0 && (i % groupSize) == (groupSize - 1) && i < (size - 1))
                dest.write ((aloe_wchar) ' ');
        }

        dest.writeNull();
        return s;
        */
    }
    
    /**
      | Parses the string as a hexadecimal number.
      | 
      | Non-hexadecimal characters in the
      | string are ignored.
      | 
      | If the string contains too many characters,
      | then the lowest significant digits
      | are returned, e.g. "ffff12345678"
      | would produce 0x12345678.
      | 
      | -----------
      | @return
      | 
      | a 32-bit number which is the value of
      | the string in hex.
      |
      */
    pub fn get_hex_value32(&self) -> i32 {
        
        todo!();
        /*
            return CharacterFunctions::HexParser<int>  ::parse (text);
        */
    }
    
    /**
      | Parses the string as a hexadecimal number.
      | 
      | Non-hexadecimal characters in the
      | string are ignored.
      | 
      | If the string contains too many characters,
      | then the lowest significant digits
      | are returned, e.g. "ffff1234567812345678"
      | would produce 0x1234567812345678.
      | 
      | -----------
      | @return
      | 
      | a 64-bit number which is the value of
      | the string in hex.
      |
      */
    pub fn get_hex_value64(&self) -> i64 {
        
        todo!();
        /*
            return CharacterFunctions::HexParser<int64>::parse (text);
        */
    }
    
    pub fn get_string_from_windows_1252codepage(
        data: *const u8,
        num:  usize) -> AloeString {
        
        todo!();
        /*
            HeapBlock<aloe_wchar> unicode (num + 1);

        for (size_t i = 0; i < num; ++i)
            unicode[i] = CharacterFunctions::getUnicodeCharFromWindows1252Codepage ((uint8) data[i]);

        unicode[num] = 0;
        return CharPointer_UTF32 (unicode);
        */
    }
    
    /**
      | Creates a string from data in an unknown
      | format.
      | 
      | This looks at some binary data and tries
      | to guess whether it's Unicode or 8-bit
      | characters, then returns a string that
      | represents it correctly.
      | 
      | Should be able to handle Unicode endianness
      | correctly, by looking at the first two
      | bytes.
      |
      */
    pub fn create_string_from_data(&mut self, 
        unknown_data: *const c_void,
        size:         i32) -> AloeString {
        
        todo!();
        /*
            auto* data = static_cast<const uint8*> (unknownData);

        if (size <= 0 || data == nullptr)
            return {};

        if (size == 1)
            return charToString ((aloe_wchar) data[0]);

        if (CharPointer_UTF16::isByteOrderMarkBigEndian (data)
             || CharPointer_UTF16::isByteOrderMarkLittleEndian (data))
        {
            const int numChars = size / 2 - 1;

            StringCreationHelper builder ((size_t) numChars);

            auto src = unalignedPointerCast<const uint16*> (data + 2);

            if (CharPointer_UTF16::isByteOrderMarkBigEndian (data))
            {
                for (int i = 0; i < numChars; ++i)
                    builder.write ((aloe_wchar) ByteOrder::swapIfLittleEndian (src[i]));
            }
            else
            {
                for (int i = 0; i < numChars; ++i)
                    builder.write ((aloe_wchar) ByteOrder::swapIfBigEndian (src[i]));
            }

            builder.write (0);
            return std::move (builder.result);
        }

        auto* start = (const char*) data;

        if (size >= 3 && CharPointer_UTF8::isByteOrderMark (data))
        {
            start += 3;
            size -= 3;
        }

        if (CharPointer_UTF8::isValidString (start, size))
            return AloeString (CharPointer_UTF8 (start),
                           CharPointer_UTF8 (start + size));

        return getStringFromWindows1252Codepage (start, (size_t) size);
        */
    }
}

pub const emptyChar: wchar_t = 0;

///-------------------------------
pub struct StringEncodingConverter<
CharPointerType_Src:  HasCharType,
CharPointerType_Dest: HasCharType> 
{
    phantom1: std::marker::PhantomData<CharPointerType_Src>,
    phantom2: std::marker::PhantomData<CharPointerType_Dest>,
}

impl<
CharPointerType_Src:  HasCharType,
CharPointerType_Dest: HasCharType
>
StringEncodingConverter<CharPointerType_Src,CharPointerType_Dest> {

    pub fn convert_generic(s: &AloeString) -> CharPointerType_Dest {
        
        todo!();
        /*
            auto& source = const_cast<AloeString&> (s);

            using DestChar = typename CharPointerType_Dest::CharType;

            if (source.isEmpty())
                return CharPointerType_Dest (reinterpret_cast<const DestChar*> (&emptyChar));

            CharPointerType_Src text (source.getCharPointer());
            auto extraBytesNeeded = CharPointerType_Dest::getBytesRequiredFor (text) + sizeof (typename CharPointerType_Dest::CharType);
            auto endOffset = (text.sizeInBytes() + 3) & ~3u; // the new string must be word-aligned or many Windows
                                                             // functions will fail to read it correctly!
            source.preallocateBytes (endOffset + extraBytesNeeded);
            text = source.getCharPointer();

            void* const newSpace = addBytesToPointer (text.getAddress(), (int) endOffset);
            const CharPointerType_Dest extraSpace (static_cast<DestChar*> (newSpace));

           #if ALOE_DEBUG // (This just avoids spurious warnings from valgrind about the uninitialised bytes at the end of the buffer..)
            auto bytesToClear = (size_t) jmin ((int) extraBytesNeeded, 4);
            zeromem (addBytesToPointer (newSpace, extraBytesNeeded - bytesToClear), bytesToClear);
           #endif

            CharPointerType_Dest (extraSpace).writeAll (text);
            return extraSpace;
        */
    }
    pub fn convert_utf8(source: &AloeString) -> CharPointer_UTF8 {
        
        todo!();
        /*
            return CharPointer_UTF8 (unalignedPointerCast<CharPointer_UTF8::CharType*> (source.getCharPointer().getAddress()));
        */
    }

    pub fn convert_utf16(source: &AloeString) -> CharPointer_UTF16 {
        
        todo!();
        /*
            return CharPointer_UTF16 (unalignedPointerCast<CharPointer_UTF16::CharType*> (source.getCharPointer().getAddress()));
        */
    }

    pub fn convert_utf32(source: &AloeString) -> CharPointer_UTF32 {
        
        todo!();
        /*
            return CharPointer_UTF32 (unalignedPointerCast<CharPointer_UTF32::CharType*> (source.getCharPointer().getAddress()));
        */
    }
}

impl AloeString {
    
    /**
      | Returns a pointer to a UTF-8 version
      | of this string.
      | 
      | Because it returns a reference to the
      | string's internal data, the pointer
      | that is returned must not be stored anywhere,
      | as it can be deleted whenever the string
      | changes.
      | 
      | To find out how many bytes you need to
      | store this string as UTF-8, you can call
      | CharPointer_UTF8::getBytesRequiredFor
      | (myString.getCharPointer())
      | 
      | @see toRawUTF8, getCharPointer, toUTF16,
      | toUTF32
      |
      */
    pub fn toutf8(&self) -> CharPointer_UTF8 {
        
        todo!();
        /*
            return StringEncodingConverter<CharPointerType, CharPointer_UTF8 >::convert (*this);
        */
    }
    
    /**
      | Returns a pointer to a UTF-16 version
      | of this string.
      | 
      | Because it returns a reference to the
      | string's internal data, the pointer
      | that is returned must not be stored anywhere,
      | as it can be deleted whenever the string
      | changes.
      | 
      | To find out how many bytes you need to
      | store this string as UTF-16, you can
      | call CharPointer_UTF16::getBytesRequiredFor
      | (myString.getCharPointer())
      | 
      | @see getCharPointer, toUTF8, toUTF32
      |
      */
    pub fn toutf16(&self) -> CharPointer_UTF16 {
        
        todo!();
        /*
            return StringEncodingConverter<CharPointerType, CharPointer_UTF16>::convert (*this);
        */
    }
    
    /**
      | Returns a pointer to a UTF-32 version
      | of this string.
      | 
      | Because it returns a reference to the
      | string's internal data, the pointer
      | that is returned must not be stored anywhere,
      | as it can be deleted whenever the string
      | changes.
      | 
      | @see getCharPointer, toUTF8, toUTF16
      |
      */
    pub fn toutf32(&self) -> CharPointer_UTF32 {
        
        todo!();
        /*
            return StringEncodingConverter<CharPointerType, CharPointer_UTF32>::convert (*this);
        */
    }
    
    /**
      | Returns a pointer to a UTF-8 version
      | of this string.
      | 
      | Because it returns a reference to the
      | string's internal data, the pointer
      | that is returned must not be stored anywhere,
      | as it can be deleted whenever the string
      | changes.
      | 
      | To find out how many bytes you need to
      | store this string as UTF-8, you can call
      | CharPointer_UTF8::getBytesRequiredFor
      | (myString.getCharPointer())
      | 
      | @see getCharPointer, toUTF8, toUTF16,
      | toUTF32
      |
      */
    pub fn to_rawutf8(&self) -> *const u8 {
        
        todo!();
        /*
            return toUTF8().getAddress();
        */
    }
    
    /**
      | Returns a pointer to a wchar_t version
      | of this string.
      | 
      | Because it returns a reference to the
      | string's internal data, the pointer
      | that is returned must not be stored anywhere,
      | as it can be deleted whenever the string
      | changes.
      | 
      | Bear in mind that the wchar_t type is
      | different on different platforms,
      | so on Windows, this will be equivalent
      | to calling toUTF16(), on unix it'll
      | be the same as calling toUTF32(), etc.
      | 
      | @see getCharPointer, toUTF8, toUTF16,
      | toUTF32
      |
      */
    pub fn to_wide_char_pointer(&self) -> *const wchar_t {
        
        todo!();
        /*
            return StringEncodingConverter<CharPointerType, CharPointer_wchar_t>::convert (*this).getAddress();
        */
    }
    
    pub fn to_std_string(&self) -> AloeString {
        
        todo!();
        /*
            return std::string (toRawUTF8());
        */
    }
}

pub struct StringCopier<
    CharPointerType_Src:  HasCharType, 
    CharPointerType_Dest: HasCharType> 
{
    phantom1: std::marker::PhantomData<CharPointerType_Src>,
    phantom2: std::marker::PhantomData<CharPointerType_Dest>,
}

impl<
CharPointerType_Src: HasCharType,
CharPointerType_Dest: HasCharType
> 
StringCopier<CharPointerType_Src,CharPointerType_Dest> {

    pub fn copy_to_buffer(
        source:                CharPointerType_Src,
        buffer:                *mut <CharPointerType_Dest as HasCharType>::CharType,
        max_buffer_size_bytes: usize) -> usize {
        
        todo!();
        /*
            jassert (((ssize_t) maxBufferSizeBytes) >= 0); // keep this value positive!

            if (buffer == nullptr)
                return CharPointerType_Dest::getBytesRequiredFor (source) + sizeof (typename CharPointerType_Dest::CharType);

            return CharPointerType_Dest (buffer).writeWithDestByteLimit (source, maxBufferSizeBytes);
        */
    }
}

impl AloeString {
    
    /**
      | Copies the string to a buffer as UTF-8
      | characters.
      | 
      | Returns the number of bytes copied to
      | the buffer, including the terminating
      | null character.
      | 
      | To find out how many bytes you need to
      | store this string as UTF-8, you can call
      | CharPointer_UTF8::getBytesRequiredFor
      | (myString.getCharPointer())
      | 
      | -----------
      | @param destBuffer
      | 
      | the place to copy it to; if this is a null
      | pointer, the method just returns the
      | number of bytes required (including
      | the terminating null character).
      | ----------
      | @param maxBufferSizeBytes
      | 
      | the size of the destination buffer,
      | in bytes. If the string won't fit, it'll
      | put in as many as it can while still allowing
      | for a terminating null char at the end,
      | and will return the number of bytes that
      | were actually used.
      | 
      | @see CharPointer_UTF8::writeWithDestByteLimit
      |
      */
    pub fn copy_toutf8(&self, 
        buffer:                *mut <CharPointer_UTF8 as HasCharType>::CharType,
        max_buffer_size_bytes: usize) -> usize {
        
        todo!();
        /*
            return StringCopier<CharPointerType, CharPointer_UTF8>::copyToBuffer (text, buffer, maxBufferSizeBytes);
        */
    }
    
    /**
      | Copies the string to a buffer as UTF-16
      | characters.
      | 
      | Returns the number of bytes copied to
      | the buffer, including the terminating
      | null character.
      | 
      | To find out how many bytes you need to
      | store this string as UTF-16, you can
      | call CharPointer_UTF16::getBytesRequiredFor
      | (myString.getCharPointer())
      | 
      | -----------
      | @param destBuffer
      | 
      | the place to copy it to; if this is a null
      | pointer, the method just returns the
      | number of bytes required (including
      | the terminating null character).
      | ----------
      | @param maxBufferSizeBytes
      | 
      | the size of the destination buffer,
      | in bytes. If the string won't fit, it'll
      | put in as many as it can while still allowing
      | for a terminating null char at the end,
      | and will return the number of bytes that
      | were actually used. @see CharPointer_UTF16::writeWithDestByteLimit
      |
      */
    pub fn copy_toutf16(&self, 
        buffer:                *mut <CharPointer_UTF16 as HasCharType>::CharType,
        max_buffer_size_bytes: usize) -> usize {
        
        todo!();
        /*
            return StringCopier<CharPointerType, CharPointer_UTF16>::copyToBuffer (text, buffer, maxBufferSizeBytes);
        */
    }
    
    /**
      | Copies the string to a buffer as UTF-32
      | characters.
      | 
      | Returns the number of bytes copied to
      | the buffer, including the terminating
      | null character.
      | 
      | To find out how many bytes you need to
      | store this string as UTF-32, you can
      | call CharPointer_UTF32::getBytesRequiredFor
      | (myString.getCharPointer())
      | 
      | -----------
      | @param destBuffer
      | 
      | the place to copy it to; if this is a null
      | pointer, the method just returns the
      | number of bytes required (including
      | the terminating null character).
      | ----------
      | @param maxBufferSizeBytes
      | 
      | the size of the destination buffer,
      | in bytes. If the string won't fit, it'll
      | put in as many as it can while still allowing
      | for a terminating null char at the end,
      | and will return the number of bytes that
      | were actually used. @see CharPointer_UTF32::writeWithDestByteLimit
      |
      */
    pub fn copy_toutf32(&self, 
        buffer:                *mut <CharPointer_UTF32 as HasCharType>::CharType,
        max_buffer_size_bytes: usize) -> usize {
        
        todo!();
        /*
            return StringCopier<CharPointerType, CharPointer_UTF32>::copyToBuffer (text, buffer, maxBufferSizeBytes);
        */
    }
    
    /**
      | Returns the number of bytes required
      | to represent this string as UTF8.
      | 
      | The number returned does NOT include
      | the trailing zero.
      | 
      | @see toUTF8, copyToUTF8
      |
      */
    pub fn get_num_bytes_asutf8(&self) -> usize {
        
        todo!();
        /*
            return CharPointer_UTF8::getBytesRequiredFor (text);
        */
    }
    
    /**
      | Creates a AloeString from a UTF-8 encoded
      | buffer.
      | 
      | If the size is < 0, it'll keep reading
      | until it hits a zero.
      |
      */
    pub fn fromutf8(
        &mut self, 
        buffer:            *const u8,
        buffer_size_bytes: Option<i32>) -> AloeString 
    {
        let buffer_size_bytes: i32 = buffer_size_bytes.unwrap_or(-1);
        
        todo!();
        /*
            if (buffer != nullptr)
        {
            if (bufferSizeBytes < 0)
                return AloeString (CharPointer_UTF8 (buffer));

            if (bufferSizeBytes > 0)
            {
                jassert (CharPointer_UTF8::isValidString (buffer, bufferSizeBytes));
                return AloeString (CharPointer_UTF8 (buffer), CharPointer_UTF8 (buffer + bufferSizeBytes));
            }
        }

        return {};
        */
    }
}

/*
impl &str {
    
    pub fn default() -> Self {
    
        todo!();
        /*


            : text ((const CharPointerType::CharType*) "\0\0\0")
        */
    }
    
    pub fn new_from_string_literal(string_literal: *const u8) -> Self {
    
        todo!();
        /*


            #if ALOE_STRING_UTF_TYPE != 8
        : text (nullptr), stringCopy (stringLiteral)
       #else
        : text (stringLiteral)
       #endif

       #if ALOE_STRING_UTF_TYPE != 8
        text = stringCopy.getCharPointer();
       #endif

        jassert (stringLiteral != nullptr); // This must be a valid string literal, not a null pointer!!

       #if ALOE_NATIVE_WCHAR_IS_UTF8
        /*  If you get an assertion here, then you're trying to create a string from 8-bit data
            that contains values greater than 127. These can NOT be correctly converted to unicode
            because there's no way for the AloeString class to know what encoding was used to
            create them. The source data could be UTF-8, ASCII or one of many local code-pages.

            To get around this problem, you must be more explicit when you pass an ambiguous 8-bit
            string to the &str class - so for example if your source data is actually UTF-8,
            you'd call &str (CharPointer_UTF8 ("my utf8 string..")), and it would be able to
            correctly convert the multi-byte characters to unicode. It's *highly* recommended that
            you use UTF-8 with escape characters in your source code to represent extended characters,
            because there's no other way to represent these strings in a way that isn't dependent on
            the compiler, source code editor and platform.
        */
        jassert (CharPointer_ASCII::isValidString (stringLiteral, std::numeric_limits<int>::max()));
       #endif
        */
    }
    
    pub fn new_from_char_ptr(string_literal: CharPointerType) -> Self {
    
        todo!();
        /*


            : text (stringLiteral)
        jassert (stringLiteral.getAddress() != nullptr); // This must be a valid string literal, not a null pointer!!
        */
    }
    
    /**
      | Creates a string from a UTF-8 encoded
      | std::string.
      |
      */
    pub fn new_from_ref_string(string: &AloeString) -> Self {
    
        todo!();
        /*
        : text(string.getCharPointer()),

        
        */
    }
    
    pub fn new_from_ref_string2(string: &AloeString) -> Self {
    
        todo!();
        /*
        : string_ref(string.c_str()),

        
        */
    }
}
*/

impl AloeString {
    
    pub fn reduce_length_of_float_string(input: &AloeString) -> AloeString {
        
        todo!();
        /*
            const auto start = input.getCharPointer();
        const auto end = start + (int) input.length();
        auto trimStart = end;
        auto trimEnd = trimStart;
        auto exponentTrimStart = end;
        auto exponentTrimEnd = exponentTrimStart;

        decltype (*start) currentChar = '\0';

        for (auto c = end - 1; c > start; --c)
        {
            currentChar = *c;

            if (currentChar == '0' && c + 1 == trimStart)
            {
                --trimStart;
            }
            else if (currentChar == '.')
            {
                if (trimStart == c + 1 && trimStart != end && *trimStart == '0')
                    ++trimStart;

                break;
            }
            else if (currentChar == 'e' || currentChar == 'E')
            {
                auto cNext = c + 1;

                if (cNext != end)
                {
                    if (*cNext == '-')
                        ++cNext;

                    exponentTrimStart = cNext;

                    if (cNext != end && *cNext == '+')
                        ++cNext;

                    exponentTrimEnd = cNext;
                }

                while (cNext != end && *cNext++ == '0')
                    exponentTrimEnd = cNext;

                if (exponentTrimEnd == end)
                    exponentTrimStart = c;

                trimStart = c;
                trimEnd = trimStart;
            }
        }

        if ((trimStart != trimEnd && currentChar == '.') || exponentTrimStart != exponentTrimEnd)
        {
            if (trimStart == trimEnd)
                return AloeString (start, exponentTrimStart) + AloeString (exponentTrimEnd, end);

            if (exponentTrimStart == exponentTrimEnd)
                return AloeString (start, trimStart) + AloeString (trimEnd, end);

            if (trimEnd == exponentTrimStart)
                return AloeString (start, trimStart) + AloeString (exponentTrimEnd, end);

            return AloeString (start, trimStart) + AloeString (trimEnd, exponentTrimStart) + AloeString (exponentTrimEnd, end);
        }

        return input;
        */
    }
}

pub fn serialise_double(input: f64) -> AloeString {
    
    todo!();
    /*
        auto absInput = std::abs (input);

        if (absInput >= 1.0e6 || absInput <= 1.0e-5)
            return reduceLengthOfFloatString ({ input, 15, true });

        int intInput = (int) input;

        if ((double) intInput == input)
            return { input, 1 };

        auto numberOfDecimalPlaces = [absInput]
        {
            if (absInput < 1.0)
            {
                if (absInput >= 1.0e-3)
                {
                    if (absInput >= 1.0e-1) return 16;
                    if (absInput >= 1.0e-2) return 17;
                    return 18;
                }

                if (absInput >= 1.0e-4) return 19;
                return 20;
            }

            if (absInput < 1.0e3)
            {
                if (absInput < 1.0e1) return 15;
                if (absInput < 1.0e2) return 14;
                return 13;
            }

            if (absInput < 1.0e4) return 12;
            if (absInput < 1.0e5) return 11;
            return 10;
        }();

        return reduceLengthOfFloatString (AloeString (input, numberOfDecimalPlaces));
    */
}

#[cfg(ALOE_UNIT_TESTS)]
macro_rules! stringify2 {
    ($X:ident) => {
        /*
                #X
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
macro_rules! stringify {
    ($X:ident) => {
        /*
                STRINGIFY2(X)
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct StringTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod string_tests {
    use super::*;

    pub struct TestUTFConversion<CharPointerType> {

    }

    impl TestUTFConversion<CharPointerType> {

        pub fn test(
            test: &mut UnitTest,
            r:    &mut Random)  {
            
            todo!();
            /*
                AloeString s (createRandomWideCharString (r));

                    typename CharPointerType::CharType buffer [300];

                    memset (buffer, 0xff, sizeof (buffer));
                    CharPointerType (buffer).writeAll (s.toUTF32());
                    test.expectEquals (AloeString (CharPointerType (buffer)), s);

                    memset (buffer, 0xff, sizeof (buffer));
                    CharPointerType (buffer).writeAll (s.toUTF16());
                    test.expectEquals (AloeString (CharPointerType (buffer)), s);

                    memset (buffer, 0xff, sizeof (buffer));
                    CharPointerType (buffer).writeAll (s.toUTF8());
                    test.expectEquals (AloeString (CharPointerType (buffer)), s);

                    test.expect (CharPointerType::isValidString (buffer, (int) strlen ((const char*) buffer)));
            */
        }
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for StringTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("AloeString class", UnitTestCategories::text
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl StringTests {

    pub fn create_random_wide_char_string(r: &mut Random) -> AloeString {
        
        todo!();
        /*
            aloe_wchar buffer[50] = { 0 };

            for (int i = 0; i < numElementsInArray (buffer) - 1; ++i)
            {
                if (r.nextBool())
                {
                    do
                    {
                        buffer[i] = (aloe_wchar) (1 + r.nextInt (0x10ffff - 1));
                    }
                    while (! CharPointer_UTF16::canRepresent (buffer[i]));
                }
                else
                    buffer[i] = (aloe_wchar) (1 + r.nextInt (0xff));
            }

            return CharPointer_UTF32 (buffer);
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            Random r = getRandom();

            {
                beginTest ("Basics");

                expect (AloeString().length() == 0);
                expect (AloeString() == AloeString());
                AloeString s1, s2 ("abcd");
                expect (s1.isEmpty() && ! s1.isNotEmpty());
                expect (s2.isNotEmpty() && ! s2.isEmpty());
                expect (s2.length() == 4);
                s1 = "abcd";
                expect (s2 == s1 && s1 == s2);
                expect (s1 == "abcd" && s1 == L"abcd");
                expect (AloeString ("abcd") == AloeString (L"abcd"));
                expect (AloeString ("abcdefg", 4) == L"abcd");
                expect (AloeString ("abcdefg", 4) == AloeString (L"abcdefg", 4));
                expect (AloeString::charToString ('x') == "x");
                expect (AloeString::charToString (0) == AloeString());
                expect (s2 + "e" == "abcde" && s2 + 'e' == "abcde");
                expect (s2 + L'e' == "abcde" && s2 + L"e" == "abcde");
                expect (s1.equalsIgnoreCase ("abcD") && s1 < "abce" && s1 > "abbb");
                expect (s1.startsWith ("ab") && s1.startsWith ("abcd") && ! s1.startsWith ("abcde"));
                expect (s1.startsWithIgnoreCase ("aB") && s1.endsWithIgnoreCase ("CD"));
                expect (s1.endsWith ("bcd") && ! s1.endsWith ("aabcd"));
                expectEquals (s1.indexOf (AloeString()), 0);
                expectEquals (s1.indexOfIgnoreCase (AloeString()), 0);
                expect (s1.startsWith (AloeString()) && s1.endsWith (AloeString()) && s1.contains (AloeString()));
                expect (s1.contains ("cd") && s1.contains ("ab") && s1.contains ("abcd"));
                expect (s1.containsChar ('a'));
                expect (! s1.containsChar ('x'));
                expect (! s1.containsChar (0));
                expect (AloeString ("abc foo bar").containsWholeWord ("abc") && AloeString ("abc foo bar").containsWholeWord ("abc"));
            }

            {
                beginTest ("Operations");

                AloeString s ("012345678");
                expect (s.hashCode() != 0);
                expect (s.hashCode64() != 0);
                expect (s.hashCode() != (s + s).hashCode());
                expect (s.hashCode64() != (s + s).hashCode64());
                expect (s.compare (AloeString ("012345678")) == 0);
                expect (s.compare (AloeString ("012345679")) < 0);
                expect (s.compare (AloeString ("012345676")) > 0);
                expect (AloeString("a").compareNatural ("A") == 0);
                expect (AloeString("A").compareNatural ("B") < 0);
                expect (AloeString("a").compareNatural ("B") < 0);
                expect (AloeString("10").compareNatural ("2") > 0);
                expect (AloeString("Abc 10").compareNatural ("aBC 2") > 0);
                expect (AloeString("Abc 1").compareNatural ("aBC 2") < 0);
                expect (s.substring (2, 3) == AloeString::charToString (s[2]));
                expect (s.substring (0, 1) == AloeString::charToString (s[0]));
                expect (s.getLastCharacter() == s [s.length() - 1]);
                expect (AloeString::charToString (s.getLastCharacter()) == s.getLastCharacters (1));
                expect (s.substring (0, 3) == L"012");
                expect (s.substring (0, 100) == s);
                expect (s.substring (-1, 100) == s);
                expect (s.substring (3) == "345678");
                expect (s.indexOf (AloeString (L"45")) == 4);
                expect (AloeString ("444445").indexOf ("45") == 4);
                expect (AloeString ("444445").lastIndexOfChar ('4') == 4);
                expect (AloeString ("45454545x").lastIndexOf (AloeString (L"45")) == 6);
                expect (AloeString ("45454545x").lastIndexOfAnyOf ("456") == 7);
                expect (AloeString ("45454545x").lastIndexOfAnyOf (AloeString (L"456x")) == 8);
                expect (AloeString ("abABaBaBa").lastIndexOfIgnoreCase ("aB") == 6);
                expect (s.indexOfChar (L'4') == 4);
                expect (s + s == "012345678012345678");
                expect (s.startsWith (s));
                expect (s.startsWith (s.substring (0, 4)));
                expect (s.startsWith (s.dropLastCharacters (4)));
                expect (s.endsWith (s.substring (5)));
                expect (s.endsWith (s));
                expect (s.contains (s.substring (3, 6)));
                expect (s.contains (s.substring (3)));
                expect (s.startsWithChar (s[0]));
                expect (s.endsWithChar (s.getLastCharacter()));
                expect (s [s.length()] == 0);
                expect (AloeString ("abcdEFGH").toLowerCase() == AloeString ("abcdefgh"));
                expect (AloeString ("abcdEFGH").toUpperCase() == AloeString ("ABCDEFGH"));

                expect (AloeString (&str ("abc")) == "abc");
                expect (AloeString (&str ("abc")) == &str ("abc"));
                expect (AloeString ("abc") + &str ("def") == "abcdef");

                AloeString s2 ("123");
                s2 << ((int) 4) << ((short) 5) << "678" << L"9" << '0';
                s2 += "xyz";
                expect (s2 == "1234567890xyz");
                s2 += (int) 123;
                expect (s2 == "1234567890xyz123");
                s2 += (int64) 123;
                expect (s2 == "1234567890xyz123123");
                s2 << &str ("def");
                expect (s2 == "1234567890xyz123123def");

                // int16
                {
                    AloeString numStr (std::numeric_limits<int16>::max());
                    expect (numStr == "32767");
                }
                {
                    AloeString numStr (std::numeric_limits<int16>::min());
                    expect (numStr == "-32768");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<int16>::max();
                    expect (numStr == "32767");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<int16>::min();
                    expect (numStr == "-32768");
                }
                // int32
                {
                    AloeString numStr (std::numeric_limits<int32>::max());
                    expect (numStr == "2147483647");
                }
                {
                    AloeString numStr (std::numeric_limits<int32>::min());
                    expect (numStr == "-2147483648");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<int32>::max();
                    expect (numStr == "2147483647");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<int32>::min();
                    expect (numStr == "-2147483648");
                }
                // uint32
                {
                    AloeString numStr (std::numeric_limits<uint32>::max());
                    expect (numStr == "4294967295");
                }
                {
                    AloeString numStr (std::numeric_limits<uint32>::min());
                    expect (numStr == "0");
                }
                // int64
                {
                    AloeString numStr (std::numeric_limits<int64>::max());
                    expect (numStr == "9223372036854775807");
                }
                {
                    AloeString numStr (std::numeric_limits<int64>::min());
                    expect (numStr == "-9223372036854775808");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<int64>::max();
                    expect (numStr == "9223372036854775807");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<int64>::min();
                    expect (numStr == "-9223372036854775808");
                }
                // uint64
                {
                    AloeString numStr (std::numeric_limits<uint64>::max());
                    expect (numStr == "18446744073709551615");
                }
                {
                    AloeString numStr (std::numeric_limits<uint64>::min());
                    expect (numStr == "0");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<uint64>::max();
                    expect (numStr == "18446744073709551615");
                }
                {
                    AloeString numStr;
                    numStr << std::numeric_limits<uint64>::min();
                    expect (numStr == "0");
                }
                // size_t
                {
                    AloeString numStr (std::numeric_limits<size_t>::min());
                    expect (numStr == "0");
                }

                beginTest ("Numeric conversions");
                expect (AloeString().getIntValue() == 0);
                expect (AloeString().getDoubleValue() == 0.0);
                expect (AloeString().getFloatValue() == 0.0f);
                expect (s.getIntValue() == 12345678);
                expect (s.getLargeIntValue() == (int64) 12345678);
                expect (s.getDoubleValue() == 12345678.0);
                expect (s.getFloatValue() == 12345678.0f);
                expect (AloeString (-1234).getIntValue() == -1234);
                expect (AloeString ((int64) -1234).getLargeIntValue() == -1234);
                expect (AloeString (-1234.56).getDoubleValue() == -1234.56);
                expect (AloeString (-1234.56f).getFloatValue() == -1234.56f);
                expect (AloeString (std::numeric_limits<int>::max()).getIntValue() == std::numeric_limits<int>::max());
                expect (AloeString (std::numeric_limits<int>::min()).getIntValue() == std::numeric_limits<int>::min());
                expect (AloeString (std::numeric_limits<int64>::max()).getLargeIntValue() == std::numeric_limits<int64>::max());
                expect (AloeString (std::numeric_limits<int64>::min()).getLargeIntValue() == std::numeric_limits<int64>::min());
                expect (("xyz" + s).getTrailingIntValue() == s.getIntValue());
                expect (AloeString ("xyz-5").getTrailingIntValue() == -5);
                expect (AloeString ("-12345").getTrailingIntValue() == -12345);
                expect (s.getHexValue32() == 0x12345678);
                expect (s.getHexValue64() == (int64) 0x12345678);
                expect (AloeString::toHexString (0x1234abcd).equalsIgnoreCase ("1234abcd"));
                expect (AloeString::toHexString ((int64) 0x1234abcd).equalsIgnoreCase ("1234abcd"));
                expect (AloeString::toHexString ((short) 0x12ab).equalsIgnoreCase ("12ab"));
                expect (AloeString::toHexString ((size_t) 0x12ab).equalsIgnoreCase ("12ab"));
                expect (AloeString::toHexString ((long) 0x12ab).equalsIgnoreCase ("12ab"));
                expect (AloeString::toHexString ((int8)  -1).equalsIgnoreCase ("ff"));
                expect (AloeString::toHexString ((int16) -1).equalsIgnoreCase ("ffff"));
                expect (AloeString::toHexString ((int32) -1).equalsIgnoreCase ("ffffffff"));
                expect (AloeString::toHexString ((int64) -1).equalsIgnoreCase ("ffffffffffffffff"));

                unsigned char data[] = { 1, 2, 3, 4, 0xa, 0xb, 0xc, 0xd };
                expect (AloeString::toHexString (data, 8, 0).equalsIgnoreCase ("010203040a0b0c0d"));
                expect (AloeString::toHexString (data, 8, 1).equalsIgnoreCase ("01 02 03 04 0a 0b 0c 0d"));
                expect (AloeString::toHexString (data, 8, 2).equalsIgnoreCase ("0102 0304 0a0b 0c0d"));

                expectEquals (AloeString (12345.67, 4),     AloeString ("12345.6700"));
                expectEquals (AloeString (12345.67, 6),     AloeString ("12345.670000"));
                expectEquals (AloeString (2589410.5894, 7), AloeString ("2589410.5894000"));
                expectEquals (AloeString (12345.67, 8),     AloeString ("12345.67000000"));
                expectEquals (AloeString (1e19, 4),         AloeString ("10000000000000000000.0000"));
                expectEquals (AloeString (1e-34, 36),       AloeString ("0.000000000000000000000000000000000100"));
                expectEquals (AloeString (1.39, 1),         AloeString ("1.4"));

                expectEquals (AloeString (12345.67, 4,     true), AloeString ("1.2346e+04"));
                expectEquals (AloeString (12345.67, 6,     true), AloeString ("1.234567e+04"));
                expectEquals (AloeString (2589410.5894, 7, true), AloeString ("2.5894106e+06"));
                expectEquals (AloeString (12345.67, 8,     true), AloeString ("1.23456700e+04"));
                expectEquals (AloeString (1e19, 4,         true), AloeString ("1.0000e+19"));
                expectEquals (AloeString (1e-34, 5,        true), AloeString ("1.00000e-34"));
                expectEquals (AloeString (1.39, 1,         true), AloeString ("1.4e+00"));

                beginTest ("Subsections");
                AloeString s3;
                s3 = "abcdeFGHIJ";
                expect (s3.equalsIgnoreCase ("ABCdeFGhiJ"));
                expect (s3.compareIgnoreCase (L"ABCdeFGhiJ") == 0);
                expect (s3.containsIgnoreCase (s3.substring (3)));
                expect (s3.indexOfAnyOf ("xyzf", 2, true) == 5);
                expect (s3.indexOfAnyOf (AloeString (L"xyzf"), 2, false) == -1);
                expect (s3.indexOfAnyOf ("xyzF", 2, false) == 5);
                expect (s3.containsAnyOf (AloeString (L"zzzFs")));
                expect (s3.startsWith ("abcd"));
                expect (s3.startsWithIgnoreCase (AloeString (L"abCD")));
                expect (s3.startsWith (AloeString()));
                expect (s3.startsWithChar ('a'));
                expect (s3.endsWith (AloeString ("HIJ")));
                expect (s3.endsWithIgnoreCase (AloeString (L"Hij")));
                expect (s3.endsWith (AloeString()));
                expect (s3.endsWithChar (L'J'));
                expect (s3.indexOf ("HIJ") == 7);
                expect (s3.indexOf (AloeString (L"HIJK")) == -1);
                expect (s3.indexOfIgnoreCase ("hij") == 7);
                expect (s3.indexOfIgnoreCase (AloeString (L"hijk")) == -1);
                expect (s3.toStdString() == s3.toRawUTF8());

                AloeString s4 (s3);
                s4.append (AloeString ("xyz123"), 3);
                expect (s4 == s3 + "xyz");

                expect (AloeString (1234) < AloeString (1235));
                expect (AloeString (1235) > AloeString (1234));
                expect (AloeString (1234) >= AloeString (1234));
                expect (AloeString (1234) <= AloeString (1234));
                expect (AloeString (1235) >= AloeString (1234));
                expect (AloeString (1234) <= AloeString (1235));

                AloeString s5 ("word word2 word3");
                expect (s5.containsWholeWord (AloeString ("word2")));
                expect (s5.indexOfWholeWord ("word2") == 5);
                expect (s5.containsWholeWord (AloeString (L"word")));
                expect (s5.containsWholeWord ("word3"));
                expect (s5.containsWholeWord (s5));
                expect (s5.containsWholeWordIgnoreCase (AloeString (L"Word2")));
                expect (s5.indexOfWholeWordIgnoreCase ("Word2") == 5);
                expect (s5.containsWholeWordIgnoreCase (AloeString (L"Word")));
                expect (s5.containsWholeWordIgnoreCase ("Word3"));
                expect (! s5.containsWholeWordIgnoreCase (AloeString (L"Wordx")));
                expect (! s5.containsWholeWordIgnoreCase ("xWord2"));
                expect (s5.containsNonWhitespaceChars());
                expect (s5.containsOnly ("ordw23 "));
                expect (! AloeString (" \n\r\t").containsNonWhitespaceChars());

                expect (s5.matchesWildcard (AloeString (L"wor*"), false));
                expect (s5.matchesWildcard ("wOr*", true));
                expect (s5.matchesWildcard (AloeString (L"*word3"), true));
                expect (s5.matchesWildcard ("*word?", true));
                expect (s5.matchesWildcard (AloeString (L"Word*3"), true));
                expect (! s5.matchesWildcard (AloeString (L"*34"), true));
                expect (AloeString ("xx**y").matchesWildcard ("*y", true));
                expect (AloeString ("xx**y").matchesWildcard ("x*y", true));
                expect (AloeString ("xx**y").matchesWildcard ("xx*y", true));
                expect (AloeString ("xx**y").matchesWildcard ("xx*", true));
                expect (AloeString ("xx?y").matchesWildcard ("x??y", true));
                expect (AloeString ("xx?y").matchesWildcard ("xx?y", true));
                expect (! AloeString ("xx?y").matchesWildcard ("xx?y?", true));
                expect (AloeString ("xx?y").matchesWildcard ("xx??", true));

                expectEquals (s5.fromFirstOccurrenceOf (AloeString(), true, false), s5);
                expectEquals (s5.fromFirstOccurrenceOf ("xword2", true, false), s5.substring (100));
                expectEquals (s5.fromFirstOccurrenceOf (AloeString (L"word2"), true, false), s5.substring (5));
                expectEquals (s5.fromFirstOccurrenceOf ("Word2", true, true), s5.substring (5));
                expectEquals (s5.fromFirstOccurrenceOf ("word2", false, false), s5.getLastCharacters (6));
                expectEquals (s5.fromFirstOccurrenceOf ("Word2", false, true), s5.getLastCharacters (6));

                expectEquals (s5.fromLastOccurrenceOf (AloeString(), true, false), s5);
                expectEquals (s5.fromLastOccurrenceOf ("wordx", true, false), s5);
                expectEquals (s5.fromLastOccurrenceOf ("word", true, false), s5.getLastCharacters (5));
                expectEquals (s5.fromLastOccurrenceOf ("worD", true, true), s5.getLastCharacters (5));
                expectEquals (s5.fromLastOccurrenceOf ("word", false, false), s5.getLastCharacters (1));
                expectEquals (s5.fromLastOccurrenceOf ("worD", false, true), s5.getLastCharacters (1));

                expect (s5.upToFirstOccurrenceOf (AloeString(), true, false).isEmpty());
                expectEquals (s5.upToFirstOccurrenceOf ("word4", true, false), s5);
                expectEquals (s5.upToFirstOccurrenceOf ("word2", true, false), s5.substring (0, 10));
                expectEquals (s5.upToFirstOccurrenceOf ("Word2", true, true), s5.substring (0, 10));
                expectEquals (s5.upToFirstOccurrenceOf ("word2", false, false), s5.substring (0, 5));
                expectEquals (s5.upToFirstOccurrenceOf ("Word2", false, true), s5.substring (0, 5));

                expectEquals (s5.upToLastOccurrenceOf (AloeString(), true, false), s5);
                expectEquals (s5.upToLastOccurrenceOf ("zword", true, false), s5);
                expectEquals (s5.upToLastOccurrenceOf ("word", true, false), s5.dropLastCharacters (1));
                expectEquals (s5.dropLastCharacters(1).upToLastOccurrenceOf ("word", true, false), s5.dropLastCharacters (1));
                expectEquals (s5.upToLastOccurrenceOf ("Word", true, true), s5.dropLastCharacters (1));
                expectEquals (s5.upToLastOccurrenceOf ("word", false, false), s5.dropLastCharacters (5));
                expectEquals (s5.upToLastOccurrenceOf ("Word", false, true), s5.dropLastCharacters (5));

                expectEquals (s5.replace ("word", "xyz", false), AloeString ("xyz xyz2 xyz3"));
                expect (s5.replace ("Word", "xyz", true) == "xyz xyz2 xyz3");
                expect (s5.dropLastCharacters (1).replace ("Word", AloeString ("xyz"), true) == L"xyz xyz2 xyz");
                expect (s5.replace ("Word", "", true) == " 2 3");
                expectEquals (s5.replace ("Word2", "xyz", true), AloeString ("word xyz word3"));
                expect (s5.replaceCharacter (L'w', 'x') != s5);
                expectEquals (s5.replaceCharacter ('w', L'x').replaceCharacter ('x', 'w'), s5);
                expect (s5.replaceCharacters ("wo", "xy") != s5);
                expectEquals (s5.replaceCharacters ("wo", "xy").replaceCharacters ("xy", "wo"), s5);
                expectEquals (s5.retainCharacters ("1wordxya"), AloeString ("wordwordword"));
                expect (s5.retainCharacters (AloeString()).isEmpty());
                expect (s5.removeCharacters ("1wordxya") == " 2 3");
                expectEquals (s5.removeCharacters (AloeString()), s5);
                expect (s5.initialSectionContainingOnly ("word") == L"word");
                expect (AloeString ("word").initialSectionContainingOnly ("word") == L"word");
                expectEquals (s5.initialSectionNotContaining (AloeString ("xyz ")), AloeString ("word"));
                expectEquals (s5.initialSectionNotContaining (AloeString (";[:'/")), s5);
                expect (! s5.isQuotedString());
                expect (s5.quoted().isQuotedString());
                expect (! s5.quoted().unquoted().isQuotedString());
                expect (! AloeString ("x'").isQuotedString());
                expect (AloeString ("'x").isQuotedString());

                AloeString s6 (" \t xyz  \t\r\n");
                expectEquals (s6.trim(), AloeString ("xyz"));
                expect (s6.trim().trim() == "xyz");
                expectEquals (s5.trim(), s5);
                expectEquals (s6.trimStart().trimEnd(), s6.trim());
                expectEquals (s6.trimStart().trimEnd(), s6.trimEnd().trimStart());
                expectEquals (s6.trimStart().trimStart().trimEnd().trimEnd(), s6.trimEnd().trimStart());
                expect (s6.trimStart() != s6.trimEnd());
                expectEquals (("\t\r\n " + s6 + "\t\n \r").trim(), s6.trim());
                expect (AloeString::repeatedString ("xyz", 3) == L"xyzxyzxyz");
            }

            {
                beginTest ("UTF conversions");

                TestUTFConversion <CharPointer_UTF32>::test (*this, r);
                TestUTFConversion <CharPointer_UTF8>::test (*this, r);
                TestUTFConversion <CharPointer_UTF16>::test (*this, r);
            }

            {
                beginTest ("StringArray");

                StringArray s;
                s.addTokens ("4,3,2,1,0", ";,", "x");
                expectEquals (s.size(), 5);

                expectEquals (s.joinIntoString ("-"), AloeString ("4-3-2-1-0"));
                s.remove (2);
                expectEquals (s.joinIntoString ("--"), AloeString ("4--3--1--0"));
                expectEquals (s.joinIntoString (""), AloeString ("4310"));
                s.clear();
                expectEquals (s.joinIntoString ("x"), AloeString());

                StringArray toks;
                toks.addTokens ("x,,", ";,", "");
                expectEquals (toks.size(), 3);
                expectEquals (toks.joinIntoString ("-"), AloeString ("x--"));
                toks.clear();

                toks.addTokens (",x,", ";,", "");
                expectEquals (toks.size(), 3);
                expectEquals (toks.joinIntoString ("-"), AloeString ("-x-"));
                toks.clear();

                toks.addTokens ("x,'y,z',", ";,", "'");
                expectEquals (toks.size(), 3);
                expectEquals (toks.joinIntoString ("-"), AloeString ("x-'y,z'-"));
            }

            {
                beginTest ("var");

                var v1 = 0;
                var v2 = 0.16;
                var v3 = "0.16";
                var v4 = (int64) 0;
                var v5 = 0.0;
                expect (! v2.equals (v1));
                expect (! v1.equals (v2));
                expect (v2.equals (v3));
                expect (! v3.equals (v1));
                expect (! v1.equals (v3));
                expect (v1.equals (v4));
                expect (v4.equals (v1));
                expect (v5.equals (v4));
                expect (v4.equals (v5));
                expect (! v2.equals (v4));
                expect (! v4.equals (v2));
            }

            {
                beginTest ("Significant figures");

                // Integers

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (13, 1), AloeString ("10"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (13, 2), AloeString ("13"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (13, 3), AloeString ("13.0"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (13, 4), AloeString ("13.00"));

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (19368, 1), AloeString ("20000"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (19348, 3), AloeString ("19300"));

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (-5, 1), AloeString ("-5"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (-5, 3), AloeString ("-5.00"));

                // Zero

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (0, 1), AloeString ("0"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (0, 2), AloeString ("0.0"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (0, 3), AloeString ("0.00"));

                // Floating point

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (19.0, 1), AloeString ("20"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (19.0, 2), AloeString ("19"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (19.0, 3), AloeString ("19.0"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (19.0, 4), AloeString ("19.00"));

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (-5.45, 1), AloeString ("-5"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (-5.45, 3), AloeString ("-5.45"));

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (12345.6789, 9), AloeString ("12345.6789"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (12345.6789, 8), AloeString ("12345.679"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (12345.6789, 5), AloeString ("12346"));

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (0.00028647, 6), AloeString ("0.000286470"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (0.0028647,  6), AloeString ("0.00286470"));
                expectEquals (AloeString::toDecimalStringWithSignificantFigures (2.8647,     6), AloeString ("2.86470"));

                expectEquals (AloeString::toDecimalStringWithSignificantFigures (-0.0000000000019, 1), AloeString ("-0.000000000002"));
            }

            {
                beginTest ("Float trimming");

                {
                    StringPairArray tests;
                    tests.set ("1", "1");
                    tests.set ("1.0", "1.0");
                    tests.set ("-1", "-1");
                    tests.set ("-100", "-100");
                    tests.set ("110", "110");
                    tests.set ("9090", "9090");
                    tests.set ("1000.0", "1000.0");
                    tests.set ("1.0", "1.0");
                    tests.set ("-1.00", "-1.0");
                    tests.set ("1.20", "1.2");
                    tests.set ("1.300", "1.3");
                    tests.set ("1.301", "1.301");
                    tests.set ("1e", "1");
                    tests.set ("-1e+", "-1");
                    tests.set ("1e-", "1");
                    tests.set ("1e0", "1");
                    tests.set ("1e+0", "1");
                    tests.set ("1e-0", "1");
                    tests.set ("1e000", "1");
                    tests.set ("1e+000", "1");
                    tests.set ("-1e-000", "-1");
                    tests.set ("1e100", "1e100");
                    tests.set ("100e100", "100e100");
                    tests.set ("100.0e0100", "100.0e100");
                    tests.set ("-1e1", "-1e1");
                    tests.set ("1e10", "1e10");
                    tests.set ("-1e+10", "-1e10");
                    tests.set ("1e-10", "1e-10");
                    tests.set ("1e0010", "1e10");
                    tests.set ("1e-0010", "1e-10");
                    tests.set ("1e-1", "1e-1");
                    tests.set ("-1.0e1", "-1.0e1");
                    tests.set ("1.0e-1", "1.0e-1");
                    tests.set ("1.00e-1", "1.0e-1");
                    tests.set ("1.001e1", "1.001e1");
                    tests.set ("1.010e+1", "1.01e1");
                    tests.set ("-1.1000e1", "-1.1e1");

                    for (auto& input : tests.getAllKeys())
                        expectEquals (reduceLengthOfFloatString (input), tests[input]);
                }

                {
                    std::map<double, AloeString> tests;
                    tests[1] = "1.0";
                    tests[1.1] = "1.1";
                    tests[1.01] = "1.01";
                    tests[0.76378] = "7.6378e-1";
                    tests[-10] = "-1.0e1";
                    tests[10.01] = "1.001e1";
                    tests[10691.01] = "1.069101e4";
                    tests[0.0123] = "1.23e-2";
                    tests[-3.7e-27] = "-3.7e-27";
                    tests[1e+40] = "1.0e40";

                    for (auto& test : tests)
                        expectEquals (reduceLengthOfFloatString (AloeString (test.first, 15, true)), test.second);
                }
            }

            {
                beginTest ("Serialisation");

                std::map <double, AloeString> tests;

                tests[364] = "364.0";
                tests[1e7] = "1.0e7";
                tests[12345678901] = "1.2345678901e10";

                tests[1234567890123456.7] = "1.234567890123457e15";
                tests[12345678.901234567] = "1.234567890123457e7";
                tests[1234567.8901234567] = "1.234567890123457e6";
                tests[123456.78901234567] = "123456.7890123457";
                tests[12345.678901234567] = "12345.67890123457";
                tests[1234.5678901234567] = "1234.567890123457";
                tests[123.45678901234567] = "123.4567890123457";
                tests[12.345678901234567] = "12.34567890123457";
                tests[1.2345678901234567] = "1.234567890123457";
                tests[0.12345678901234567] = "0.1234567890123457";
                tests[0.012345678901234567] = "0.01234567890123457";
                tests[0.0012345678901234567] = "0.001234567890123457";
                tests[0.00012345678901234567] = "0.0001234567890123457";
                tests[0.000012345678901234567] = "0.00001234567890123457";
                tests[0.0000012345678901234567] = "1.234567890123457e-6";
                tests[0.00000012345678901234567] = "1.234567890123457e-7";

                for (auto& test : tests)
                {
                    expectEquals (serialiseDouble (test.first), test.second);
                    expectEquals (serialiseDouble (-test.first), "-" + test.second);
                }
            }

            {
                beginTest ("Loops");

                AloeString str (CharPointer_UTF8 ("\xc2\xaf\\_(\xe3\x83\x84)_/\xc2\xaf"));
                std::vector<aloe_wchar> parts { 175, 92, 95, 40, 12484, 41, 95, 47, 175 };
                size_t index = 0;

                for (auto c : str)
                    expectEquals (c, parts[index++]);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static StringTests stringUnitTests;
    */
}
