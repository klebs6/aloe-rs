crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_LookupTable.h]

/**
  | Class for efficiently approximating
  | expensive arithmetic operations.
  | 
  | The approximation is based on linear
  | interpolation between pre-calculated
  | values. The approximated function
  | should be passed as a callable object
  | to the constructor along with the number
  | of data points to be pre-calculated.
  | The accuracy of the approximation can
  | be increased by using more points at
  | the cost of a larger memory footprint.
  | 
  | Consider using LookupTableTransform
  | as an easy-to-use alternative.
  | 
  | Example:
  | 
  | LookupTable<float> lut ([] (size_t
  | i) { return std::sqrt ((float) i); },
  | 64); auto outValue = lut[17];
  | 
  | @see LookupTableTransform
  | 
  | @tags{DSP}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LookupTable<FloatType> {

    data: Vec<FloatType>,
}

impl<FloatType> Default for LookupTable<FloatType> {
    
    /**
      | Creates an uninitialised LookupTable
      | object.
      | 
      | You need to call initialise() before
      | using the object. Prefer using the non-default
      | constructor instead.
      | 
      | @see initialise
      |
      */
    fn default() -> Self {
        todo!();
        /*
            data.resize (1)
        */
    }
}

impl<FloatType> Index<FloatType> for LookupTable<FloatType> {

    type Output = FloatType;
    
    /**
      | @see getUnchecked
      |
      */
    #[inline] fn index(&self, index: FloatType) -> &Self::Output {
        todo!();
        /*
            return getUnchecked (index);
        */
    }
}

impl<FloatType> LookupTable<FloatType> {

    /**
      | Calculates the approximated value
      | for the given index without range checking.
      | 
      | Use this if you can guarantee that the
      | index is non-negative and less than
      | numPoints. Otherwise use get().
      | 
      | -----------
      | @param index
      | 
      | The approximation is calculated for
      | this non-integer index.
      | 
      | -----------
      | @return
      | 
      | The approximated value at the given
      | index.
      | 
      | @see get, operator[]
      |
      */
    pub fn get_unchecked(&self, index: FloatType) -> FloatType {
        
        todo!();
        /*
            jassert (isInitialised());  // Use the non-default constructor or call initialise() before first use
            jassert (isPositiveAndBelow (index, FloatType (getNumPoints())));

            auto i = truncatePositiveToUnsignedInt (index);
            auto f = index - FloatType (i);
            jassert (isPositiveAndBelow (f, FloatType (1)));

            auto x0 = data.getUnchecked (static_cast<int> (i));
            auto x1 = data.getUnchecked (static_cast<int> (i + 1));

            return jmap (f, x0, x1);
        */
    }

    /**
      | Calculates the approximated value
      | for the given index with range checking.
      | 
      | This can be called with any input indices.
      | If the provided index is out-of-range
      | either the bottom or the top element
      | of the LookupTable is returned.
      | 
      | If the index is guaranteed to be in range
      | use the faster getUnchecked() instead.
      | 
      | -----------
      | @param index
      | 
      | The approximation is calculated for
      | this non-integer index.
      | 
      | -----------
      | @return
      | 
      | The approximated value at the given
      | index.
      | 
      | @see getUnchecked, operator[]
      |
      */
    pub fn get(&self, index: FloatType) -> FloatType {
        
        todo!();
        /*
            if (index >= (FloatType) getNumPoints())
                index = static_cast<FloatType> (getGuardIndex());
            else if (index < 0)
                index = {};

            return getUnchecked (index);
        */
    }

    /**
      | Returns the size of the LookupTable,
      | i.e., the number of pre-calculated
      | data points.
      |
      */
    pub fn get_num_points(&self) -> usize {
        
        todo!();
        /*
            return static_cast<size_t> (data.size()) - 1;
        */
    }

    /**
      | Returns true if the LookupTable is initialised
      | and ready to be used.
      |
      */
    pub fn is_initialised(&self) -> bool {
        
        todo!();
        /*
            return data.size() > 1;
        */
    }

    pub fn get_required_buffer_size(num_points_to_use: usize) -> usize {
        
        todo!();
        /*
            return numPointsToUse + 1;
        */
    }


    pub fn get_guard_index(&self) -> usize {
        
        todo!();
        /*
            return getRequiredBufferSize (getNumPoints()) - 1;
        */
    }
}

/**
  | Class for approximating expensive
  | arithmetic operations.
  | 
  | Once initialised, this class can be
  | used just like the function it approximates
  | via operator().
  | 
  | Example:
  | 
  | LookupTableTransform<float> tanhApprox
  | ([] (float x) { return std::tanh (x);
  | }, -5.0f, 5.0f, 64);
  | 
  | auto outValue = tanhApprox (4.2f);
  | 
  | Note: If you try to call the function
  | with an input outside the provided range,
  | it will return either the first or the
  | last recorded LookupTable value.
  | 
  | @see LookupTable
  | 
  | @tags{DSP}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LookupTableTransform<FloatType> {
    lookup_table:    LookupTable<FloatType>,
    min_input_value: FloatType,
    max_input_value: FloatType,
    scaler:          FloatType,
    offset:          FloatType,
}

impl<FloatType> Default for LookupTableTransform<FloatType> {
    
    /**
      | Creates an uninitialised LookupTableTransform
      | object.
      | 
      | You need to call initialise() before
      | using the object. Prefer using the non-default
      | constructor instead.
      | 
      | @see initialise
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}


impl<FloatType> Index<FloatType> for LookupTableTransform<FloatType> {

    type Output = FloatType;
    
    /**
      | @see processSampleUnchecked
      |
      */
    #[inline] fn index(&self, index: FloatType) -> &Self::Output {
        todo!();
        /*
            return processSampleUnchecked (index);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_LookupTable.cpp]
impl<FloatType> LookupTableTransform<FloatType> {

    /**
      | Creates and initialises a LookupTableTransform
      | object.
      | 
      | -----------
      | @param functionToApproximate
      | 
      | The function to be approximated. This
      | should be a mapping from a FloatType
      | to FloatType.
      | ----------
      | @param minInputValueToUse
      | 
      | The lowest input value used. The approximation
      | will fail for values lower than this.
      | ----------
      | @param maxInputValueToUse
      | 
      | The highest input value used. The approximation
      | will fail for values higher than this.
      | ----------
      | @param numPoints
      | 
      | The number of pre-calculated values
      | stored.
      |
      */
    pub fn new(
        function_to_approximate: &fn(_0: FloatType) -> FloatType,
        min_input_value_to_use:  FloatType,
        max_input_value_to_use:  FloatType,
        num_points:              usize) -> Self {
    
        todo!();
        /*
            initialise (functionToApproximate, minInputValueToUse, maxInputValueToUse, numPoints);
        */
    }

    /**
      | Calculates the approximated value
      | for the given input value without range
      | checking.
      | 
      | Use this if you can guarantee that the
      | input value is within the range specified
      | in the constructor or initialise(),
      | otherwise use processSample().
      | 
      | -----------
      | @param value
      | 
      | The approximation is calculated for
      | this input value.
      | 
      | -----------
      | @return
      | 
      | The approximated value for the provided
      | input value.
      | 
      | @see processSample, operator(), operator[]
      |
      */
    pub fn process_sample_unchecked(&self, value: FloatType) -> FloatType {
        
        todo!();
        /*
            jassert (value >= minInputValue && value <= maxInputValue);
            return lookupTable[scaler * value + offset];
        */
    }

    /**
      | Calculates the approximated value
      | for the given input value with range
      | checking.
      | 
      | This can be called with any input values.
      | Out-of-range input values will be clipped
      | to the specified input range.
      | 
      | If the index is guaranteed to be in range
      | use the faster processSampleUnchecked()
      | instead.
      | 
      | -----------
      | @param value
      | 
      | The approximation is calculated for
      | this input value.
      | 
      | -----------
      | @return
      | 
      | The approximated value for the provided
      | input value.
      | 
      | @see processSampleUnchecked, operator(),
      | operator[]
      |
      */
    pub fn process_sample(&self, value: FloatType) -> FloatType {
        
        todo!();
        /*
            auto index = scaler * jlimit (minInputValue, maxInputValue, value) + offset;
            jassert (isPositiveAndBelow (index, FloatType (lookupTable.getNumPoints())));

            return lookupTable[index];
        */
    }

    /**
      | @see processSample
      |
      */
    pub fn invoke(&self, index: FloatType) -> FloatType {
        
        todo!();
        /*
            return processSample (index);
        */
    }

    /**
      | Processes an array of input values without
      | range checking @see process
      |
      */
    pub fn process_unchecked(&self, 
        input:       *const FloatType,
        output:      *mut FloatType,
        num_samples: usize)  {
        
        todo!();
        /*
            for (size_t i = 0; i < numSamples; ++i)
                output[i] = processSampleUnchecked (input[i]);
        */
    }

    /**
      | Processes an array of input values with
      | range checking @see processUnchecked
      |
      */
    pub fn process(&self, 
        input:       *const FloatType,
        output:      *mut FloatType,
        num_samples: usize)  {
        
        todo!();
        /*
            for (size_t i = 0; i < numSamples; ++i)
                output[i] = processSample (input[i]);
        */
    }
}

impl<FloatType> LookupTable<FloatType> {

    /**
      | Creates and initialises a LookupTable
      | object.
      | 
      | -----------
      | @param functionToApproximate
      | 
      | The function to be approximated. This
      | should be a mapping from the integer
      | range [0, numPointsToUse - 1].
      | ----------
      | @param numPointsToUse
      | 
      | The number of pre-calculated values
      | stored.
      |
      */
    pub fn new(
        function_to_approximate: &fn(_0: usize) -> FloatType,
        num_points_to_use:       usize) -> Self {
    
        todo!();
        /*
            initialise (functionToApproximate, numPointsToUse);
        */
    }
    
    /**
      | Initialises or changes the parameters
      | of a LookupTable object.
      | 
      | This function can be used to change what
      | function is approximated by an already
      | constructed LookupTable along with
      | the number of data points used. If the
      | function to be approximated won't ever
      | change, prefer using the non-default
      | constructor.
      | 
      | -----------
      | @param functionToApproximate
      | 
      | The function to be approximated. This
      | should be a mapping from the integer
      | range [0, numPointsToUse - 1].
      | ----------
      | @param numPointsToUse
      | 
      | The number of pre-calculated values
      | stored.
      |
      */
    pub fn initialise(&mut self, 
        function_to_approximate: &fn(_0: usize) -> FloatType,
        num_points_to_use:       usize)  {
        
        todo!();
        /*
            data.resize (static_cast<int> (getRequiredBufferSize (numPointsToUse)));

        for (size_t i = 0; i < numPointsToUse; ++i)
        {
            auto value = functionToApproximate (i);

            jassert (! std::isnan (value));
            jassert (! std::isinf (value));
            // Make sure functionToApproximate returns a sensible value for the entire specified range.
            // E.g., this won't work for zero:  [] (size_t i) { return 1.0f / i; }

            data.getReference (static_cast<int> (i)) = value;
        }

        prepare();
        */
    }
    
    pub fn prepare(&mut self)  {
        
        todo!();
        /*
            auto guardIndex = static_cast<int> (getGuardIndex());
        data.getReference (guardIndex) = data.getUnchecked (guardIndex - 1);
        */
    }
}

impl<FloatType> LookupTableTransform<FloatType> {

    /**
      | Initialises or changes the parameters
      | of a LookupTableTransform object.
      | 
      | -----------
      | @param functionToApproximate
      | 
      | The function to be approximated. This
      | should be a mapping from a FloatType
      | to FloatType.
      | ----------
      | @param minInputValueToUse
      | 
      | The lowest input value used. The approximation
      | will fail for values lower than this.
      | ----------
      | @param maxInputValueToUse
      | 
      | The highest input value used. The approximation
      | will fail for values higher than this.
      | ----------
      | @param numPoints
      | 
      | The number of pre-calculated values
      | stored.
      |
      */
    pub fn initialise(&mut self, 
        function_to_approximate: &fn(_0: FloatType) -> FloatType,
        min_input_value_to_use:  FloatType,
        max_input_value_to_use:  FloatType,
        num_points:              usize)  {
        
        todo!();
        /*
        jassert (maxInputValueToUse > minInputValueToUse);

        minInputValue = minInputValueToUse;
        maxInputValue = maxInputValueToUse;
        scaler = FloatType (numPoints - 1) / (maxInputValueToUse - minInputValueToUse);
        offset = -minInputValueToUse * scaler;

        const auto initFn = [functionToApproximate, minInputValueToUse, maxInputValueToUse, numPoints] (size_t i)
        {
            return functionToApproximate (
                jlimit (
                    minInputValueToUse, maxInputValueToUse,
                    jmap (FloatType (i), FloatType (0), FloatType (numPoints - 1), minInputValueToUse, maxInputValueToUse))
                );
        };

        lookupTable.initialise (initFn, numPoints);
        */
    }

    /**
      | Calculates the maximum relative error
      | of the approximation for the specified
      | parameter set.
      | 
      | The closer the returned value is to zero
      | the more accurate the approximation
      | is.
      | 
      | This function compares the approximated
      | output of this class to the function
      | it approximates at a range of points
      | and returns the maximum relative error.
      | This can be used to determine if the approximation
      | is suitable for the given problem. The
      | accuracy of the approximation can generally
      | be improved by increasing numPoints.
      | 
      | -----------
      | @param functionToApproximate
      | 
      | The approximated function. This should
      | be a mapping from a FloatType to FloatType.
      | ----------
      | @param minInputValue
      | 
      | The lowest input value used.
      | ----------
      | @param maxInputValue
      | 
      | The highest input value used.
      | ----------
      | @param numPoints
      | 
      | The number of pre-calculated values
      | stored.
      | ----------
      | @param numTestPoints
      | 
      | The number of input values used for error
      | calculation. Higher numbers can increase
      | the accuracy of the error calculation.
      | If it's zero then 100 * numPoints will
      | be used.
      |
      */
    pub fn calculate_max_relative_error(
        &mut self, 
        function_to_approximate: &fn(_0: FloatType) -> FloatType,
        min_input_value:         FloatType,
        max_input_value:         FloatType,
        num_points:              usize,
        num_test_points:         Option<usize>) -> f64 {

        let num_test_points: usize = num_test_points.unwrap_or(0);
        
        todo!();
        /*
        jassert (maxInputValue > minInputValue);

        if (numTestPoints == 0)
            numTestPoints = 100 * numPoints;    // use default

        LookupTableTransform transform (functionToApproximate, minInputValue, maxInputValue, numPoints);

        double maxError = 0;

        for (size_t i = 0; i < numTestPoints; ++i)
        {
            auto inputValue = jmap (FloatType (i), FloatType (0), FloatType (numTestPoints - 1), minInputValue, maxInputValue);
            auto approximatedOutputValue = transform.processSample (inputValue);
            auto referenceOutputValue = functionToApproximate (inputValue);

            maxError = jmax (maxError, calculateRelativeDifference ((double) referenceOutputValue, (double) approximatedOutputValue));
        }

        return maxError;
        */
    }
    
    pub fn calculate_relative_difference(&mut self, x: f64, y: f64) -> f64 {
        
        todo!();
        /*
            static const auto eps = std::numeric_limits<double>::min();

        auto absX = std::abs (x);
        auto absY = std::abs (y);
        auto absDiff = std::abs (x - y);

        if (absX < eps)
        {
            if (absY >= eps)
                return absDiff / absY;

            return absDiff;    // return the absolute error if both numbers are too close to zero
        }

        return absDiff / std::min (absX, absY);
        */
    }
}
