crate::ix!();

/**
  | Writes OSC data to an internal memory
  | buffer, which grows as required.
  | 
  | The data that was written into the stream
  | can then be accessed later as a contiguous
  | block of memory.
  | 
  | This class implements the Open Sound
  | Control 1.0 Specification for the format
  | in which the OSC data will be written
  | into the buffer.
  |
  */
#[no_copy]
#[leak_detector]
pub struct OSCOutputStream {
    output: MemoryOutputStream,
}

impl OSCOutputStream {

    /**
      | Returns a pointer to the data that has
      | been written to the stream.
      |
      */
    pub fn get_data(&self)  {
        
        todo!();
        /*
            return output.getData();
        */
    }

    /**
      | Returns the number of bytes of data that
      | have been written to the stream.
      |
      */
    pub fn get_data_size(&self) -> usize {
        
        todo!();
        /*
            return output.getDataSize();
        */
    }
    
    pub fn write_int32(&mut self, value: i32) -> bool {
        
        todo!();
        /*
            return output.writeIntBigEndian (value);
        */
    }
    
    pub fn write_uint64(&mut self, value: u64) -> bool {
        
        todo!();
        /*
            return output.writeInt64BigEndian (int64 (value));
        */
    }
    
    pub fn write_float32(&mut self, value: f32) -> bool {
        
        todo!();
        /*
            return output.writeFloatBigEndian (value);
        */
    }
    
    pub fn write_string(&mut self, value: &String) -> bool {
        
        todo!();
        /*
            if (! output.writeString (value))
                return false;

            const size_t numPaddingZeros = ~value.getNumBytesAsUTF8() & 3;

            return output.writeRepeatedByte ('\0', numPaddingZeros);
        */
    }
    
    pub fn write_blob(&mut self, blob: &MemoryBlock) -> bool {
        
        todo!();
        /*
            if (! (output.writeIntBigEndian ((int) blob.getSize())
                    && output.write (blob.getData(), blob.getSize())))
                return false;

            const size_t numPaddingZeros = ~(blob.getSize() - 1) & 3;

            return output.writeRepeatedByte (0, numPaddingZeros);
        */
    }
    
    pub fn write_colour(&mut self, colour: OSCColour) -> bool {
        
        todo!();
        /*
            return output.writeIntBigEndian ((int32) colour.toInt32());
        */
    }
    
    pub fn write_time_tag(&mut self, time_tag: OSCTimeTag) -> bool {
        
        todo!();
        /*
            return output.writeInt64BigEndian (int64 (timeTag.getRawTimeTag()));
        */
    }
    
    pub fn write_address(&mut self, address: &OSCAddress) -> bool {
        
        todo!();
        /*
            return writeString (address.toString());
        */
    }
    
    pub fn write_address_pattern(&mut self, ap: &OSCAddressPattern) -> bool {
        
        todo!();
        /*
            return writeString (ap.toString());
        */
    }
    
    pub fn write_type_tag_string(&mut self, type_list: &OSCTypeList) -> bool {
        
        todo!();
        /*
            output.writeByte (',');

            if (typeList.size() > 0)
                output.write (typeList.begin(), (size_t) typeList.size());

            output.writeByte ('\0');

            size_t bytesWritten = (size_t) typeList.size() + 1;
            size_t numPaddingZeros = ~bytesWritten & 0x03;

            return output.writeRepeatedByte ('\0', numPaddingZeros);
        */
    }
    
    pub fn write_argument(&mut self, arg: &OSCArgument) -> bool {
        
        todo!();
        /*
            switch (arg.getType())
            {
                case OSCTypes::int32:       return writeInt32 (arg.getInt32());
                case OSCTypes::float32:     return writeFloat32 (arg.getFloat32());
                case OSCTypes::string:      return writeString (arg.getString());
                case OSCTypes::blob:        return writeBlob (arg.getBlob());
                case OSCTypes::colour:      return writeColour (arg.getColour());

                default:
                    // In this very unlikely case you supplied an invalid OSCType!
                    jassertfalse;
                    return false;
            }
        */
    }
    
    pub fn write_message(&mut self, msg: &OSCMessage) -> bool {
        
        todo!();
        /*
            if (! writeAddressPattern (msg.getAddressPattern()))
                return false;

            OSCTypeList typeList;

            for (auto& arg : msg)
                typeList.add (arg.getType());

            if (! writeTypeTagString (typeList))
                return false;

            for (auto& arg : msg)
                if (! writeArgument (arg))
                    return false;

            return true;
        */
    }
    
    pub fn write_bundle(&mut self, bundle: &OSCBundle) -> bool {
        
        todo!();
        /*
            if (! writeString ("#bundle"))
                return false;

            if (! writeTimeTag (bundle.getTimeTag()))
                return false;

            for (auto& element : bundle)
                if (! writeBundleElement (element))
                    return false;

            return true;
        */
    }
    
    pub fn write_bundle_element(&mut self, element: &OSCBundleElement) -> bool {
        
        todo!();
        /*
            const int64 startPos = output.getPosition();

            if (! writeInt32 (0))   // writing dummy value for element size
                return false;

            if (element.isBundle())
            {
                if (! writeBundle (element.getBundle()))
                    return false;
            }
            else
            {
                if (! writeMessage (element.getMessage()))
                    return false;
            }

            const int64 endPos = output.getPosition();
            const int64 elementSize = endPos - (startPos + 4);

            return output.setPosition (startPos)
                     && writeInt32 ((int32) elementSize)
                     && output.setPosition (endPos);
        */
    }
}
