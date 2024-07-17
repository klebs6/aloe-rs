crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/hashing/aloe_MD5.h]

/**
  | MD5 checksum class.
  | 
  | Create one of these with a block of source
  | data or a stream, and it calculates the
  | MD5 checksum of that data.
  | 
  | You can then retrieve this checksum
  | as a 16-byte block, or as a hex string.
  | @see SHA256
  | 
  | @tags{Cryptography}
  |
  */
#[leak_detector]
#[derive(Copy,Clone)]
pub struct MD5 {

    result: [u8; 16],
}

impl Default for MD5 {
    
    /**
      | Creates a null MD5 object.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl PartialEq<MD5> for MD5 {
    
    #[inline] fn eq(&self, other: &MD5) -> bool {
        todo!();
        /*
            return memcmp (result, other.result, sizeof (result)) == 0;
        */
    }
}

impl Eq for MD5 {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/hashing/aloe_MD5.cpp]
impl MD5 {

    /**
      | Returns a pointer to the 16-byte array
      | of result data.
      |
      */
    pub fn get_checksum_data_array(&self) -> *const u8 {
        
        todo!();
        /*
            return result;
        */
    }

    /**
      | Creates a checksum for a block of binary
      | data.
      |
      */
    pub fn new_from_raw(
        data:      *const c_void,
        num_bytes: usize) -> Self {
    
        todo!();
        /*


            MD5Generator generator;
        generator.processBlock (data, numBytes);
        generator.finish (result);
        */
    }
    
    /**
      | Creates a checksum for a block of binary
      | data.
      |
      */
    pub fn new_from_memory_block(data: &MemoryBlock) -> Self {
    
        todo!();
        /*
        : md5(data.getData(), data.getSize()),

        
        */
    }
    
    /**
      | Creates a checksum of the characters
      | in a UTF-8 buffer. E.g.
      | 
      | @code MD5 checksum (myString.toUTF8());
      | @endcode
      |
      */
    pub fn new_from_chars(utf8: CharPointer_UTF8) -> Self {
    
        todo!();
        /*


            : MD5 (utf8.getAddress(), utf8.getAddress() != nullptr ? utf8.sizeInBytes() - 1 : 0)
        */
    }
    
    /**
      | Creates an MD5 from a little-endian
      | UTF-32 encoded string.
      | 
      | Note that this method is provided for
      | backwards-compatibility with the
      | old version of this class, which had
      | a constructor that took a string and
      | performed this operation on it. In new
      | code, you shouldn't use this, and are
      | recommended to use the constructor
      | that takes a CharPointer_UTF8 instead.
      |
      */
    pub fn fromutf32(&mut self, text: &str) -> MD5 {
        
        todo!();
        /*
            MD5 m;
        MD5Generator generator;

        for (auto t = text.text; t.isNotEmpty();)
        {
            auto unicodeChar = ByteOrder::swapIfBigEndian ((uint32_t) t.getAndAdvance());
            generator.processBlock (&unicodeChar, sizeof (unicodeChar));
        }

        generator.finish (m.result);
        return m;
        */
    }
    
    /**
      | Creates a checksum for the input from
      | a stream.
      | 
      | This will read up to the given number
      | of bytes from the stream, and produce
      | the checksum of that. If the number of
      | bytes to read is negative, it'll read
      | until the stream is exhausted.
      |
      */
    pub fn new_from_stream(
        input:             &mut dyn Read,
        num_bytes_to_read: Option<i64>

    ) -> Self {

        let num_bytes_to_read: i64 =
            num_bytes_to_read.unwrap_or(-1);
    
        todo!();
        /*


            processStream (input, numBytesToRead);
        */
    }
    
    /**
      | Creates a checksum for the contents
      | of a file.
      |
      */
    pub fn new_from_file(file: &File) -> Self {
    
        todo!();
        /*


            FileInputStream fin (file);

        if (fin.openedOk())
            processStream (fin, -1);
        */
    }
    
    pub fn process_stream(&mut self, 
        input:             &mut dyn Read,
        num_bytes_to_read: i64)  {
        
        todo!();
        /*
            MD5Generator generator;

        if (numBytesToRead < 0)
            numBytesToRead = std::numeric_limits<int64>::max();

        while (numBytesToRead > 0)
        {
            uint8_t tempBuffer[512];
            auto bytesRead = input.read (tempBuffer, (int) jmin (numBytesToRead, (int64) sizeof (tempBuffer)));

            if (bytesRead <= 0)
                break;

            numBytesToRead -= bytesRead;
            generator.processBlock (tempBuffer, (size_t) bytesRead);
        }

        generator.finish (result);
        */
    }

    /**
      | Returns the checksum as a 16-byte block
      | of data.
      |
      */
    pub fn get_raw_checksum_data(&self) -> MemoryBlock {
        
        todo!();
        /*
            return MemoryBlock (result, sizeof (result));
        */
    }

    /**
      | Returns the checksum as a 32-digit hex
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



///---------------------
pub struct MD5Generator {
    buffer: [u8; 64],
    state:  [u32; 4], // = { 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476 };
    count:  [u32; 2],
}

impl MD5Generator {

    pub fn process_block(&mut self, 
        data:      *const c_void,
        data_size: usize)  {
        
        todo!();
        /*
            auto bufferPos = ((count[0] >> 3) & 0x3f);

            count[0] += (uint32_t) (dataSize << 3);

            if (count[0] < ((uint32_t) dataSize << 3))
                count[1]++;

            count[1] += (uint32_t) (dataSize >> 29);

            auto spaceLeft = (size_t) 64 - (size_t) bufferPos;
            size_t i = 0;

            if (dataSize >= spaceLeft)
            {
                memcpy (buffer + bufferPos, data, spaceLeft);
                transform (buffer);

                for (i = spaceLeft; i + 64 <= dataSize; i += 64)
                    transform (static_cast<const char*> (data) + i);

                bufferPos = 0;
            }

            memcpy (buffer + bufferPos, static_cast<const char*> (data) + i, dataSize - i);
        */
    }
    
    pub fn transform(&mut self, buffer_to_transform: *const c_void)  {
        
        todo!();
        /*
            auto a = state[0];
            auto b = state[1];
            auto c = state[2];
            auto d = state[3];

            uint32_t x[16];
            copyWithEndiannessConversion (x, bufferToTransform, 64);

            enum Constants
            {
                S11 = 7, S12 = 12, S13 = 17, S14 = 22, S21 = 5, S22 = 9,  S23 = 14, S24 = 20,
                S31 = 4, S32 = 11, S33 = 16, S34 = 23, S41 = 6, S42 = 10, S43 = 15, S44 = 21
            };

            FF (a, b, c, d, x[ 0], S11, 0xd76aa478);     FF (d, a, b, c, x[ 1], S12, 0xe8c7b756);
            FF (c, d, a, b, x[ 2], S13, 0x242070db);     FF (b, c, d, a, x[ 3], S14, 0xc1bdceee);
            FF (a, b, c, d, x[ 4], S11, 0xf57c0faf);     FF (d, a, b, c, x[ 5], S12, 0x4787c62a);
            FF (c, d, a, b, x[ 6], S13, 0xa8304613);     FF (b, c, d, a, x[ 7], S14, 0xfd469501);
            FF (a, b, c, d, x[ 8], S11, 0x698098d8);     FF (d, a, b, c, x[ 9], S12, 0x8b44f7af);
            FF (c, d, a, b, x[10], S13, 0xffff5bb1);     FF (b, c, d, a, x[11], S14, 0x895cd7be);
            FF (a, b, c, d, x[12], S11, 0x6b901122);     FF (d, a, b, c, x[13], S12, 0xfd987193);
            FF (c, d, a, b, x[14], S13, 0xa679438e);     FF (b, c, d, a, x[15], S14, 0x49b40821);

            GG (a, b, c, d, x[ 1], S21, 0xf61e2562);     GG (d, a, b, c, x[ 6], S22, 0xc040b340);
            GG (c, d, a, b, x[11], S23, 0x265e5a51);     GG (b, c, d, a, x[ 0], S24, 0xe9b6c7aa);
            GG (a, b, c, d, x[ 5], S21, 0xd62f105d);     GG (d, a, b, c, x[10], S22, 0x02441453);
            GG (c, d, a, b, x[15], S23, 0xd8a1e681);     GG (b, c, d, a, x[ 4], S24, 0xe7d3fbc8);
            GG (a, b, c, d, x[ 9], S21, 0x21e1cde6);     GG (d, a, b, c, x[14], S22, 0xc33707d6);
            GG (c, d, a, b, x[ 3], S23, 0xf4d50d87);     GG (b, c, d, a, x[ 8], S24, 0x455a14ed);
            GG (a, b, c, d, x[13], S21, 0xa9e3e905);     GG (d, a, b, c, x[ 2], S22, 0xfcefa3f8);
            GG (c, d, a, b, x[ 7], S23, 0x676f02d9);     GG (b, c, d, a, x[12], S24, 0x8d2a4c8a);

            HH (a, b, c, d, x[ 5], S31, 0xfffa3942);     HH (d, a, b, c, x[ 8], S32, 0x8771f681);
            HH (c, d, a, b, x[11], S33, 0x6d9d6122);     HH (b, c, d, a, x[14], S34, 0xfde5380c);
            HH (a, b, c, d, x[ 1], S31, 0xa4beea44);     HH (d, a, b, c, x[ 4], S32, 0x4bdecfa9);
            HH (c, d, a, b, x[ 7], S33, 0xf6bb4b60);     HH (b, c, d, a, x[10], S34, 0xbebfbc70);
            HH (a, b, c, d, x[13], S31, 0x289b7ec6);     HH (d, a, b, c, x[ 0], S32, 0xeaa127fa);
            HH (c, d, a, b, x[ 3], S33, 0xd4ef3085);     HH (b, c, d, a, x[ 6], S34, 0x04881d05);
            HH (a, b, c, d, x[ 9], S31, 0xd9d4d039);     HH (d, a, b, c, x[12], S32, 0xe6db99e5);
            HH (c, d, a, b, x[15], S33, 0x1fa27cf8);     HH (b, c, d, a, x[ 2], S34, 0xc4ac5665);

            II (a, b, c, d, x[ 0], S41, 0xf4292244);     II (d, a, b, c, x[ 7], S42, 0x432aff97);
            II (c, d, a, b, x[14], S43, 0xab9423a7);     II (b, c, d, a, x[ 5], S44, 0xfc93a039);
            II (a, b, c, d, x[12], S41, 0x655b59c3);     II (d, a, b, c, x[ 3], S42, 0x8f0ccc92);
            II (c, d, a, b, x[10], S43, 0xffeff47d);     II (b, c, d, a, x[ 1], S44, 0x85845dd1);
            II (a, b, c, d, x[ 8], S41, 0x6fa87e4f);     II (d, a, b, c, x[15], S42, 0xfe2ce6e0);
            II (c, d, a, b, x[ 6], S43, 0xa3014314);     II (b, c, d, a, x[13], S44, 0x4e0811a1);
            II (a, b, c, d, x[ 4], S41, 0xf7537e82);     II (d, a, b, c, x[11], S42, 0xbd3af235);
            II (c, d, a, b, x[ 2], S43, 0x2ad7d2bb);     II (b, c, d, a, x[ 9], S44, 0xeb86d391);

            state[0] += a;
            state[1] += b;
            state[2] += c;
            state[3] += d;
        */
    }
    
    pub fn finish(&mut self, result: *mut u8)  {
        
        todo!();
        /*
            uint8_t encodedLength[8];
            copyWithEndiannessConversion (encodedLength, count, 8);

            // Pad out to 56 mod 64.
            auto index = (count[0] >> 3) & 0x3f;
            auto paddingLength = (index < 56 ? 56 : 120) - index;

            uint8_t paddingBuffer[64] = { 0x80 }; // first byte is 0x80, remaining bytes are zero.

            processBlock (paddingBuffer, (size_t) paddingLength);
            processBlock (encodedLength, 8);

            copyWithEndiannessConversion (result, state, 16);
        */
    }
    
    pub fn copy_with_endianness_conversion(
        output:    *mut c_void,
        input:     *const c_void,
        num_bytes: usize)  {
        
        todo!();
        /*
            #if ALOE_LITTLE_ENDIAN
            memcpy (output, input, numBytes);
           #else
            auto dst = static_cast<uint8_t*> (output);
            auto src = static_cast<const uint8_t*> (input);

            for (size_t i = 0; i < numBytes; i += 4)
            {
                dst[i + 0] = src[i + 3];
                dst[i + 1] = src[i + 2];
                dst[i + 2] = src[i + 1];
                dst[i + 3] = src[i + 0];
            }
           #endif
        */
    }

    pub fn rotate_left(x: u32, n: u32) -> u32 {
        
        todo!();
        /*
            return (x << n) | (x >> (32 - n));
        */
    }

    pub fn f(
        x: u32,
        y: u32,
        z: u32) -> u32 {
        
        todo!();
        /*
            return (x & y) | (~x & z);
        */
    }

    pub fn g(
        x: u32,
        y: u32,
        z: u32) -> u32 {
        
        todo!();
        /*
            return (x & z) | (y & ~z);
        */
    }

    pub fn h(
        x: u32,
        y: u32,
        z: u32) -> u32 {
        
        todo!();
        /*
            return x ^ y ^ z;
        */
    }

    pub fn i(
        x: u32,
        y: u32,
        z: u32) -> u32 {
        
        todo!();
        /*
            return y ^ (x | ~z);
        */
    }

    pub fn ff(
        a:  &mut u32,
        b:  u32,
        c:  u32,
        d:  u32,
        x:  u32,
        s:  u32,
        ac: u32)  {
        
        todo!();
        /*
            a = rotateLeft (a + F (b, c, d) + x + ac, s) + b;
        */
    }

    pub fn gg(
        a:  &mut u32,
        b:  u32,
        c:  u32,
        d:  u32,
        x:  u32,
        s:  u32,
        ac: u32)  {
        
        todo!();
        /*
            a = rotateLeft (a + G (b, c, d) + x + ac, s) + b;
        */
    }

    pub fn hh(
        a:  &mut u32,
        b:  u32,
        c:  u32,
        d:  u32,
        x:  u32,
        s:  u32,
        ac: u32)  {
        
        todo!();
        /*
            a = rotateLeft (a + H (b, c, d) + x + ac, s) + b;
        */
    }

    pub fn ii(
        a:  &mut u32,
        b:  u32,
        c:  u32,
        d:  u32,
        x:  u32,
        s:  u32,
        ac: u32)  {
        
        todo!();
        /*
            a = rotateLeft (a + I (b, c, d) + x + ac, s) + b;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct MD5Tests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for MD5Tests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("MD5", UnitTestCategories::cryptography
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl MD5Tests {

    pub fn test(&mut self, 
        input:    *const u8,
        expected: *const u8)  {
        
        todo!();
        /*
            {
                MD5 hash (input, strlen (input));
                expectEquals (hash.toHexString(), String (expected));
            }

            {
                MemoryInputStream m (input, strlen (input), false);
                MD5 hash (m);
                expectEquals (hash.toHexString(), String (expected));
            }
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("MD5");

            test ("", "d41d8cd98f00b204e9800998ecf8427e");
            test ("The quick brown fox jumps over the lazy dog",  "9e107d9d372bb6826bd81d3542a419d6");
            test ("The quick brown fox jumps over the lazy dog.", "e4d909c290d0fb1ca068ffaddf22cbd0");

            expectEquals (MD5 (CharPointer_UTF8(nullptr)).toHexString(), String ("d41d8cd98f00b204e9800998ecf8427e"));
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static MD5Tests MD5UnitTests;
    */
}
