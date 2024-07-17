crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/maths/aloe_NormalisableRange.h]

/**
  | Represents a mapping between an arbitrary
  | range of values and a normalised 0->1
  | range.
  | 
  | The properties of the mapping also include
  | an optional snapping interval and skew-factor.
  | 
  | @see Range
  | 
  | @tags{Core}
  |
  */
pub struct NormalisableRange<ValueType> {

    /**
      | The minimum value of the non-normalised
      | range.
      |
      */
    start:                        ValueType, // default = 0

    /**
      | The maximum value of the non-normalised
      | range.
      |
      */
    end:                          ValueType, // default = 1

    /**
      | The snapping interval that should be used
      | (for a non-normalised value). Use 0 for
      | a continuous range. If you have used
      | a lambda function for
      | snapToLegalValueFunction in the
      | constructor of this class then the
      | interval is ignored.
      */
    interval:                     ValueType, // default = 0

    /**
      | An optional skew factor that alters the way values
      | are distribute across the range. The skew factor
      | lets you skew the mapping logarithmically so that
      | larger or smaller values are given a larger proportion
      | of the available space. A factor of 1.0 has no
      | skewing effect at all. If the factor is < 1.0,
      | the lower end of the range will fill more of the
      | slider's length; if the factor is > 1.0, the upper
      | end of the range will be expanded. If you have
      | used lambda functions for convertFrom0to1Func and
      | convertFrom0to1Func in the constructor of this
      | class then the skew value is ignored.
      */
    skew:                         ValueType, // default = 1

    /**
      | If true, the skew factor applies from
      | the middle of the slider to each of its
      | ends.
      |
      */
    symmetric_skew:               bool, // default = false

    convert_from_0to_1function:   <Self as HasValueRemapFunction>::ValueRemapFunction,
    convert_to_0to_1function:     <Self as HasValueRemapFunction>::ValueRemapFunction,
    snap_to_legal_value_function: <Self as HasValueRemapFunction>::ValueRemapFunction,
}

impl<ValueType> PartialEq<NormalisableRange<ValueType>> for NormalisableRange<ValueType> {
    
    fn eq(&self, other: &NormalisableRange<ValueType>) -> bool {
        todo!();
        /*
            return std::tie (a.start, a.end, a.interval, a.skew, a.symmetricSkew)
               == std::tie (b.start, b.end, b.interval, b.skew, b.symmetricSkew);
        */
    }
}

impl<ValueType> Eq for NormalisableRange<ValueType> {}

impl<ValueType> Default for NormalisableRange<ValueType> {
    
    /**
      | Creates a continuous range that performs
      | a dummy mapping.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

pub trait HasValueRemapFunction {
    type ValueRemapFunction;
}

impl<ValueType> HasValueRemapFunction for NormalisableRange<ValueType> {

    /**
      | A function object which can remap a value
      | in some way based on the start and end
      | of a range.
      |
      */
    type ValueRemapFunction = fn(
        range_start:    ValueType,
        range_end:      ValueType,
        value_to_remap: ValueType
    ) -> ValueType;
}

impl<ValueType> NormalisableRange<ValueType> {

    /**
      | Creates a NormalisableRange with a
      | given range, interval and skew factor.
      |
      */
    pub fn new(
        range_start:        ValueType,
        range_end:          ValueType,
        interval_value:     ValueType,
        skew_factor:        ValueType,
        use_symmetric_skew: Option<bool>) -> Self {

        let use_symmetric_skew: bool =
            use_symmetric_skew.unwrap_or(false);

        todo!();
        /*
        : start(rangeStart),
        : end(rangeEnd),
        : interval(intervalValue),
        : skew(skewFactor),
        : symmetric_skew(useSymmetricSkew),

            checkInvariants();
        */
    }

    /**
      | Creates a NormalisableRange with a
      | given range, continuous interval,
      | but a dummy skew-factor.
      |
      */
    pub fn new_from_range_by_start_and_end(
        range_start: ValueType,
        range_end:   ValueType) -> Self {
    
        todo!();
        /*
        : start(rangeStart),
        : end(rangeEnd),

            checkInvariants();
        */
    }

    /**
      | Creates a NormalisableRange with a
      | given range and interval, but a dummy
      | skew-factor.
      |
      */
    pub fn new_from_range_by_start_and_end_with_interval(
        range_start:    ValueType,
        range_end:      ValueType,
        interval_value: ValueType) -> Self {
    
        todo!();
        /*
        : start(rangeStart),
        : end(rangeEnd),
        : interval(intervalValue),

            checkInvariants();
        */
    }

    /**
      | Creates a NormalisableRange with a
      | given range, continuous interval,
      | but a dummy skew-factor.
      |
      */
    pub fn new_from_range(range: Range<ValueType>) -> Self {
    
        todo!();
        /*
        : normalisable_range(range.getStart(), range.getEnd()),

        
        */
    }

    /**
      | Creates a NormalisableRange with a
      | given range and interval, but a dummy
      | skew-factor.
      |
      */
    pub fn new_from_range_with_interval(
        range:          Range<ValueType>,
        interval_value: ValueType) -> Self {
    
        todo!();
        /*
        : normalisable_range(range.getStart(), range.getEnd(), intervalValue),

        
        */
    }

    /**
      | Creates a NormalisableRange with a
      | given range and an injective mapping
      | function.
      | 
      | -----------
      | @param rangeStart
      | 
      | The minimum value in the range.
      | ----------
      | @param rangeEnd
      | 
      | The maximum value in the range.
      | ----------
      | @param convertFrom0To1Func
      | 
      | A function which uses the current start
      | and end of this NormalisableRange and
      | produces a mapped value from a normalised
      | value.
      | ----------
      | @param convertTo0To1Func
      | 
      | A function which uses the current start
      | and end of this NormalisableRange and
      | produces a normalised value from a mapped
      | value.
      | ----------
      | @param snapToLegalValueFunc
      | 
      | A function which uses the current start
      | and end of this NormalisableRange to
      | take a mapped value and snap it to the
      | nearest legal value.
      |
      */
    pub fn new_from_range_by_start_and_end_with_conversion_and_snap(
        range_start:              ValueType,
        range_end:                ValueType,
        convert_from_0to_1func:   <Self as HasValueRemapFunction>::ValueRemapFunction,
        convert_to_0to_1func:     <Self as HasValueRemapFunction>::ValueRemapFunction,
        snap_to_legal_value_func: <Self as HasValueRemapFunction>::ValueRemapFunction) -> Self {
    
        todo!();
        /*


            : start (rangeStart),
              end   (rangeEnd),
              convertFrom0To1Function  (std::move (convertFrom0To1Func)),
              convertTo0To1Function    (std::move (convertTo0To1Func)),
              snapToLegalValueFunction (std::move (snapToLegalValueFunc))

            checkInvariants();
        */
    }

    /**
      | Uses the properties of this mapping
      | to convert a non-normalised value to
      | its 0->1 representation.
      |
      */
    pub fn convert_to0to1(&self, v: ValueType) -> ValueType {
        
        todo!();
        /*
            if (convertTo0To1Function != nullptr)
                return clampTo0To1 (convertTo0To1Function (start, end, v));

            auto proportion = clampTo0To1 ((v - start) / (end - start));

            if (skew == static_cast<ValueType> (1))
                return proportion;

            if (! symmetricSkew)
                return std::pow (proportion, skew);

            auto distanceFromMiddle = static_cast<ValueType> (2) * proportion - static_cast<ValueType> (1);

            return (static_cast<ValueType> (1) + std::pow (std::abs (distanceFromMiddle), skew)
                                               * (distanceFromMiddle < ValueType() ? static_cast<ValueType> (-1)
                                                                                   : static_cast<ValueType> (1)))
                   / static_cast<ValueType> (2);
        */
    }

    /**
      | Uses the properties of this mapping
      | to convert a normalised 0->1 value to
      | its full-range representation.
      |
      */
    pub fn convert_from0to1(&self, proportion: ValueType) -> ValueType {
        
        todo!();
        /*
            proportion = clampTo0To1 (proportion);

            if (convertFrom0To1Function != nullptr)
                return convertFrom0To1Function (start, end, proportion);

            if (! symmetricSkew)
            {
                if (skew != static_cast<ValueType> (1) && proportion > ValueType())
                    proportion = std::exp (std::log (proportion) / skew);

                return start + (end - start) * proportion;
            }

            auto distanceFromMiddle = static_cast<ValueType> (2) * proportion - static_cast<ValueType> (1);

            if (skew != static_cast<ValueType> (1) && distanceFromMiddle != static_cast<ValueType> (0))
                distanceFromMiddle = std::exp (std::log (std::abs (distanceFromMiddle)) / skew)
                                     * (distanceFromMiddle < ValueType() ? static_cast<ValueType> (-1)
                                                                         : static_cast<ValueType> (1));

            return start + (end - start) / static_cast<ValueType> (2) * (static_cast<ValueType> (1) + distanceFromMiddle);
        */
    }

    /**
      | Takes a non-normalised value and snaps
      | it based on either the interval property
      | of this NormalisableRange or the lambda
      | function supplied to the constructor.
      |
      */
    pub fn snap_to_legal_value(&self, v: ValueType) -> ValueType {
        
        todo!();
        /*
            if (snapToLegalValueFunction != nullptr)
                return snapToLegalValueFunction (start, end, v);

            if (interval > ValueType())
                v = start + interval * std::floor ((v - start) / interval + static_cast<ValueType> (0.5));

            return (v <= start || end <= start) ? start : (v >= end ? end : v);
        */
    }

    /**
      | Returns the extent of the normalisable
      | range.
      |
      */
    pub fn get_range(&self) -> Range<ValueType> {
        
        todo!();
        /*
            return { start, end };
        */
    }

    /**
      | Given a value which is between the start
      | and end points, this sets the skew such
      | that convertFrom0to1 (0.5) will return
      | this value.
      | 
      | If you have used lambda functions for
      | convertFrom0to1Func and convertFrom0to1Func
      | in the constructor of this class then
      | the skew value is ignored.
      | 
      | -----------
      | @param centrePointValue
      | 
      | this must be greater than the start of
      | the range and less than the end.
      |
      */
    pub fn set_skew_for_centre(&mut self, centre_point_value: ValueType)  {
        
        todo!();
        /*
            jassert (centrePointValue > start);
            jassert (centrePointValue < end);

            symmetricSkew = false;
            skew = std::log (static_cast<ValueType> (0.5)) / std::log ((centrePointValue - start) / (end - start));
            checkInvariants();
        */
    }
    
    pub fn check_invariants(&self)  {
        
        todo!();
        /*
            jassert (end > start);
            jassert (interval >= ValueType());
            jassert (skew > ValueType());
        */
    }
    
    pub fn clamp_to_0to1(value: ValueType) -> ValueType {
        
        todo!();
        /*
            auto clampedValue = jlimit (static_cast<ValueType> (0), static_cast<ValueType> (1), value);

            // If you hit this assertion then either your normalisation function is not working
            // correctly or your input is out of the expected bounds.
            jassert (clampedValue == value);

            return clampedValue;
        */
    }
}
