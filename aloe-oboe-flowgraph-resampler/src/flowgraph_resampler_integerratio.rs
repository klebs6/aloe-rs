crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/ResamplerIntegerRatio.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/resampler/ResamplerIntegerRatio.cpp]

/**
  | Represent the ratio of two integers.
  |
  */
pub struct ResamplerIntegerRatio {
    numerator:   i32,
    denominator: i32,
}

impl ResamplerIntegerRatio {

    pub fn new(
        numerator:   i32,
        denominator: i32) -> Self {
    
        todo!();
        /*
        : numerator(numerator),
        : denominator(denominator),

        
        */
    }

    pub fn get_numerator(&mut self) -> i32 {
        
        todo!();
        /*
            return mNumerator;
        */
    }
    
    pub fn get_denominator(&mut self) -> i32 {
        
        todo!();
        /*
            return mDenominator;
        */
    }
    
    /**
      | Reduce by removing common prime factors.
      |
      */
    pub fn reduce(&mut self)  {
        
        todo!();
        /*
            for (int prime : kPrimes) {
            if (mNumerator < prime || mDenominator < prime) {
                break;
            }

            // Find biggest prime factor for numerator.
            while (true) {
                int top = mNumerator / prime;
                int bottom = mDenominator / prime;
                if ((top >= 1)
                    && (bottom >= 1)
                    && (top * prime == mNumerator) // divided evenly?
                    && (bottom * prime == mDenominator)) {
                    mNumerator = top;
                    mDenominator = bottom;
                } else {
                    break;
                }
            }

        }
        */
    }
}

/**
  | Enough primes to cover the common sample
  | rates.
  |
  */
pub const kPrimes: &[i32] = &[
          2,   3,   5,   7,  11,  13,  17,  19,  23,  29, 31, 37, 41,
         43,  47,  53,  59,  61,  67,  71,  73,  79,  83, 89, 97,
        101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
        151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199
];
