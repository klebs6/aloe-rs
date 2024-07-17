crate::ix!();

/**
  | Allows a block of data to be accessed
  | as a stream of OSC data.
  | 
  | The memory is shared and will be neither
  | copied nor owned by the OSCInputStream.
  | 
  | This class is implementing the Open
  | Sound Control 1.0 Specification for
  | interpreting the data.
  | 
  | -----------
  | @note
  | 
  | Some older implementations of OSC may
  | omit the OSC Type Tag string in OSC messages.
  | This class will treat such OSC messages
  | as format errors.
  |
  */
pub struct OSCInputStream {
    input: MemoryInputStream,
}

impl OSCInputStream {

    /**
      | Creates an OSCInputStream.
      | 
      | -----------
      | @param sourceData
      | 
      | the block of data to use as the stream's
      | source
      | ----------
      | @param sourceDataSize
      | 
      | the number of bytes in the source data
      | block
      |
      */
    pub fn new(
        source_data:      *const c_void,
        source_data_size: usize) -> Self {
    
        todo!();
        /*
        : input(sourceData, sourceDataSize, false),

        
        */
    }

        
    /**
      | Returns a pointer to the source data
      | block from which this stream is reading.
      |
      */
    pub fn get_data(&self)  {
        
        todo!();
        /*
            return input.getData();
        */
    }

    /**
      | Returns the number of bytes of source
      | data in the block from which this stream
      | is reading.
      |
      */
    pub fn get_data_size(&self) -> usize {
        
        todo!();
        /*
            return input.getDataSize();
        */
    }

    /**
      | Returns the current position of the
      | stream.
      |
      */
    pub fn get_position(&mut self) -> u64 {
        
        todo!();
        /*
            return (uint64) input.getPosition();
        */
    }

    /**
      | Attempts to set the current position
      | of the stream. Returns true if this was
      | successful.
      |
      */
    pub fn set_position(&mut self, pos: i64) -> bool {
        
        todo!();
        /*
            return input.setPosition (pos);
        */
    }

    /**
      | Returns the total amount of data in bytes
      | accessible by this stream.
      |
      */
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return input.getTotalLength();
        */
    }

    /**
      | Returns true if the stream has no more
      | data to read.
      |
      */
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return input.isExhausted();
        */
    }
    
    pub fn read_int32(&mut self) -> i32 {
        
        todo!();
        /*
            checkBytesAvailable (4, "OSC input stream exhausted while reading int32");
                return input.readIntBigEndian();
        */
    }
    
    pub fn read_uint64(&mut self) -> u64 {
        
        todo!();
        /*
            checkBytesAvailable (8, "OSC input stream exhausted while reading uint64");
                return (uint64) input.readInt64BigEndian();
        */
    }
    
    pub fn read_float32(&mut self) -> f32 {
        
        todo!();
        /*
            checkBytesAvailable (4, "OSC input stream exhausted while reading float");
                return input.readFloatBigEndian();
        */
    }
    
    pub fn read_string(&mut self) -> String {
        
        todo!();
        /*
            checkBytesAvailable (4, "OSC input stream exhausted while reading string");

                auto posBegin = (size_t) getPosition();
                auto s = input.readString();
                auto posEnd = (size_t) getPosition();

                if (static_cast<const char*> (getData()) [posEnd - 1] != '\0')
                    throw OSCFormatError ("OSC input stream exhausted before finding null terminator of string");

                size_t bytesRead = posEnd - posBegin;
                readPaddingZeros (bytesRead);

                return s;
        */
    }
    
    pub fn read_blob(&mut self) -> MemoryBlock {
        
        todo!();
        /*
            checkBytesAvailable (4, "OSC input stream exhausted while reading blob");

                auto blobDataSize = input.readIntBigEndian();
                checkBytesAvailable ((blobDataSize + 3) % 4, "OSC input stream exhausted before reaching end of blob");

                MemoryBlock blob;
                auto bytesRead = input.readIntoMemoryBlock (blob, (ssize_t) blobDataSize);
                readPaddingZeros (bytesRead);

                return blob;
        */
    }
    
    pub fn read_colour(&mut self) -> OSCColour {
        
        todo!();
        /*
            checkBytesAvailable (4, "OSC input stream exhausted while reading colour");
                return OSCColour::fromInt32 ((uint32) input.readIntBigEndian());
        */
    }
    
    pub fn read_time_tag(&mut self) -> OSCTimeTag {
        
        todo!();
        /*
            checkBytesAvailable (8, "OSC input stream exhausted while reading time tag");
                return OSCTimeTag (uint64 (input.readInt64BigEndian()));
        */
    }
    
    pub fn read_address(&mut self) -> OSCAddress {
        
        todo!();
        /*
            return OSCAddress (readString());
        */
    }
    
    pub fn read_address_pattern(&mut self) -> OSCAddressPattern {
        
        todo!();
        /*
            return OSCAddressPattern (readString());
        */
    }
    
    pub fn read_type_tag_string(&mut self) -> OSCTypeList {
        
        todo!();
        /*
            OSCTypeList typeList;

                checkBytesAvailable (4, "OSC input stream exhausted while reading type tag string");

                if (input.readByte() != ',')
                    throw OSCFormatError ("OSC input stream format error: expected type tag string");

                for (;;)
                {
                    if (isExhausted())
                        throw OSCFormatError ("OSC input stream exhausted while reading type tag string");

                    const OSCType type = input.readByte();

                    if (type == 0)
                        break;  // encountered null terminator. list is complete.

                    if (! OSCTypes::isSupportedType (type))
                        throw OSCFormatError ("OSC input stream format error: encountered unsupported type tag");

                    typeList.add (type);
                }

                auto bytesRead = (size_t) typeList.size() + 2;
                readPaddingZeros (bytesRead);

                return typeList;
        */
    }
    
    pub fn read_argument(&mut self, ty: OSCType) -> OSCArgument {
        
        todo!();
        /*
            switch (type)
                {
                    case OSCTypes::int32:       return OSCArgument (readInt32());
                    case OSCTypes::float32:     return OSCArgument (readFloat32());
                    case OSCTypes::string:      return OSCArgument (readString());
                    case OSCTypes::blob:        return OSCArgument (readBlob());
                    case OSCTypes::colour:      return OSCArgument (readColour());

                    default:
                        // You supplied an invalid OSCType when calling readArgument! This should never happen.
                        jassertfalse;
                        throw OSCInternalError ("OSC input stream: internal error while reading message argument");
                }
        */
    }
    
    pub fn read_message(&mut self) -> OSCMessage {
        
        todo!();
        /*
            auto ap = readAddressPattern();
                auto types = readTypeTagString();

                OSCMessage msg (ap);

                for (auto& type : types)
                    msg.addArgument (readArgument (type));

                return msg;
        */
    }
    
    pub fn read_bundle(&mut self, max_bytes_to_read: Option<usize>) -> OSCBundle {

        let max_bytes_to_read = max_bytes_to_read.unwrap_or(usize::MAX);
        
        todo!();
        /*
            // maxBytesToRead is only passed in here in case this bundle is a nested
                // bundle, so we know when to consider the next element *not* part of this
                // bundle anymore (but part of the outer bundle) and return.

                checkBytesAvailable (16, "OSC input stream exhausted while reading bundle");

                if (readString() != "#bundle")
                    throw OSCFormatError ("OSC input stream format error: bundle does not start with string '#bundle'");

                OSCBundle bundle (readTimeTag());

                size_t bytesRead = 16; // already read "#bundle" and timeTag
                auto pos = getPosition();

                while (! isExhausted() && bytesRead < maxBytesToRead)
                {
                    bundle.addElement (readElement());

                    auto newPos = getPosition();
                    bytesRead += (size_t) (newPos - pos);
                    pos = newPos;
                }

                return bundle;
        */
    }
    
    pub fn read_element(&mut self) -> OSCBundleElement {
        
        todo!();
        /*
            checkBytesAvailable (4, "OSC input stream exhausted while reading bundle element size");

                auto elementSize = (size_t) readInt32();

                if (elementSize < 4)
                    throw OSCFormatError ("OSC input stream format error: invalid bundle element size");

                return readElementWithKnownSize (elementSize);
        */
    }
    
    pub fn read_element_with_known_size(&mut self, element_size: usize) -> OSCBundleElement {
        
        todo!();
        /*
            checkBytesAvailable ((int64) elementSize, "OSC input stream exhausted while reading bundle element content");

                auto firstContentChar = static_cast<const char*> (getData()) [getPosition()];

                if (firstContentChar == '/')  return OSCBundle::Element (readMessageWithCheckedSize (elementSize));
                if (firstContentChar == '#')  return OSCBundle::Element (readBundleWithCheckedSize (elementSize));

                throw OSCFormatError ("OSC input stream: invalid bundle element content");
        */
    }
    
    pub fn read_padding_zeros(&mut self, bytes_read: usize)  {
        
        todo!();
        /*
            size_t numZeros = ~(bytesRead - 1) & 0x03;

                while (numZeros > 0)
                {
                    if (isExhausted() || input.readByte() != 0)
                        throw OSCFormatError ("OSC input stream format error: missing padding zeros");

                    --numZeros;
                }
        */
    }
    
    pub fn read_bundle_with_checked_size(&mut self, size: usize) -> OSCBundle {
        
        todo!();
        /*
            auto begin = (size_t) getPosition();
                auto maxBytesToRead = size - 4; // we've already read 4 bytes (the bundle size)

                OSCBundle bundle (readBundle (maxBytesToRead));

                if (getPosition() - begin != size)
                    throw OSCFormatError ("OSC input stream format error: wrong element content size encountered while reading");

                return bundle;
        */
    }
    
    pub fn read_message_with_checked_size(&mut self, size: usize) -> OSCMessage {
        
        todo!();
        /*
            auto begin = (size_t) getPosition();
                auto message = readMessage();

                if (getPosition() - begin != size)
                    throw OSCFormatError ("OSC input stream format error: wrong element content size encountered while reading");

                return message;
        */
    }
    
    pub fn check_bytes_available(&mut self, 
        required_bytes: i64,
        message:        *const u8)  {
        
        todo!();
        /*
            if (input.getNumBytesRemaining() < requiredBytes)
                    throw OSCFormatError (message);
        */
    }
}
