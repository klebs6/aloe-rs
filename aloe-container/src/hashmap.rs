crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_HashMap.h]

pub const DEFAULT_HASH_TABLE_SIZE: usize = 101;

/**
  | A simple class to generate hash functions
  | for some primitive types, intended
  | for use with the HashMap class. @see
  | HashMap
  | 
  | @tags{Core}
  |
  */
pub struct DefaultHashFunctions {

}

impl DefaultHashFunctions {

    /**
      | Generates a simple hash from an unsigned
      | int.
      |
      */
    pub fn generate_hash_from_u32(
        key:         u32,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return (int) (key % (uint32) upperLimit);
        */
    }

    /**
      | Generates a simple hash from an integer.
      |
      */
    pub fn generate_hash_from_i32(
        key:         i32,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return generateHash ((uint32) key, upperLimit);
        */
    }

    /**
      | Generates a simple hash from a uint64.
      |
      */
    pub fn generate_hash_from_u64(
        key:         u64,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return (int) (key % (uint64) upperLimit);
        */
    }

    /**
      | Generates a simple hash from an int64.
      |
      */
    pub fn generate_hash_from_i64(
        key:         i64,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return generateHash ((uint64) key, upperLimit);
        */
    }

    /**
      | Generates a simple hash from a string.
      |
      */
    pub fn generate_hash_from_string(
        key:         &String,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return generateHash ((uint32) key.hashCode(), upperLimit);
        */
    }

    /**
      | Generates a simple hash from a variant.
      |
      */
    pub fn generate_hash_from_var(
        key:         &Var,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return generateHash (key.toString(), upperLimit);
        */
    }

    /**
      | Generates a simple hash from a void ptr.
      |
      */
    pub fn generate_hash_from_ptr(
        key:         *const c_void,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return generateHash ((uint64) (pointer_sized_uint) key, upperLimit);
        */
    }

    /**
      | Generates a simple hash from a UUID.
      |
      */
    pub fn generate_hash_from_uuid(
        key:         &Uuid,
        upper_limit: i32) -> i32 {
        
        todo!();
        /*
            return generateHash (key.hash(), upperLimit);
        */
    }
}

/**
  | Holds a set of mappings between some key/value
  | pairs.
  |
  | The types of the key and value objects are set
  | as template parameters.  You can also specify
  | a class to supply a hash function that
  | converts a key value into an hashed
  | integer. This class must have the form:
  |
  | @code
  | struct MyHashGenerator
  | {
  |     int generateHash (MyKeyType key, int upperLimit) const
  |     {
  |         // The function must return a value 0 <= x < upperLimit
  |         return someFunctionOfMyKeyType (key) % upperLimit;
  |     }
  | };
  | @endcode
  |
  | Like the Vec class, the key and value types
  | are expected to be copy-by-value types, so if
  | you define them to be pointer types, this
  | class won't delete the objects that they point
  | to.
  |
  | If you don't supply a class for the
  | HashFunctionType template parameter, the
  | default one provides some simple mappings for
  | strings and ints.
  |
  | @code
  |
  | HashMap<int, String> hash;
  | hash.set (1, "item1");
  | hash.set (2, "item2");
  |
  | DBG (hash [1]); // prints "item1"
  | DBG (hash [2]); // prints "item2"
  |
  | // This iterates the map, printing all of its key -> value pairs..
  | for (HashMap<int, String>::Iterator i (hash); i.next();)
  |     DBG (i.getKey() << " -> " << i.getValue());
  |
  | @endcode
  |
  | @tparam HashFunctionType The class of hash function, which must be copy-constructible.
  | @see CriticalSection, DefaultHashFunctions, NamedValueSet, SortedSet
  |
  | @tags{Core}
  */
#[no_copy]
#[leak_detector]
pub struct HashMap<
    K,
    V,
    HashFunctionType           = DefaultHashFunctions,
    TypeOfCriticalSectionToUse: HasScopedLockType = DummyCriticalSection
> 
{
    hash_function_to_use: HashFunctionType,
    hash_slots:           Vec<*mut hash_map::HashEntry<K,V>>,
    total_num_items:      i32, // default = 0
    lock:                 TypeOfCriticalSectionToUse,
}

pub mod hash_map {

    use super::*;
 
    /**
      | Returns the type of scoped lock to use
      | for locking this array
      |
      */
    pub type ScopedLockType<TypeOfCriticalSectionToUse: HasScopedLockType> 
        = <TypeOfCriticalSectionToUse as HasScopedLockType>::ScopedLockType;

    #[no_copy]
    pub struct HashEntry<K,V> {
        key:        K,
        value:      V,
        next_entry: *mut HashEntry<K,V>,
    }

    impl<K,V> HashEntry<K,V> {

        pub fn new_hash_entry(
            k:    K,
            val:  V,
            next: *mut HashEntry<K,V>) -> Self {
        
            todo!();
            /*
            : key(k),
            : value(val),
            : next_entry(next),

            
            */
        }
    }
    
    /** 
      | Iterates over the items in a HashMap.
      |
      | To use it, repeatedly call next() until it
      | returns false, e.g.
      |
      | @code
      | HashMap <String, String> myMap;
      |
      | HashMap<String, String>::Iterator i (myMap);
      |
      | while (i.next())
      | {
      | DBG (i.getKey() << " -> " << i.getValue());
      | }
      |
      | @endcode
      |
      | The order in which items are iterated
      | bears no resemblance to the order in which
      | they were originally added!
      |
      | Obviously as soon as you call any
      | non-const methods on the original
      | hash-map, any iterators that were created
      | beforehand will cease to be valid, and
      | should not be used.
      |
      | @see HashMap
      */
    #[leak_detector]
    pub struct Iterator<'a,K,V> {
        hash_map: &'a HashMap<K,V>,
        entry:    *mut HashEntry<K,V>,
        index:    i32,
    }

    impl<'a,K,V> Deref for Iterator<'a,K,V> {

        type Target = V;
        
        #[inline] fn deref(&self) -> &Self::Target {
            todo!();
            /*
                return getValue();
            */
        }
    }

    impl<'a,K,V> Iterator<'a,K,V> {

        pub fn new_from_hashmap(hash_map_to_iterate: &HashMap<K,V>) -> Self {
        
            todo!();
            /*
            : hash_map(hashMapToIterate),
            : entry(nullptr),
            : index(0),
            */
        }
        
        pub fn new_from_iterator(other: &Iterator<K,V>) -> Self {
        
            todo!();
            /*
            : hash_map(other.hashMap),
            : entry(other.entry),
            : index(other.index),

            
            */
        }

        /**
          | Moves to the next item, if one is available.
          | 
          | When this returns true, you can get the
          | item's key and value using getKey()
          | and getValue(). If it returns false,
          | the iteration has finished and you should
          | stop.
          |
          */
        pub fn next(&mut self) -> bool {
            
            todo!();
            /*
                if (entry != nullptr)
                        entry = entry->nextEntry;

                    while (entry == nullptr)
                    {
                        if (index >= hashMap.getNumSlots())
                            return false;

                        entry = hashMap.hashSlots.getUnchecked (index++);
                    }

                    return true;
            */
        }

        /**
          | Returns the current item's key.
          | 
          | This should only be called when a call
          | to next() has just returned true.
          |
          */
        pub fn get_key(&self) -> K {
            
            todo!();
            /*
                return entry != nullptr ? entry->key : K();
            */
        }

        /**
          | Returns the current item's value.
          | 
          | This should only be called when a call
          | to next() has just returned true.
          |
          */
        pub fn get_value(&self) -> V {
            
            todo!();
            /*
                return entry != nullptr ? entry->value : V();
            */
        }

        /**
          | Resets the iterator to its starting
          | position.
          |
          */
        pub fn reset(&mut self)  {
            
            todo!();
            /*
                entry = nullptr;
                    index = 0;
            */
        }
        
        pub fn prefix_increment(&mut self) -> &mut Iterator<K,V> {
            
            todo!();
            /*
                next(); return *this;
            */
        }
        
        pub fn reset_to_end(&mut self)  {
            
            todo!();
            /*
                index = hashMap.getNumSlots();
            */
        }
    }

}

impl<
K,
V,
HashFunctionType,
TypeOfCriticalSectionToUse: HasScopedLockType
> 

Index<K> 

for HashMap<K,V,HashFunctionType,TypeOfCriticalSectionToUse> {

    type Output = V;
    
    /**
      | Returns the value corresponding to
      | a given key.
      | 
      | If the map doesn't contain the key, a
      | default instance of the value type is
      | returned.
      | 
      | -----------
      | @param keyToLookFor
      | 
      | the key of the item being requested
      |
      */
    fn index(&self, key_to_look_for: K) -> &Self::Output {
        todo!();
        /*
            const ScopedLockType sl (getLock());

            if (auto* entry = getEntry (getSlot (keyToLookFor), keyToLookFor))
                return entry->value;

            return V();
        */
    }
}

//------------------------------
impl<
K,
V,
HashFunctionType,
TypeOfCriticalSectionToUse: HasScopedLockType
> 

Drop for HashMap<K,V,HashFunctionType,TypeOfCriticalSectionToUse> {

    fn drop(&mut self) {
        todo!();
        /* 
            clear();
         */
    }
}

//------------------------------
impl<
K,
V,
HashFunctionType: Default,
TypeOfCriticalSectionToUse: HasScopedLockType
>

HashMap<K,V,HashFunctionType,TypeOfCriticalSectionToUse> 
{
    /**
      | Creates an empty hash-map.
      | 
      | -----------
      | @param numberOfSlots
      | 
      | Specifies the number of hash entries
      | the map will use. This will be the "upperLimit"
      | parameter that is passed to your generateHash()
      | function. The number of hash slots will
      | grow automatically if necessary, or
      | it can be remapped manually using remapTable().
      | ----------
      | @param hashFunction
      | 
      | An instance of HashFunctionType, which
      | will be copied and stored to use with
      | the HashMap. This parameter can be omitted
      | if HashFunctionType has a default constructor.
      |
      */
    pub fn new_empty(
        number_of_slots: Option<i32>,
        hash_function:   Option<HashFunctionType>) -> Self {

        let number_of_slots: i32 =
            number_of_slots.unwrap_or(DEFAULT_HASH_TABLE_SIZE as i32);

        let hash_function: HashFunctionType =
            hash_function.unwrap_or(HashFunctionType::default());

        todo!();
        /*


            : hashFunctionToUse (hashFunction)
            hashSlots.insertMultiple (0, nullptr, numberOfSlots);
        */
    }

    /**
      | Removes all values from the map.
      | 
      | -----------
      | @note
      | 
      | that this will clear the content, but
      | won't affect the number of slots (see
      | remapTable and getNumSlots).
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            const ScopedLockType sl (getLock());

            for (auto i = hashSlots.size(); --i >= 0;)
            {
                auto* h = hashSlots.getUnchecked(i);

                while (h != nullptr)
                {
                    const std::unique_ptr<HashEntry> deleter (h);
                    h = h->nextEntry;
                }

                hashSlots.set (i, nullptr);
            }

            totalNumItems = 0;
        */
    }
    
    /**
      | Returns the current number of items
      | in the map.
      |
      */
    #[inline] pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return totalNumItems;
        */
    }

    /**
      | Returns a reference to the value corresponding
      | to a given key.
      | 
      | If the map doesn't contain the key, a
      | default instance of the value type is
      | added to the map and a reference to this
      | is returned.
      | 
      | -----------
      | @param keyToLookFor
      | 
      | the key of the item being requested
      |
      */
    #[inline] pub fn get_reference(&mut self, key_to_look_for: K) -> &mut V {
        
        todo!();
        /*
            const ScopedLockType sl (getLock());
            auto hashIndex = generateHashFor (keyToLookFor, getNumSlots());

            auto* firstEntry = hashSlots.getUnchecked (hashIndex);

            if (auto* entry = getEntry (firstEntry, keyToLookFor))
                return entry->value;

            auto* entry = new HashEntry (keyToLookFor, V(), firstEntry);
            hashSlots.set (hashIndex, entry);
            ++totalNumItems;

            if (totalNumItems > (getNumSlots() * 3) / 2)
                remapTable (getNumSlots() * 2);

            return entry->value;
        */
    }

    /**
      | Returns true if the map contains an item
      | with the specified key.
      |
      */
    pub fn contains(&self, key_to_look_for: K) -> bool {
        
        todo!();
        /*
            const ScopedLockType sl (getLock());

            return (getEntry (getSlot (keyToLookFor), keyToLookFor) != nullptr);
        */
    }

    /**
      | Returns true if the hash contains at
      | least one occurrence of a given value.
      |
      */
    pub fn contains_value(&self, value_to_look_for: V) -> bool {
        
        todo!();
        /*
            const ScopedLockType sl (getLock());

            for (auto i = getNumSlots(); --i >= 0;)
                for (auto* entry = hashSlots.getUnchecked(i); entry != nullptr; entry = entry->nextEntry)
                    if (entry->value == valueToLookFor)
                        return true;

            return false;
        */
    }

    /**
      | Adds or replaces an element in the hash-map.
      | 
      | If there's already an item with the given
      | key, this will replace its value. Otherwise,
      | a new item will be added to the map.
      |
      */
    pub fn set(&mut self, 
        new_key:   K,
        new_value: V)  {
        
        todo!();
        /*
            getReference (newKey) = newValue;
        */
    }

    /**
      | Removes an item with the given key.
      |
      */
    pub fn remove(&mut self, key_to_remove: K)  {
        
        todo!();
        /*
            const ScopedLockType sl (getLock());
            auto hashIndex = generateHashFor (keyToRemove, getNumSlots());
            auto* entry = hashSlots.getUnchecked (hashIndex);
            HashEntry* previous = nullptr;

            while (entry != nullptr)
            {
                if (entry->key == keyToRemove)
                {
                    const std::unique_ptr<HashEntry> deleter (entry);

                    entry = entry->nextEntry;

                    if (previous != nullptr)
                        previous->nextEntry = entry;
                    else
                        hashSlots.set (hashIndex, entry);

                    --totalNumItems;
                }
                else
                {
                    previous = entry;
                    entry = entry->nextEntry;
                }
            }
        */
    }

    /**
      | Removes all items with the given value.
      |
      */
    pub fn remove_value(&mut self, value_to_remove: V)  {
        
        todo!();
        /*
            const ScopedLockType sl (getLock());

            for (auto i = getNumSlots(); --i >= 0;)
            {
                auto* entry = hashSlots.getUnchecked(i);
                HashEntry* previous = nullptr;

                while (entry != nullptr)
                {
                    if (entry->value == valueToRemove)
                    {
                        const std::unique_ptr<HashEntry> deleter (entry);

                        entry = entry->nextEntry;

                        if (previous != nullptr)
                            previous->nextEntry = entry;
                        else
                            hashSlots.set (i, entry);

                        --totalNumItems;
                    }
                    else
                    {
                        previous = entry;
                        entry = entry->nextEntry;
                    }
                }
            }
        */
    }

    /**
      | Remaps the hash-map to use a different
      | number of slots for its hash function.
      | 
      | Each slot corresponds to a single hash-code,
      | and each one can contain multiple items.
      | 
      | @see getNumSlots()
      |
      */
    pub fn remap_table(&mut self, new_number_of_slots: i32)  {
        
        todo!();
        /*
            const ScopedLockType sl (getLock());

            Vec<HashEntry*> newSlots;
            newSlots.insertMultiple (0, nullptr, newNumberOfSlots);

            for (auto i = getNumSlots(); --i >= 0;)
            {
                HashEntry* nextEntry = nullptr;

                for (auto* entry = hashSlots.getUnchecked(i); entry != nullptr; entry = nextEntry)
                {
                    auto hashIndex = generateHashFor (entry->key, newNumberOfSlots);

                    nextEntry = entry->nextEntry;
                    entry->nextEntry = newSlots.getUnchecked (hashIndex);

                    newSlots.set (hashIndex, entry);
                }
            }

            hashSlots.swapWith (newSlots);
        */
    }

    /**
      | Returns the number of slots which are
      | available for hashing.
      | 
      | Each slot corresponds to a single hash-code,
      | and each one can contain multiple items.
      | 
      | @see getNumSlots()
      |
      */
    #[inline] pub fn get_num_slots(&self) -> i32 {
        
        todo!();
        /*
            return hashSlots.size();
        */
    }

    /**
      | Efficiently swaps the contents of two
      | hash-maps.
      |
      */
    pub fn swap_with<OtherHashMapType>(&mut self, other_hash_map: &mut OtherHashMapType)  {
    
        todo!();
        /*
            const ScopedLockType lock1 (getLock());
            const typename OtherHashMapType::ScopedLockType lock2 (otherHashMap.getLock());

            hashSlots.swapWith (otherHashMap.hashSlots);
            std::swap (totalNumItems, otherHashMap.totalNumItems);
        */
    }

    /**
      | Returns the CriticalSection that locks
      | this structure.
      | 
      | To lock, you can call getLock().enter()
      | and getLock().exit(), or preferably
      | use an object of ScopedLockType as an
      | RAII lock for it.
      |
      */
    #[inline] pub fn get_lock(&self) -> &TypeOfCriticalSectionToUse {
        
        todo!();
        /*
            return lock;
        */
    }

    /**
      | Returns a start iterator for the values
      | in this tree.
      |
      */
    pub fn begin(&self) -> hash_map::Iterator<K,V> {
        
        todo!();
        /*
            Iterator i (*this); i.next(); return i;
        */
    }

    /**
      | Returns an end iterator for the values
      | in this tree.
      |
      */
    pub fn end(&self) -> hash_map::Iterator<K,V> {
        
        todo!();
        /*
            Iterator i (*this); i.resetToEnd(); return i;
        */
    }
    
    pub fn generate_hash_for(&self, 
        key:       K,
        num_slots: i32) -> i32 {
        
        todo!();
        /*
            const int hash = hashFunctionToUse.generateHash (key, numSlots);
            jassert (isPositiveAndBelow (hash, numSlots)); // your hash function is generating out-of-range numbers!
            return hash;
        */
    }
    
    pub fn get_entry(
        first_entry:     *mut hash_map::HashEntry<K,V>, 
        key_to_look_for: K) -> *mut hash_map::HashEntry<K,V> 
    {
        todo!();
        /*
            for (auto* entry = firstEntry; entry != nullptr; entry = entry->nextEntry)
                if (entry->key == keyToLookFor)
                    return entry;

            return nullptr;
        */
    }
    
    #[inline] pub fn get_slot(&self, key: K) -> *mut hash_map::HashEntry<K,V> {
        
        todo!();
        /*
            return hashSlots.getUnchecked (generateHashFor (key, getNumSlots()));
        */
    }
}
