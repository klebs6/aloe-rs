crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/maths/aloe_BigInteger.h]

pub const BIG_INTEGER_NUM_PREALLOCATED_INTS: usize = 4;

/**
  | An arbitrarily large integer class.
  | 
  | A BigInteger can be used in a similar
  | way to a normal integer, but has no size
  | limit (except for memory and performance
  | constraints).
  | 
  | Negative values are possible, but the
  | value isn't stored as 2s-complement,
  | so be careful if you use negative values
  | and look at the values of individual
  | bits.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct BigInteger {
    heap_allocation: HeapBlock<u32>,
    preallocated:    [u32; BIG_INTEGER_NUM_PREALLOCATED_INTS],
    allocated_size:  usize,
    highest_bit:     i32, // default = -1
    negative:        bool, // default = false
}

impl Default for BigInteger {
    
    /**
      | Creates an empty BigInteger
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/maths/aloe_BigInteger.cpp]

impl Index<i32> for BigInteger {

    type Output = bool;
    
    /**
      | Returns the value of a specified bit
      | in the number.
      | 
      | If the index is out-of-range, the result
      | will be false.
      |
      */
    #[inline] fn index(&self, bit: i32) -> &Self::Output {
        todo!();
        /*
            return bit <= highestBit && bit >= 0
                 && ((getValues() [bitToIndex (bit)] & bitToMask (bit)) != 0);
        */
    }
}

lazy_static!{
    /*
    bool BigInteger::operator== (const BigInteger& other) const     { return compare (other) == 0; }
    bool BigInteger::operator!= (const BigInteger& other) const     { return compare (other) != 0; }
    bool BigInteger::operator<  (const BigInteger& other) const     { return compare (other) <  0; }
    bool BigInteger::operator<= (const BigInteger& other) const     { return compare (other) <= 0; }
    bool BigInteger::operator>  (const BigInteger& other) const     { return compare (other) >  0; }
    bool BigInteger::operator>= (const BigInteger& other) const     { return compare (other) >= 0; }
    */
}

impl fmt::Display for BigInteger {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let base      = 10;
        let min_chars = 1;

        let decimal_representation = self.to_string(base, Some(min_chars));
        
        // Use the `write!` macro to write to the formatter
        write!(f, "{}", decimal_representation)
    }
}

impl BigInteger {
    
    pub fn new() -> Self {
    
        todo!();
        /*
        : allocated_size(numPreallocatedInts),

            for (int i = 0; i < numPreallocatedInts; ++i)
            preallocated[i] = 0;
        */
    }
    
    /**
      | Creates a BigInteger containing an
      | integer value in its low bits.
      | 
      | The low 32 bits of the number are initialised
      | with the absolute value passed in, and
      | its sign is set to reflect the sign of
      | the number.
      |
      */
    pub fn new_from_i32(value: i32) -> Self {
    
        todo!();
        /*


            : allocatedSize (numPreallocatedInts),
          highestBit (31),
          negative (value < 0)

        preallocated[0] = (uint32) std::abs (value);

        for (int i = 1; i < numPreallocatedInts; ++i)
            preallocated[i] = 0;

        highestBit = getHighestBit();
        */
    }
    
    /**
      | Creates a BigInteger containing an
      | integer value in its low bits.
      | 
      | The low 32 bits of the number are initialised
      | with this value.
      |
      */
    pub fn new_from_u32(value: u32) -> Self {
    
        todo!();
        /*


            : allocatedSize (numPreallocatedInts),
          highestBit (31)

        preallocated[0] = value;

        for (int i = 1; i < numPreallocatedInts; ++i)
            preallocated[i] = 0;

        highestBit = getHighestBit();
        */
    }
    
    /**
      | Creates a BigInteger containing an
      | integer value in its low bits.
      | 
      | The low 64 bits of the number are initialised
      | with the absolute value passed in, and
      | its sign is set to reflect the sign of
      | the number.
      |
      */
    pub fn new_from_i64(value: i64) -> Self {
    
        todo!();
        /*


            : allocatedSize (numPreallocatedInts),
          highestBit (63),
          negative (value < 0)

        if (value < 0)
            value = -value;

        preallocated[0] = (uint32) value;
        preallocated[1] = (uint32) (value >> 32);

        for (int i = 2; i < numPreallocatedInts; ++i)
            preallocated[i] = 0;

        highestBit = getHighestBit();
        */
    }
    
    pub fn new_from_ref(other: &BigInteger) -> Self {
    
        todo!();
        /*


            : allocatedSize (other.allocatedSize),
          highestBit (other.getHighestBit()),
          negative (other.negative)

        if (allocatedSize > numPreallocatedInts)
            heapAllocation.malloc (allocatedSize);

        memcpy (getValues(), other.getValues(), sizeof (uint32) * allocatedSize);
        */
    }
    
    pub fn new_from_other(other: BigInteger) -> Self {
    
        todo!();
        /*


            : heapAllocation (std::move (other.heapAllocation)),
          allocatedSize (other.allocatedSize),
          highestBit (other.highestBit),
          negative (other.negative)

        memcpy (preallocated, other.preallocated, sizeof (preallocated));
        */
    }
    
    pub fn assign_from(&mut self, other: BigInteger) -> &mut BigInteger {
        
        todo!();
        /*
            heapAllocation = std::move (other.heapAllocation);
        memcpy (preallocated, other.preallocated, sizeof (preallocated));
        allocatedSize = other.allocatedSize;
        highestBit = other.highestBit;
        negative = other.negative;
        return *this;
        */
    }
    
    /**
      | Swaps the internal contents of this
      | with another object.
      |
      */
    pub fn swap_with(&mut self, other: &mut BigInteger)  {
        
        todo!();
        /*
            for (int i = 0; i < numPreallocatedInts; ++i)
            std::swap (preallocated[i], other.preallocated[i]);

        heapAllocation.swapWith (other.heapAllocation);
        std::swap (allocatedSize, other.allocatedSize);
        std::swap (highestBit, other.highestBit);
        std::swap (negative, other.negative);
        */
    }
    
    pub fn assign_from_ref(&mut self, other: &BigInteger) -> &mut BigInteger {
        
        todo!();
        /*
            if (this != &other)
        {
            highestBit = other.getHighestBit();
            auto newAllocatedSize = (size_t) jmax ((size_t) numPreallocatedInts, sizeNeededToHold (highestBit));

            if (newAllocatedSize <= numPreallocatedInts)
                heapAllocation.free();
            else if (newAllocatedSize != allocatedSize)
                heapAllocation.malloc (newAllocatedSize);

            allocatedSize = newAllocatedSize;

            memcpy (getValues(), other.getValues(), sizeof (uint32) * allocatedSize);
            negative = other.negative;
        }

        return *this;
        */
    }
    
    pub fn get_values(&self) -> *mut u32 {
        
        todo!();
        /*
            jassert (heapAllocation != nullptr || allocatedSize <= numPreallocatedInts);

        return heapAllocation != nullptr ? heapAllocation
                                         : const_cast<uint32*> (preallocated);
        */
    }
    
    pub fn ensure_size(&mut self, num_vals: usize) -> *mut u32 {
        
        todo!();
        /*
            if (numVals > allocatedSize)
        {
            auto oldSize = allocatedSize;
            allocatedSize = ((numVals + 2) * 3) / 2;

            if (heapAllocation == nullptr)
            {
                heapAllocation.calloc (allocatedSize);
                memcpy (heapAllocation, preallocated, sizeof (uint32) * numPreallocatedInts);
            }
            else
            {
                heapAllocation.realloc (allocatedSize);

                for (auto* values = getValues(); oldSize < allocatedSize; ++oldSize)
                    values[oldSize] = 0;
            }
        }

        return getValues();
        */
    }
    
    /**
      | Attempts to get the lowest 32 bits of
      | the value as an integer.
      | 
      | If the value is bigger than the integer
      | limits, this will return only the lower
      | bits.
      |
      */
    pub fn to_integer(&self) -> i32 {
        
        todo!();
        /*
            auto n = (int) (getValues()[0] & 0x7fffffff);
        return negative ? -n : n;
        */
    }
    
    /**
      | Attempts to get the lowest 64 bits of
      | the value as an integer.
      | 
      | If the value is bigger than the integer
      | limits, this will return only the lower
      | bits.
      |
      */
    pub fn to_int64(&self) -> i64 {
        
        todo!();
        /*
            auto* values = getValues();
        auto n = (((int64) (values[1] & 0x7fffffff)) << 32) | values[0];
        return negative ? -n : n;
        */
    }
    
    /**
      | Returns a range of bits as a new BigInteger.
      | 
      | e.g. getBitRangeAsInt (0, 64) would
      | return the lowest 64 bits. @see getBitRangeAsInt
      |
      */
    pub fn get_bit_range(&self, 
        start_bit: i32,
        num_bits:  i32) -> BigInteger {
        
        todo!();
        /*
            BigInteger r;
        numBits = jmax (0, jmin (numBits, getHighestBit() + 1 - startBit));
        auto* destValues = r.ensureSize (sizeNeededToHold (numBits));
        r.highestBit = numBits;

        for (int i = 0; numBits > 0;)
        {
            destValues[i++] = getBitRangeAsInt (startBit, (int) jmin (32, numBits));
            numBits -= 32;
            startBit += 32;
        }

        r.highestBit = r.getHighestBit();
        return r;
        */
    }
    
    /**
      | Returns a range of bits as an integer
      | value.
      | 
      | e.g. getBitRangeAsInt (0, 32) would
      | return the lowest 32 bits.
      | 
      | Asking for more than 32 bits isn't allowed
      | (obviously) - for that, use getBitRange().
      |
      */
    pub fn get_bit_range_as_int(&self, 
        start_bit: i32,
        num_bits:  i32) -> u32 {
        
        todo!();
        /*
            if (numBits > 32)
        {
            jassertfalse;  // use getBitRange() if you need more than 32 bits..
            numBits = 32;
        }

        numBits = jmin (numBits, highestBit + 1 - startBit);

        if (numBits <= 0)
            return 0;

        auto pos = bitToIndex (startBit);
        auto offset = startBit & 31;
        auto endSpace = 32 - numBits;
        auto* values = getValues();

        auto n = ((uint32) values [pos]) >> offset;

        if (offset > endSpace)
            n |= ((uint32) values [pos + 1]) << (32 - offset);

        return n & (((uint32) 0xffffffff) >> endSpace);
        */
    }
    
    /**
      | Sets a range of bits to an integer value.
      | 
      | Copies the given integer onto a range
      | of bits, starting at startBit, and using
      | up to numBits of the available bits.
      |
      */
    pub fn set_bit_range_as_int(&mut self, 
        start_bit:    i32,
        num_bits:     i32,
        value_to_set: u32)  {
        
        todo!();
        /*
            if (numBits > 32)
        {
            jassertfalse;
            numBits = 32;
        }

        for (int i = 0; i < numBits; ++i)
        {
            setBit (startBit + i, (valueToSet & 1) != 0);
            valueToSet >>= 1;
        }
        */
    }
    
    /**
      | Resets the value to 0.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            heapAllocation.free();
        allocatedSize = numPreallocatedInts;
        highestBit = -1;
        negative = false;

        for (int i = 0; i < numPreallocatedInts; ++i)
            preallocated[i] = 0;
        */
    }
    
    /**
      | Sets a specified bit to 1.
      |
      */
    pub fn set_bit(&mut self, bit: i32)  {
        
        todo!();
        /*
            if (bit >= 0)
        {
            if (bit > highestBit)
            {
                ensureSize (sizeNeededToHold (bit));
                highestBit = bit;
            }

            getValues() [bitToIndex (bit)] |= bitToMask (bit);
        }
        */
    }
    
    /**
      | Sets or clears a specified bit.
      |
      */
    pub fn set_or_clear_bit(
        &mut self, 
        bit:           i32,
        should_be_set: bool
    )
    {
        todo!();
        /*
            if (shouldBeSet)
            setBit (bit);
        else
            clearBit (bit);
        */
    }
    
    /**
      | Clears a particular bit in the number.
      |
      */
    pub fn clear_bit(&mut self, bit: i32)  {
        
        todo!();
        /*
            if (bit >= 0 && bit <= highestBit)
        {
            getValues() [bitToIndex (bit)] &= ~bitToMask (bit);

            if (bit == highestBit)
                highestBit = getHighestBit();
        }
        */
    }
    
    /**
      | Sets a range of bits to be either on or
      | off.
      | 
      | -----------
      | @param startBit
      | 
      | the first bit to change
      | ----------
      | @param numBits
      | 
      | the number of bits to change
      | ----------
      | @param shouldBeSet
      | 
      | whether to turn these bits on or off
      |
      */
    pub fn set_range(&mut self, 
        start_bit:     i32,
        num_bits:      i32,
        should_be_set: bool)  {
        
        todo!();
        /*
            while (--numBits >= 0)
            setBit (startBit++, shouldBeSet);
        */
    }
    
    /**
      | Inserts a bit an a given position, shifting
      | up any bits above it.
      |
      */
    pub fn insert_bit(&mut self, 
        bit:           i32,
        should_be_set: bool)  {
        
        todo!();
        /*
            if (bit >= 0)
            shiftBits (1, bit);

        setBit (bit, shouldBeSet);
        */
    }
    
    /**
      | Returns true if no bits are set.
      |
      */
    pub fn is_zero(&self) -> bool {
        
        todo!();
        /*
            return getHighestBit() < 0;
        */
    }
    
    /**
      | Returns true if the value is 1.
      |
      */
    pub fn is_one(&self) -> bool {
        
        todo!();
        /*
            return getHighestBit() == 0 && ! negative;
        */
    }
    
    /**
      | Returns true if the value is less than
      | zero. @see setNegative, negate
      |
      */
    pub fn is_negative(&self) -> bool {
        
        todo!();
        /*
            return negative && ! isZero();
        */
    }
    
    /**
      | Changes the sign of the number to be positive
      | or negative. @see isNegative, negate
      |
      */
    pub fn set_negative(&mut self, neg: bool)  {
        
        todo!();
        /*
            negative = neg;
        */
    }
    
    /**
      | Inverts the sign of the number. @see
      | isNegative, setNegative
      |
      */
    pub fn negate(&mut self)  {
        
        todo!();
        /*
            negative = (! negative) && ! isZero();
        */
    }
    
    /**
      | Returns the total number of set bits
      | in the value.
      |
      */
    pub fn count_number_of_set_bits(&self) -> i32 {
        
        todo!();
        /*
            int total = 0;
        auto* values = getValues();

        for (int i = (int) sizeNeededToHold (highestBit); --i >= 0;)
            total += countNumberOfBits (values[i]);

        return total;
        */
    }
    
    /**
      | Returns the index of the highest set
      | bit in the number.
      | 
      | If the value is zero, this will return
      | -1.
      |
      */
    pub fn get_highest_bit(&self) -> i32 {
        
        todo!();
        /*
            auto* values = getValues();

        for (int i = (int) bitToIndex (highestBit); i >= 0; --i)
            if (uint32 n = values[i])
                return findHighestSetBit (n) + (i << 5);

        return -1;
        */
    }
    
    /**
      | Looks for the index of the next set bit
      | after a given starting point.
      | 
      | This searches from startIndex (inclusive)
      | upwards for the first set bit, and returns
      | its index. If no set bits are found, it
      | returns -1.
      |
      */
    pub fn find_next_set_bit(&self, i: i32) -> i32 {
        
        todo!();
        /*
            auto* values = getValues();

        for (; i <= highestBit; ++i)
            if ((values [bitToIndex (i)] & bitToMask (i)) != 0)
                return i;

        return -1;
        */
    }
    
    /**
      | Looks for the index of the next clear
      | bit after a given starting point.
      | 
      | This searches from startIndex (inclusive)
      | upwards for the first clear bit, and
      | returns its index.
      |
      */
    pub fn find_next_clear_bit(&self, i: i32) -> i32 {
        
        todo!();
        /*
            auto* values = getValues();

        for (; i <= highestBit; ++i)
            if ((values [bitToIndex (i)] & bitToMask (i)) == 0)
                break;

        return i;
        */
    }
    
    /**
      | Divides this value by another one and
      | returns the remainder.
      | 
      | This number is divided by other, leaving
      | the quotient in this number, with the
      | remainder being copied to the other
      | BigInteger passed in.
      |
      */
    pub fn divide_by(&mut self, 
        divisor:   &BigInteger,
        remainder: &mut BigInteger)  {
        
        todo!();
        /*
            if (this == &divisor)
            return divideBy (BigInteger (divisor), remainder);

        jassert (this != &remainder); // (can't handle passing itself in to get the remainder)

        auto divHB = divisor.getHighestBit();
        auto ourHB = getHighestBit();

        if (divHB < 0 || ourHB < 0)
        {
            // division by zero
            remainder.clear();
            clear();
        }
        else
        {
            auto wasNegative = isNegative();

            swapWith (remainder);
            remainder.setNegative (false);
            clear();

            BigInteger temp (divisor);
            temp.setNegative (false);

            auto leftShift = ourHB - divHB;
            temp <<= leftShift;

            while (leftShift >= 0)
            {
                if (remainder.compareAbsolute (temp) >= 0)
                {
                    remainder -= temp;
                    setBit (leftShift);
                }

                if (--leftShift >= 0)
                    temp >>= 1;
            }

            negative = wasNegative ^ divisor.isNegative();
            remainder.setNegative (wasNegative);
        }
        */
    }
    
    /**
      | Does a signed comparison of two BigIntegers.
      | 
      | Return values are:
      | 
      | - 0 if the numbers are the same
      | 
      | - < 0 if this number is smaller than the
      | other
      | 
      | - > 0 if this number is bigger than the
      | other
      |
      */
    pub fn compare(&self, other: &BigInteger) -> i32 {
        
        todo!();
        /*
            auto isNeg = isNegative();

        if (isNeg == other.isNegative())
        {
            auto absComp = compareAbsolute (other);
            return isNeg ? -absComp : absComp;
        }

        return isNeg ? -1 : 1;
        */
    }
    
    /**
      | Compares the magnitudes of two BigIntegers,
      | ignoring their signs.
      | 
      | Return values are:
      | 
      | - 0 if the numbers are the same
      | 
      | - < 0 if this number is smaller than the
      | other
      | 
      | - > 0 if this number is bigger than the
      | other
      |
      */
    pub fn compare_absolute(&self, other: &BigInteger) -> i32 {
        
        todo!();
        /*
            auto h1 = getHighestBit();
        auto h2 = other.getHighestBit();

        if (h1 > h2) return 1;
        if (h1 < h2) return -1;

        auto* values = getValues();
        auto* otherValues = other.getValues();

        for (int i = (int) bitToIndex (h1); i >= 0; --i)
            if (values[i] != otherValues[i])
                return values[i] > otherValues[i] ? 1 : -1;

        return 0;
        */
    }
    
    pub fn shift_left(&mut self, 
        bits:      i32,
        start_bit: i32)  {
        
        todo!();
        /*
            if (startBit > 0)
        {
            for (int i = highestBit; i >= startBit; --i)
                setBit (i + bits, (*this) [i]);

            while (--bits >= 0)
                clearBit (bits + startBit);
        }
        else
        {
            auto* values = ensureSize (sizeNeededToHold (highestBit + bits));
            auto wordsToMove = bitToIndex (bits);
            auto numOriginalInts = bitToIndex (highestBit);
            highestBit += bits;

            if (wordsToMove > 0)
            {
                for (int i = (int) numOriginalInts; i >= 0; --i)
                    values[(size_t) i + wordsToMove] = values[i];

                for (size_t j = 0; j < wordsToMove; ++j)
                    values[j] = 0;

                bits &= 31;
            }

            if (bits != 0)
            {
                auto invBits = 32 - bits;

                for (size_t i = bitToIndex (highestBit); i > wordsToMove; --i)
                    values[i] = (values[i] << bits) | (values[i - 1] >> invBits);

                values[wordsToMove] = values[wordsToMove] << bits;
            }

            highestBit = getHighestBit();
        }
        */
    }
    
    pub fn shift_right(&mut self, 
        bits:      i32,
        start_bit: i32)  {
        
        todo!();
        /*
            if (startBit > 0)
        {
            for (int i = startBit; i <= highestBit; ++i)
                setBit (i, (*this) [i + bits]);

            highestBit = getHighestBit();
        }
        else
        {
            if (bits > highestBit)
            {
                clear();
            }
            else
            {
                auto wordsToMove = bitToIndex (bits);
                auto top = 1 + bitToIndex (highestBit) - wordsToMove;
                highestBit -= bits;
                auto* values = getValues();

                if (wordsToMove > 0)
                {
                    for (size_t i = 0; i < top; ++i)
                        values[i] = values[i + wordsToMove];

                    for (size_t i = 0; i < wordsToMove; ++i)
                        values[top + i] = 0;

                    bits &= 31;
                }

                if (bits != 0)
                {
                    auto invBits = 32 - bits;
                    --top;

                    for (size_t i = 0; i < top; ++i)
                        values[i] = (values[i] >> bits) | (values[i + 1] << invBits);

                    values[top] = (values[top] >> bits);
                }

                highestBit = getHighestBit();
            }
        }
        */
    }
    
    /**
      | Shifts a section of bits left or right.
      | 
      | -----------
      | @param howManyBitsLeft
      | 
      | how far to move the bits (+ve numbers
      | shift it left, -ve numbers shift it right).
      | ----------
      | @param startBit
      | 
      | the first bit to affect - if this is > 0,
      | only bits above that index will be affected.
      |
      */
    pub fn shift_bits(&mut self, 
        bits:      i32,
        start_bit: i32)  {
        
        todo!();
        /*
            if (highestBit >= 0)
        {
            if (bits < 0)
                shiftRight (-bits, startBit);
            else if (bits > 0)
                shiftLeft (bits, startBit);
        }
        */
    }
    
    /**
      | Returns the largest value that will
      | divide both this value and the argument.
      |
      */
    pub fn find_greatest_common_divisor(&self, n: BigInteger) -> BigInteger {
        
        todo!();
        /*
            auto m = *this;

        while (! n.isZero())
        {
            if (std::abs (m.getHighestBit() - n.getHighestBit()) <= 16)
                return simpleGCD (&m, &n);

            BigInteger temp2;
            m.divideBy (n, temp2);

            m.swapWith (n);
            n.swapWith (temp2);
        }

        return m;
        */
    }
    
    /**
      | Performs a combined exponent and modulo
      | operation.
      | 
      | This BigInteger's value becomes (this
      | ^ exponent) % modulus.
      |
      */
    pub fn exponent_modulo(&mut self, 
        exponent: &BigInteger,
        modulus:  &BigInteger)  {
        
        todo!();
        /*
            *this %= modulus;
        auto exp = exponent;
        exp %= modulus;

        if (modulus.getHighestBit() <= 32 || modulus % 2 == 0)
        {
            auto a = *this;
            auto n = exp.getHighestBit();

            for (int i = n; --i >= 0;)
            {
                *this *= *this;

                if (exp[i])
                    *this *= a;

                if (compareAbsolute (modulus) >= 0)
                    *this %= modulus;
            }
        }
        else
        {
            auto Rfactor = modulus.getHighestBit() + 1;
            BigInteger R (1);
            R.shiftLeft (Rfactor, 0);

            BigInteger R1, m1, g;
            g.extendedEuclidean (modulus, R, m1, R1);

            if (! g.isOne())
            {
                BigInteger a (*this);

                for (int i = exp.getHighestBit(); --i >= 0;)
                {
                    *this *= *this;

                    if (exp[i])
                        *this *= a;

                    if (compareAbsolute (modulus) >= 0)
                        *this %= modulus;
                }
            }
            else
            {
                auto am  = (*this * R) % modulus;
                auto xm = am;
                auto um = R % modulus;

                for (int i = exp.getHighestBit(); --i >= 0;)
                {
                    xm.montgomeryMultiplication (xm, modulus, m1, Rfactor);

                    if (exp[i])
                        xm.montgomeryMultiplication (am, modulus, m1, Rfactor);
                }

                xm.montgomeryMultiplication (1, modulus, m1, Rfactor);
                swapWith (xm);
            }
        }
        */
    }
    
    /**
      | Performs the Montgomery Multiplication
      | with modulo.
      | 
      | This object is left containing the result
      | value: ((this * other) * R1) % modulus.
      | 
      | To get this result, we need modulus,
      | modulusp and k such as R = 2^k, with modulus
      | * modulusp - R * R1 = GCD(modulus, R) =
      | 1
      |
      */
    pub fn montgomery_multiplication(&mut self, 
        other:    &BigInteger,
        modulus:  &BigInteger,
        modulusp: &BigInteger,
        k:        i32)  {
        
        todo!();
        /*
            *this *= other;
        auto t = *this;

        setRange (k, highestBit - k + 1, false);
        *this *= modulusp;

        setRange (k, highestBit - k + 1, false);
        *this *= modulus;
        *this += t;
        shiftRight (k, 0);

        if (compare (modulus) >= 0)
            *this -= modulus;
        else if (isNegative())
            *this += modulus;
        */
    }
    
    /**
      | Performs the Extended Euclidean algorithm.
      | 
      | This method will set the xOut and yOut
      | arguments such that (a * xOut) - (b * yOut)
      | = GCD (a, b).
      | 
      | On return, this object is left containing
      | the value of the GCD.
      |
      */
    pub fn extended_euclidean(&mut self, 
        a: &BigInteger,
        b: &BigInteger,
        x: &mut BigInteger,
        y: &mut BigInteger)  {
        
        todo!();
        /*
            BigInteger p(a), q(b), gcd(1);
        Vec<BigInteger> tempValues;

        while (! q.isZero())
        {
            tempValues.add (p / q);
            gcd = q;
            q = p % q;
            p = gcd;
        }

        x.clear();
        y = 1;

        for (int i = 1; i < tempValues.size(); ++i)
        {
            auto& v = tempValues.getReference (tempValues.size() - i - 1);

            if ((i & 1) != 0)
                x += y * v;
            else
                y += x * v;
        }

        if (gcd.compareAbsolute (y * b - x * a) != 0)
        {
            x.negate();
            x.swapWith (y);
            x.negate();
        }

        swapWith (gcd);
        */
    }
    
    /**
      | Performs an inverse modulo on the value.
      | i.e. the result is (this ^ -1) mod (modulus).
      |
      */
    pub fn inverse_modulo(&mut self, modulus: &BigInteger)  {
        
        todo!();
        /*
            if (modulus.isOne() || modulus.isNegative())
        {
            clear();
            return;
        }

        if (isNegative() || compareAbsolute (modulus) >= 0)
            *this %= modulus;

        if (isOne())
            return;

        if (findGreatestCommonDivisor (modulus) != 1)
        {
            clear();  // not invertible!
            return;
        }

        BigInteger a1 (modulus), a2 (*this),
                   b1 (modulus), b2 (1);

        while (! a2.isOne())
        {
            BigInteger temp1, multiplier (a1);
            multiplier.divideBy (a2, temp1);

            temp1 = a2;
            temp1 *= multiplier;
            auto temp2 = a1;
            temp2 -= temp1;
            a1 = a2;
            a2 = temp2;

            temp1 = b2;
            temp1 *= multiplier;
            temp2 = b1;
            temp2 -= temp1;
            b1 = b2;
            b2 = temp2;
        }

        while (b2.isNegative())
            b2 += modulus;

        b2 %= modulus;
        swapWith (b2);
        */
    }
    
    /**
      | Converts the number to a string.
      | 
      | Specify a base such as 2 (binary), 8 (octal),
      | 10 (decimal), 16 (hex).
      | 
      | If minimumNumCharacters is greater
      | than 0, the returned string will be padded
      | with leading zeros to reach at least
      | that length.
      |
      */
    pub fn to_string(
        &self, 
        base:                   i32,
        minimum_num_characters: Option<i32>) -> String {

        let minimum_num_characters: i32 = minimum_num_characters.unwrap_or(1);
        
        todo!();
        /*
            String s;
        auto v = *this;

        if (base == 2 || base == 8 || base == 16)
        {
            auto bits = (base == 2) ? 1 : (base == 8 ? 3 : 4);
            static const char hexDigits[] = "0123456789abcdef";

            for (;;)
            {
                auto remainder = v.getBitRangeAsInt (0, bits);
                v >>= bits;

                if (remainder == 0 && v.isZero())
                    break;

                s = String::charToString ((aloe_wchar) (uint8) hexDigits [remainder]) + s;
            }
        }
        else if (base == 10)
        {
            const BigInteger ten (10);
            BigInteger remainder;

            for (;;)
            {
                v.divideBy (ten, remainder);

                if (remainder.isZero() && v.isZero())
                    break;

                s = String (remainder.getBitRangeAsInt (0, 8)) + s;
            }
        }
        else
        {
            jassertfalse; // can't do the specified base!
            return {};
        }

        s = s.paddedLeft ('0', minimumNumCharacters);

        return isNegative() ? "-" + s : s;
        */
    }
    
    /**
      | Reads the numeric value from a string.
      | 
      | Specify a base such as 2 (binary), 8 (octal),
      | 10 (decimal), 16 (hex).
      | 
      | Any invalid characters will be ignored.
      |
      */
    pub fn parse_string(&mut self, 
        text: &str,
        base: i32)  {
        
        todo!();
        /*
            clear();
        auto t = text.text.findEndOfWhitespace();

        setNegative (*t == (aloe_wchar) '-');

        if (base == 2 || base == 8 || base == 16)
        {
            auto bits = (base == 2) ? 1 : (base == 8 ? 3 : 4);

            for (;;)
            {
                auto c = t.getAndAdvance();
                auto digit = CharacterFunctions::getHexDigitValue (c);

                if (((uint32) digit) < (uint32) base)
                {
                    *this <<= bits;
                    *this += digit;
                }
                else if (c == 0)
                {
                    break;
                }
            }
        }
        else if (base == 10)
        {
            const BigInteger ten ((uint32) 10);

            for (;;)
            {
                auto c = t.getAndAdvance();

                if (c >= '0' && c <= '9')
                {
                    *this *= ten;
                    *this += (int) (c - '0');
                }
                else if (c == 0)
                {
                    break;
                }
            }
        }
        */
    }
    
    /**
      | Turns the number into a block of binary
      | data.
      | 
      | The data is arranged as little-endian,
      | so the first byte of data is the low 8 bits
      | of the number, and so on.
      | 
      | @see loadFromMemoryBlock
      |
      */
    pub fn to_memory_block(&self) -> MemoryBlock {
        
        todo!();
        /*
            auto numBytes = (getHighestBit() + 8) >> 3;
        MemoryBlock mb ((size_t) numBytes);
        auto* values = getValues();

        for (int i = 0; i < numBytes; ++i)
            mb[i] = (char) ((values[i / 4] >> ((i & 3) * 8)) & 0xff);

        return mb;
        */
    }
    
    /**
      | Converts a block of raw data into a number.
      | 
      | The data is arranged as little-endian,
      | so the first byte of data is the low 8 bits
      | of the number, and so on.
      | 
      | @see toMemoryBlock
      |
      */
    pub fn load_from_memory_block(&mut self, data: &MemoryBlock)  {
        
        todo!();
        /*
            auto numBytes = data.getSize();
        auto numInts = 1 + (numBytes / sizeof (uint32));
        auto* values = ensureSize (numInts);

        for (int i = 0; i < (int) numInts - 1; ++i)
            values[i] = (uint32) ByteOrder::littleEndianInt (addBytesToPointer (data.getData(), (size_t) i * sizeof (uint32)));

        values[numInts - 1] = 0;

        for (int i = (int) (numBytes & ~3u); i < (int) numBytes; ++i)
            this->setBitRangeAsInt (i << 3, 8, (uint32) data [i]);

        highestBit = (int) numBytes * 8;
        highestBit = getHighestBit();
        */
    }
}
