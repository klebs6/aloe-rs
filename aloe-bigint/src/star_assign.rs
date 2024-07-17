crate::ix!();

impl AddAssign<&BigInteger> for BigInteger {
    
    #[inline] fn add_assign(&mut self, other: &BigInteger) {
        todo!();
        /*
            if (this == &other)
            return operator+= (BigInteger (other));

        if (other.isNegative())
            return operator-= (-other);

        if (isNegative())
        {
            if (compareAbsolute (other) < 0)
            {
                auto temp = *this;
                temp.negate();
                *this = other;
                *this -= temp;
            }
            else
            {
                negate();
                *this -= other;
                negate();
            }
        }
        else
        {
            highestBit = jmax (highestBit, other.highestBit) + 1;

            auto numInts = sizeNeededToHold (highestBit);
            auto* values = ensureSize (numInts);
            auto* otherValues = other.getValues();
            int64 remainder = 0;

            for (size_t i = 0; i < numInts; ++i)
            {
                remainder += values[i];

                if (i < other.allocatedSize)
                    remainder += otherValues[i];

                values[i] = (uint32) remainder;
                remainder >>= 32;
            }

            jassert (remainder == 0);
            highestBit = getHighestBit();
        }

        return *this;
        */
    }
}

impl SubAssign<&BigInteger> for BigInteger {
    
    #[inline] fn sub_assign(&mut self, other: &BigInteger) {
        todo!();
        /*
            if (this == &other)
        {
            clear();
            return *this;
        }

        if (other.isNegative())
            return operator+= (-other);

        if (isNegative())
        {
            negate();
            *this += other;
            negate();
            return *this;
        }

        if (compareAbsolute (other) < 0)
        {
            auto temp = other;
            swapWith (temp);
            *this -= temp;
            negate();
            return *this;
        }

        auto numInts = sizeNeededToHold (getHighestBit());
        auto maxOtherInts = sizeNeededToHold (other.getHighestBit());
        jassert (numInts >= maxOtherInts);
        auto* values = getValues();
        auto* otherValues = other.getValues();
        int64 amountToSubtract = 0;

        for (size_t i = 0; i < numInts; ++i)
        {
            if (i < maxOtherInts)
                amountToSubtract += (int64) otherValues[i];

            if (values[i] >= amountToSubtract)
            {
                values[i] = (uint32) (values[i] - amountToSubtract);
                amountToSubtract = 0;
            }
            else
            {
                const int64 n = ((int64) values[i] + (((int64) 1) << 32)) - amountToSubtract;
                values[i] = (uint32) n;
                amountToSubtract = 1;
            }
        }

        highestBit = getHighestBit();
        return *this;
        */
    }
}

impl MulAssign<&BigInteger> for BigInteger {
    
    #[inline] fn mul_assign(&mut self, other: &BigInteger) {
        todo!();
        /*
            if (this == &other)
            return operator*= (BigInteger (other));

        auto n = getHighestBit();
        auto t = other.getHighestBit();

        auto wasNegative = isNegative();
        setNegative (false);

        BigInteger total;
        total.highestBit = n + t + 1;
        auto* totalValues = total.ensureSize (sizeNeededToHold (total.highestBit) + 1);

        n >>= 5;
        t >>= 5;

        auto m = other;
        m.setNegative (false);

        auto* mValues = m.getValues();
        auto* values = getValues();

        for (int i = 0; i <= t; ++i)
        {
            uint32 c = 0;

            for (int j = 0; j <= n; ++j)
            {
                auto uv = (uint64) totalValues[i + j] + (uint64) values[j] * (uint64) mValues[i] + (uint64) c;
                totalValues[i + j] = (uint32) uv;
                c = static_cast<uint32> (uv >> 32);
            }

            totalValues[i + n + 1] = c;
        }

        total.highestBit = total.getHighestBit();
        total.setNegative (wasNegative ^ other.isNegative());
        swapWith (total);

        return *this;
        */
    }
}

impl DivAssign<&BigInteger> for BigInteger {
    
    #[inline] fn div_assign(&mut self, other: &BigInteger) {
        todo!();
        /*
            BigInteger remainder;
        divideBy (other, remainder);
        return *this;
        */
    }
}

impl BitOrAssign<&BigInteger> for BigInteger {
    
    #[inline] fn bitor_assign(&mut self, other: &BigInteger) {
        todo!();
        /*
            if (this == &other)
            return *this;

        // this operation doesn't take into account negative values..
        jassert (isNegative() == other.isNegative());

        if (other.highestBit >= 0)
        {
            auto* values = ensureSize (sizeNeededToHold (other.highestBit));
            auto* otherValues = other.getValues();

            auto n = (int) bitToIndex (other.highestBit) + 1;

            while (--n >= 0)
                values[n] |= otherValues[n];

            if (other.highestBit > highestBit)
                highestBit = other.highestBit;

            highestBit = getHighestBit();
        }

        return *this;
        */
    }
}

impl BitAndAssign<&BigInteger> for BigInteger {
    
    #[inline] fn bitand_assign(&mut self, other: &BigInteger) {
        todo!();
        /*
            if (this == &other)
            return *this;

        // this operation doesn't take into account negative values..
        jassert (isNegative() == other.isNegative());

        auto* values = getValues();
        auto* otherValues = other.getValues();

        auto n = (int) allocatedSize;

        while (n > (int) other.allocatedSize)
            values[--n] = 0;

        while (--n >= 0)
            values[n] &= otherValues[n];

        if (other.highestBit < highestBit)
            highestBit = other.highestBit;

        highestBit = getHighestBit();
        return *this;
        */
    }
}

impl BitXorAssign<&BigInteger> for BigInteger {
    
    #[inline] fn bitxor_assign(&mut self, other: &BigInteger) {
        todo!();
        /*
            if (this == &other)
        {
            clear();
            return *this;
        }

        // this operation will only work with the absolute values
        jassert (isNegative() == other.isNegative());

        if (other.highestBit >= 0)
        {
            auto* values = ensureSize (sizeNeededToHold (other.highestBit));
            auto* otherValues = other.getValues();

            auto n = (int) bitToIndex (other.highestBit) + 1;

            while (--n >= 0)
                values[n] ^= otherValues[n];

            if (other.highestBit > highestBit)
                highestBit = other.highestBit;

            highestBit = getHighestBit();
        }

        return *this;
        */
    }
}

lazy_static!{
    /*
    BigInteger& operator%= (const BigInteger& divisor)
    {
        BigInteger remainder;
        divideBy (divisor, remainder);
        swapWith (remainder);
        return *this;
    }
    */
}

lazy_static!{
    /*
    BigInteger& BigInteger::operator++()      { return operator+= (1); }
    BigInteger& BigInteger::operator--()      { return operator-= (1); }
    BigInteger  BigInteger::operator++ (int)  { const auto old (*this); operator+= (1); return old; }
    BigInteger  BigInteger::operator-- (int)  { const auto old (*this); operator-= (1); return old; }

    BigInteger  BigInteger::operator-() const                            { auto b (*this); b.negate(); return b; }
    BigInteger  BigInteger::operator+   (const BigInteger& other) const  { auto b (*this); return b += other; }
    BigInteger  BigInteger::operator-   (const BigInteger& other) const  { auto b (*this); return b -= other; }
    BigInteger  BigInteger::operator*   (const BigInteger& other) const  { auto b (*this); return b *= other; }
    BigInteger  BigInteger::operator/   (const BigInteger& other) const  { auto b (*this); return b /= other; }
    BigInteger  BigInteger::operator|   (const BigInteger& other) const  { auto b (*this); return b |= other; }
    BigInteger  BigInteger::operator&   (const BigInteger& other) const  { auto b (*this); return b &= other; }
    BigInteger  BigInteger::operator^   (const BigInteger& other) const  { auto b (*this); return b ^= other; }
    BigInteger  BigInteger::operator%   (const BigInteger& other) const  { auto b (*this); return b %= other; }
    BigInteger  BigInteger::operator<<  (const int numBits) const        { auto b (*this); return b <<= numBits; }
    BigInteger  BigInteger::operator>>  (const int numBits) const        { auto b (*this); return b >>= numBits; }
    BigInteger& BigInteger::operator<<= (const int numBits)              { shiftBits (numBits, 0);  return *this; }
    BigInteger& BigInteger::operator>>= (const int numBits)              { shiftBits (-numBits, 0); return *this; }
    */
}
