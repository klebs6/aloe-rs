crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_StringPairArray.h]

/**
  | A container for holding a set of strings
  | which are keyed by another string.
  | 
  | @see StringArray
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct StringPairArray {
    keys:        StringArray,
    values:      StringArray,
    ignore_case: bool,
}

impl PartialEq<StringPairArray> for StringPairArray {
    
    /**
      | Compares two arrays.
      | 
      | Comparisons are case-sensitive.
      | 
      | -----------
      | @return
      | 
      | true only if the other array contains
      | exactly the same strings with the same
      | keys
      |
      */
    #[inline] fn eq(&self, other: &StringPairArray) -> bool {
        todo!();
        /*
            auto num = size();

        if (num != other.size())
            return false;

        for (int i = 0; i < num; ++i)
        {
            if (keys[i] == other.keys[i]) // optimise for the case where the keys are in the same order
            {
                if (values[i] != other.values[i])
                    return false;
            }
            else
            {
                // if we encounter keys that are in a different order, search remaining items by brute force..
                for (int j = i; j < num; ++j)
                {
                    auto otherIndex = other.keys.indexOf (keys[j], other.ignoreCase);

                    if (otherIndex < 0 || values[j] != other.values[otherIndex])
                        return false;
                }

                return true;
            }
        }

        return true;
        */
    }
}

impl Eq for StringPairArray {}

impl Index<&str> for StringPairArray {

    type Output = AloeString;
    
    /**
      | Finds the value corresponding to a key
      | string.
      | 
      | If no such key is found, this will just
      | return an empty string. To check whether
      | a given key actually exists (because
      | it might actually be paired with an empty
      | string), use the getAllKeys() method
      | to obtain a list.
      | 
      | Obviously the reference returned shouldn't
      | be stored for later use, as the string
      | it refers to may disappear when the array
      | changes.
      | 
      | @see getValue
      |
      */
    #[inline] fn index(&self, key: &str) -> &Self::Output {
        todo!();
        /*
            return values[keys.indexOf (key, ignoreCase)];
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_StringPairArray.cpp]
impl StringPairArray {

    pub fn get_all_keys(&self) -> &StringArray {
        
        todo!();
        /*
            return keys;
        */
    }

    /**
      | Returns a list of all values in the array.
      |
      */
    pub fn get_all_values(&self) -> &StringArray {
        
        todo!();
        /*
            return values;
        */
    }

    /**
      | Returns the number of strings in the
      | array
      |
      */
    #[inline] pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return keys.size();
        */
    }
    
    /**
      | Creates an empty array
      |
      */
    pub fn new(should_ignore_case: Option<bool>) -> Self {

        let should_ignore_case: bool = should_ignore_case.unwrap_or(true);
    
        todo!();
        /*
        : ignore_case(shouldIgnoreCase),

        
        */
    }
    
    /**
      | Creates a copy of another array
      |
      */
    pub fn new_from_other(other: &StringPairArray) -> Self {
    
        todo!();
        /*
            : keys (other.keys),
          values (other.values),
          ignoreCase (other.ignoreCase)
        */
    }
    
    /**
      | Copies the contents of another string
      | array into this one
      |
      */
    pub fn assign_from(&mut self, other: &StringPairArray) -> &mut StringPairArray {
        
        todo!();
        /*
            keys = other.keys;
        values = other.values;
        return *this;
        */
    }
    
    /**
      | Finds the value corresponding to a key
      | string.
      | 
      | If no such key is found, this will just
      | return the value provided as a default.
      | @see operator[]
      |
      */
    pub fn get_value(&self, 
        key:                  &str,
        default_return_value: &AloeString) -> AloeString {
        
        todo!();
        /*
            auto i = keys.indexOf (key, ignoreCase);

        if (i >= 0)
            return values[i];

        return defaultReturnValue;
        */
    }
    
    /**
      | Returns true if the given key exists.
      |
      */
    pub fn contains_key(&self, key: &str) -> bool {
        
        todo!();
        /*
            return keys.contains (key, ignoreCase);
        */
    }

    /**
      | Adds or amends a key/value pair.
      | 
      | If a value already exists with this key,
      | its value will be overwritten, otherwise
      | the key/value pair will be added to the
      | array.
      |
      */
    pub fn set(&mut self, 
        key:   &AloeString,
        value: &AloeString)  {
        
        todo!();
        /*
            auto i = keys.indexOf (key, ignoreCase);

        if (i >= 0)
        {
            values.set (i, value);
        }
        else
        {
            keys.add (key);
            values.add (value);
        }
        */
    }

    /**
      | Adds the items from another array to
      | this one.
      | 
      | This is equivalent to using set() to
      | add each of the pairs from the other array.
      |
      */
    pub fn add_array(&mut self, other: &StringPairArray)  {
        
        todo!();
        /*
            for (int i = 0; i < other.size(); ++i)
            set (other.keys[i], other.values[i]);
        */
    }

    /**
      | Removes all elements from the array.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            keys.clear();
        values.clear();
        */
    }

    /**
      | Removes a string from the array based
      | on its key.
      | 
      | If the key isn't found, nothing will
      | happen.
      |
      */
    pub fn remove(&mut self, key: &str)  {
        
        todo!();
        /*
            remove (keys.indexOf (key, ignoreCase));
        */
    }

    /**
      | Removes a string from the array based
      | on its index.
      | 
      | If the index is out-of-range, no action
      | will be taken.
      |
      */
    pub fn remove_by_idx(&mut self, index: i32)  {
        
        todo!();
        /*
            keys.remove (index);
        values.remove (index);
        */
    }

    /**
      | Indicates whether to use a case-insensitive
      | search when looking up a key string.
      |
      */
    pub fn set_ignores_case(&mut self, should_ignore_case: bool)  {
        
        todo!();
        /*
            ignoreCase = shouldIgnoreCase;
        */
    }

    /**
      | Indicates whether a case-insensitive
      | search is used when looking up a key string.
      |
      */
    pub fn get_ignores_case(&self) -> bool {
        
        todo!();
        /*
            return ignoreCase;
        */
    }

    /**
      | Returns a descriptive string containing
      | the items.
      | 
      | This is handy for dumping the contents
      | of an array.
      |
      */
    pub fn get_description(&self) -> AloeString {
        
        todo!();
        /*
            AloeString s;

        for (int i = 0; i < keys.size(); ++i)
        {
            s << keys[i] << " = " << values[i];

            if (i < keys.size())
                s << ", ";
        }

        return s;
        */
    }

    /**
      | Reduces the amount of storage being
      | used by the array.
      | 
      | Arrays typically allocate slightly
      | more storage than they need, and after
      | removing elements, they may have quite
      | a lot of unused space allocated.
      | 
      | This method will reduce the amount of
      | allocated storage to a minimum.
      |
      */
    pub fn minimise_storage_overheads(&mut self)  {
        
        todo!();
        /*
            keys.minimiseStorageOverheads();
        values.minimiseStorageOverheads();
        */
    }

    /**
      | Adds the contents of a map to this StringPairArray.
      |
      */
    pub fn add_map(&mut self, to_add: &HashMap<AloeString,AloeString>)  {
        
        todo!();
        /*
            // If we just called `set` for each item in `toAdd`, that would
        // perform badly when adding to large StringPairArrays, as `set`
        // has to loop through the whole container looking for matching keys.
        // Instead, we use a temporary map to give us better lookup performance.
        std::map<AloeString, int> contents;

        const auto normaliseKey = [this] (const AloeString& key)
        {
            return ignoreCase ? key.toLowerCase() : key;
        };

        for (auto i = 0; i != size(); ++i)
            contents.emplace (normaliseKey (getAllKeys().getReference (i)), i);

        for (const auto& pair : toAdd)
        {
            const auto key = normaliseKey (pair.first);
            const auto it = contents.find (key);

            if (it != contents.cend())
            {
                values.getReference (it->second) = pair.second;
            }
            else
            {
                contents.emplace (key, static_cast<int> (contents.size()));
                keys.add (pair.first);
                values.add (pair.second);
            }
        }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct StringPairArrayTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for StringPairArrayTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("StringPairArray", UnitTestCategories::text
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl StringPairArrayTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("addMap respects case sensitivity of StringPairArray");
            {
                StringPairArray insensitive { true };
                insensitive.addMap ({ { "duplicate", "a" },
                                      { "Duplicate", "b" } });

                expect (insensitive.size() == 1);
                expectEquals (insensitive["DUPLICATE"], "a"_S);

                StringPairArray sensitive { false };
                sensitive.addMap ({ { "duplicate", "a"_S },
                                    { "Duplicate", "b"_S } });

                expect (sensitive.size() == 2);
                expectEquals (sensitive["duplicate"], "a"_S);
                expectEquals (sensitive["Duplicate"], "b"_S);
                expectEquals (sensitive["DUPLICATE"], ""_S);
            }

            beginTest ("addMap overwrites existing pairs");
            {
                StringPairArray insensitive { true };
                insensitive.set ("key", "value");
                insensitive.addMap ({ { "KEY", "VALUE" } });

                expect (insensitive.size() == 1);
                expectEquals (insensitive.getAllKeys()[0], "key"_S);
                expectEquals (insensitive.getAllValues()[0], "VALUE"_S);

                StringPairArray sensitive { false };
                sensitive.set ("key", "value");
                sensitive.addMap ({ { "KEY", "VALUE" },
                                    { "key", "another value" } });

                expect (sensitive.size() == 2);
                expect (sensitive.getAllKeys() == StringArray { "key", "KEY" });
                expect (sensitive.getAllValues() == StringArray { "another value", "VALUE" });
            }

            beginTest ("addMap doesn't change the order of existing keys");
            {
                StringPairArray array;
                array.set ("a", "a");
                array.set ("z", "z");
                array.set ("b", "b");
                array.set ("y", "y");
                array.set ("c", "c");

                array.addMap ({ { "B", "B" },
                                { "0", "0" },
                                { "Z", "Z" } });

                expect (array.getAllKeys() == StringArray { "a", "z", "b", "y", "c", "0" });
                expect (array.getAllValues() == StringArray { "a", "Z", "B", "y", "c", "0" });
            }

            beginTest ("addMap has equivalent behaviour to addArray");
            {
                StringPairArray initial;
                initial.set ("aaa", "aaa");
                initial.set ("zzz", "zzz");
                initial.set ("bbb", "bbb");

                auto withAddMap = initial;
                withAddMap.addMap ({ { "ZZZ", "ZZZ" },
                                     { "ddd", "ddd" } });

                auto withAddArray = initial;
                withAddArray.addArray ([]
                {
                    StringPairArray toAdd;
                    toAdd.set ("ZZZ", "ZZZ");
                    toAdd.set ("ddd", "ddd");
                    return toAdd;
                }());

                expect (withAddMap == withAddArray);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static StringPairArrayTests stringPairArrayTests;
    */
}
