crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/frequency/aloe_Windowing.h]

/**
  | A class which provides multiple windowing
  | functions useful for filter design and
  | spectrum analyzers.
  |
  | The different functions provided here can be
  | used by creating either a WindowingFunction
  | object, or a static function to fill an array
  | with the windowing method samples.
  |
  | @tags{DSP}
  */
#[no_copy]
#[leak_detector]
pub struct WindowingFunction<FloatType: num::Float> {
    window_table: Vec<FloatType>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/frequency/aloe_Windowing.cpp]
impl<FloatType: num::Float> WindowingFunction<FloatType> {

    /**
      | This constructor automatically fills
      | a buffer of the specified size using
      | the fillWindowingTables function
      | and the specified arguments.
      | 
      | @see fillWindowingTables
      |
      */
    pub fn new(
        size:      usize,
        ty:        WindowingMethod,
        normalise: Option<bool>,
        beta:      Option<FloatType>

    ) -> Self {

        let normalise: bool = normalise.unwrap_or(true);
        let beta: FloatType = beta.unwrap_or(FloatType::from(0.0).unwrap());
    
        todo!();
        /*
            fillWindowingTables (size, type, normalise, beta);
        */
    }
    
    /**
      | Fills the content of the object array
      | with a given windowing method table.
      | 
      | -----------
      | @param size
      | 
      | the size of the destination buffer allocated
      | in the object
      | ----------
      | @param type
      | 
      | the type of windowing method being used
      | ----------
      | @param normalise
      | 
      | if the result must be normalised, creating
      | a DC amplitude response of one
      | ----------
      | @param beta
      | 
      | an optional argument useful only for
      | Kaiser's method which must be positive
      | and sets the properties of the method
      | (bandwidth and attenuation increases
      | with beta)
      |
      */
    pub fn fill_windowing_tables(
        &mut self, 
        size:      usize,
        ty:        WindowingMethod,
        normalise: Option<bool>,
        beta:      Option<FloatType>

    ) {

        let normalise = normalise.unwrap_or(true);
        let beta      = beta.unwrap_or(FloatType::from(0.0).unwrap());
        
        todo!();
        /*
            windowTable.resize (static_cast<int> (size));
        fillWindowingTables (windowTable.getRawDataPointer(), size, type, normalise, beta);
        */
    }
    
    /**
      | Fills the content of an array with a given
      | windowing method table.
      | 
      | -----------
      | @param samples
      | 
      | the destination buffer pointer
      | ----------
      | @param size
      | 
      | the size of the destination buffer allocated
      | in the object
      | ----------
      | @param normalise
      | 
      | if the result must be normalised, creating
      | a DC amplitude response of one
      | ----------
      | @param beta
      | 
      | an optional argument useful only for
      | Kaiser's method, which must be positive
      | and sets the properties of the method
      | (bandwidth and attenuation increases
      | with beta)
      |
      */
    pub fn fill_windowing_tables_with_given_windowing_method_table(
        &mut self, 
        samples:   *mut FloatType,
        size:      usize,
        ty:        WindowingMethod,
        normalise: Option<bool>,
        beta:      Option<FloatType>

    ) {

        let normalise = normalise.unwrap_or(true);
        let beta      = beta.unwrap_or(FloatType::from(0.0).unwrap());
        
        todo!();
        /*
            switch (type)
        {
            case rectangular:
            {
                for (size_t i = 0; i < size; ++i)
                    samples[i] = static_cast<FloatType> (1);
            }
            break;

            case triangular:
            {
                auto halfSlots = static_cast<FloatType> (0.5) * static_cast<FloatType> (size - 1);

                for (size_t i = 0; i < size; ++i)
                    samples[i] = static_cast<FloatType> (1.0) - std::abs ((static_cast<FloatType> (i) - halfSlots) / halfSlots);
            }
            break;

            case hann:
            {
                for (size_t i = 0; i < size; ++i)
                {
                    auto cos2 = ncos<FloatType> (2, i, size);
                    samples[i] = static_cast<FloatType> (0.5 - 0.5 * cos2);
                }
            }
            break;

            case hamming:
            {
                for (size_t i = 0; i < size; ++i)
                {
                    auto cos2 = ncos<FloatType> (2, i, size);
                    samples[i] = static_cast<FloatType> (0.54 - 0.46 * cos2);
                }
            }
            break;

            case blackman:
            {
                constexpr FloatType alpha = 0.16f;

                for (size_t i = 0; i < size; ++i)
                {
                    auto cos2 = ncos<FloatType> (2, i, size);
                    auto cos4 = ncos<FloatType> (4, i, size);

                    samples[i] = static_cast<FloatType> (0.5 * (1 - alpha) - 0.5 * cos2 + 0.5 * alpha * cos4);
                }
            }
            break;

            case blackmanHarris:
            {
                for (size_t i = 0; i < size; ++i)
                {
                    auto cos2 = ncos<FloatType> (2, i, size);
                    auto cos4 = ncos<FloatType> (4, i, size);
                    auto cos6 = ncos<FloatType> (6, i, size);

                    samples[i] = static_cast<FloatType> (0.35875 - 0.48829 * cos2 + 0.14128 * cos4 - 0.01168 * cos6);
                }
            }
            break;

            case flatTop:
            {
                for (size_t i = 0; i < size; ++i)
                {
                    auto cos2 = ncos<FloatType> (2, i, size);
                    auto cos4 = ncos<FloatType> (4, i, size);
                    auto cos6 = ncos<FloatType> (6, i, size);
                    auto cos8 = ncos<FloatType> (8, i, size);

                    samples[i] = static_cast<FloatType> (1.0 - 1.93 * cos2 + 1.29 * cos4 - 0.388 * cos6 + 0.028 * cos8);
                }
            }
            break;

            case kaiser:
            {
                const double factor = 1.0 / SpecialFunctions::besselI0 (beta);
                const auto doubleSize = (double) size;

                for (size_t i = 0; i < size; ++i)
                    samples[i] = static_cast<FloatType> (SpecialFunctions::besselI0 (beta * std::sqrt (1.0 - std::pow (((double) i - 0.5 * (doubleSize - 1.0))
                                                                                                                         / ( 0.5 * (doubleSize - 1.0)), 2.0)))
                                                          * factor);
            }
            break;

            case numWindowingMethods:
            default:
                jassertfalse;
                break;
        }

        // DC frequency amplitude must be one
        if (normalise)
        {
            FloatType sum (0);

            for (size_t i = 0; i < size; ++i)
                sum += samples[i];

            auto factor = static_cast<FloatType> (size) / sum;

            FloatVectorOperations::multiply (samples, factor, static_cast<int> (size));
        }
        */
    }
    
    /**
      | Multiplies the content of a buffer with
      | the given window.
      |
      */
    pub fn multiply_with_windowing_table(
        &mut self, 
        samples: *mut FloatType,
        size:    usize

    ) {
        
        todo!();
        /*
            FloatVectorOperations::multiply (samples, windowTable.getRawDataPointer(), jmin (static_cast<int> (size), windowTable.size()));
        */
    }
    
    /**
      | Returns the name of a given windowing
      | method.
      |
      */
    pub fn get_windowing_method_name(&mut self, ty: WindowingMethod) -> *const u8 {
        
        todo!();
        /*
            switch (type)
        {
            case rectangular:          return "Rectangular";
            case triangular:           return "Triangular";
            case hann:                 return "Hann";
            case hamming:              return "Hamming";
            case blackman:             return "Blackman";
            case blackmanHarris:       return "Blackman-Harris";
            case flatTop:              return "Flat Top";
            case kaiser:               return "Kaiser";
            case numWindowingMethods:
            default: jassertfalse;     return "";
        }
        */
    }
}
