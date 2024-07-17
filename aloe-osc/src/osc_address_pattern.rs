crate::ix!();

/**
  | An OSC address pattern.
  | 
  | Extends an OSC address by additionally
  | allowing the following wildcards:
  | ?, *, [], {}
  | 
  | OSC messages always have an OSC address
  | pattern to specify the destination(s)
  | of the message.
  | 
  | @see OSCMessage, OSCAddress, OSCMessageListener
  | 
  | @tags{OSC}
  |
  */
pub struct OSCAddressPattern {
    osc_symbols:                    StringArray,
    as_string:                      String,
    was_initialised_with_wildcards: bool,
}

impl PartialEq<OSCAddressPattern> for OSCAddressPattern {
    
    /**
      | Compares two OSCAddressPattern objects.
      | 
      | 
      | -----------
      | @return
      | 
      | true if they contain the same address
      | pattern, false otherwise.
      |
      */
    #[inline] fn eq(&self, other: &OSCAddressPattern) -> bool {
        todo!();
        /*
            return asString == other.asString;
        */
    }
}

impl Eq for OSCAddressPattern {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCAddress.cpp]
impl From<&str> for OSCAddressPattern {

    /**
      | Constructs a new OSCAddressPattern
      | from a String. @throw OSCFormatError
      | if the string is not a valid OSC address
      | pattern.
      |
      */
    fn from(address: &str) -> Self {
    
        todo!();
        /*


            : oscSymbols (OSCAddressTokeniser<OSCAddressPattern>::tokenise (address)),
          asString (address.trimCharactersAtEnd ("/")),
          wasInitialisedWithWildcards (asString.containsAnyOf ("*?{}[]"))
        */
    }
}

impl From<*const u8> for OSCAddressPattern {
    
    /**
      | Constructs a new OSCAddressPattern
      | from a C string. @throw OSCFormatError
      | of the string is not a valid OSC address
      | pattern.
      |
      */
    fn from(address: *const u8) -> Self {
    
        todo!();
        /*


            : oscSymbols (OSCAddressTokeniser<OSCAddressPattern>::tokenise (String (address))),
          asString (String (address).trimCharactersAtEnd ("/")),
          wasInitialisedWithWildcards (asString.containsAnyOf ("*?{}[]"))
        */
    }
}

impl OSCAddressPattern {

    /**
      | Checks whether the OSCAddressPattern
      | contains any of the allowed OSC address
      | pattern wildcards: ?, *, [], {}
      | 
      | -----------
      | @return
      | 
      | true if the OSCAddressPattern contains
      | OSC wildcards, false otherwise.
      |
      */
    pub fn contains_wildcards(&self) -> bool {
        
        todo!();
        /*
            return wasInitialisedWithWildcards;
        */
    }
    
    /**
      | Checks if the OSCAddressPattern matches
      | an OSC address with the wildcard rules
      | defined by the OpenSoundControl 1.0
      | specification.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the OSCAddressPattern matches
      | the given OSC address, false otherwise.
      |
      */
    pub fn matches(&self, address: &OSCAddress) -> bool {
        
        todo!();
        /*
            if (! containsWildcards())
            return asString == address.asString;

        if (oscSymbols.size() != address.oscSymbols.size())
            return false;

        for (int i = 0; i < oscSymbols.size(); ++i)
            if (! matchOscPattern (oscSymbols[i], address.oscSymbols[i]))
                return false;

        return true;
        */
    }
    
    /**
      | Converts the OSCAddressPattern to
      | a String.
      | 
      | -----------
      | @note
      | 
      | Trailing slashes are always removed
      | automatically.
      | 
      | -----------
      | @return
      | 
      | a String object that represents the
      | OSC address pattern.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return asString;
        */
    }
}
