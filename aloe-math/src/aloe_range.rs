crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/maths/aloe_Range.h]

/**
  | A general-purpose range object, that
  | simply represents any linear range
  | with a start and end point.
  | 
  | -----------
  | @note
  | 
  | when checking whether values fall within
  | the range, the start value is considered
  | to be inclusive, and the end of the range
  | exclusive.
  | 
  | The templated parameter is expected
  | to be a primitive integer or floating
  | point type, though class types could
  | also be used if they behave in a number-like
  | way.
  | 
  | @tags{Core}
  |
  */
pub struct Range<ValueType> {
    start: ValueType,
    end:   ValueType,
}

impl<ValueType> Default for Range<ValueType> {
    
    /**
      | Constructs an empty range.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl<ValueType> AddAssign<&ValueType> for Range<ValueType> {
    
    /**
      | Adds an amount to the start and end of
      | the range.
      |
      */
    fn add_assign(&mut self, other: &ValueType) {
        todo!();
        /*
            start += amountToAdd;
            end += amountToAdd;
            return *this;
        */
    }
}

impl<ValueType> SubAssign<ValueType> for Range<ValueType> {
    
    /**
      | Subtracts an amount from the start and
      | end of the range.
      |
      */
    fn sub_assign(&mut self, amount_to_subtract: ValueType) {
        todo!();
        /*
            start -= amountToSubtract;
            end -= amountToSubtract;
            return *this;
        */
    }
}

impl<ValueType> Add<ValueType> for Range<ValueType> {

    type Output = Range<ValueType>;

    /**
      Returns a range that is equal to this one with
      an amount added to its start and end.
      */
    #[inline]fn add(self, other: ValueType) -> Self::Output {
        todo!();
        /*
            return Range (start + amountToAdd, end + amountToAdd);
        */
    }
}

impl<ValueType> Sub<ValueType> for Range<ValueType> {

    type Output = Range<ValueType>;

    /**
      | Returns a range that is equal to this one with
      | the specified amount subtracted from its start
      | and end.
      */
    #[inline]fn sub(self, other: ValueType) -> Self::Output {
        todo!();
        /*
            return Range (start - amountToSubtract, end - amountToSubtract);
        */
    }
}

impl<ValueType> PartialEq<Range<ValueType>> for Range<ValueType> {
    
    #[inline] fn eq(&self, other: &Range<ValueType>) -> bool {
        todo!();
        /*
            return start == other.start && end == other.end;
        */
    }
}

impl<ValueType> Eq for Range<ValueType> {}

impl<ValueType> Range<ValueType> {

    /**
      | Constructs a range with given start
      | and end values.
      |
      */
    pub fn new(
        start_value: ValueType,
        end_value:   ValueType) -> Self {
    
        todo!();
        /*
        : start(startValue),
        : end(jmax (startValue, endValue)),

        
        */
    }

    /**
      | Returns the range that lies between
      | two positions (in either order).
      |
      */
    pub fn between(
        position1: ValueType,
        position2: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return position1 < position2 ? Range (position1, position2)
                                         : Range (position2, position1);
        */
    }

    /**
      | Returns a range with a given start and
      | length.
      |
      */
    pub fn with_start_and_length(
        start_value: ValueType,
        length:      ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            jassert (length >= ValueType());
            return Range (startValue, startValue + length);
        */
    }

    /**
      | Returns a range with the specified start
      | position and a length of zero.
      |
      */
    pub fn empty_range(start: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (start, start);
        */
    }
    
    /**
      | Returns the start of the range.
      |
      */
    #[inline] pub fn get_start(&self) -> ValueType {
        
        todo!();
        /*
            return start;
        */
    }

    /**
      | Returns the length of the range.
      |
      */
    #[inline] pub fn get_length(&self) -> ValueType {
        
        todo!();
        /*
            return end - start;
        */
    }

    /**
      | Returns the end of the range.
      |
      */
    #[inline] pub fn get_end(&self) -> ValueType {
        
        todo!();
        /*
            return end;
        */
    }

    /**
      | Returns true if the range has a length
      | of zero.
      |
      */
    #[inline] pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return start == end;
        */
    }

    /**
      | Changes the start position of the range,
      | leaving the end position unchanged.
      | 
      | If the new start position is higher than
      | the current end of the range, the end
      | point will be pushed along to equal it,
      | leaving an empty range at the new position.
      |
      */
    pub fn set_start(&mut self, new_start: ValueType)  {
        
        todo!();
        /*
            start = newStart;
            if (end < newStart)
                end = newStart;
        */
    }

    /**
      | Returns a range with the same end as this
      | one, but a different start.
      | 
      | If the new start position is higher than
      | the current end of the range, the end
      | point will be pushed along to equal it,
      | returning an empty range at the new position.
      |
      */
    pub fn with_start(&self, new_start: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (newStart, jmax (newStart, end));
        */
    }

    /**
      | Returns a range with the same length
      | as this one, but moved to have the given
      | start position.
      |
      */
    pub fn moved_to_start_at(&self, new_start: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (newStart, end + (newStart - start));
        */
    }

    /**
      | Changes the end position of the range,
      | leaving the start unchanged.
      | 
      | If the new end position is below the current
      | start of the range, the start point will
      | be pushed back to equal the new end point.
      |
      */
    pub fn set_end(&mut self, new_end: ValueType)  {
        
        todo!();
        /*
            end = newEnd;
            if (newEnd < start)
                start = newEnd;
        */
    }

    /**
      | Returns a range with the same start position
      | as this one, but a different end.
      | 
      | If the new end position is below the current
      | start of the range, the start point will
      | be pushed back to equal the new end point.
      |
      */
    pub fn with_end(&self, new_end: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (jmin (start, newEnd), newEnd);
        */
    }

    /**
      | Returns a range with the same length
      | as this one, but moved to have the given
      | end position.
      |
      */
    pub fn moved_to_end_at(&self, new_end: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (start + (newEnd - end), newEnd);
        */
    }

    /**
      | Changes the length of the range.
      | 
      | Lengths less than zero are treated as
      | zero.
      |
      */
    pub fn set_length(&mut self, new_length: ValueType)  {
        
        todo!();
        /*
            end = start + jmax (ValueType(), newLength);
        */
    }

    /**
      | Returns a range with the same start as
      | this one, but a different length.
      | 
      | Lengths less than zero are treated as
      | zero.
      |
      */
    pub fn with_length(&self, new_length: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (start, start + newLength);
        */
    }

    /**
      | Returns a range which has its start moved
      | down and its end moved up by the given
      | amount.
      | 
      | -----------
      | @return
      | 
      | The returned range will be (start - amount,
      | end + amount)
      |
      */
    pub fn expanded(&self, amount: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (start - amount, end + amount);
        */
    }
    
    /**
      | Returns true if the given position lies
      | inside this range.
      | 
      | When making this comparison, the start
      | value is considered to be inclusive,
      | and the end of the range exclusive.
      |
      */
    pub fn contains(&self, position: ValueType) -> bool {
        
        todo!();
        /*
            return start <= position && position < end;
        */
    }

    /**
      | Returns the nearest value to the one
      | supplied, which lies within the range.
      |
      */
    pub fn clip_value(&self, value: ValueType) -> ValueType {
        
        todo!();
        /*
            return jlimit (start, end, value);
        */
    }

    /**
      | Returns true if the given range lies
      | entirely inside this range.
      |
      */
    pub fn contains_range(&self, other: Range<ValueType>) -> bool {
        
        todo!();
        /*
            return start <= other.start && end >= other.end;
        */
    }

    /**
      | Returns true if the given range intersects
      | this one.
      |
      */
    pub fn intersects(&self, other: Range<ValueType>) -> bool {
        
        todo!();
        /*
            return other.start < end && start < other.end;
        */
    }

    /**
      | Returns the range that is the intersection
      | of the two ranges, or an empty range with
      | an undefined start position if they
      | don't overlap.
      |
      */
    pub fn get_intersection_with(&self, other: Range<ValueType>) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (jmax (start, other.start),
                          jmin (end, other.end));
        */
    }

    /**
      | Returns the smallest range that contains
      | both this one and the other one.
      |
      */
    pub fn get_union_with_range(&self, other: Range<ValueType>) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (jmin (start, other.start),
                          jmax (end, other.end));
        */
    }

    /**
      | Returns the smallest range that contains
      | both this one and the given value.
      |
      */
    pub fn get_union_with(&self, value_to_include: ValueType) -> Range<ValueType> {
        
        todo!();
        /*
            return Range (jmin (valueToInclude, start),
                          jmax (valueToInclude, end));
        */
    }

    /**
      | Returns a given range, after moving
      | it forwards or backwards to fit it within
      | this range.
      | 
      | If the supplied range has a greater length
      | than this one, the return value will
      | be this range.
      | 
      | Otherwise, if the supplied range is
      | smaller than this one, the return value
      | will be the new range, shifted forwards
      | or backwards so that it doesn't extend
      | beyond this one, but keeping its original
      | length.
      |
      */
    pub fn constrain_range(&self, range_to_constrain: Range<ValueType>) -> Range<ValueType> {
        
        todo!();
        /*
            const ValueType otherLen = rangeToConstrain.getLength();
            return getLength() <= otherLen
                    ? *this
                    : rangeToConstrain.movedToStartAt (jlimit (start, end - otherLen, rangeToConstrain.getStart()));
        */
    }

    /**
      | Scans an array of values for its min and
      | max, and returns these as a Range.
      |
      */
    pub fn find_min_and_max(
        values:     *const ValueType,
        num_values: i32) -> Range<ValueType> {
        
        todo!();
        /*
            if (numValues <= 0)
                return Range();

            const ValueType first (*values++);
            Range r (first, first);

            while (--numValues > 0) // (> 0 rather than >= 0 because we've already taken the first sample)
            {
                const ValueType v (*values++);

                if (r.end < v)    r.end = v;
                if (v < r.start)  r.start = v;
            }

            return r;
        */
    }
}
