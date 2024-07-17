crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_SparseSet.h]

/**
  | Holds a set of primitive values, storing
  | them as a set of ranges.
  | 
  | This container acts like an array, but
  | can efficiently hold large contiguous
  | ranges of values. It's quite a specialised
  | class, mostly useful for things like
  | keeping the set of selected rows in a
  | listbox.
  | 
  | The type used as a template parameter
  | must be an integer type, such as int,
  | short, int64, etc.
  | 
  | @tags{Core}
  |
  */
pub struct SparseSet<Type> {
    ranges: Vec<Range<Type>>,
}

impl<Type> Default for SparseSet<Type> {

    fn default() -> Self {
        todo!();
    }
}

impl<Type> Index<Type> for SparseSet<Type> {

    type Output = Type;
    
    /**
      | Returns one of the values in the set.
      | 
      | -----------
      | @param index
      | 
      | the index of the value to retrieve, in
      | the range 0 to (size() - 1).
      | 
      | -----------
      | @return
      | 
      | the value at this index, or 0 if it's out-of-range
      |
      */
    #[inline] fn index(&self, index: Type) -> &Self::Output {
        todo!();
        /*
            Type total = {};

            for (auto& r : ranges)
            {
                auto end = total + r.getLength();

                if (index < end)
                    return r.getStart() + (index - total);

                total = end;
            }

            return {};
        */
    }
}

impl<Type> PartialEq<SparseSet<Type>> for SparseSet<Type> {
    
    #[inline] fn eq(&self, other: &SparseSet<Type>) -> bool {
        todo!();
        /*
            return ranges == other.ranges;
        */
    }
}

impl<Type> Eq for SparseSet<Type> {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_SparseSet.cpp]
impl<Type> SparseSet<Type> {

    pub fn new(other: SparseSet<Type>) -> Self {
    
        todo!();
        /*
        : ranges(std::move (other.ranges)),

        
        */
    }
    
    pub fn assign_from(&mut self, other: SparseSet<Type>) -> &mut SparseSet<Type> {
        
        todo!();
        /*
            ranges = std::move (other.ranges); return *this;
        */
    }
    
    /**
      | Clears the set.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            ranges.clear();
        */
    }

    /**
      | Checks whether the set is empty.
      | 
      | This is much quicker than using (size()
      | == 0).
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return ranges.isEmpty();
        */
    }

    /**
      | Returns the number of values in the set.
      | 
      | Because of the way the data is stored,
      | this method can take longer if there
      | are a lot of items in the set. Use isEmpty()
      | for a quick test of whether there are
      | any items.
      |
      */
    pub fn size(&self) -> Type {
        
        todo!();
        /*
            Type total = {};

            for (auto& r : ranges)
                total += r.getLength();

            return total;
        */
    }

    /**
      | Checks whether a particular value is
      | in the set.
      |
      */
    pub fn contains(&self, value_to_look_for: Type) -> bool {
        
        todo!();
        /*
            for (auto& r : ranges)
            {
                if (r.getStart() > valueToLookFor)
                    break;

                if (r.getEnd() > valueToLookFor)
                    return true;
            }

            return false;
        */
    }

    /**
      | Returns the number of contiguous blocks
      | of values. @see getRange
      |
      */
    pub fn get_num_ranges(&self) -> i32 {
        
        todo!();
        /*
            return ranges.size();
        */
    }

    /**
      | Returns one of the contiguous ranges
      | of values stored.
      | 
      | -----------
      | @param rangeIndex
      | 
      | the index of the range to look up, between
      | 0 and (getNumRanges() - 1) @see getTotalRange
      |
      */
    pub fn get_range(&self, range_index: i32) -> Range<Type> {
        
        todo!();
        /*
            return ranges[rangeIndex];
        */
    }

    /**
      | Returns the range between the lowest
      | and highest values in the set. @see getRange
      |
      */
    pub fn get_total_range(&self) -> Range<Type> {
        
        todo!();
        /*
            if (ranges.isEmpty())
                return {};

            return { ranges.getFirst().getStart(),
                     ranges.getLast().getEnd() };
        */
    }

    /**
      | Adds a range of contiguous values to
      | the set. e.g. addRange (Range \<int\>
      | (10, 14)) will add (10, 11, 12, 13) to
      | the set.
      |
      */
    pub fn add_range(&mut self, range: Range<Type>)  {
        
        todo!();
        /*
            if (! range.isEmpty())
            {
                removeRange (range);
                ranges.add (range);
                std::sort (ranges.begin(), ranges.end(),
                           [] (Range<Type> a, Range<Type> b) { return a.getStart() < b.getStart(); });
                simplify();
            }
        */
    }

    /**
      | Removes a range of values from the set.
      | e.g. removeRange (Range\<int\> (10,
      | 14)) will remove (10, 11, 12, 13) from
      | the set.
      |
      */
    pub fn remove_range(&mut self, range_to_remove: Range<Type>)  {
        
        todo!();
        /*
            if (getTotalRange().intersects (rangeToRemove) && ! rangeToRemove.isEmpty())
            {
                for (int i = ranges.size(); --i >= 0;)
                {
                    auto& r = ranges.getReference(i);

                    if (r.getEnd() <= rangeToRemove.getStart())
                        break;

                    if (r.getStart() >= rangeToRemove.getEnd())
                        continue;

                    if (rangeToRemove.contains (r))
                    {
                        ranges.remove (i);
                    }
                    else if (r.contains (rangeToRemove))
                    {
                        auto r1 = r.withEnd (rangeToRemove.getStart());
                        auto r2 = r.withStart (rangeToRemove.getEnd());

                        // this should be covered in if (rangeToRemove.contains (r))
                        jassert (! r1.isEmpty() || ! r2.isEmpty());

                        r = r1;

                        if (r.isEmpty())
                            r = r2;

                        if (! r1.isEmpty() && ! r2.isEmpty())
                            ranges.insert (i + 1, r2);
                    }
                    else if (rangeToRemove.getEnd() > r.getEnd())
                    {
                        r.setEnd (rangeToRemove.getStart());
                    }
                    else
                    {
                        r.setStart (rangeToRemove.getEnd());
                    }
                }
            }
        */
    }

    /**
      | Does an XOR of the values in a given range.
      |
      */
    pub fn invert_range(&mut self, range: Range<Type>)  {
        
        todo!();
        /*
            SparseSet newItems;
            newItems.addRange (range);

            for (auto& r : ranges)
                newItems.removeRange (r);

            removeRange (range);

            for (auto& r : newItems.ranges)
                addRange (r);
        */
    }

    /**
      | Checks whether any part of a given range
      | overlaps any part of this set.
      |
      */
    pub fn overlaps_range(&self, range: Range<Type>) -> bool {
        
        todo!();
        /*
            if (! range.isEmpty())
                for (auto& r : ranges)
                    if (r.intersects (range))
                        return true;

            return false;
        */
    }

    /**
      | Checks whether the whole of a given range
      | is contained within this one.
      |
      */
    pub fn contains_range(&self, range: Range<Type>) -> bool {
        
        todo!();
        /*
            if (! range.isEmpty())
                for (auto& r : ranges)
                    if (r.contains (range))
                        return true;

            return false;
        */
    }

    /**
      | Returns the set as a list of ranges, which
      | you may want to iterate over.
      |
      */
    pub fn get_ranges(&self) -> &[Range<Type>] {
        
        todo!();
        /*
            return ranges;
        */
    }
    
    pub fn simplify(&mut self)  {
        
        todo!();
        /*
            for (int i = ranges.size(); --i > 0;)
            {
                auto& r1 = ranges.getReference (i - 1);
                auto& r2 = ranges.getReference (i);

                if (r1.getEnd() == r2.getStart())
                {
                    r1.setEnd (r2.getEnd());
                    ranges.remove (i);
                }
            }
        */
    }
}

#[test] fn sparse_set_tests() {

    todo!();

    /*
    #if ALOE_UNIT_TESTS

    class SparseSetTests  : public UnitTest
    {

        SparseSetTests()
            : UnitTest ("SparseSet class", UnitTestCategories::containers)
        {}

        void runTest() override
        {
            beginTest ("basic operations");
            {
                SparseSet<int> set;

                expect (set.isEmpty());
                expectEquals (set.size(), 0);
                expectEquals (set.getNumRanges(), 0);
                expect (set.getTotalRange().isEmpty());

                set.addRange ({0, 10});
                expect (! set.isEmpty());
                expectEquals (set.size(), 10);
                expectEquals (set.getNumRanges(), 1);
                expect (! set.getTotalRange().isEmpty());
                expect (set.getRange (0) == Range<int> (0, 10));

                expectEquals (set[0], 0);
                expectEquals (set[5], 5);
                expectEquals (set[9], 9);
                // Index out of range yields a default value for a type
                expectEquals (set[10], 0);
                expect (set.contains (0));
                expect (set.contains (9));
                expect (! set.contains (10));
            }

            beginTest ("adding ranges");
            {
                SparseSet<int> set;

                // Adding same range twice should yield just a single range
                set.addRange ({0, 10});
                set.addRange ({0, 10});
                expectEquals (set.getNumRanges(), 1);
                expect (set.getRange (0) == Range<int> (0, 10));

                // Adding already included range does not increase num ranges
                set.addRange ({0, 2});
                expectEquals (set.getNumRanges(), 1);
                set.addRange ({8, 10});
                expectEquals (set.getNumRanges(), 1);
                set.addRange ({2, 5});
                expectEquals (set.getNumRanges(), 1);

                // Adding non adjacent range includes total number of ranges
                set.addRange ({-10, -5});
                expectEquals (set.getNumRanges(), 2);
                expect (set.getRange (0) == Range<int> (-10, -5));
                expect (set.getRange (1) == Range<int> (0, 10));
                expect (set.getTotalRange() == Range<int> (-10, 10));

                set.addRange ({15, 20});
                expectEquals (set.getNumRanges(), 3);
                expect (set.getRange (0) == Range<int> (-10, -5));
                expect (set.getRange (1) == Range<int> (0, 10));
                expect (set.getRange (2) == Range<int> (15, 20));
                expect (set.getTotalRange() == Range<int> (-10, 20));

                // Adding adjacent ranges merges them.
                set.addRange ({-5, -3});
                expectEquals (set.getNumRanges(), 3);
                expect (set.getRange (0) == Range<int> (-10, -3));
                expect (set.getRange (1) == Range<int> (0, 10));
                expect (set.getRange (2) == Range<int> (15, 20));
                expect (set.getTotalRange() == Range<int> (-10, 20));

                set.addRange ({20, 25});
                expectEquals (set.getNumRanges(), 3);
                expect (set.getRange (0) == Range<int> (-10, -3));
                expect (set.getRange (1) == Range<int> (0, 10));
                expect (set.getRange (2) == Range<int> (15, 25));
                expect (set.getTotalRange() == Range<int> (-10, 25));

                // Adding range containing other ranges merges them
                set.addRange ({-50, 50});
                expectEquals (set.getNumRanges(), 1);
                expect (set.getRange (0) == Range<int> (-50, 50));
                expect (set.getTotalRange() == Range<int> (-50, 50));
            }

            beginTest ("removing ranges");
            {
                SparseSet<int> set;

                set.addRange ({-20, -10});
                set.addRange ({0, 10});
                set.addRange ({20, 30});
                expectEquals (set.getNumRanges(), 3);

                // Removing ranges not included in the set has no effect
                set.removeRange ({-5, 5});
                expectEquals (set.getNumRanges(), 3);

                // Removing partially overlapping range
                set.removeRange ({-15, 5});
                expectEquals (set.getNumRanges(), 3);
                expect (set.getRange (0) == Range<int> (-20, -15));
                expect (set.getRange (1) == Range<int> (5, 10));
                expect (set.getRange (2) == Range<int> (20, 30));

                // Removing subrange of existing range
                set.removeRange ({20, 22});
                expectEquals (set.getNumRanges(), 3);
                expect (set.getRange (2) == Range<int> (22, 30));

                set.removeRange ({28, 30});
                expectEquals (set.getNumRanges(), 3);
                expect (set.getRange (2) == Range<int> (22, 28));

                set.removeRange ({24, 26});
                expectEquals (set.getNumRanges(), 4);
                expect (set.getRange (0) == Range<int> (-20, -15));
                expect (set.getRange (1) == Range<int> (5, 10));
                expect (set.getRange (2) == Range<int> (22, 24));
                expect (set.getRange (3) == Range<int> (26, 28));
            }

            beginTest ("XORing ranges");
            {
                SparseSet<int> set;
                set.addRange ({0, 10});

                set.invertRange ({0, 10});
                expectEquals (set.getNumRanges(), 0);
                set.invertRange ({0, 10});
                expectEquals (set.getNumRanges(), 1);

                set.invertRange ({4, 6});
                expectEquals (set.getNumRanges(), 2);
                expect (set.getRange (0) == Range<int> (0, 4));
                expect (set.getRange (1) == Range<int> (6, 10));

                set.invertRange ({-2, 2});
                expectEquals (set.getNumRanges(), 3);
                expect (set.getRange (0) == Range<int> (-2, 0));
                expect (set.getRange (1) == Range<int> (2, 4));
                expect (set.getRange (2) == Range<int> (6, 10));
            }

            beginTest ("range contains & overlaps checks");
            {
                SparseSet<int> set;
                set.addRange ({0, 10});

                expect (set.containsRange (Range<int> (0, 2)));
                expect (set.containsRange (Range<int> (8, 10)));
                expect (set.containsRange (Range<int> (0, 10)));

                expect (! set.containsRange (Range<int> (-2, 0)));
                expect (! set.containsRange (Range<int> (-2, 10)));
                expect (! set.containsRange (Range<int> (10, 12)));
                expect (! set.containsRange (Range<int> (0, 12)));

                expect (set.overlapsRange (Range<int> (0, 2)));
                expect (set.overlapsRange (Range<int> (8, 10)));
                expect (set.overlapsRange (Range<int> (0, 10)));

                expect (! set.overlapsRange (Range<int> (-2, 0)));
                expect (  set.overlapsRange (Range<int> (-2, 10)));
                expect (! set.overlapsRange (Range<int> (10, 12)));
                expect (  set.overlapsRange (Range<int> (0, 12)));
            }
        }
    };

    static SparseSetTests sparseSetTests;

    #endif
    */
}

