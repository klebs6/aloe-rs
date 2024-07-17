crate::ix!();

pub struct SHA256Processor {
    state:  [u32; 8], // = { 0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19 };
    length: u64,      // default = 0
}

impl SHA256Processor {

    /**
       expects 64 bytes of data
      */
    pub fn process_full_block(&mut self, data: *const c_void)  {
        
        todo!();
        /*
            const uint32_t constants[] =
            {
                0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
                0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
                0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
                0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
                0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
                0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
                0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
                0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
            };

            uint32_t block[16], s[8];
            memcpy (s, state, sizeof (s));

            auto d = static_cast<const uint8_t*> (data);

            for (auto& b : block)
            {
                b = (uint32_t (d[0]) << 24) | (uint32_t (d[1]) << 16) | (uint32_t (d[2]) << 8) | d[3];
                d += 4;
            }

            auto convolve = [&] (uint32_t i, uint32_t j)
            {
                s[(7 - i) & 7] += S1 (s[(4 - i) & 7]) + ch (s[(4 - i) & 7], s[(5 - i) & 7], s[(6 - i) & 7]) + constants[i + j]
                                     + (j != 0 ? (block[i & 15] += s1 (block[(i - 2) & 15]) + block[(i - 7) & 15] + s0 (block[(i - 15) & 15]))
                                               : block[i]);
                s[(3 - i) & 7] += s[(7 - i) & 7];
                s[(7 - i) & 7] += S0 (s[(0 - i) & 7]) + maj (s[(0 - i) & 7], s[(1 - i) & 7], s[(2 - i) & 7]);
            };

            for (uint32_t j = 0; j < 64; j += 16)
                for (uint32_t i = 0; i < 16; ++i)
                    convolve (i, j);

            for (int i = 0; i < 8; ++i)
                state[i] += s[i];

            length += 64;
        */
    }
    
    pub fn process_final_block(&mut self, 
        data:      *const c_void,
        num_bytes: u32)  {
        
        todo!();
        /*
            jassert (numBytes < 64);

            length += numBytes;
            length *= 8; // (the length is stored as a count of bits, not bytes)

            uint8_t finalBlocks[128];

            memcpy (finalBlocks, data, numBytes);
            finalBlocks[numBytes++] = 128; // append a '1' bit

            while (numBytes != 56 && numBytes < 64 + 56)
                finalBlocks[numBytes++] = 0; // pad with zeros..

            for (int i = 8; --i >= 0;)
                finalBlocks[numBytes++] = (uint8_t) (length >> (i * 8)); // append the length.

            jassert (numBytes == 64 || numBytes == 128);

            processFullBlock (finalBlocks);

            if (numBytes > 64)
                processFullBlock (finalBlocks + 64);
        */
    }
    
    pub fn copy_result(&self, result: *mut u8)  {
        
        todo!();
        /*
            for (auto s : state)
            {
                *result++ = (uint8_t) (s >> 24);
                *result++ = (uint8_t) (s >> 16);
                *result++ = (uint8_t) (s >> 8);
                *result++ = (uint8_t) s;
            }
        */
    }
    
    pub fn process_stream(&mut self, 
        input:             &mut dyn Read,
        num_bytes_to_read: i64,
        result:            *mut u8)  {
        
        todo!();
        /*
            if (numBytesToRead < 0)
                numBytesToRead = std::numeric_limits<int64_t>::max();

            for (;;)
            {
                uint8_t buffer[64];
                auto bytesRead = input.read (buffer, (int) jmin (numBytesToRead, (int64_t) sizeof (buffer)));

                if (bytesRead < (int) sizeof (buffer))
                {
                    processFinalBlock (buffer, (unsigned int) bytesRead);
                    break;
                }

                numBytesToRead -= (int64_t) sizeof (buffer);
                processFullBlock (buffer);
            }

            copyResult (result);
        */
    }

    pub fn rotate(x: u32, y: u32) -> u32 {
        
        todo!();
        /*
            return (x >> y) | (x << (32 - y));
        */
    }

    pub fn ch(
        x: u32,
        y: u32,
        z: u32) -> u32 {
        
        todo!();
        /*
            return z ^ ((y ^ z) & x);
        */
    }

    pub fn maj(
        x: u32,
        y: u32,
        z: u32) -> u32 {
        
        todo!();
        /*
            return y ^ ((y ^ z) & (x ^ y));
        */
    }

    pub fn s0(x: u32) -> u32 {
        
        todo!();
        /*
            return rotate (x, 7)  ^ rotate (x, 18) ^ (x >> 3);
        */
    }

    pub fn s1(x: u32) -> u32 {
        
        todo!();
        /*
            return rotate (x, 17) ^ rotate (x, 19) ^ (x >> 10);
        */
    }

    //S0
    pub fn capital_s0(x: u32) -> u32 {
        
        todo!();
        /*
            return rotate (x, 2)  ^ rotate (x, 13) ^ rotate (x, 22);
        */
    }

    //S1
    pub fn capital_s1(x: u32) -> u32 {
        
        todo!();
        /*
            return rotate (x, 6)  ^ rotate (x, 11) ^ rotate (x, 25);
        */
    }
}

