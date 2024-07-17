crate::ix!();

#[inline] pub fn bit_to_mask(bit: i32) -> u32 {
    
    todo!();
    /*
        return (uint32) 1 << (bit & 31);
    */
}

#[inline] pub fn bit_to_index(bit: i32) -> usize {
    
    todo!();
    /*
        return (size_t) (bit >> 5);
    */
}

#[inline] pub fn size_needed_to_hold(highest_bit: i32) -> usize {
    
    todo!();
    /*
        return (size_t) (highestBit >> 5) + 1;
    */
}

pub fn find_highest_set_bit(n: u32) -> i32 {
    
    todo!();
    /*
        jassert (n != 0); // (the built-in functions may not work for n = 0)

      #if ALOE_GCC || ALOE_CLANG
        return 31 - __builtin_clz (n);
      #elif ALOE_MSVC
        unsigned long highest;
        _BitScanReverse (&highest, n);
        return (int) highest;
      #else
        n |= (n >> 1);
        n |= (n >> 2);
        n |= (n >> 4);
        n |= (n >> 8);
        n |= (n >> 16);
        return countNumberOfBits (n >> 1);
      #endif
    */
}

pub fn simplegcd(
        m: *mut BigInteger,
        n: *mut BigInteger) -> BigInteger {
    
    todo!();
    /*
        while (! m->isZero())
        {
            if (n->compareAbsolute (*m) > 0)
                std::swap (m, n);

            *m -= *n;
        }

        return *n;
    */
}

