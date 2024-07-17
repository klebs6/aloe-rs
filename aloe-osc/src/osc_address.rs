crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCAddress.h]

/**
  | An OSC address.
  | 
  | This address always starts with a forward
  | slash and has a format similar to an Url,
  | with several address parts separated
  | by slashes.
  | 
  | Only a subset of ASCII characters are
  | allowed in OSC addresses; see
  | 
  | OpenSoundControl 1.0 specification
  | for details.
  | 
  | OSC addresses can be used to register
  | ListenerWithOSCAddress objects to
  | an
  | 
  | OSCReceiver if you wish them to only
  | listen to certain messages with matching
  | OSC address patterns.
  | 
  | @see OSCReceiver, OSCAddressPattern,
  | OSCMessage
  | 
  | @tags{OSC}
  |
  */
pub struct OSCAddress {
    osc_symbols: StringArray,
    as_string:   String,
}

impl PartialEq<OSCAddress> for OSCAddress {
    
    /**
      | Compares two OSCAddress objects.
      | 
      | 
      | -----------
      | @return
      | 
      | true if they contain the same address,
      | false otherwise.
      |
      */
    #[inline] fn eq(&self, other: &OSCAddress) -> bool {
        todo!();
        /*
            return asString == other.asString;
        */
    }
}

impl Eq for OSCAddress {}

impl From<&str> for OSCAddress {
    
    /**
      | Constructs a new OSCAddress from a String.
      | @throw OSCFormatError if the string
      | is not a valid OSC address.
      |
      */
    fn from(address: &str) -> Self {
    
        todo!();
        /*


            : oscSymbols (OSCAddressTokeniser<OSCAddress>::tokenise (address)),
          asString (address.trimCharactersAtEnd ("/"))
        */
    }
}

impl From<*const u8> for OSCAddress {
    
    /**
      | Constructs a new OSCAddress from a C
      | string. @throw OSCFormatError of the
      | string is not a valid OSC address.
      |
      */
    fn from(address: *const u8) -> Self {
    
        todo!();
        /*


            : oscSymbols (OSCAddressTokeniser<OSCAddress>::tokenise (String (address))),
          asString (String (address).trimCharactersAtEnd ("/"))
        */
    }
}

impl OSCAddress {
    
    /**
      | Converts the OSCAddress to a String.
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
      | OSC address.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return asString;
        */
    }
}
