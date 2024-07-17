crate::ix!();

pub trait OutputStreamInterface:
Flush
+ SetPosition 
+ GetPosition 
+ OutputStreamWrite 
+ OutputStreamWriteByte 
+ OutputStreamWriteBool 
+ OutputStreamWriteShort 
+ OutputStreamWriteShortBigEndian 
+ OutputStreamWriteInt 
+ OutputStreamWriteIntBigEndian 
+ OutputStreamWriteInt64 
+ OutputStreamWriteInt64BigEndian 
+ OutputStreamWriteFloat 
+ OutputStreamWriteFloatBigEndian 
+ OutputStreamWriteDouble 
+ OutputStreamWriteDoubleBigEndian 
+ OutputStreamWriteRepeatedByte 
+ OutputStreamWriteCompressedInt 
+ OutputStreamWriteString 
+ OutputStreamWriteText 
+ OutputStreamWriteFromInputStream 
{ }

pub trait Flush {

    /**
      | If the stream is using a buffer, this
      | will ensure it gets written out to the
      | destination.
      |
      */
    fn flush(&mut self);
}

pub trait SetPosition {

    /**
      | Tries to move the stream's output position.
      | 
      | Not all streams will be able to seek to
      | a new position - this will return false
      | if it fails to work.
      | 
      | @see getPosition
      |
      */
    fn set_position(&mut self, new_position: i64) -> bool;
}

pub trait GetPosition {

    /**
      | Returns the stream's current position.
      | 
      | @see setPosition
      |
      */
    fn get_position(&mut self) -> i64;
}

pub trait OutputStreamWrite {

    /**
      | Writes a block of data to the stream.
      | 
      | When creating a subclass of OutputStream,
      | this is the only write method that needs
      | to be overloaded - the base class has
      | methods for writing other types of data
      | which use this to do the work.
      | 
      | -----------
      | @param dataToWrite
      | 
      | the target buffer to receive the data.
      | This must not be null.
      | ----------
      | @param numberOfBytes
      | 
      | the number of bytes to write.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason
      |
      */
    fn write(&mut self, 
        data_to_write:   *const c_void,
        number_of_bytes: usize) -> bool;
}

pub trait OutputStreamWriteByte {

    /**
      | Writes a single byte to the stream.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readByte
      |
      */
    fn write_byte(&mut self, byte: u8) -> bool {
        
        todo!();
        /*
        
        */
    }
}

pub trait OutputStreamWriteBool {

    /**
      | Writes a boolean to the stream as a single
      | byte. This is encoded as a binary byte
      | (not as text) with a value of 1 or 0.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readBool
      |
      */
    fn write_bool(&mut self, bool_value: bool) -> bool {
        
        todo!();
        /*
        
        */
    }
}

pub trait OutputStreamWriteShort {

    /**
      | Writes a 16-bit integer to the stream
      | in a little-endian byte order. This
      | will write two bytes to the stream: (value
      | & 0xff), then (value >> 8).
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readShort
      |
      */
    fn write_short(&mut self, value: i16) -> bool {
        
        todo!();
        /*
            auto v = ByteOrder::swapIfBigEndian ((uint16) value);
        return write (&v, 2);
        */
    }
}

pub trait OutputStreamWriteShortBigEndian {

    /**
      | Writes a 16-bit integer to the stream
      | in a big-endian byte order. This will
      | write two bytes to the stream: (value
      | >> 8), then (value & 0xff).
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readShortBigEndian
      |
      */
    fn write_short_big_endian(&mut self, value: i16) -> bool {
        
        todo!();
        /*
            auto v = ByteOrder::swapIfLittleEndian ((uint16) value);
        return write (&v, 2);
        */
    }
}

pub trait OutputStreamWriteInt {

    /**
      | Writes a 32-bit integer to the stream
      | in a little-endian byte order.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readInt
      |
      */
    fn write_int(&mut self, value: i32) -> bool {
        
        todo!();
        /*
            auto v = ByteOrder::swapIfBigEndian ((uint32) value);
        return write (&v, 4);
        */
    }
}

pub trait OutputStreamWriteIntBigEndian {

    /**
      | Writes a 32-bit integer to the stream
      | in a big-endian byte order.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readIntBigEndian
      |
      */
    fn write_int_big_endian(&mut self, value: i32) -> bool {
        
        todo!();
        /*
            auto v = ByteOrder::swapIfLittleEndian ((uint32) value);
        return write (&v, 4);
        */
    }
}

pub trait OutputStreamWriteInt64 {

    /**
      | Writes a 64-bit integer to the stream
      | in a little-endian byte order.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readInt64
      |
      */
    fn write_int64(&mut self, value: i64) -> bool {
        
        todo!();
        /*
            auto v = ByteOrder::swapIfBigEndian ((uint64) value);
        return write (&v, 8);
        */
    }
}

pub trait OutputStreamWriteInt64BigEndian {

    /**
      | Writes a 64-bit integer to the stream
      | in a big-endian byte order.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readInt64BigEndian
      |
      */
    fn write_int_64big_endian(&mut self, value: i64) -> bool {
        
        todo!();
        /*
            auto v = ByteOrder::swapIfLittleEndian ((uint64) value);
        return write (&v, 8);
        */
    }
}

pub trait OutputStreamWriteFloat {

    /**
      | Writes a 32-bit floating point value
      | to the stream in a binary format. The
      | binary 32-bit encoding of the float
      | is written as a little-endian int.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readFloat
      |
      */
    fn write_float(&mut self, value: f32) -> bool {
        
        todo!();
        /*
            union { int asInt; float asFloat; } n;
        n.asFloat = value;
        return writeInt (n.asInt);
        */
    }
}

pub trait OutputStreamWriteFloatBigEndian {

    /**
      | Writes a 32-bit floating point value
      | to the stream in a binary format. The
      | binary 32-bit encoding of the float
      | is written as a big-endian int.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readFloatBigEndian
      |
      */
    fn write_float_big_endian(&mut self, value: f32) -> bool {
        
        todo!();
        /*
            union { int asInt; float asFloat; } n;
        n.asFloat = value;
        return writeIntBigEndian (n.asInt);
        */
    }
}

pub trait OutputStreamWriteDouble {

    /**
      | Writes a 64-bit floating point value
      | to the stream in a binary format. The
      | eight raw bytes of the double value are
      | written out as a little-endian 64-bit
      | int.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readDouble
      |
      */
    fn write_double(&mut self, value: f64) -> bool {
        
        todo!();
        /*
            union { int64 asInt; double asDouble; } n;
        n.asDouble = value;
        return writeInt64 (n.asInt);
        */
    }
}

pub trait OutputStreamWriteDoubleBigEndian {

    /**
      | Writes a 64-bit floating point value
      | to the stream in a binary format. The
      | eight raw bytes of the double value are
      | written out as a big-endian 64-bit int.
      | @see InputStream::readDoubleBigEndian
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason
      |
      */
    fn write_double_big_endian(&mut self, value: f64) -> bool {
        
        todo!();
        /*
            union { int64 asInt; double asDouble; } n;
        n.asDouble = value;
        return writeInt64BigEndian (n.asInt);
        */
    }
}

pub trait OutputStreamWriteRepeatedByte {

    /**
      | Writes a byte to the output stream a given
      | number of times.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason
      |
      */
    fn write_repeated_byte(&mut self, 
        byte:                u8,
        num_times_to_repeat: usize) -> bool {
        
        todo!();
        /*
            for (size_t i = 0; i < numTimesToRepeat; ++i)
            if (! writeByte ((char) byte))
                return false;

        return true;
        */
    }
}

pub trait OutputStreamWriteCompressedInt {

    /**
      | Writes a condensed binary encoding
      | of a 32-bit integer.
      | 
      | If you're storing a lot of integers which
      | are unlikely to have very large values,
      | this can save a lot of space, because
      | values under 0xff will only take up 2
      | bytes, under 0xffff only 3 bytes, etc.
      | 
      | The format used is: number of significant
      | bytes + up to 4 bytes in little-endian
      | order.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readCompressedInt
      |
      */
    fn write_compressed_int(&mut self, value: i32) -> bool {
        
        todo!();
        /*
            auto un = (value < 0) ? (unsigned int) -value
                              : (unsigned int) value;

        uint8 data[5];
        int num = 0;

        while (un > 0)
        {
            data[++num] = (uint8) un;
            un >>= 8;
        }

        data[0] = (uint8) num;

        if (value < 0)
            data[0] |= 0x80;

        return write (data, (size_t) num + 1);
        */
    }
}

pub trait OutputStreamWriteString {

    /**
      | Stores a string in the stream in a binary
      | format.
      | 
      | This isn't the method to use if you're
      | trying to append text to the end of a text-file!
      | It's intended for storing a string so
      | that it can be retrieved later by InputStream::readString().
      | 
      | It writes the string to the stream as
      | UTF8, including the null termination
      | character.
      | 
      | For appending text to a file, instead
      | use writeText, or operator<<
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason @see InputStream::readString,
      | writeText, operator<<
      |
      */
    fn write_string(&mut self, text: &String) -> bool {
        
        todo!();
        /*
            auto numBytes = text.getNumBytesAsUTF8() + 1;

       #if (ALOE_STRING_UTF_TYPE == 8)
        return write (text.toRawUTF8(), numBytes);
       #else
        // (This avoids using toUTF8() to prevent the memory bloat that it would leave behind
        // if lots of large, persistent strings were to be written to streams).
        HeapBlock<char> temp (numBytes);
        text.copyToUTF8 (temp, numBytes);
        return write (temp, numBytes);
       #endif
        */
    }
}

pub trait OutputStreamWriteText {

    /**
      | Writes a string of text to the stream.
      | 
      | It can either write the text as UTF-8
      | or UTF-16, and can also add the UTF-16
      | byte-order-mark bytes (0xff, 0xfe)
      | to indicate the endianness (these should
      | only be used at the start of a file).
      | 
      | If lineEndings is nullptr, then line
      | endings in the text won't be modified.
      | If you pass "\\n" or "\\r\\n" then this
      | function will replace any existing
      | line feeds.
      | 
      | -----------
      | @return
      | 
      | false if the write operation fails for
      | some reason
      |
      */
    fn write_text(&mut self, 
        text:                       &String,
        asutf16:                    bool,
        write_utf16byte_order_mark: bool,
        lf:                         *const u8) -> bool {
        
        todo!();
        /*
            bool replaceLineFeedWithUnix    = lf != nullptr && lf[0] == '\n' && lf[1] == 0;
        bool replaceLineFeedWithWindows = lf != nullptr && lf[0] == '\r' && lf[1] == '\n' && lf[2] == 0;

        // The line-feed passed in must be either nullptr, or "\n" or "\r\n"
        jassert (lf == nullptr || replaceLineFeedWithWindows || replaceLineFeedWithUnix);

        if (asUTF16)
        {
            if (writeUTF16ByteOrderMark)
                write ("\x0ff\x0fe", 2);

            auto src = text.getCharPointer();
            bool lastCharWasReturn = false;

            for (;;)
            {
                auto c = src.getAndAdvance();

                if (c == 0)
                    break;

                if (replaceLineFeedWithWindows)
                {
                    if (c == '\n' && ! lastCharWasReturn)
                        writeShort ((short) '\r');

                    lastCharWasReturn = (c == L'\r');
                }
                else if (replaceLineFeedWithUnix && c == '\r')
                {
                    continue;
                }

                if (! writeShort ((short) c))
                    return false;
            }
        }
        else
        {
            const char* src = text.toRawUTF8();

            if (replaceLineFeedWithWindows)
            {
                for (auto t = src;;)
                {
                    if (*t == '\n')
                    {
                        if (t > src)
                            if (! write (src, (size_t) (t - src)))
                                return false;

                        if (! write ("\r\n", 2))
                            return false;

                        src = t + 1;
                    }
                    else if (*t == '\r')
                    {
                        if (t[1] == '\n')
                            ++t;
                    }
                    else if (*t == 0)
                    {
                        if (t > src)
                            if (! write (src, (size_t) (t - src)))
                                return false;

                        break;
                    }

                    ++t;
                }
            }
            else if (replaceLineFeedWithUnix)
            {
                for (;;)
                {
                    auto c = *src++;

                    if (c == 0)
                        break;

                    if (c != '\r')
                        if (! writeByte (c))
                            return false;
                }
            }
            else
            {
                return write (src, text.getNumBytesAsUTF8());
            }
        }

        return true;
        */
    }
}

pub trait OutputStreamWriteFromInputStream {

    /**
      | Reads data from an input stream and writes
      | it to this stream.
      | 
      | -----------
      | @param source
      | 
      | the stream to read from
      | ----------
      | @param maxNumBytesToWrite
      | 
      | the number of bytes to read from the stream
      | (if this is less than zero, it will keep
      | reading until the input is exhausted)
      | 
      | -----------
      | @return
      | 
      | the number of bytes written
      |
      */
    fn write_from_input_stream<R: Read>(
        &mut self, 
        source:             &mut R,
        num_bytes_to_write: i64

    ) -> i64 {
        
        todo!();
        /*
            if (numBytesToWrite < 0)
            numBytesToWrite = std::numeric_limits<int64>::max();

        int64 numWritten = 0;

        while (numBytesToWrite > 0)
        {
            char buffer[8192];
            auto num = source.read (buffer, (int) jmin (numBytesToWrite, (int64) sizeof (buffer)));

            if (num <= 0)
                break;

            write (buffer, (size_t) num);

            numBytesToWrite -= num;
            numWritten += num;
        }

        return numWritten;
        */
    }
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_OutputStream.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_OutputStream.cpp]

/**
  | The base class for streams that write
  | data to some kind of destination.
  | 
  | Input and output streams are used throughout
  | the library - subclasses can override
  | some or all of the virtual functions
  | to implement their behaviour.
  | 
  | @see InputStream, MemoryOutputStream,
  | FileOutputStream
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
#[no_copy]
pub struct OutputStream {
    new_line_string: String,
}

impl Drop for OutputStream {

    /**
      | Destructor.
      | 
      | Some subclasses might want to do things
      | like call flush() during their destructors.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
       #if ALOE_DEBUG
        if (! DanglingStreamChecker::hasBeenDestroyed)
            danglingStreamChecker.activeStreams.removeFirstMatchingValue (this);
       #endif
     */
    }
}

impl Default for OutputStream {

    fn default() -> Self {
    
        todo!();
        /*
            : newLineString (NewLine::getDefault())

       #if ALOE_DEBUG
        if (! DanglingStreamChecker::hasBeenDestroyed)
            danglingStreamChecker.activeStreams.add (this);
       #endif
        */
    }
}

impl OutputStream {

    /**
      | Sets the string to write to the stream
      | when a new line is written.
      | 
      | By default this will be set the value
      | of NewLine::getDefault().
      |
      */
    pub fn set_new_line_string(&mut self, new_line_string_to_use: &String)  {
        
        todo!();
        /*
            newLineString = newLineStringToUse;
        */
    }

    /**
      | Returns the current new-line string
      | that was set by setNewLineString().
      |
      */
    pub fn get_new_line_string(&self) -> &String {
        
        todo!();
        /*
            return newLineString;
        */
    }

    pub fn write_byte(&mut self, byte: u8) -> bool {
        
        todo!();
        /*
            return write (&byte, 1);
        */
    }

    pub fn write_bool(&mut self, b: bool) -> bool {
        
        todo!();
        /*
            return writeByte (b ? (char) 1
                            : (char) 0);
        */
    }
}

impl Shl<i32> for &mut OutputStream {

    type Output = OutputStream;

    /**
      | Writes a number to a stream as 8-bit characters
      | in the default system encoding.
      |
      */
    #[inline] fn shl(self, rhs: i32) -> Self::Output {
        todo!();
        /*
            writeIntToStream (stream, number);
        return stream;
        */
    }
}

impl Shl<i64> for &mut OutputStream {

    type Output = OutputStream;
    
    /**
      | Writes a number to a stream as 8-bit characters
      | in the default system encoding.
      |
      */
    #[inline] fn shl(self, rhs: i64) -> Self::Output {
        todo!();
        /*
            writeIntToStream (stream, number);
        return stream;
        */
    }
}

impl Shl<f64> for &mut OutputStream {

    type Output = OutputStream;

    /**
      | Writes a number to a stream as 8-bit characters
      | in the default system encoding.
      |
      */
    #[inline] fn shl(self, rhs: f64) -> Self::Output {
        todo!();
        /*
            return stream << String (number);
        */
    }
}

impl Shl<u8> for &mut OutputStream {

    type Output = OutputStream;

    /**
      | Writes a character to a stream.
      |
      */
    #[inline] fn shl(self, rhs: u8) -> Self::Output {
        todo!();
        /*
            stream.writeByte (character);
        return stream;
        */
    }
}

impl Shl<*const u8> for &mut OutputStream {

    type Output = OutputStream;
    
    /**
      | Writes a null-terminated text string
      | to a stream.
      |
      */
    #[inline] fn shl(self, rhs: *const u8) -> Self::Output {
        todo!();
        /*
            stream.write (text, strlen (text));
        return stream;
        */
    }
}

impl Shl<&MemoryBlock> for &mut OutputStream {

    type Output = OutputStream;
    
    /**
      | Writes a block of data from a MemoryBlock
      | to a stream.
      |
      */
    #[inline] fn shl(self, rhs: &MemoryBlock) -> Self::Output {
        todo!();
        /*
            if (! data.isEmpty())
            stream.write (data.getData(), data.getSize());

        return stream;
        */
    }
}

impl Shl<&File> for &mut OutputStream {

    type Output = OutputStream;

    /**
      | Writes the contents of a file to a stream.
      |
      */
    #[inline] fn shl(self, rhs: &File) -> Self::Output {
        todo!();
        /*
            FileInputStream in (fileToRead);

        if (in.openedOk())
            return stream << in;

        return stream;
        */
    }
}

impl<R: Read> Shl<&mut R> for &mut OutputStream {

    type Output = OutputStream;
    
    /**
      | Writes the complete contents of an input
      | stream to an output stream.
      |
      */
    #[inline] fn shl(self, rhs: &mut R) -> Self::Output {
        todo!();
        /*
            stream.writeFromInputStream (streamToRead, -1);
        return stream;
        */
    }
}

impl Shl<&NewLine> for &mut OutputStream {

    type Output = OutputStream;

    /**
      | Writes a new-line to a stream.
      | 
      | You can use the predefined symbol 'newLine'
      | to invoke this, e.g.
      | 
      | @code
      | 
      | myOutputStream << "Hello World" <<
      | newLine << newLine;
      | 
      | @endcode
      | 
      | @see OutputStream::setNewLineString
      |
      */
    #[inline] fn shl(self, rhs: &NewLine) -> Self::Output {
        todo!();
        /*
            return stream << stream.getNewLineString();
        */
    }
}

#[cfg(ALOE_DEBUG)]
#[derive(Default)]
pub struct DanglingStreamChecker {
    active_streams: Vec<*mut c_void,CriticalSection>,
}

#[cfg(ALOE_DEBUG)]
pub mod dangling_stream_checker {
    use super::*;
    lazy_static!{
        /*
        static bool hasBeenDestroyed;
        bool DanglingStreamChecker::hasBeenDestroyed = false;
        */
    }
}

#[cfg(ALOE_DEBUG)]
impl DanglingStreamChecker {

}

#[cfg(ALOE_DEBUG)]
impl Drop for DanglingStreamChecker {
    fn drop(&mut self) {
        todo!();
        /* 
            /*
                It's always a bad idea to leak any object, but if you're leaking output
                streams, then there's a good chance that you're failing to flush a file
                to disk properly, which could result in corrupted data and other similar
                nastiness..
            */
            jassert (activeStreams.size() == 0);

            // We need to flag when this helper struct has been destroyed to prevent some
            // nasty order-of-static-destruction issues
            hasBeenDestroyed = true;
         */
    }
}

#[cfg(ALOE_DEBUG)]
lazy_static!{
    /*
    static DanglingStreamChecker danglingStreamChecker;
    */
}

///--------------------------------

pub fn write_int_to_stream<IntegerType>(
        stream: &mut OutputStream,
        number: IntegerType)  {

    todo!();
    /*
        char buffer[NumberToStringConverters::charsNeededForInt];
        char* end = buffer + numElementsInArray (buffer);
        const char* start = NumberToStringConverters::numberToString (end, number);
        stream.write (start, (size_t) (end - start - 1));
    */
}
