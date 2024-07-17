crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCMessage.h]

/**
  | An OSC Message.
  | 
  | An OSCMessage consists of an OSCAddressPattern
  | and zero or more OSCArguments.
  | 
  | OSC messages are the elementary objects
  | that are used to exchange any data via
  | OSC. An OSCSender can send OSCMessage
  | objects to an OSCReceiver.
  | 
  | @tags{OSC}
  |
  */
pub struct OSCMessage {
    address_pattern: OSCAddressPattern,
    arguments:       Vec<OSCArgument>,
}

impl IndexMut<i32> for OSCMessage {
    
    /**
      | Returns a reference to the OSCArgument
      | at index i in the OSCMessage object.
      | 
      | This method does not check the range
      | and results in undefined behaviour
      | in case i < 0 or i >= size().
      |
      */
    #[inline] fn index_mut(&mut self, i: i32) -> &mut Self::Output {
        todo!();
        /*
            return arguments.getReference (i);
        */
    }
}

impl Index<i32> for OSCMessage {
    type Output = OSCArgument;
    
    #[inline] fn index(&self, i: i32) -> &Self::Output {
        todo!();
        /*
            return arguments.getReference (i);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCMessage.cpp]
impl From<&OSCAddressPattern> for OSCMessage {

    /**
      | Constructs an OSCMessage object with
      | the given address pattern and no arguments.
      | 
      | -----------
      | @param ap
      | 
      | the address pattern of the message.
      | This must be a valid OSC address (starting
      | with a forward slash) and may contain
      | 
      | OSC wildcard expressions. You can pass
      | in a string literal or a aloe String (they
      | will be converted to an OSCAddressPattern
      | automatically).
      |
      */
    fn from(ap: &OSCAddressPattern) -> Self {
    
        todo!();
        /*
        : address_pattern(ap),
        */
    }
}
    
impl OSCMessage {
    
    /**
      | Constructs an OSCMessage object with
      | the given address pattern and list of
      | arguments.
      | 
      | -----------
      | @param ap
      | 
      | the address pattern of the message.
      | This must be a valid OSC address (starting
      | with a forward slash) and may contain
      | 
      | OSC wildcard expressions. You can pass
      | in a string literal or a aloe String (they
      | will be converted to an OSCAddressPattern
      | automatically).
      | ----------
      | @param arg1
      | 
      | the first argument of the message.
      | ----------
      | @param args
      | 
      | an optional list of further arguments
      | to add to the message.
      |
      */
    pub fn new_with_address_pattern_and_list_of_arguments<Arg1, Args>(
        ap:   &OSCAddressPattern,
        arg1: Arg1,
        args: Args) -> Self {
    
        todo!();
        /*
        : address_pattern(ap),

            addArguments (std::forward<Arg1> (arg1), std::forward<Args> (args)...);
        */
    }
    
    /**
      | Sets the address pattern of the OSCMessage.
      | 
      | -----------
      | @param ap
      | 
      | the address pattern of the message.
      | This must be a valid OSC address (starting
      | with a forward slash) and may contain
      | 
      | OSC wildcard expressions. You can pass
      | in a string literal or a aloe String (they
      | will be converted to an OSCAddressPattern
      | automatically).
      |
      */
    pub fn set_address_pattern(&mut self, ap: &OSCAddressPattern)  {
        
        todo!();
        /*
            addressPattern = ap;
        */
    }
    
    /**
      | Returns the address pattern of the OSCMessage.
      |
      */
    pub fn get_address_pattern(&self) -> OSCAddressPattern {
        
        todo!();
        /*
            return addressPattern;
        */
    }

    /**
      | Returns the number of OSCArgument objects
      | that belong to this OSCMessage.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return arguments.size();
        */
    }

    /**
      | Returns true if the OSCMessage contains
      | no OSCArgument objects; false otherwise.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return arguments.isEmpty();
        */
    }

    /**
      | Returns a pointer to the first OSCArgument
      | in the OSCMessage object.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    pub fn begin_mut(&mut self) -> *mut OSCArgument {
        
        todo!();
        /*
            return arguments.begin();
        */
    }

    /**
      | Returns a pointer to the first OSCArgument
      | in the OSCMessage object.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    pub fn begin(&self) -> *const OSCArgument {
        
        todo!();
        /*
            return arguments.begin();
        */
    }

    /**
      | Returns a pointer to the last OSCArgument
      | in the OSCMessage object.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    pub fn end_mut(&mut self) -> *mut OSCArgument {
        
        todo!();
        /*
            return arguments.end();
        */
    }

    /**
      | Returns a pointer to the last OSCArgument
      | in the OSCMessage object.
      | 
      | This method is provided for compatibility
      | with standard C++ iteration mechanisms.
      |
      */
    pub fn end(&self) -> *const OSCArgument {
        
        todo!();
        /*
            return arguments.end();
        */
    }

    pub fn add_arguments_with_args<Arg1, Args>(
        &mut self, 
        arg1: Arg1,
        args: Args

    ) {
    
        todo!();
        /*
            addArgument (arg1);
            addArguments (std::forward<Args> (args)...);
        */
    }
    
    pub fn add_arguments(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Removes all arguments from the OSCMessage.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            arguments.clear();
        */
    }
    
    /**
      | Creates a new OSCArgument of type int32
      | with the given value, and adds it to the
      | OSCMessage object.
      |
      */
    pub fn add_int32(&mut self, value: i32)  {
        
        todo!();
        /*
            arguments.add (OSCArgument (value));
        */
    }
    
    /**
      | Creates a new OSCArgument of type float32
      | with the given value, and adds it to the
      | OSCMessage object.
      |
      */
    pub fn add_float32(&mut self, value: f32)  {
        
        todo!();
        /*
            arguments.add (OSCArgument (value));
        */
    }
    
    /**
      | Creates a new OSCArgument of type string
      | with the given value, and adds it to the
      | OSCMessage object.
      |
      */
    pub fn add_string(&mut self, value: &String)  {
        
        todo!();
        /*
            arguments.add (OSCArgument (value));
        */
    }
    
    /**
      | Creates a new OSCArgument of type blob
      | with binary data content copied from
      | the given MemoryBlock.
      | 
      | -----------
      | @note
      | 
      | If the argument passed is an lvalue,
      | this may copy the binary data.
      |
      */
    pub fn add_blob(&mut self, blob: MemoryBlock)  {
        
        todo!();
        /*
            arguments.add (OSCArgument (std::move (blob)));
        */
    }
    
    /**
      | Creates a new OSCArgument of type colour
      | with the given value, and adds it to the
      | OSCMessage object.
      |
      */
    pub fn add_colour(&mut self, colour: OSCColour)  {
        
        todo!();
        /*
            arguments.add (OSCArgument (colour));
        */
    }
    
    /**
      | Adds the OSCArgument argument to the
      | OSCMessage object.
      | 
      | -----------
      | @note
      | 
      | This method will result in a copy of the
      | OSCArgument object if it is passed as
      | an lvalue. If the OSCArgument is of type
      | blob, this will also copy the underlying
      | binary data. In general, you should
      | use addInt32, addFloat32, etc. instead.
      |
      */
    pub fn add_argument(&mut self, arg: OSCArgument)  {
        
        todo!();
        /*
            arguments.add (arg);
        */
    }
}
