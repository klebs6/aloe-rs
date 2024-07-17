crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_Identifier.h]

/**
  | Represents a string identifier, designed
  | for accessing properties by name.
  | 
  | Comparing two Identifier objects is
  | very fast (an O(1) operation), but creating
  | them can be slower than just using a String
  | directly, so the optimal way to use them
  | is to keep some static Identifier objects
  | for the things you use often.
  | 
  | @see NamedValueSet, ValueTree
  | 
  | @tags{Core}
  |
  */
pub struct Identifier {
    name: String,
}

pub mod identifier {

    use super::*;

    /**
      | A null identifier.
      |
      */
    lazy_static!{
        /*
        static Identifier null;
        */
    }
}

impl Default for Identifier {
    
    /**
      | Creates a null identifier.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl PartialEq<Identifier> for Identifier {
    
    /**
      | Compares two identifiers. This is a
      | very fast operation.
      |
      */
    fn eq(&self, other: &Identifier) -> bool {
        todo!();
        /*
            return name.getCharPointer() == other.name.getCharPointer();
        */
    }
}

impl Eq for Identifier {}

impl PartialOrd<str> for Identifier {

    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.name.as_str().partial_cmp(other)
    }
}

impl PartialEq<str> for Identifier {

    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl Into<CharPointerType> for Identifier {
    
    /**
      | Returns this identifier's raw string
      | pointer.
      |
      */
    #[inline] fn into(self) -> CharPointerType {
        todo!();
        /*
            return name.getCharPointer();
        */
    }
}

impl Into<String> for Identifier {

    fn into(self) -> String {
        self.name
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_Identifier.cpp]
impl Identifier {

    pub fn new(x: &str) -> Self {

        Self {
            name: x.to_string()
        }
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.name
    }

    /**
      | Returns this identifier as a string.
      |
      */
    pub fn to_string(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }

    /**
      | Returns this identifier's raw string
      | pointer.
      |
      */
    pub fn get_char_pointer(&self) -> CharPointerType {
        
        todo!();
        /*
            return name.getCharPointer();
        */
    }

    /**
      | Returns true if this Identifier is not
      | null
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return name.isNotEmpty();
        */
    }

    /**
      | Returns true if this Identifier is null
      |
      */
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return name.isEmpty();
        */
    }

    /**
      | Creates a copy of another identifier.
      |
      */
    pub fn assign_from_other(&mut self, other: Identifier) -> &mut Identifier {
        
        todo!();
        /*
            name = std::move (other.name);
        return *this;
        */
    }
    
    /**
      | Creates a copy of another identifier.
      |
      */
    pub fn assign_from_ref(&mut self, other: &Identifier) -> &mut Identifier {
        
        todo!();
        /*
            name = other.name;
        return *this;
        */
    }

    /**
      | Creates a copy of another identifier.
      |
      */
    pub fn new_from_other_ref(other: &Identifier) -> Self {
    
        todo!();
        /*
        : name(other.name),
        */
    }
    
    /**
      | Creates a copy of another identifier.
      |
      */
    pub fn new_from_other(other: Identifier) -> Self {
    
        todo!();
        /*
        : name(std::move (other.name)),
        */
    }

    
    /**
      | Creates an identifier with a specified
      | name.
      | 
      | Because this name may need to be used
      | in contexts such as script variables
      | or XML tags, it must only contain ascii
      | letters and digits, or the underscore
      | character.
      |
      */
    pub fn new_from_string_ref(nm: &str) -> Self {
    
        todo!();
        /*
        : name (StringPool::getGlobalPool().getPooledString (nm))
        // An Identifier cannot be created from an empty string!
        jassert (nm.isNotEmpty());
        */
    }
    
    /**
      | Creates an identifier with a specified
      | name.
      | 
      | Because this name may need to be used
      | in contexts such as script variables
      | or XML tags, it must only contain ascii
      | letters and digits, or the underscore
      | character.
      |
      */
    pub fn new_from_raw_ptr(nm: *const u8) -> Self {
    
        todo!();
        /*
        : name (StringPool::getGlobalPool().getPooledString (nm))

        // An Identifier cannot be created from an empty string!
        jassert (nm != nullptr && nm[0] != 0);
        */
    }
    
    /**
      | Creates an identifier with a specified
      | name.
      | 
      | Because this name may need to be used
      | in contexts such as script variables
      | or XML tags, it must only contain ascii
      | letters and digits, or the underscore
      | character.
      |
      */
    pub fn new_from_char_range(
        start: CharPointerType,
        end:   CharPointerType) -> Self {
    
        todo!();
        /*
        : name (StringPool::getGlobalPool().getPooledString (start, end))

        // An Identifier cannot be created from an empty string!
        jassert (start < end);
        */
    }
    
    /**
      | Checks a given string for characters
      | that might not be valid in an Identifier.
      | 
      | Since Identifiers are used as a script
      | variables and XML attributes, they
      | should only contain alphanumeric characters,
      | underscores, or the '-' and ':' characters.
      |
      */
    pub fn is_valid_identifier(&mut self, possible_identifier: &String) -> bool {
        
        todo!();
        /*
            return possibleIdentifier.isNotEmpty()
                && possibleIdentifier.containsOnly ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-:#@$%");
        */
    }
}

lazy_static!{
    /*
    Identifier Identifier::null;
    */
}
