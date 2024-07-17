crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/hashing/aloe_SHA256.h]

/**
  | SHA-256 secure hash generator.
  | 
  | Create one of these objects from a block
  | of source data or a stream, and it calculates
  | the SHA-256 hash of that data.
  | 
  | You can retrieve the hash as a raw 32-byte
  | block, or as a 64-digit hex string. @see
  | MD5
  | 
  | @tags{Cryptography}
  |
  */
#[leak_detector]
pub struct SHA256 {

    result: [u8; 32],
}

impl Default for SHA256 {
    
    /**
      | Creates an empty SHA256 object.
      | 
      | The default constructor just creates
      | a hash filled with zeros. (This is not
      | equal to the hash of an empty block of
      | data).
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl PartialEq<SHA256> for SHA256 {
    
    #[inline] fn eq(&self, other: &SHA256) -> bool {
        todo!();
        /*
            return memcmp (result, other.result, sizeof (result)) == 0;
        */
    }
}

impl Eq for SHA256 {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/hashing/aloe_SHA256.cpp]
impl SHA256 {

    /**
      | Creates a hash from a block of raw data.
      |
      */
    pub fn new_from_memory_block(data: &MemoryBlock) -> Self {
    
        todo!();
        /*
            process (data.getData(), data.getSize());
        */
    }

    /**
      | Creates a hash from a block of raw data.
      |
      */
    pub fn new_from_raw(
        data:      *const c_void,
        num_bytes: usize) -> Self {
    
        todo!();
        /*
            process (data, numBytes);
        */
    }

    /**
      | Creates a hash from the contents of a
      | stream.
      | 
      | This will read from the stream until
      | the stream is exhausted, or until maxBytesToRead
      | bytes have been read. If maxBytesToRead
      | is negative, the entire stream will
      | be read.
      |
      */
    pub fn new_from_stream(
        input:             &mut dyn Read,
        num_bytes_to_read: Option<i64>

    ) -> Self {

        let num_bytes_to_read: i64 = num_bytes_to_read.unwrap_or(-1);
    
        todo!();
        /*


            SHA256Processor processor;
        processor.processStream (input, numBytesToRead, result);
        */
    }

    /**
      | Reads a file and generates the hash of
      | its contents.
      | 
      | If the file can't be opened, the hash
      | will be left uninitialised (i.e. full
      | of zeros).
      |
      */
    pub fn new_from_file(file: &File) -> Self {
    
        todo!();
        /*


            FileInputStream fin (file);

        if (fin.getStatus().wasOk())
        {
            SHA256Processor processor;
            processor.processStream (fin, -1, result);
        }
        else
        {
            zerostruct (result);
        }
        */
    }

    /**
      | Creates a checksum from a UTF-8 buffer.
      | E.g.
      | 
      | @code SHA256 checksum (myString.toUTF8());
      | @endcode
      |
      */
    pub fn new_from_chars(utf8: CharPointer_UTF8) -> Self {
    
        todo!();
        /*


            jassert (utf8.getAddress() != nullptr);
        process (utf8.getAddress(), utf8.sizeInBytes() - 1);
        */
    }

    pub fn process(&mut self, 
        data:      *const c_void,
        num_bytes: usize)  {
        
        todo!();
        /*
            MemoryInputStream m (data, numBytes, false);
        SHA256Processor processor;
        processor.processStream (m, -1, result);
        */
    }

    /**
      | Returns the hash as a 32-byte block of
      | data.
      |
      */
    pub fn get_raw_data(&self) -> MemoryBlock {
        
        todo!();
        /*
            return MemoryBlock (result, sizeof (result));
        */
    }

    /**
      | Returns the checksum as a 64-digit hex
      | string.
      |
      */
    pub fn to_hex_string(&self) -> String {
        
        todo!();
        /*
            return String::toHexString (result, sizeof (result), 0);
        */
    }
}
