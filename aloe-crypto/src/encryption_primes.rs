crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/encryption/aloe_Primes.h]

/**
  | Prime number creation class.
  | 
  | This class contains static methods
  | for generating and testing prime numbers.
  | 
  | @see BigInteger
  | 
  | @tags{Cryptography}
  |
  */
#[no_copy]
pub struct Primes {


}

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/encryption/aloe_Primes.cpp]
impl Primes {

    /**
      | Creates a random prime number with a
      | given bit-length.
      | 
      | The certainty parameter specifies
      | how many iterations to use when testing
      | for primality. A safe value might be
      | anything over about 20-30.
      | 
      | The randomSeeds parameter lets you
      | optionally pass it a set of values with
      | which to seed the random number generation,
      | improving the security of the keys generated.
      |
      */
    pub fn create_probable_prime(
        &mut self, 
        bit_length:       i32,
        certainty:        i32,
        random_seeds:     *const i32,
        num_random_seeds: Option<i32>

    ) -> BigInteger {

        let num_random_seeds: i32 = num_random_seeds.unwrap_or(0);
        
        todo!();
        /*
            using namespace PrimesHelpers;
        int defaultSeeds [16];

        if (numRandomSeeds <= 0)
        {
            randomSeeds = defaultSeeds;
            numRandomSeeds = numElementsInArray (defaultSeeds);
            Random r1, r2;

            for (int j = 10; --j >= 0;)
            {
                r1.setSeedRandomly();

                for (int i = numRandomSeeds; --i >= 0;)
                    defaultSeeds[i] ^= r1.nextInt() ^ r2.nextInt();
            }
        }

        BigInteger smallSieve;
        const int smallSieveSize = 15000;
        createSmallSieve (smallSieveSize, smallSieve);

        BigInteger p;

        for (int i = numRandomSeeds; --i >= 0;)
        {
            BigInteger p2;

            Random r (randomSeeds[i]);
            r.fillBitsRandomly (p2, 0, bitLength);

            p ^= p2;
        }

        p.setBit (bitLength - 1);
        p.clearBit (0);

        const int searchLen = jmax (1024, (bitLength / 20) * 64);

        while (p.getHighestBit() < bitLength)
        {
            p += 2 * searchLen;

            BigInteger sieve;
            bigSieve (p, searchLen, sieve,
                      smallSieve, smallSieveSize);

            BigInteger candidate;

            if (findCandidate (p, sieve, searchLen, candidate, certainty))
                return candidate;
        }

        jassertfalse;
        return BigInteger();
        */
    }
     /**
      | Tests a number to see if it's prime.
      | 
      | This isn't a bulletproof test, it uses
      | a Miller-Rabin test to determine whether
      | the number is prime.
      | 
      | The certainty parameter specifies
      | how many iterations to use when testing
      | - a safe value might be anything over
      | about 20-30.
      |
      */
    pub fn is_probably_prime(&mut self, 
        number:    &BigInteger,
        certainty: i32) -> bool {
        
        todo!();
        /*
            using namespace PrimesHelpers;

        if (! number[0])
            return false;

        if (number.getHighestBit() <= 10)
        {
            const unsigned int num = number.getBitRangeAsInt (0, 10);

            for (unsigned int i = num / 2; --i > 1;)
                if (num % i == 0)
                    return false;

            return true;
        }
        else
        {
            if (number.findGreatestCommonDivisor (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23) != 1)
                return false;

            return passesMillerRabin (number, certainty);
        }
        */
    }
}

pub mod primes_helpers {

    use super::*;

    pub fn create_small_sieve(
            num_bits: i32,
            result:   &mut BigInteger)  {
        
        todo!();
        /*
            result.setBit (numBits);
                result.clearBit (numBits); // to enlarge the array

                result.setBit (0);
                int n = 2;

                do
                {
                    for (int i = n + n; i < numBits; i += n)
                        result.setBit (i);

                    n = result.findNextClearBit (n + 1);
                }
                while (n <= (numBits >> 1));
        */
    }

    pub fn big_sieve(
            base:             &BigInteger,
            num_bits:         i32,
            result:           &mut BigInteger,
            small_sieve:      &BigInteger,
            small_sieve_size: i32)  {
        
        todo!();
        /*
            jassert (! base[0]); // must be even!

                result.setBit (numBits);
                result.clearBit (numBits);  // to enlarge the array

                int index = smallSieve.findNextClearBit (0);

                do
                {
                    const unsigned int prime = ((unsigned int) index << 1) + 1;

                    BigInteger r (base), remainder;
                    r.divideBy (prime, remainder);

                    unsigned int i = prime - remainder.getBitRangeAsInt (0, 32);

                    if (r.isZero())
                        i += prime;

                    if ((i & 1) == 0)
                        i += prime;

                    i = (i - 1) >> 1;

                    while (i < (unsigned int) numBits)
                    {
                        result.setBit ((int) i);
                        i += prime;
                    }

                    index = smallSieve.findNextClearBit (index + 1);
                }
                while (index < smallSieveSize);
        */
    }

    pub fn find_candidate(
            base:      &BigInteger,
            sieve:     &BigInteger,
            num_bits:  i32,
            result:    &mut BigInteger,
            certainty: i32) -> bool {
        
        todo!();
        /*
            for (int i = 0; i < numBits; ++i)
                {
                    if (! sieve[i])
                    {
                        result = base + (unsigned int) ((i << 1) + 1);

                        if (Primes::isProbablyPrime (result, certainty))
                            return true;
                    }
                }

                return false;
        */
    }

    pub fn passes_miller_rabin(
            n:          &BigInteger,
            iterations: i32) -> bool {
        
        todo!();
        /*
            const BigInteger one (1), two (2);
                const BigInteger nMinusOne (n - one);

                BigInteger d (nMinusOne);
                const int s = d.findNextSetBit (0);
                d >>= s;

                BigInteger smallPrimes;
                int numBitsInSmallPrimes = 0;

                for (;;)
                {
                    numBitsInSmallPrimes += 256;
                    createSmallSieve (numBitsInSmallPrimes, smallPrimes);

                    const int numPrimesFound = numBitsInSmallPrimes - smallPrimes.countNumberOfSetBits();

                    if (numPrimesFound > iterations + 1)
                        break;
                }

                int smallPrime = 2;

                while (--iterations >= 0)
                {
                    smallPrime = smallPrimes.findNextClearBit (smallPrime + 1);

                    BigInteger r (smallPrime);
                    r.exponentModulo (d, n);

                    if (r != one && r != nMinusOne)
                    {
                        for (int j = 0; j < s; ++j)
                        {
                            r.exponentModulo (two, n);

                            if (r == nMinusOne)
                                break;
                        }

                        if (r != nMinusOne)
                            return false;
                    }
                }

                return true;
        */
    }
}
