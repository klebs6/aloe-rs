crate::ix!();

pub fn osc_pattern_matcher_match_<CharPtrType>(
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (pattern == patternEnd)
                return matchTerminator (target, targetEnd);

            auto c = pattern.getAndAdvance();

            switch (c)
            {
                case '?':   return matchAnyChar (pattern, patternEnd, target, targetEnd);
                case '*':   return matchAnyOrNoChars (pattern, patternEnd, target, targetEnd);
                case '{':   return matchInsideStringSet (pattern, patternEnd, target, targetEnd);
                case '[':   return matchInsideCharSet (pattern, patternEnd, target, targetEnd);
                default:    return matchChar (c, pattern, patternEnd, target, targetEnd);
            }
    */
}

pub fn osc_pattern_matcher_match_terminator<CharPtrType>(
    target:     CharPtrType,
    target_end: CharPtrType) -> bool {
    
    todo!();
    /*
        return target == targetEnd;
    */
}

pub fn osc_pattern_matcher_match_char<CharPtrType>(
    c:           wchar_t,
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (target == targetEnd || c != target.getAndAdvance())
                return false;

            return match (pattern, patternEnd, target, targetEnd);
    */
}

pub fn osc_pattern_matcher_match_any_char<CharPtrType>(
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (target == targetEnd)
                return false;

            return match (pattern, patternEnd, ++target, targetEnd);
    */
}

pub fn osc_pattern_matcher_match_any_or_no_chars<CharPtrType>(
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (target == targetEnd)
                return pattern == patternEnd;

            if (match (pattern, patternEnd, target, targetEnd))
                return true;

            return matchAnyOrNoChars (pattern, patternEnd, ++target, targetEnd);
    */
}

pub fn osc_pattern_matcher_match_inside_string_set<CharPtrType>(
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (pattern == patternEnd)
                return false;

            // Note: In case this code is ever moved into the more generic CharPointerFunctions,
            // the next two lines probably will not compile as soon as this class is used with a
            // Char template type parameter that is not the same type as String::Char.
            StringArray set;
            String currentElement;

            while (pattern != patternEnd)
            {
                auto c = pattern.getAndAdvance();

                switch (c)
                {
                    case '}':
                        set.add (currentElement);
                        currentElement.clear();
                        return matchStringSet (set, pattern, patternEnd, target, targetEnd);

                    case ',':
                        set.add (currentElement);
                        currentElement.clear();
                        continue;

                    default:
                        currentElement += c;
                        continue;
                }
            }

            return false;
    */
}

pub fn osc_pattern_matcher_match_string_set<CharPtrType>(
    set:         &StringArray,
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (set.size() == 0)
                return match (pattern, patternEnd, target, targetEnd);

            for (auto& str : set)
                if (str.getCharPointer().compareUpTo (target, str.length()) == 0)
                    if (match (pattern, patternEnd, target + str.length(), targetEnd))
                        return true;

            return false;
    */
}

pub fn osc_pattern_matcher_match_inside_char_set<CharPtrType>(
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (pattern == patternEnd)
                return false;

            Vec<aloe_wchar> set;
            bool setIsNegated = false;

            while (pattern != patternEnd)
            {
                auto c = pattern.getAndAdvance();

                switch (c)
                {
                    case ']':
                        return matchCharSet (set, setIsNegated, pattern, patternEnd, target, targetEnd);

                    case '-':
                        if (! addCharRangeToSet (set, pattern, patternEnd, target, targetEnd))
                            return false;

                        break;

                    case '!':
                        if (set.size() == 0 && setIsNegated == false)
                        {
                            setIsNegated = true;
                            break;
                        }
                        // else = special case: fall through to default and treat '!' as a non-special character.
                        ALOE_FALLTHROUGH

                    default:
                        set.add (c);
                        break;
                }
            }

            return false;
    */
}

pub fn osc_pattern_matcher_match_char_set<CharPtrType>(
    set:            &[wchar_t],
    set_is_negated: bool,
    pattern:        CharPtrType,
    pattern_end:    CharPtrType,
    target:         CharPtrType,
    target_end:     CharPtrType) -> bool {
    
    todo!();
    /*
        if (set.size() == 0)
                return match (pattern, patternEnd, target, targetEnd);

            if (target == targetEnd)
                return false;

            return setIsNegated ? matchCharSetNegated (set, pattern, patternEnd, target, targetEnd)
                                : matchCharSetNotNegated (set, pattern, patternEnd, target, targetEnd);
    */
}

pub fn osc_pattern_matcher_match_char_set_negated<CharPtrType>(
    set:         &[wchar_t],
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        for (auto c : set)
                if (*target == c)
                    return false;

            return match (pattern, patternEnd, target + 1, targetEnd);
    */
}

pub fn osc_pattern_matcher_match_char_set_not_negated<CharPtrType>(
    set:         &[wchar_t],
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        for (auto c : set)
                if (*target == c)
                    if (match (pattern, patternEnd, target + 1, targetEnd))
                        return true;

            return false;
    */
}

pub fn osc_pattern_matcher_add_char_range_to_set<CharPtrType>(
    set:         &mut Vec<wchar_t>,
    pattern:     CharPtrType,
    pattern_end: CharPtrType,
    target:      CharPtrType,
    target_end:  CharPtrType) -> bool {
    
    todo!();
    /*
        if (target == targetEnd)
                return false;

            auto rangeStart = set.getLast();
            auto rangeEnd = pattern.getAndAdvance();

            if (rangeEnd == ']')
            {
                set.add ('-');  // special case: '-' has no special meaning at the end.
                return true;
            }

            if (rangeEnd == ',' || rangeEnd == '{' || rangeEnd == '}' || set.size() == 0)
                return false;

            while (rangeEnd > rangeStart)
                set.add (++rangeStart);

            return true;
    */
}

pub fn match_osc_pattern(pattern: &str, target: &str) -> bool {
    
    todo!();
        /*
            return OSCPatternMatcherImpl<CharPointerType>::match (pattern.getCharPointer(),
                                                                          pattern.getCharPointer().findTerminatingNull(),
                                                                          target.getCharPointer(),
                                                                          target.getCharPointer().findTerminatingNull());
        */
}
