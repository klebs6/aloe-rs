crate::ix!();

/**
  | The base class for streams that read
  | data.
  | 
  | Input and output streams are used throughout
  | the library - subclasses can override
  | some or all of the virtual functions
  | to implement their behaviour.
  | 
  | @see OutputStream, MemoryInputStream,
  | BufferedInputStream, FileInputStream
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
#[no_copy]
pub trait InputStream: 
Default 
+ GetTotalLength
+ GetNumBytesRemaining
+ IsExhausted
+ ReadInputStream
+ ReadInputStreamByte
+ ReadBool
+ ReadShort
+ ReadShortBigEndian
+ ReadInt
+ ReadIntBigEndian
+ ReadInt64
+ ReadInt64BigEndian
+ ReadFloat
+ ReadFloatBigEndian
+ ReadDouble
+ ReadDoubleBigEndian
+ ReadCompressedInt
+ ReadNextLine
+ ReadString
+ ReadEntireStreamAsString
+ ReadIntoMemoryBlock
+ GetPosition
+ SetPosition
+ SkipNextBytes
{

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_InputStream.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_InputStream.cpp]

pub trait GetTotalLength {

    /**
      | Returns the total number of bytes available
      | for reading in this stream.
      | 
      | Note that this is the number of bytes
      | available from the start of the stream,
      | not from the current position.
      | 
      | If the size of the stream isn't actually
      | known, this will return -1.
      | 
      | @see getNumBytesRemaining
      |
      */
    fn get_total_length(&mut self) -> i64;
}

pub trait GetNumBytesRemaining {

    /**
      | Returns the number of bytes available
      | for reading, or a negative value if the
      | remaining length is not known. @see
      | getTotalLength
      |
      */
    fn get_num_bytes_remaining(&mut self) -> i64 {
        
        todo!();
        /*
            auto len = getTotalLength();

        if (len >= 0)
            len -= getPosition();

        return len;
        */
    }
}

pub trait IsExhausted {

    /**
      | Returns true if the stream has no more
      | data to read.
      |
      */
    fn is_exhausted(&mut self) -> bool;
}

pub trait ReadInputStream {

    /**
      | Reads some data from the stream into
      | a memory buffer.
      | 
      | This is the only read method that subclasses
      | actually need to implement, as the InputStream
      | base class implements the other read
      | methods in terms of this one (although
      | it's often more efficient for subclasses
      | to implement them directly).
      | 
      | -----------
      | @param destBuffer
      | 
      | the destination buffer for the data.
      | This must not be null.
      | ----------
      | @param maxBytesToRead
      | 
      | the maximum number of bytes to read -
      | make sure the memory block passed in
      | is big enough to contain this many bytes.
      | This value must not be negative.
      | 
      | -----------
      | @return
      | 
      | the actual number of bytes that were
      | read, which may be less than maxBytesToRead
      | if the stream is exhausted before it
      | gets that far
      |
      */
    fn read(&mut self, 
        dest_buffer:       *mut c_void,
        max_bytes_to_read: usize) -> isize {
        
        todo!();
        /*
            ssize_t totalRead = 0;

        while (size > 0)
        {
            auto numToRead = (int) std::min (size, (size_t) 0x70000000);
            auto numRead = read (addBytesToPointer (destBuffer, totalRead), numToRead);
            jassert (numRead <= numToRead);

            if (numRead < 0) return (ssize_t) numRead;
            if (numRead == 0) break;

            size -= (size_t) numRead;
            totalRead += numRead;
        }

        return totalRead;
        */
    }
}

pub trait ReadInputStreamByte {

    /**
      | Reads a byte from the stream.
      | 
      | If the stream is exhausted, this will
      | return zero. @see OutputStream::writeByte
      |
      */
    fn read_byte(&mut self) -> u8 {
        
        todo!();
        /*
            char temp = 0;
        read (&temp, 1);
        return temp;
        */
    }
}

pub trait ReadBool {

    /**
      | Reads a boolean from the stream.
      | 
      | The bool is encoded as a single byte -
      | non-zero for true, 0 for false.
      | 
      | If the stream is exhausted, this will
      | return false. @see OutputStream::writeBool
      |
      */
    fn read_bool(&mut self) -> bool {
        
        todo!();
        /*
            return readByte() != 0;
        */
    }
}

pub trait ReadShort {

    /**
      | Reads two bytes from the stream as a little-endian
      | 16-bit value.
      | 
      | If the next two bytes read are byte1 and
      | byte2, this returns (byte1 | (byte2
      | << 8)).
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeShort, readShortBigEndian
      |
      */
    fn read_short(&mut self) -> i16 {
        
        todo!();
        /*
            char temp[2];

        if (read (temp, 2) == 2)
            return (short) ByteOrder::littleEndianShort (temp);

        return 0;
        */
    }
}

pub trait ReadShortBigEndian {

    /**
      | Reads two bytes from the stream as a little-endian
      | 16-bit value.
      | 
      | If the next two bytes read are byte1 and
      | byte2, this returns (byte2 | (byte1
      | << 8)).
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeShortBigEndian,
      | readShort
      |
      */
    fn read_short_big_endian(&mut self) -> i16 {
        
        todo!();
        /*
            char temp[2];

        if (read (temp, 2) == 2)
            return (short) ByteOrder::bigEndianShort (temp);

        return 0;
        */
    }
}

pub trait ReadInt {

    /**
      | Reads four bytes from the stream as a
      | little-endian 32-bit value.
      | 
      | If the next four bytes are byte1 to byte4,
      | this returns (byte1 | (byte2 << 8) | (byte3
      | << 16) | (byte4 << 24)).
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeInt, readIntBigEndian
      |
      */
    fn read_int(&mut self) -> i32 {
        
        todo!();
        /*
            char temp[4];

        if (read (temp, 4) == 4)
            return (int) ByteOrder::littleEndianInt (temp);

        return 0;
        */
    }
}

pub trait ReadIntBigEndian {

    /**
      | Reads four bytes from the stream as a
      | big-endian 32-bit value.
      | 
      | If the next four bytes are byte1 to byte4,
      | this returns (byte4 | (byte3 << 8) | (byte2
      | << 16) | (byte1 << 24)).
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeIntBigEndian,
      | readInt
      |
      */
    fn read_int_big_endian(&mut self) -> i32 {
        
        todo!();
        /*
            char temp[4];

        if (read (temp, 4) == 4)
            return (int) ByteOrder::bigEndianInt (temp);

        return 0;
        */
    }
}

pub trait ReadInt64 {

    /**
      | Reads eight bytes from the stream as
      | a little-endian 64-bit value.
      | 
      | If the next eight bytes are byte1 to byte8,
      | this returns (byte1 | (byte2 << 8) | (byte3
      | << 16) | (byte4 << 24) | (byte5 << 32) |
      | (byte6 << 40) | (byte7 << 48) | (byte8
      | << 56)).
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeInt64, readInt64BigEndian
      |
      */
    fn read_int64(&mut self) -> i64 {
        
        todo!();
        /*
            union { uint8 asBytes[8]; uint64 asInt64; } n;

        if (read (n.asBytes, 8) == 8)
            return (int64) ByteOrder::swapIfBigEndian (n.asInt64);

        return 0;
        */
    }
}

pub trait ReadInt64BigEndian {

    /**
      | Reads eight bytes from the stream as
      | a big-endian 64-bit value.
      | 
      | If the next eight bytes are byte1 to byte8,
      | this returns (byte8 | (byte7 << 8) | (byte6
      | << 16) | (byte5 << 24) | (byte4 << 32) |
      | (byte3 << 40) | (byte2 << 48) | (byte1
      | << 56)).
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeInt64BigEndian,
      | readInt64
      |
      */
    fn read_int_64big_endian(&mut self) -> i64 {
        
        todo!();
        /*
            union { uint8 asBytes[8]; uint64 asInt64; } n;

        if (read (n.asBytes, 8) == 8)
            return (int64) ByteOrder::swapIfLittleEndian (n.asInt64);

        return 0;
        */
    }
}

pub trait ReadFloat {

    /**
      | Reads four bytes as a 32-bit floating
      | point value.
      | 
      | The raw 32-bit encoding of the float
      | is read from the stream as a little-endian
      | int.
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeFloat, readDouble
      |
      */
    fn read_float(&mut self) -> f32 {
        
        todo!();
        /*
            static_assert (sizeof (int32) == sizeof (float), "Union assumes float has the same size as an int32");
        union { int32 asInt; float asFloat; } n;
        n.asInt = (int32) readInt();
        return n.asFloat;
        */
    }
}

pub trait ReadFloatBigEndian {

    /**
      | Reads four bytes as a 32-bit floating
      | point value.
      | 
      | The raw 32-bit encoding of the float
      | is read from the stream as a big-endian
      | int.
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero. @see OutputStream::writeFloatBigEndian,
      | readDoubleBigEndian
      |
      */
    fn read_float_big_endian(&mut self) -> f32 {
        
        todo!();
        /*
            union { int32 asInt; float asFloat; } n;
        n.asInt = (int32) readIntBigEndian();
        return n.asFloat;
        */
    }
}

pub trait ReadDouble {

    /**
      | Reads eight bytes as a 64-bit floating
      | point value.
      | 
      | The raw 64-bit encoding of the double
      | is read from the stream as a little-endian
      | int64.
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeDouble,
      | readFloat
      |
      */
    fn read_double(&mut self) -> f64 {
        
        todo!();
        /*
            union { int64 asInt; double asDouble; } n;
        n.asInt = readInt64();
        return n.asDouble;
        */
    }
}

pub trait ReadDoubleBigEndian {

    /**
      | Reads eight bytes as a 64-bit floating
      | point value.
      | 
      | The raw 64-bit encoding of the double
      | is read from the stream as a big-endian
      | int64.
      | 
      | If the stream is exhausted partway through
      | reading the bytes, this will return
      | zero.
      | 
      | @see OutputStream::writeDoubleBigEndian,
      | readFloatBigEndian
      |
      */
    fn read_double_big_endian(&mut self) -> f64 {
        
        todo!();
        /*
            union { int64 asInt; double asDouble; } n;
        n.asInt = readInt64BigEndian();
        return n.asDouble;
        */
    }
}

pub trait ReadCompressedInt {

    /**
      | Reads an encoded 32-bit number from
      | the stream using a space-saving compressed
      | format.
      | 
      | For small values, this is more space-efficient
      | than using readInt() and OutputStream::writeInt()
      | 
      | The format used is: number of significant
      | bytes + up to 4 bytes in little-endian
      | order.
      | 
      | @see OutputStream::writeCompressedInt()
      |
      */
    fn read_compressed_int(&mut self) -> i32 {
        
        todo!();
        /*
            auto sizeByte = (uint8) readByte();

        if (sizeByte == 0)
            return 0;

        const int numBytes = (sizeByte & 0x7f);

        if (numBytes > 4)
        {
            jassertfalse;  // trying to read corrupt data - this method must only be used
                           // to read data that was written by OutputStream::writeCompressedInt()
            return 0;
        }

        char bytes[4] = {};

        if (read (bytes, numBytes) != numBytes)
            return 0;

        auto num = (int) ByteOrder::littleEndianInt (bytes);
        return (sizeByte >> 7) ? -num : num;
        */
    }
}

pub trait ReadNextLine {

    /**
      | Reads a UTF-8 string from the stream,
      | up to the next linefeed or carriage return.
      | 
      | This will read up to the next "\n" or "\r\n"
      | or end-of-stream.
      | 
      | After this call, the stream's position
      | will be left pointing to the next character
      | following the line-feed, but the linefeeds
      | aren't included in the string that is
      | returned.
      |
      */
    fn read_next_line(&mut self) -> String {
        
        todo!();
        /*
            MemoryOutputStream buffer;

        for (;;)
        {
            auto c = readByte();

            if (c == 0 || c == '\n')
                break;

            if (c == '\r')
            {
                auto lastPos = getPosition();

                if (readByte() != '\n')
                    setPosition (lastPos);

                break;
            }

            buffer.writeByte (c);
        }

        return buffer.toUTF8();
        */
    }
}

pub trait ReadString {

    /**
      | Reads a zero-terminated UTF-8 string
      | from the stream.
      | 
      | This will read characters from the stream
      | until it hits a null character or end-of-stream.
      | 
      | @see OutputStream::writeString,
      | readEntireStreamAsString
      |
      */
    fn read_string(&mut self) -> String {
        
        todo!();
        /*
            MemoryOutputStream buffer;

        for (;;)
        {
            auto c = readByte();
            buffer.writeByte (c);

            if (c == 0)
                return buffer.toUTF8();
        }
        */
    }
}

pub trait ReadEntireStreamAsString {

    /**
      | Tries to read the whole stream and turn
      | it into a string.
      | 
      | This will read from the stream's current
      | position until the end-of-stream.
      | 
      | It can read from UTF-8 data, or UTF-16
      | if it detects suitable header-bytes.
      |
      */
    fn read_entire_stream_as_string(&mut self) -> String {
        
        todo!();
        /*
            MemoryOutputStream mo;
        mo << *this;
        return mo.toString();
        */
    }
}

pub trait ReadIntoMemoryBlock {

    /**
      | Reads from the stream and appends the
      | data to a MemoryBlock.
      | 
      | -----------
      | @param destBlock
      | 
      | the block to append the data onto
      | ----------
      | @param maxNumBytesToRead
      | 
      | if this is a positive value, it sets a
      | limit to the number of bytes that will
      | be read - if it's negative, data will
      | be read until the stream is exhausted.
      | 
      | -----------
      | @return
      | 
      | the number of bytes that were added to
      | the memory block
      |
      */
    fn read_into_memory_block(
        &mut self, 
        block:     &mut MemoryBlock,
        num_bytes: Option<isize>

    ) -> usize {

        let num_bytes = num_bytes.unwrap_or(-1);
        
        todo!();
        /*
            MemoryOutputStream mo (block, true);
        return (size_t) mo.writeFromInputStream (*this, numBytes);
        */
    }
}

pub trait GetPosition {

    /**
      | Returns the offset of the next byte that
      | will be read from the stream.
      | 
      | @see setPosition
      |
      */
    fn get_position(&mut self) -> i64;
}

pub trait SetPosition {

    /**
      | Tries to move the current read position
      | of the stream.
      | 
      | The position is an absolute number of
      | bytes from the stream's start.
      | 
      | Some streams might not be able to do this,
      | in which case they should do nothing
      | and return false. Others might be able
      | to manage it by resetting themselves
      | and skipping to the correct position,
      | although this is obviously a bit slow.
      | 
      | -----------
      | @return
      | 
      | true if the stream manages to reposition
      | itself correctly @see getPosition
      |
      */
    fn set_position(&mut self, new_position: i64) -> bool;
}

pub trait SkipNextBytes {

    /**
      | Reads and discards a number of bytes
      | from the stream.
      | 
      | Some input streams might implement
      | this more efficiently, but the base
      | class will just keep reading data until
      | the requisite number of bytes have been
      | done. For large skips it may be quicker
      | to call setPosition() with the required
      | position.
      |
      */
    fn skip_next_bytes(&mut self, num_bytes_to_skip: i64)  {
        
        todo!();
        /*
            if (numBytesToSkip > 0)
        {
            auto skipBufferSize = (int) jmin (numBytesToSkip, (int64) 16384);
            HeapBlock<char> temp (skipBufferSize);

            while (numBytesToSkip > 0 && ! isExhausted())
                numBytesToSkip -= read (temp, (int) jmin (numBytesToSkip, (int64) skipBufferSize));
        }
        */
    }
}
