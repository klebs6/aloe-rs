crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/encryption/aloe_RSAKey.h]

/**
  | RSA public/private key-pair encryption class.
  |
  | An object of this type makes up one half of
  | a public/private RSA key pair. Use the
  | createKeyPair() method to create a matching
  | pair for encoding/decoding.
  |
  | If you need to use this class in conjunction
  | with a compatible enc/decryption algorithm on
  | a webserver, you can achieve the same thing in
  | PHP like this:
  |
  | @code
  |
  | // get this from: phpseclib.sourceforge.net
  | include ('Math/BigInteger.php');  
  |
  | function applyToValue ($message, $key_part1, $key_part2)
  | {
  |     $result = new Math_BigInteger();
  |     $zero  = new Math_BigInteger();
  |     $value = new Math_BigInteger (strrev ($message), 256);
  |     $part1 = new Math_BigInteger ($key_part1, 16);
  |     $part2 = new Math_BigInteger ($key_part2, 16);
  |
  |     while (! $value->equals ($zero))
  |     {
  |         $result = $result->multiply ($part2);
  |         list ($value, $remainder) = $value->divide ($part2);
  |         $result = $result->add ($remainder->modPow ($part1, $part2));
  |     }
  |
  |     return ($result->toBytes());
  | }
  | @endcode
  |
  | ..or in Java with something like this:
  |
  | @code
  | public class RSAKey
  | {
  |     static BigInteger applyToValue (BigInteger value, String key_part1, String key_part2)
  |     {
  |         BigInteger result = BigInteger.ZERO;
  |         BigInteger part1 = new BigInteger (key_part1, 16);
  |         BigInteger part2 = new BigInteger (key_part2, 16);
  |
  |         if (part1.equals (BigInteger.ZERO) || part2.equals (BigInteger.ZERO)
  |              || value.compareTo (BigInteger.ZERO) <= 0)
  |             return result;
  |
  |         while (! value.equals (BigInteger.ZERO))
  |         {
  |             result = result.multiply (part2);
  |             BigInteger[] div = value.divideAndRemainder (part2);
  |             value = div[0];
  |             result = result.add (div[1].modPow (part1, part2));
  |         }
  |
  |         return result;
  |     }
  | }
  | @endcode
  |
  | Disclaimer: neither of the code snippets above
  | are tested! Please let me know if you have any
  | corrections for them!
  |
  | @tags{Cryptography}
  */
#[leak_detector]
pub struct RSAKey {
    part1: BigInteger,
    part2: BigInteger,
}

impl Default for RSAKey {
    
    /**
      | Creates a null key object.
      | 
      | Initialise a pair of objects for use
      | with the createKeyPair() method.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl PartialEq<RSAKey> for RSAKey {
    
    #[inline] fn eq(&self, other: &RSAKey) -> bool {
        todo!();
        /*
            return part1 == other.part1 && part2 == other.part2;
        */
    }
}

impl Eq for RSAKey {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_cryptography/encryption/aloe_RSAKey.cpp]
impl RSAKey {

    /**
      | Loads a key from an encoded string representation.
      | 
      | This reloads a key from a string created
      | by the toString() method.
      |
      */
    pub fn new(string_representation: &String) -> Self {
    
        todo!();
        /*


            if (s.containsChar (','))
        {
            part1.parseString (s.upToFirstOccurrenceOf (",", false, false), 16);
            part2.parseString (s.fromFirstOccurrenceOf (",", false, false), 16);
        }
        else
        {
            // the string needs to be two hex numbers, comma-separated..
            jassertfalse;
        }
        */
    }
    
    /**
      | Returns true if the object is a valid
      | key, or false if it was created by the
      | default constructor.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return operator!= (RSAKey());
        */
    }

    /**
      | Turns the key into a string representation.
      | 
      | This can be reloaded using the constructor
      | that takes a string.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return part1.toString (16) + "," + part2.toString (16);
        */
    }

    /**
      | Encodes or decodes a value.
      | 
      | Call this on the public key object to
      | encode some data, then use the matching
      | private key object to decode it.
      | 
      | Returns false if the operation couldn't
      | be completed, e.g. if this key hasn't
      | been initialised correctly.
      | 
      | -----------
      | @note
      | 
      | This method dumbly applies this key
      | to this data. If you encode some data
      | and then try to decode it with a key that
      | doesn't match, this method will still
      | happily do its job and return true, but
      | the result won't be what you were expecting.
      | It's your responsibility to check that
      | the result is what you wanted.
      |
      */
    pub fn apply_to_value(&self, value: &mut BigInteger) -> bool {
        
        todo!();
        /*
            if (part1.isZero() || part2.isZero() || value <= 0)
        {
            jassertfalse;   // using an uninitialised key
            value.clear();
            return false;
        }

        BigInteger result;

        while (! value.isZero())
        {
            result *= part2;

            BigInteger remainder;
            value.divideBy (part2, remainder);

            remainder.exponentModulo (part1, part2);

            result += remainder;
        }

        value.swapWith (result);
        return true;
        */
    }

    pub fn find_best_common_divisor(
        &mut self, 
        p: &BigInteger,
        q: &BigInteger

    ) -> BigInteger {
        
        todo!();
        /*
            // try 3, 5, 9, 17, etc first because these only contain 2 bits and so
        // are fast to divide + multiply
        for (int i = 2; i <= 65536; i *= 2)
        {
            const BigInteger e (1 + i);

            if (e.findGreatestCommonDivisor (p).isOne() && e.findGreatestCommonDivisor (q).isOne())
                return e;
        }

        BigInteger e (4);

        while (! (e.findGreatestCommonDivisor (p).isOne() && e.findGreatestCommonDivisor (q).isOne()))
            ++e;

        return e;
        */
    }

    /**
      | Creates a public/private key-pair.
      | 
      | Each key will perform one-way encryption
      | that can only be reversed by using the
      | other key.
      | 
      | The numBits parameter specifies the
      | size of key, e.g. 128, 256, 512 bit. Bigger
      | sizes are more secure, but this method
      | will take longer to execute.
      | 
      | The randomSeeds parameter lets you
      | optionally pass it a set of values with
      | which to seed the random number generation,
      | improving the security of the keys generated.
      | If you supply these, make sure you provide
      | more than 2 values, and the more your
      | provide, the better the security.
      |
      */
    pub fn create_key_pair(
        &mut self, 
        public_key:       &mut RSAKey,
        private_key:      &mut RSAKey,
        num_bits:         i32,
        random_seeds:     *const i32,
        num_random_seeds: Option<i32>

    ) {

        let num_random_seeds: i32 = num_random_seeds.unwrap_or(0);
        
        todo!();
        /*
            jassert (numBits > 16); // not much point using less than this..
        jassert (numRandomSeeds == 0 || numRandomSeeds >= 2); // you need to provide plenty of seeds here!

        BigInteger p (Primes::createProbablePrime (numBits / 2, 30, randomSeeds, numRandomSeeds / 2));
        BigInteger q (Primes::createProbablePrime (numBits - numBits / 2, 30, randomSeeds == nullptr ? nullptr : (randomSeeds + numRandomSeeds / 2), numRandomSeeds - numRandomSeeds / 2));

        const BigInteger n (p * q);
        const BigInteger m (--p * --q);
        const BigInteger e (findBestCommonDivisor (p, q));

        BigInteger d (e);
        d.inverseModulo (m);

        publicKey.part1 = e;
        publicKey.part2 = n;

        privateKey.part1 = d;
        privateKey.part2 = n;
        */
    }
}
