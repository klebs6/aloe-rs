crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCArgument.h]

pub struct OSCArgumentU {
    int_value:   i32,
    float_value: f32,
}

/**
  | An OSC argument.
  | 
  | An OSC argument is a value of one of the
  | following types: int32, float32, string,
  | or blob (raw binary data).
  | 
  | OSCMessage objects are essentially
  | arrays of OSCArgument objects.
  | 
  | @tags{OSC}
  |
  */
pub struct OSCArgument {
    ty:           OSCType,
    u:            OSCArgumentU,
    string_value: String,
    blob:         MemoryBlock,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCArgument.cpp]
impl From<i32> for OSCArgument {

    /**
      | Constructs an OSCArgument with type
      | int32 and a given value.
      |
      */
    fn from(v: i32) -> Self {
    
        todo!();
        /*
        : ty(OSCTypes::int32),
        : int_value(v),

        
        */
    }
}

impl From<f32> for OSCArgument {
    
    /**
      | Constructs an OSCArgument with type
      | float32 and a given value.
      |
      */
    fn from(v: f32) -> Self {
    
        todo!();
        /*
        : ty(OSCTypes::float32),
        : float_value(v),

        
        */
    }
}

impl From<&String> for OSCArgument {
    
    /**
      | Constructs an OSCArgument with type
      | string and a given value
      |
      */
    fn from(s: &String) -> Self {
    
        todo!();
        /*
        : ty(OSCTypes::string),
        : string_value(s),

        
        */
    }
}

impl From<MemoryBlock> for OSCArgument {
    
    /**
      | Constructs an OSCArgument with type
      | blob and copies dataSize bytes from
      | the memory pointed to by data into the
      | blob.
      | 
      | The data owned by the blob will be released
      | when the OSCArgument object gets destructed.
      |
      */
    fn from(b: MemoryBlock) -> Self {
    
        todo!();
        /*
        : ty(OSCTypes::blob),
        : blob(std::move (b)),

        
        */
    }
}

impl From<OSCColour> for OSCArgument {
    
    /**
      | Constructs an OSCArgument with type
      | colour and a given colour value
      |
      */
    fn from(c: OSCColour) -> Self {
    
        todo!();
        /*


            : type (OSCTypes::colour),  intValue ((int32) c.toInt32())
        */
    }
}

impl OSCArgument {

    /**
      | Returns the type of the OSCArgument
      | as an OSCType.
      | 
      | OSCType is a char type, and its value
      | will be the OSC type tag of the type.
      |
      */
    pub fn get_type(&self) -> OSCType {
        
        todo!();
        /*
            return type;
        */
    }

    /**
      | Returns whether the type of the OSCArgument
      | is int32.
      |
      */
    pub fn is_int32(&self) -> bool {
        
        todo!();
        /*
            return type == OSCTypes::int32;
        */
    }

    /**
      | Returns whether the type of the OSCArgument
      | is float.
      |
      */
    pub fn is_float32(&self) -> bool {
        
        todo!();
        /*
            return type == OSCTypes::float32;
        */
    }

    /**
      | Returns whether the type of the OSCArgument
      | is string.
      |
      */
    pub fn is_string(&self) -> bool {
        
        todo!();
        /*
            return type == OSCTypes::string;
        */
    }

    /**
      | Returns whether the type of the OSCArgument
      | is blob.
      |
      */
    pub fn is_blob(&self) -> bool {
        
        todo!();
        /*
            return type == OSCTypes::blob;
        */
    }

    /**
      | Returns whether the type of the OSCArgument
      | is colour.
      |
      */
    pub fn is_colour(&self) -> bool {
        
        todo!();
        /*
            return type == OSCTypes::colour;
        */
    }

    
    /**
      | Returns the value of the OSCArgument
      | as a string.
      | 
      | If the type of the OSCArgument is not
      | string, the behaviour is undefined.
      |
      */
    pub fn get_string(&self) -> String {
        
        todo!();
        /*
            if (isString())
            return stringValue;

        jassertfalse; // you must check the type of an argument before attempting to get its value!
        return {};
        */
    }
    
    /**
      | Returns the value of the OSCArgument
      | as an int32.
      | 
      | If the type of the OSCArgument is not
      | int32, the behaviour is undefined.
      |
      */
    pub fn get_int32(&self) -> i32 {
        
        todo!();
        /*
            if (isInt32())
            return intValue;

        jassertfalse; // you must check the type of an argument before attempting to get its value!
        return 0;
        */
    }
    
    /**
      | Returns the value of the OSCArgument
      | as a float32.
      | 
      | If the type of the OSCArgument is not
      | float32, the behaviour is undefined.
      |
      */
    pub fn get_float32(&self) -> f32 {
        
        todo!();
        /*
            if (isFloat32())
            return floatValue;

        jassertfalse; // you must check the type of an argument before attempting to get its value!
        return 0.0f;
        */
    }
    
    /**
      | Returns the binary data contained in
      | the blob and owned by the OSCArgument,
      | as a reference to a Aloe MemoryBlock
      | object.
      | 
      | If the type of the OSCArgument is not
      | blob, the behaviour is undefined.
      |
      */
    pub fn get_blob(&self) -> &MemoryBlock {
        
        todo!();
        /*
            // you must check the type of an argument before attempting to get its value!
        jassert (isBlob());

        return blob;
        */
    }
    
    /**
      | Returns the value of the OSCArgument
      | as an OSCColour.
      | 
      | If the type of the OSCArgument is not
      | a colour, the behaviour is undefined.
      |
      */
    pub fn get_colour(&self) -> OSCColour {
        
        todo!();
        /*
            if (isColour())
            return OSCColour::fromInt32 ((uint32) intValue);

        jassertfalse; // you must check the type of an argument before attempting to get its value!
        return { 0, 0, 0, 0 };
        */
    }
}
