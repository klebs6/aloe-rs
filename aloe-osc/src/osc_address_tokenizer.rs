crate::ix!();

pub struct OSCAddressTokeniser<OSCAddressType> {
    _p0: PhantomData<OSCAddressType>,
}

impl<OSCAddressType> OSCAddressTokeniser<OSCAddressType> {
    
    pub fn is_printable_ascii_char(c: wchar_t) -> bool {
        
        todo!();
        /*
            return c >= ' ' && c <= '~';
        */
    }
    
    pub fn is_disallowed_char(c: wchar_t) -> bool {
        
        todo!();
        /*
           type Traits = OSCAddressTokeniserTraits<OSCAddressType>;

           return CharPointer_ASCII (Traits::getDisallowedChars()).indexOf (c, false) >= 0;
        */
    }
    
    pub fn contains_only_allowed_printable_ascii_chars(string: &String) -> bool {
        
        todo!();
        /*
            for (auto charPtr = string.getCharPointer(); ! charPtr.isEmpty();)
                {
                    auto c = charPtr.getAndAdvance();

                    if (! isPrintableASCIIChar (c) || isDisallowedChar (c))
                        return false;
                }

                return true;
        */
    }
    
    pub fn tokenise(address: &String) -> StringArray {
        
        todo!();
        /*
            if (address.isEmpty())
                    throw OSCFormatError ("OSC format error: address string cannot be empty.");

                if (! address.startsWithChar ('/'))
                    throw OSCFormatError ("OSC format error: address string must start with a forward slash.");

                StringArray oscSymbols;
                oscSymbols.addTokens (address, "/", "");
                oscSymbols.removeEmptyStrings (false);

                for (auto& token : oscSymbols)
                    if (! containsOnlyAllowedPrintableASCIIChars (token))
                        throw OSCFormatError ("OSC format error: encountered characters not allowed in address string.");

                return oscSymbols;
        */
    }
}
