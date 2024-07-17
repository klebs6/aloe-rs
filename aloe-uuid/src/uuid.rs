crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/misc/aloe_Uuid.h]

/**
  | A universally unique 128-bit identifier.
  | 
  | This class generates very random unique
  | numbers. It's vanishingly unlikely
  | that two identical UUIDs would ever
  | be created by chance. The values are
  | formatted to meet the RFC 4122 version
  | 4 standard.
  | 
  | The class includes methods for saving
  | the ID as a string or as raw binary data.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
#[derive(Clone)]
pub struct Uuid {
    uuid: [u8; 16],
}

impl Default for Uuid {
    
    /**
      | Creates a new unique ID, compliant with
      | RFC 4122 version 4.
      |
      */
    fn default() -> Self {

        todo!();
        /*


            Random r;

        for (size_t i = 0; i < sizeof (uuid); ++i)
            uuid[i] = (uint8) (r.nextInt (256));

        // To make it RFC 4122 compliant, need to force a few bits...
        uuid[6] = (uuid[6] & 0x0f) | 0x40;
        uuid[8] = (uuid[8] & 0x3f) | 0x80;
        */
    }
}

impl PartialEq<Uuid> for Uuid {
    
    #[inline] fn eq(&self, other: &Uuid) -> bool {
        todo!();
        /*
            return memcmp (uuid, other.uuid, sizeof (uuid)) == 0;
        */
    }
}

impl Eq for Uuid {}

impl Ord for Uuid {
    
    #[inline] fn cmp(&self, other: &Uuid) -> std::cmp::Ordering {
        todo!();
        /*
            return compare (other) < 0;
        */
    }
}

impl PartialOrd<Uuid> for Uuid {

    #[inline] fn partial_cmp(&self, other: &Uuid) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/misc/aloe_Uuid.cpp]

impl From<&String> for Uuid {

    /**
      | Creates an ID from an encoded string
      | version. @see toString
      |
      */
    fn from(x: &String) -> Self {
        todo!();
        /*
            operator= (uuidString);
        */
    }
}

impl From<*const u8> for Uuid {

    /**
      | Creates a UUID from a 16-byte array.
      | @see getRawData
      |
      */
    fn from(x: *const u8) -> Self {
        todo!();
        /*
            operator= (rawData);
        */
    }
}

impl From<&Uuid> for Uuid {

    fn from(x: &Uuid) -> Self {

        todo!();
        /*
            memcpy (uuid, other.uuid, sizeof (uuid));
        */
    }
}

impl Uuid {
    
    /**
      | Returns a null Uuid object.
      |
      */
    pub fn null_uuid() -> Uuid {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns a pointer to the internal binary
      | representation of the ID.
      | 
      | This is an array of 16 bytes. To reconstruct
      | a Uuid from its data, use the constructor
      | or operator= method that takes an array
      | of uint8s.
      |
      */
    pub fn get_raw_data(&self) -> *const u8 {
        
        todo!();
        /*
            return uuid;
        */
    }

    /**
      | Copies another UUID.
      |
      */
    pub fn assign_from_other(&mut self, other: &Uuid) -> &mut Uuid {
        
        todo!();
        /*
            memcpy (uuid, other.uuid, sizeof (uuid));
        return *this;
        */
    }
    
    pub fn compare(&self, other: Uuid) -> i32 {
        
        todo!();
        /*
            for (size_t i = 0; i < sizeof (uuid); ++i)
            if (int diff = uuid[i] - (int) other.uuid[i])
                return diff > 0 ? 1 : -1;

        return 0;
        */
    }
    
    pub fn null(&mut self) -> Uuid {
        
        todo!();
        /*
            return Uuid ((const uint8*) nullptr);
        */
    }
    
    /**
      | Returns true if the ID is zero.
      |
      */
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            for (auto i : uuid)
            if (i != 0)
                return false;

        return true;
        */
    }
    
    pub fn get_hex_region(&self, 
        start:  i32,
        length: i32) -> String {
        
        todo!();
        /*
            return String::toHexString (uuid + start, length, 0);
        */
    }
    
    /**
      | Returns a stringified version of this
      | UUID.
      | 
      | A Uuid object can later be reconstructed
      | from this string using operator= or
      | the constructor that takes a string
      | parameter.
      | 
      | -----------
      | @return
      | 
      | a 32 character hex string.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return getHexRegion (0, 16);
        */
    }
    
    /**
      | Returns a stringified version of this
      | UUID, separating it into sections with
      | dashes.
      | 
      | -----------
      | @return
      | 
      | a string in the format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
      |
      */
    pub fn to_dashed_string(&self) -> String {
        
        todo!();
        /*
            return getHexRegion (0, 4)
                + "-" + getHexRegion (4, 2)
                + "-" + getHexRegion (6, 2)
                + "-" + getHexRegion (8, 2)
                + "-" + getHexRegion (10, 6);
        */
    }
    
    /**
      | Copies from a stringified UUID.
      | 
      | The string passed in should be one that
      | was created with the toString() method.
      |
      */
    pub fn assign_from_string(&mut self, uuid_string: &String) -> &mut Uuid {
        
        todo!();
        /*
            MemoryBlock mb;
        mb.loadFromHexString (uuidString);
        mb.ensureSize (sizeof (uuid), true);
        mb.copyTo (uuid, 0, sizeof (uuid));
        return *this;
        */
    }

    /**
      | Sets this UUID from 16-bytes of raw data.
      |
      */
    pub fn assign_from_raw(&mut self, raw_data: *const u8) -> &mut Uuid {
        
        todo!();
        /*
            if (rawData != nullptr)
            memcpy (uuid, rawData, sizeof (uuid));
        else
            zeromem (uuid, sizeof (uuid));

        return *this;
        */
    }
    
    /**
      | Returns the time-low section of the
      | UUID.
      |
      */
    pub fn get_time_low(&self) -> u32 {
        
        todo!();
        /*
            return ByteOrder::bigEndianInt (uuid);
        */
    }
    
    /**
      | Returns the time-mid section of the
      | UUID.
      |
      */
    pub fn get_time_mid(&self) -> u16 {
        
        todo!();
        /*
            return ByteOrder::bigEndianShort (uuid + 4);
        */
    }
    
    /**
      | Returns the time-high-and-version
      | section of the UUID.
      |
      */
    pub fn get_time_high_and_version(&self) -> u16 {
        
        todo!();
        /*
            return ByteOrder::bigEndianShort (uuid + 6);
        */
    }
    
    /**
      | Returns the clock-seq-and-reserved
      | section of the UUID.
      |
      */
    pub fn get_clock_seq_and_reserved(&self) -> u8 {
        
        todo!();
        /*
            return uuid[8];
        */
    }
    
    /**
      | Returns the clock-seq-low section
      | of the UUID.
      |
      */
    pub fn get_clock_seq_low(&self) -> u8 {
        
        todo!();
        /*
            return uuid[9];
        */
    }
    
    /**
      | Returns the node section of the UUID.
      |
      */
    pub fn get_node(&self) -> u64 {
        
        todo!();
        /*
            return (((uint64) ByteOrder::bigEndianShort (uuid + 10)) << 32) + ByteOrder::bigEndianInt (uuid + 12);
        */
    }
    
    /**
      | Returns a hash of the UUID.
      |
      */
    pub fn hash(&self) -> u64 {
        
        todo!();
        /*
            uint64 result = 0;

        for (auto n : uuid)
            result = ((uint64) 101) * result + n;

        return result;
        */
    }
}
