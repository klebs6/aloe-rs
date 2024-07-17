crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_ByteOrder.h]

/**
  | Contains static methods for converting
  | the byte order between different endiannesses.
  | 
  | @tags{Core}
  |
  */
pub struct ByteOrder {

}

impl ByteOrder {

    /**
      | Swaps the byte order of a signed or unsigned
      | integer if the CPU is big-endian
      |
      */
    pub fn swap_if_big_endian<Type>(value: Type) -> Type {
    
        todo!();
        /*
            #if ALOE_LITTLE_ENDIAN
            return value;
           #else
            return swap (value);
           #endif
        */
    }

    /**
      | Swaps the byte order of a signed or unsigned
      | integer if the CPU is little-endian
      |
      */
    pub fn swap_if_little_endian<Type>(value: Type) -> Type {
    
        todo!();
        /*
            #if ALOE_LITTLE_ENDIAN
            return swap (value);
           #else
            return value;
           #endif
        */
    }

    /**
      | Returns true if the current CPU is big-endian.
      |
      */
    pub fn is_big_endian() -> bool {
        
        todo!();
        /*
            #if ALOE_LITTLE_ENDIAN
            return false;
           #else
            return true;
           #endif
        */
    }
    
    /**
      | Swaps the upper and lower bytes of a 16-bit
      | integer.
      |
      */
    #[inline] pub fn swap_u16(&mut self, v: u16) -> u16 {
        
        todo!();
        /*
            return static_cast<uint16> ((v << 8) | (v >> 8));
        */
    }
    
    /**
      | Swaps the upper and lower bytes of a 16-bit
      | integer.
      |
      */
    #[inline] pub fn swap_i16(&mut self, v: i16) -> i16 {
        
        todo!();
        /*
            return static_cast<int16> (swap (static_cast<uint16> (v)));
        */
    }
    
    /**
      | Reverses the order of the 4 bytes in a
      | 32-bit integer.
      |
      */
    #[inline] pub fn swap_i32(&mut self, v: i32) -> i32 {
        
        todo!();
        /*
            return static_cast<int32> (swap (static_cast<uint32> (v)));
        */
    }
    
    /**
      | Reverses the order of the 8 bytes in a
      | 64-bit integer.
      |
      */
    #[inline] pub fn swap_i64(&mut self, v: i64) -> i64 {
        
        todo!();
        /*
            return static_cast<int64> (swap (static_cast<uint64> (v)));
        */
    }
    
    /**
      | Returns a garbled float which has the
      | reverse byte-order of the original.
      |
      */
    #[inline] pub fn swap_f32(&mut self, v: f32) -> f32 {
        
        todo!();
        /*
            union { uint32 asUInt; float asFloat;  } n; n.asFloat = v; n.asUInt = swap (n.asUInt); return n.asFloat;
        */
    }
    
    /**
      | Returns a garbled double which has the
      | reverse byte-order of the original.
      |
      */
    #[inline] pub fn swap_f64(&mut self, v: f64) -> f64 {
        
        todo!();
        /*
            union { uint64 asUInt; double asFloat; } n; n.asFloat = v; n.asUInt = swap (n.asUInt); return n.asFloat;
        */
    }
    
    /**
      | Reverses the order of the 4 bytes in a
      | 32-bit integer.
      |
      */
    #[inline] pub fn swap_u32(&mut self, n: u32) -> u32 {
        
        todo!();
        /*
            #if ALOE_MAC || ALOE_IOS
        return OSSwapInt32 (n);
       #elif (ALOE_GCC  || ALOE_CLANG) && ALOE_INTEL && ! ALOE_NO_INLINE_ASM
        asm("bswap %%eax" : "=a"(n) : "a"(n));
        return n;
       #elif ALOE_MSVC
        return _byteswap_ulong (n);
       #elif ALOE_ANDROID
        return bswap_32 (n);
       #else
        return (n << 24) | (n >> 24) | ((n & 0xff00) << 8) | ((n & 0xff0000) >> 8);
       #endif
        */
    }
    
    /**
      | Reverses the order of the 8 bytes in a
      | 64-bit integer.
      |
      */
    #[inline] pub fn swap_u64(&mut self, value: u64) -> u64 {
        
        todo!();
        /*
            #if ALOE_MAC || ALOE_IOS
        return OSSwapInt64 (value);
       #elif ALOE_MSVC
        return _byteswap_uint64 (value);
       #else
        return (((uint64) swap ((uint32) value)) << 32) | swap ((uint32) (value >> 32));
       #endif
        */
    }
    
    /**
      | Constructs a 16-bit integer from its
      | constituent bytes, in order of significance.
      |
      */
    #[inline] pub fn make_int16(b0: u8, b1: u8) -> u16 {
        
        todo!();
        /*
            return static_cast<uint16> (static_cast<uint16> (b0) | (static_cast<uint16> (b1) << 8));
        */
    }
    
    /**
      | Constructs a 32-bit integer from its
      | constituent bytes, in order of significance.
      |
      */
    #[inline] pub const fn make_int32(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
        (b0 as u32) | ((b1 as u32) << 8) | ((b2 as u32) << 16) | ((b3 as u32) << 24)
    }
    
    /**
      | Constructs a 64-bit integer from its
      | constituent bytes, in order of significance.
      |
      */
    #[inline] pub fn make_int64(
        b0: u8,
        b1: u8,
        b2: u8,
        b3: u8,
        b4: u8,
        b5: u8,
        b6: u8,
        b7: u8

    ) -> u64 {
        
        todo!();
        /*
            return static_cast<uint64> (b0)        | (static_cast<uint64> (b1) << 8)  | (static_cast<uint64> (b2) << 16) | (static_cast<uint64> (b3) << 24)
            | (static_cast<uint64> (b4) << 32) | (static_cast<uint64> (b5) << 40) | (static_cast<uint64> (b6) << 48) | (static_cast<uint64> (b7) << 56);
        */
    }
    
    /**
      | Turns 2 bytes into a little-endian integer.
      |
      */
    #[inline] pub fn little_endian_short(&mut self, bytes: *const c_void) -> u16 {
        
        todo!();
        /*
            return makeInt (static_cast<const uint8*> (bytes)[0], static_cast<const uint8*> (bytes)[1]);
        */
    }
    
    /**
      | Turns 4 bytes into a little-endian integer.
      |
      */
    #[inline] pub fn little_endian_int(&mut self, bytes: *const c_void) -> u32 {
        
        todo!();
        /*
            return makeInt (static_cast<const uint8*> (bytes)[0], static_cast<const uint8*> (bytes)[1],
                                                                                                                static_cast<const uint8*> (bytes)[2], static_cast<const uint8*> (bytes)[3]);
        */
    }
    
    /**
      | Turns 8 bytes into a little-endian integer.
      |
      */
    #[inline] pub fn little_endian_int64(&mut self, bytes: *const c_void) -> u64 {
        
        todo!();
        /*
            return makeInt (static_cast<const uint8*> (bytes)[0], static_cast<const uint8*> (bytes)[1],
                                                                                                                static_cast<const uint8*> (bytes)[2], static_cast<const uint8*> (bytes)[3],
                                                                                                                static_cast<const uint8*> (bytes)[4], static_cast<const uint8*> (bytes)[5],
                                                                                                                static_cast<const uint8*> (bytes)[6], static_cast<const uint8*> (bytes)[7]);
        */
    }
    
    /**
      | Turns 2 bytes into a big-endian integer.
      |
      */
    #[inline] pub fn big_endian_short(&mut self, bytes: *const c_void) -> u16 {
        
        todo!();
        /*
            return makeInt (static_cast<const uint8*> (bytes)[1], static_cast<const uint8*> (bytes)[0]);
        */
    }
    
    /**
      | Turns 4 bytes into a big-endian integer.
      |
      */
    #[inline] pub fn big_endian_int(&mut self, bytes: *const c_void) -> u32 {
        
        todo!();
        /*
            return makeInt (static_cast<const uint8*> (bytes)[3], static_cast<const uint8*> (bytes)[2],
                                                                                                                static_cast<const uint8*> (bytes)[1], static_cast<const uint8*> (bytes)[0]);
        */
    }
    
    /**
      | Turns 8 bytes into a big-endian integer.
      |
      */
    #[inline] pub fn big_endian_int64(&mut self, bytes: *const c_void) -> u64 {
        
        todo!();
        /*
            return makeInt (static_cast<const uint8*> (bytes)[7], static_cast<const uint8*> (bytes)[6],
                                                                                                                static_cast<const uint8*> (bytes)[5], static_cast<const uint8*> (bytes)[4],
                                                                                                                static_cast<const uint8*> (bytes)[3], static_cast<const uint8*> (bytes)[2],
                                                                                                                static_cast<const uint8*> (bytes)[1], static_cast<const uint8*> (bytes)[0]);
        */
    }
    
    /**
      | Converts 3 little-endian bytes into
      | a signed 24-bit value (which is sign-extended
      | to 32 bits).
      |
      */
    #[inline] pub fn little_endian_24bit(&mut self, bytes: *const c_void) -> i32 {
        
        todo!();
        /*
            return (int32) ((((uint32) static_cast<const int8*> (bytes)[2]) << 16) | (((uint32) static_cast<const uint8*> (bytes)[1]) << 8) | ((uint32) static_cast<const uint8*> (bytes)[0]));
        */
    }
    
    /**
      | Converts 3 big-endian bytes into a signed
      | 24-bit value (which is sign-extended
      | to 32 bits).
      |
      */
    #[inline] pub fn big_endian_24bit(&mut self, bytes: *const c_void) -> i32 {
        
        todo!();
        /*
            return (int32) ((((uint32) static_cast<const int8*> (bytes)[0]) << 16) | (((uint32) static_cast<const uint8*> (bytes)[1]) << 8) | ((uint32) static_cast<const uint8*> (bytes)[2]));
        */
    }
    
    /**
      | Copies a 24-bit number to 3 little-endian
      | bytes.
      |
      */
    #[inline] pub fn little_endian_24bit_to_chars(&mut self, 
        value:      i32,
        dest_bytes: *mut c_void)  {
        
        todo!();
        /*
            static_cast<uint8*> (destBytes)[0] = (uint8) value;         static_cast<uint8*> (destBytes)[1] = (uint8) (value >> 8); static_cast<uint8*> (destBytes)[2] = (uint8) (value >> 16);
        */
    }
    
    /**
      | Copies a 24-bit number to 3 big-endian
      | bytes.
      |
      */
    #[inline] pub fn big_endian_24bit_to_chars(&mut self, 
        value:      i32,
        dest_bytes: *mut c_void)  {
        
        todo!();
        /*
            static_cast<uint8*> (destBytes)[0] = (uint8) (value >> 16); static_cast<uint8*> (destBytes)[1] = (uint8) (value >> 8); static_cast<uint8*> (destBytes)[2] = (uint8) value;
        */
    }
}
