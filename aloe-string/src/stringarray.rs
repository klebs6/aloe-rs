crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_StringArray.h]

/**
  | A special array for holding a list of
  | strings.
  | 
  | @see AloeString, StringPairArray
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct StringArray {

    /**
      | This is the array holding the actual
      | strings. This is public to allow direct
      | access to array methods that may not
      | already be provided by the StringArray
      | class.
      |
      */
    strings: Vec<AloeString>,
}

impl Default for StringArray {
    
    /**
      | Creates an empty string array
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl PartialEq<StringArray> for StringArray {
    
    /**
      | Compares two arrays.
      | 
      | Comparisons are case-sensitive.
      | 
      | -----------
      | @return
      | 
      | true only if the other array contains
      | exactly the same strings in the same
      | order
      |
      */
    #[inline] fn eq(&self, other: &StringArray) -> bool {
        todo!();
        /*
            return strings == other.strings;
        */
    }
}

impl Eq for StringArray {}

impl Index<i32> for StringArray {

    type Output = AloeString;
    
    /**
      | Returns one of the strings from the array.
      | 
      | If the index is out-of-range, an empty
      | string is returned.
      | 
      | Obviously the reference returned shouldn't
      | be stored for later use, as the string
      | it refers to may disappear when the array
      | changes.
      |
      */
    #[inline] fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            if (isPositiveAndBelow (index, strings.size()))
            return strings.getReference (index);

        static AloeString empty;
        return empty;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_StringArray.cpp]
impl StringArray {

    /**
      | Creates an array containing a list of
      | strings.
      |
      */
    pub fn new<OtherElements>(
        first_value:  &str,
        other_values: OtherElements) -> Self {
    
        todo!();
        /*
            : strings (firstValue, std::forward<OtherElements> (otherValues)...)
        */
    }

    /**
      | Creates a StringArray from an array
      | of objects which can be implicitly converted
      | to Strings.
      |
      */
    pub fn new_from_slice_of_string_convertible_objects<T: ToString>(string_array: &[T]) -> Self {
    
        todo!();
        /*
            addArray (stringArray.begin(), stringArray.end());
        */
    }

    /**
      | Returns the number of strings in the
      | array
      |
      */
    #[inline] pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return strings.size();
        */
    }

    /**
      | Returns true if the array is empty, false
      | otherwise.
      |
      */
    #[inline] pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return size() == 0;
        */
    }

    /**
      | Returns a pointer to the first AloeString
      | in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn begin_mut(&mut self) -> *mut AloeString {
        
        todo!();
        /*
            return strings.begin();
        */
    }

    /**
      | Returns a pointer to the first AloeString
      | in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn begin(&self) -> *const AloeString {
        
        todo!();
        /*
            return strings.begin();
        */
    }

    /**
      | Returns a pointer to the AloeString which
      | follows the last element in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn end_mut(&mut self) -> *mut AloeString {
        
        todo!();
        /*
            return strings.end();
        */
    }

    /**
      | Returns a pointer to the AloeString which
      | follows the last element in the array.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    #[inline] pub fn end(&self) -> *const AloeString {
        
        todo!();
        /*
            return strings.end();
        */
    }

    /**
      | Adds items from a range of start/end
      | iterators of some kind of objects which
      | can be implicitly converted to Strings.
      |
      */
    pub fn add_array_with_range<Iterator>(&mut self, 
        start: Iterator,
        end:   Iterator)  {
    
        todo!();
        /*
            ensureStorageAllocated (size() + (int) static_cast<size_t> (end - start));

            while (start != end)
                strings.add (*start++);
        */
    }

    /**
      | Creates a copy of another string array
      |
      */
    pub fn new_from_other_ref(other: &StringArray) -> Self {
    
        todo!();
        /*
        : strings(other.strings),
        */
    }
    
    pub fn new_from_other(other: StringArray) -> Self {
    
        todo!();
        /*
        : strings(std::move (other.strings)),
        */
    }
    
    pub fn new_from_vec_string(other: Vec<AloeString>) -> Self {
    
        todo!();
        /*
        : strings(std::move (other)),
        */
    }
    
    /**
      | Creates an array containing a single
      | string.
      |
      */
    pub fn new_from_first_value(first_value: &AloeString) -> Self {
    
        todo!();
        /*
            strings.add (firstValue);
        */
    }

    /**
      | Creates an array from a raw array of strings.
      | 
      | -----------
      | @param strings
      | 
      | an array of strings to add
      | ----------
      | @param numberOfStrings
      | 
      | how many items there are in the array
      |
      */
    pub fn new_from_raw_string_array(
        initial_strings:   *const AloeString,
        number_of_strings: i32) -> Self {
    
        todo!();
        /*
            strings.addArray (initialStrings, numberOfStrings);
        */
    }
    
    /**
      | Creates a copy of a null-terminated
      | array of string literals.
      | 
      | Each item from the array passed-in is
      | added, until it encounters a null pointer,
      | at which point it stops.
      |
      */
    pub fn new_from_null_terminated_array_of_string_literals(initial_strings: *const *const u8) -> Self {
    
        todo!();
        /*
            strings.addNullTerminatedArray (initialStrings);
        */
    }
    
    /**
      | Creates a copy of an array of string literals.
      | 
      | -----------
      | @param strings
      | 
      | an array of strings to add. Null pointers
      | in the array will be treated as empty
      | strings
      | ----------
      | @param numberOfStrings
      | 
      | how many items there are in the array
      |
      */
    pub fn new_by_copy_of_string_literal_array(
        initial_strings:   *const *const u8,
        number_of_strings: i32) -> Self {
    
        todo!();
        /*
            strings.addArray (initialStrings, numberOfStrings);
        */
    }
    
    /**
      | Creates a copy of a null-terminated
      | array of string literals.
      | 
      | Each item from the array passed-in is
      | added, until it encounters a null pointer,
      | at which point it stops.
      |
      */
    pub fn new_by_copy_of_null_terminated_wchar_array(initial_strings: *const *const wchar_t) -> Self {
    
        todo!();
        /*
            strings.addNullTerminatedArray (initialStrings);
        */
    }
    
    /**
      | Creates a copy of an array of string literals.
      | 
      | -----------
      | @param strings
      | 
      | an array of strings to add. Null pointers
      | in the array will be treated as empty
      | strings
      | ----------
      | @param numberOfStrings
      | 
      | how many items there are in the array
      |
      */
    pub fn new_by_copy_of_wchar_string_literal_array(
        initial_strings:   *const *const wchar_t,
        number_of_strings: i32) -> Self {
    
        todo!();
        /*
            strings.addArray (initialStrings, numberOfStrings);
        */
    }
    
    /**
      | Creates an array containing a list of
      | strings.
      |
      */
    pub fn new_from_slice_of_raw_strings(string_list: &[*const u8]) -> Self {
    
        todo!();
        /*
            strings.addArray (stringList);
        */
    }
    
    /**
      | Copies a StringArray from an array of
      | objects which can be implicitly converted
      | to Strings.
      |
      */
    pub fn assign_from_string_array<Type>(&mut self, string_array: &[Type]) -> &mut StringArray {
    
        todo!();
        /*
            addArray (stringArray.begin(), stringArray.end());
            return *this;
        */
    }

    /**
      | Copies the contents of another string
      | array into this one
      |
      */
    pub fn assign_from_other_ref(&mut self, other: &StringArray) -> &mut StringArray {
        
        todo!();
        /*
            strings = other.strings;
        return *this;
        */
    }
    
    /**
      | Move assignment operator
      |
      */
    pub fn assign_from_other(&mut self, other: StringArray) -> &mut StringArray {
        
        todo!();
        /*
            strings = std::move (other.strings);
        return *this;
        */
    }
    
    /**
      | Swaps the contents of this and another
      | StringArray.
      |
      */
    pub fn swap_with(&mut self, other: &mut StringArray)  {
        
        todo!();
        /*
            strings.swapWith (other.strings);
        */
    }
    
    /**
      | Removes all elements from the array.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            strings.clear();
        */
    }

    /**
      | Removes all elements from the array
      | without freeing the array's allocated
      | storage. @see clear
      |
      */
    pub fn clear_quick(&mut self)  {
        
        todo!();
        /*
            strings.clearQuick();
        */
    }

    /**
      | Returns a reference to one of the strings
      | in the array.
      | 
      | This lets you modify a string in-place
      | in the array, but you must be sure that
      | the index is in-range.
      |
      */
    pub fn get_reference_mut(&mut self, index: i32) -> &mut AloeString {
        
        todo!();
        /*
            return strings.getReference (index);
        */
    }

    /**
      | Returns a reference to one of the strings
      | in the array.
      | 
      | This lets you modify a string in-place
      | in the array, but you must be sure that
      | the index is in-range.
      |
      */
    pub fn get_reference(&self, index: i32) -> &AloeString {
        
        todo!();
        /*
            return strings.getReference (index);
        */
    }

    /**
      | Appends a string at the end of the array.
      |
      */
    pub fn add(&mut self, new_string: AloeString)  {
        
        todo!();
        /*
            // NB: the local temp copy is to avoid a dangling pointer if the
        // argument being passed-in is a reference into this array.
        strings.add (std::move (newString));
        */
    }

    /**
      | Inserts a string into the array.
      | 
      | This will insert a string into the array
      | at the given index, moving up the other
      | elements to make room for it.
      | 
      | If the index is less than zero or greater
      | than the size of the array, the new string
      | will be added to the end of the array.
      |
      */
    pub fn insert(&mut self, 
        index:      i32,
        new_string: AloeString)  {
        
        todo!();
        /*
            // NB: the local temp copy is to avoid a dangling pointer if the
        // argument being passed-in is a reference into this array.
        strings.insert (index, std::move (newString));
        */
    }

    /**
      | Adds a string to the array as long as it's
      | not already in there.
      | 
      | The search can optionally be case-insensitive.
      | 
      | -----------
      | @return
      | 
      | true if the string has been added, false
      | otherwise.
      |
      */
    pub fn add_if_not_already_there(
        &mut self, 
        new_string:  &AloeString,
        ignore_case: Option<bool>) -> bool 
    {
        let ignore_case: bool = ignore_case.unwrap_or(false);

        todo!();
        /*
            if (contains (newString, ignoreCase))
            return false;

        add (newString);
        return true;
        */
    }

    /**
      | Appends some strings from another array
      | to the end of this one.
      | 
      | -----------
      | @param other
      | 
      | the array to add
      | ----------
      | @param startIndex
      | 
      | the first element of the other array
      | to add
      | ----------
      | @param numElementsToAdd
      | 
      | the maximum number of elements to add
      | (if this is less than zero, they are all
      | added)
      |
      */
    pub fn add_array(&mut self, 
        other_array:         &StringArray,
        start_index:         Option<i32>,
        num_elements_to_add: Option<i32>) 
    {
        let start_index:         i32 = start_index.unwrap_or(0);
        let num_elements_to_add: i32 = num_elements_to_add.unwrap_or(-1);
        
        todo!();
        /*
            jassert (this != &otherArray); // can't add from our own elements!

        if (startIndex < 0)
        {
            jassertfalse;
            startIndex = 0;
        }

        if (numElementsToAdd < 0 || startIndex + numElementsToAdd > otherArray.size())
            numElementsToAdd = otherArray.size() - startIndex;

        while (--numElementsToAdd >= 0)
            strings.add (otherArray.strings.getReference (startIndex++));
        */
    }

    /**
      | Merges the strings from another array
      | into this one.
      | 
      | This will not add a string that already
      | exists.
      | 
      | -----------
      | @param other
      | 
      | the array to add
      | ----------
      | @param ignoreCase
      | 
      | ignore case when merging
      |
      */
    pub fn merge_array(
        &mut self, 
        other_array: &StringArray,
        ignore_case: Option<bool>)
    {
        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            jassert (this != &otherArray); // can't add from our own elements!

        for (auto& s : otherArray)
            addIfNotAlreadyThere (s, ignoreCase);
        */
    }

    /**
      | Replaces one of the strings in the array
      | with another one.
      | 
      | If the index is higher than the array's
      | size, the new string will be added to
      | the end of the array; if it's less than
      | zero nothing happens.
      |
      */
    pub fn set(&mut self, 
        index:      i32,
        new_string: AloeString)  {
        
        todo!();
        /*
            strings.set (index, std::move (newString));
        */
    }
    
    /**
      | Searches for a string in the array.
      | 
      | The comparison will be case-insensitive
      | if the ignoreCase parameter is true.
      | 
      | -----------
      | @return
      | 
      | true if the string is found inside the
      | array
      |
      */
    pub fn contains(
        &self, 
        string_to_look_for: &str,
        ignore_case:        Option<bool>) -> bool 
    {
        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            return indexOf (stringToLookFor, ignoreCase) >= 0;
        */
    }

    /**
      | Searches for a string in the array.
      | 
      | The comparison will be case-insensitive
      | if the ignoreCase parameter is true.
      | 
      | -----------
      | @param stringToLookFor
      | 
      | the string to try to find
      | ----------
      | @param ignoreCase
      | 
      | whether the comparison should be case-insensitive
      | ----------
      | @param startIndex
      | 
      | the first index to start searching from
      | 
      | -----------
      | @return
      | 
      | the index of the first occurrence of
      | the string in this array, or -1 if it isn't
      | found.
      |
      */
    pub fn index_of(
        &self, 
        string_to_look_for: &str,
        ignore_case:        Option<bool>,
        idx:                Option<i32>) -> i32 
    {
        let ignore_case: bool = ignore_case.unwrap_or(false);
        let idx:          i32 = idx.unwrap_or(0);

        todo!();
        /*
            if (idx < 0)
            idx = 0;

        auto numElements = size();

        if (ignoreCase)
        {
            for (; idx < numElements; ++idx)
                if (strings.getReference(idx).equalsIgnoreCase (stringToLookFor))
                    return idx;
        }
        else
        {
            for (; idx < numElements; ++idx)
                if (stringToLookFor == strings.getReference (idx))
                    return idx;
        }

        return -1;
        */
    }

    /**
      | Moves one of the strings to a different
      | position.
      | 
      | This will move the string to a specified
      | index, shuffling along any intervening
      | elements as required.
      | 
      | So for example, if you have the array
      | { 0, 1, 2, 3, 4, 5 } then calling move (2,
      | 4) would result in { 0, 1, 3, 4, 2, 5 }.
      | 
      | -----------
      | @param currentIndex
      | 
      | the index of the value to be moved. If
      | this isn't a valid index, then nothing
      | will be done
      | ----------
      | @param newIndex
      | 
      | the index at which you'd like this value
      | to end up. If this is less than zero, the
      | value will be moved to the end of the array
      |
      */
    pub fn move_(&mut self, 
        current_index: i32,
        new_index:     i32)  {
        
        todo!();
        /*
            strings.move (currentIndex, newIndex);
        */
    }

    /**
      | Removes a string from the array.
      | 
      | If the index is out-of-range, no action
      | will be taken.
      |
      */
    pub fn remove(&mut self, index: i32)  {
        
        todo!();
        /*
            strings.remove (index);
        */
    }

    /**
      | Finds a string in the array and removes
      | it.
      | 
      | This will remove all occurrences of
      | the given string from the array.
      | 
      | The comparison may be case-insensitive
      | depending on the ignoreCase parameter.
      |
      */
    pub fn remove_string(
        &mut self, 
        string_to_remove: &str,
        ignore_case:      Option<bool>)  {

        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            if (ignoreCase)
        {
            for (int i = size(); --i >= 0;)
                if (strings.getReference(i).equalsIgnoreCase (stringToRemove))
                    strings.remove (i);
        }
        else
        {
            for (int i = size(); --i >= 0;)
                if (stringToRemove == strings.getReference (i))
                    strings.remove (i);
        }
        */
    }

    /**
      | Removes a range of elements from the
      | array.
      | 
      | This will remove a set of elements, starting
      | from the given index, and move subsequent
      | elements down to close the gap.
      | 
      | If the range extends beyond the bounds
      | of the array, it will be safely clipped
      | to the size of the array.
      | 
      | -----------
      | @param startIndex
      | 
      | the index of the first element to remove
      | ----------
      | @param numberToRemove
      | 
      | how many elements should be removed
      |
      */
    pub fn remove_range(&mut self, 
        start_index:      i32,
        number_to_remove: i32)  {
        
        todo!();
        /*
            strings.removeRange (startIndex, numberToRemove);
        */
    }

    /**
      | Removes empty strings from the array.
      | 
      | -----------
      | @param removeWhitespaceStrings
      | 
      | if true, strings that only contain whitespace
      | characters will also be removed
      |
      */
    pub fn remove_empty_strings(&mut self, remove_whitespace_strings: Option<bool>)  {
        
        let remove_whitespace_strings: bool = remove_whitespace_strings.unwrap_or(true);

        todo!();
        /*
            if (removeWhitespaceStrings)
        {
            for (int i = size(); --i >= 0;)
                if (! strings.getReference(i).containsNonWhitespaceChars())
                    strings.remove (i);
        }
        else
        {
            for (int i = size(); --i >= 0;)
                if (strings.getReference(i).isEmpty())
                    strings.remove (i);
        }
        */
    }

    /**
      | Deletes any whitespace characters
      | from the starts and ends of all the strings.
      |
      */
    pub fn trim(&mut self)  {
        
        todo!();
        /*
            for (auto& s : strings)
            s = s.trim();
        */
    }

    /**
      | Sorts the array into alphabetical order.
      | 
      | -----------
      | @param ignoreCase
      | 
      | if true, the comparisons used will not
      | be case-sensitive.
      |
      */
    pub fn sort(&mut self, ignore_case: bool)  {
        
        todo!();
        /*
            if (ignoreCase)
            std::sort (strings.begin(), strings.end(),
                       [] (const AloeString& a, const AloeString& b) { return a.compareIgnoreCase (b) < 0; });
        else
            std::sort (strings.begin(), strings.end());
        */
    }

    /**
      | Sorts the array using extra language-aware
      | rules to do a better job of comparing
      | words containing spaces and numbers.
      | @see AloeString::compareNatural()
      |
      */
    pub fn sort_natural(&mut self)  {
        
        todo!();
        /*
            std::sort (strings.begin(), strings.end(),
                   [] (const AloeString& a, const AloeString& b) { return a.compareNatural (b) < 0; });
        */
    }

    /**
      | Joins the strings in the array together
      | into one string.
      | 
      | This will join a range of elements from
      | the array into a string, separating
      | them with a given string.
      | 
      | e.g. joinIntoString (",") will turn
      | an array of "a" "b" and "c" into "a,b,c".
      | 
      | -----------
      | @param separatorString
      | 
      | the string to insert between all the
      | strings
      | ----------
      | @param startIndex
      | 
      | the first element to join
      | ----------
      | @param numberOfElements
      | 
      | how many elements to join together.
      | If this is less than zero, all available
      | elements will be used.
      |
      */
    pub fn join_into_string(&self, 
        separator:      &str,
        start:          Option<i32>,
        number_to_join: Option<i32>) -> AloeString {

        let start:          i32 = start.unwrap_or(0);
        let number_to_join: i32 = number_to_join.unwrap_or(-1);
        
        todo!();
        /*
            auto last = (numberToJoin < 0) ? size()
                                       : jmin (size(), start + numberToJoin);

        if (start < 0)
            start = 0;

        if (start >= last)
            return {};

        if (start == last - 1)
            return strings.getReference (start);

        auto separatorBytes = separator.text.sizeInBytes() - sizeof (CharPointerType::CharType);
        auto bytesNeeded = (size_t) (last - start - 1) * separatorBytes;

        for (int i = start; i < last; ++i)
            bytesNeeded += strings.getReference(i).getCharPointer().sizeInBytes() - sizeof (CharPointerType::CharType);

        AloeString result;
        result.preallocateBytes (bytesNeeded);

        auto dest = result.getCharPointer();

        while (start < last)
        {
            auto& s = strings.getReference (start);

            if (! s.isEmpty())
                dest.writeAll (s.getCharPointer());

            if (++start < last && separatorBytes > 0)
                dest.writeAll (separator.text);
        }

        dest.writeNull();
        return result;
        */
    }

    /**
      | Breaks up a string into tokens and adds
      | them to this array.
      | 
      | This will tokenise the given string
      | using whitespace characters as the
      | token delimiters, and will add these
      | tokens to the end of the array.
      | 
      | -----------
      | @return
      | 
      | the number of tokens added @see fromTokens
      |
      */
    pub fn add_tokens(&mut self, 
        text:                    &str,
        preserve_quoted_strings: bool) -> i32 {
        
        todo!();
        /*
            return addTokens (text, " \n\r\t", preserveQuotedStrings ? "\"" : "");
        */
    }

    /**
      | Breaks up a string into tokens and adds
      | them to this array.
      | 
      | This will tokenise the given string
      | (using the string passed in to define
      | the token delimiters), and will add
      | these tokens to the end of the array.
      | 
      | -----------
      | @param stringToTokenise
      | 
      | the string to tokenise
      | ----------
      | @param breakCharacters
      | 
      | a string of characters, any of which
      | will be considered to be a token delimiter.
      | ----------
      | @param quoteCharacters
      | 
      | if this string isn't empty, it defines
      | a set of characters which are treated
      | as quotes. Any text occurring between
      | quotes is not broken up into tokens.
      | 
      | -----------
      | @return
      | 
      | the number of tokens added @see fromTokens
      |
      */
    pub fn add_tokens_with_break_chars(&mut self, 
        text:             &str,
        break_characters: &str,
        quote_characters: &str) -> i32 {
        
        todo!();
        /*
            int num = 0;

        if (text.isNotEmpty())
        {
            for (auto t = text.text;;)
            {
                auto tokenEnd = CharacterFunctions::findEndOfToken (t,
                                                                    breakCharacters.text,
                                                                    quoteCharacters.text);
                strings.add (AloeString (t, tokenEnd));
                ++num;

                if (tokenEnd.isEmpty())
                    break;

                t = ++tokenEnd;
            }
        }

        return num;
        */
    }

    /**
      | Breaks up a string into lines and adds
      | them to this array.
      | 
      | This breaks a string down into lines
      | separated by \\n or \\r\\n, and adds
      | each line to the array. Line-break characters
      | are omitted from the strings that are
      | added to the array.
      |
      */
    pub fn add_lines(&mut self, source_text: &str) -> i32 {
        
        todo!();
        /*
            int numLines = 0;
        auto text = sourceText.text;
        bool finished = text.isEmpty();

        while (! finished)
        {
            for (auto startOfLine = text;;)
            {
                auto endOfLine = text;

                switch (text.getAndAdvance())
                {
                    case 0:     finished = true; break;
                    case '\n':  break;
                    case '\r':  if (*text == '\n') ++text; break;
                    default:    continue;
                }

                strings.add (AloeString (startOfLine, endOfLine));
                ++numLines;
                break;
            }
        }

        return numLines;
        */
    }

    /**
      | Returns an array containing the tokens
      | in a given string.
      | 
      | This will tokenise the given string
      | using whitespace characters as the
      | token delimiters, and return the parsed
      | tokens as an array. @see addTokens
      |
      */
    pub fn from_tokens(&mut self, 
        string_to_tokenise:      &str,
        preserve_quoted_strings: bool) -> StringArray {
        
        todo!();
        /*
            StringArray s;
        s.addTokens (stringToTokenise, preserveQuotedStrings);
        return s;
        */
    }

    /**
      | Returns an array containing the tokens
      | in a given string.
      | 
      | This will tokenise the given string
      | using the breakCharacters string to
      | define the token delimiters, and will
      | return the parsed tokens as an array.
      | 
      | -----------
      | @param stringToTokenise
      | 
      | the string to tokenise
      | ----------
      | @param breakCharacters
      | 
      | a string of characters, any of which
      | will be considered to be a token delimiter.
      | ----------
      | @param quoteCharacters
      | 
      | if this string isn't empty, it defines
      | a set of characters which are treated
      | as quotes. Any text occurring between
      | quotes is not broken up into tokens.
      | @see addTokens
      |
      */
    pub fn from_tokens_with_break_chars(&mut self, 
        string_to_tokenise: &str,
        break_characters:   &str,
        quote_characters:   &str) -> StringArray {
        
        todo!();
        /*
            StringArray s;
        s.addTokens (stringToTokenise, breakCharacters, quoteCharacters);
        return s;
        */
    }

    /**
      | Returns an array containing the lines
      | in a given string.
      | 
      | This breaks a string down into lines
      | separated by \\n or \\r\\n, and returns
      | an array containing these lines. Line-break
      | characters are omitted from the strings
      | that are added to the array.
      |
      */
    pub fn from_lines(&mut self, string_to_break_up: &str) -> StringArray {
        
        todo!();
        /*
            StringArray s;
        s.addLines (stringToBreakUp);
        return s;
        */
    }

    /**
      | Removes any duplicated elements from
      | the array.
      | 
      | If any string appears in the array more
      | than once, only the first occurrence
      | of it will be retained.
      | 
      | -----------
      | @param ignoreCase
      | 
      | whether to use a case-insensitive comparison
      |
      */
    pub fn remove_duplicates(&mut self, ignore_case: bool)  {
        
        todo!();
        /*
            for (int i = 0; i < size() - 1; ++i)
        {
            auto s = strings.getReference(i);

            for (int nextIndex = i + 1;;)
            {
                nextIndex = indexOf (s, ignoreCase, nextIndex);

                if (nextIndex < 0)
                    break;

                strings.remove (nextIndex);
            }
        }
        */
    }

    /**
      | Adds numbers to the strings in the array,
      | to make each string unique.
      | 
      | This will add numbers to the ends of groups
      | of similar strings. e.g. if there are
      | two "moose" strings, they will become
      | "moose (1)" and "moose (2)"
      | 
      | -----------
      | @param ignoreCaseWhenComparing
      | 
      | whether the comparison used is case-insensitive
      | ----------
      | @param appendNumberToFirstInstance
      | 
      | whether the first of a group of similar
      | strings also has a number appended to
      | it.
      | ----------
      | @param preNumberString
      | 
      | when adding a number, this string is
      | added before the number. If you pass
      | nullptr, a default string will be used,
      | which adds brackets around the number.
      | ----------
      | @param postNumberString
      | 
      | this string is appended after any numbers
      | that are added. If you pass nullptr,
      | a default string will be used, which
      | adds brackets around the number.
      |
      */
    pub fn append_numbers_to_duplicates(&mut self, 
        ignore_case:                     bool,
        append_number_to_first_instance: bool,
        pre_number_string:               CharPointer_UTF8,
        post_number_string:              CharPointer_UTF8)  {
        
        todo!();
        /*
            if (preNumberString.getAddress() == nullptr)
            preNumberString = CharPointer_UTF8 (" (");

        if (postNumberString.getAddress() == nullptr)
            postNumberString = CharPointer_UTF8 (")");

        for (int i = 0; i < size() - 1; ++i)
        {
            auto& s = strings.getReference(i);
            auto nextIndex = indexOf (s, ignoreCase, i + 1);

            if (nextIndex >= 0)
            {
                auto original = s;
                int number = 0;

                if (appendNumberToFirstInstance)
                    s = original + AloeString (preNumberString) + AloeString (++number) + AloeString (postNumberString);
                else
                    ++number;

                while (nextIndex >= 0)
                {
                    set (nextIndex, (*this)[nextIndex] + AloeString (preNumberString) + AloeString (++number) + AloeString (postNumberString));
                    nextIndex = indexOf (original, ignoreCase, nextIndex + 1);
                }
            }
        }
        */
    }

    /**
      | Increases the array's internal storage
      | to hold a minimum number of elements.
      | 
      | Calling this before adding a large known
      | number of elements means that the array
      | won't have to keep dynamically resizing
      | itself as the elements are added, and
      | it'll therefore be more efficient.
      |
      */
    pub fn ensure_storage_allocated(&mut self, min_num_elements: i32)  {
        
        todo!();
        /*
            strings.ensureStorageAllocated (minNumElements);
        */
    }

    /**
      | Reduces the amount of storage being
      | used by the array.
      | 
      | Arrays typically allocate slightly
      | more storage than they need, and after
      | removing elements, they may have quite
      | a lot of unused space allocated.
      | 
      | This method will reduce the amount of
      | allocated storage to a minimum.
      |
      */
    pub fn minimise_storage_overheads(&mut self)  {
        
        todo!();
        /*
            strings.minimiseStorageOverheads();
        */
    }
}
