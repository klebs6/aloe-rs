crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_HashMap_test.cpp]

pub struct HashMapTest {
    base: UnitTest,
}

impl Default for HashMapTest {
    
    fn default() -> Self {
        todo!();
        /*
            : UnitTest ("HashMap", UnitTestCategories::containers
        */
    }
}

pub mod hash_map_test {

    use super::*;
    
    pub struct AddElementsTest { }

    impl AddElementsTest {

        pub fn run<K>(u: &mut UnitTest)  {
        
            todo!();
            /*
                AssociativeMap<K, int> groundTruth;
                    HashMap<K, int> hashMap;

                    RandomKeys<K> keyOracle (300, 3827829);
                    Random valueOracle (48735);

                    int totalValues = 0;
                    for (int i = 0; i < 10000; ++i)
                    {
                        auto key = keyOracle.next();
                        auto value = valueOracle.nextInt();

                        bool contains = (groundTruth.find (key) != nullptr);
                        u.expectEquals ((int) contains, (int) hashMap.contains (key));

                        groundTruth.add (key, value);
                        hashMap.set (key, value);

                        if (! contains) totalValues++;

                        u.expectEquals (hashMap.size(), totalValues);
                    }
            */
        }
    }

    pub struct AccessTest { }

    impl AccessTest {

        pub fn run<K>(u: &mut UnitTest)  {
        
            todo!();
            /*
                AssociativeMap<K, int> groundTruth;
                    HashMap<K, int> hashMap;

                    fillWithRandomValues (hashMap, groundTruth);

                    for (auto pair : groundTruth.pairs)
                        u.expectEquals (hashMap[pair.key], pair.value);
            */
        }
    }

    pub struct RemoveTest { }

    impl RemoveTest {

        pub fn run<K>(u: &mut UnitTest)  {
        
            todo!();
            /*
                AssociativeMap<K, int> groundTruth;
                    HashMap<K, int> hashMap;

                    fillWithRandomValues (hashMap, groundTruth);
                    auto n = groundTruth.size();

                    Random r (3827387);

                    for (int i = 0; i < 100; ++i)
                    {
                        auto idx = r.nextInt (n-- - 1);
                        auto key = groundTruth.pairs.getReference (idx).key;

                        groundTruth.pairs.remove (idx);
                        hashMap.remove (key);

                        u.expect (! hashMap.contains (key));

                        for (auto pair : groundTruth.pairs)
                            u.expectEquals (hashMap[pair.key], pair.value);
                    }
            */
        }
    }

    /**
       ensure that the addresses of object
       references don't change
      */
    pub struct PersistantMemoryLocationOfValues { }

    pub mod persistant_memory_location_of_values {
        use super::*;
        pub struct AddressAndValue { 
            value:         i32,
            value_address: *const i32,
        }
    }

    impl PersistantMemoryLocationOfValues {

        pub fn run<K>(u: &mut UnitTest)  {
        
            todo!();
            /*
                AssociativeMap<K, AddressAndValue> groundTruth;
                    HashMap<K, int> hashMap;

                    RandomKeys<K> keyOracle (300, 3827829);
                    Random valueOracle (48735);

                    for (int i = 0; i < 1000; ++i)
                    {
                        auto key = keyOracle.next();
                        auto value = valueOracle.nextInt();

                        hashMap.set (key, value);

                        if (auto* existing = groundTruth.find (key))
                        {
                            // don't change the address: only the value
                            existing->value = value;
                        }
                        else
                        {
                            groundTruth.add (key, { value, &hashMap.getReference (key) });
                        }

                        for (auto pair : groundTruth.pairs)
                        {
                            const auto& hashMapValue = hashMap.getReference (pair.key);

                            u.expectEquals (hashMapValue, pair.value.value);
                            u.expect (&hashMapValue == pair.value.valueAddress);
                        }
                    }

                    auto n = groundTruth.size();
                    Random r (3827387);

                    for (int i = 0; i < 100; ++i)
                    {
                        auto idx = r.nextInt (n-- - 1);
                        auto key = groundTruth.pairs.getReference (idx).key;

                        groundTruth.pairs.remove (idx);
                        hashMap.remove (key);

                        for (auto pair : groundTruth.pairs)
                        {
                            const auto& hashMapValue = hashMap.getReference (pair.key);

                            u.expectEquals (hashMapValue, pair.value.value);
                            u.expect (&hashMapValue == pair.value.valueAddress);
                        }
                    }
            */
        }
    }

    pub struct AssociativeMap<K,V> {
        pairs: Vec<KeyValuePair<K,V>>,
    }

    pub struct KeyValuePair<K,V> { 
        key:   K,
        value: V,
    }

    impl<K,V> AssociativeMap<K,V> {

        pub fn find(&mut self, key: K) -> *mut V {
            
            todo!();
            /*
                auto n = pairs.size();

                    for (int i = 0; i < n; ++i)
                    {
                        auto& pair = pairs.getReference (i);

                        if (pair.key == key)
                            return &pair.value;
                    }

                    return nullptr;
            */
        }
        
        pub fn add(&mut self, 
            key:   K,
            value: V)  {
            
            todo!();
            /*
                if (V* v = find (key))
                        *v = value;
                    else
                        pairs.add ({key, value});
            */
        }
        
        pub fn size(&self) -> i32 {
            
            todo!();
            /*
                return pairs.size();
            */
        }
    }
    
    pub struct RandomKeys<'a, K> {
        r:    ThreadRng,
        keys: &'a [K],
    }

    impl<'a, K> RandomKeys<'a, K> {

        pub fn new(
            max_unique_keys: i32,
            seed:            i32) -> Self {
        
            todo!();
            /*
            : r(seed),

                for (int i = 0; i < maxUniqueKeys; ++i)
                        keys.add (generateRandomKey (r));
            */
        }
        
        pub fn next(&mut self) -> &K {
            
            todo!();
            /*
                int i = r.nextInt (keys.size() - 1);
                    return keys.getReference (i);
            */
        }
    }

    impl<'a> RandomKeys<'a, i32> {

        pub fn generate_random_key_into_i32(&mut self, rnd: &mut ThreadRng) -> i32 {
            
            todo!();
            /*
                return rnd.nextInt();
            */
        }
    }

    impl<'a> RandomKeys<'a, *mut c_void> {
        
        pub fn generate_random_key(&mut self, rnd: &mut ThreadRng)  {
            
            todo!();
            /*
                return reinterpret_cast<void*> (rnd.nextInt64());
            */
        }
    }

    impl<'a> RandomKeys<'a, String> {

        pub fn generate_random_key_into_string(&mut self, rnd: &mut ThreadRng) -> String {
            
            todo!();
            /*
                String str;

            int len = rnd.nextInt (8)+1;
            for (int i = 0; i < len; ++i)
                str += static_cast<char> (rnd.nextInt (95) + 32);

            return str;
            */
        }
    }
}

impl HashMapTest {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            doTest<AddElementsTest> ("AddElementsTest");
            doTest<AccessTest> ("AccessTest");
            doTest<RemoveTest> ("RemoveTest");
            doTest<PersistantMemoryLocationOfValues> ("PersistantMemoryLocationOfValues");
        */
    }
    
    pub fn do_test<Test>(&mut self, test_name: &String)  {
    
        todo!();
        /*
            beginTest (testName);

            Test::template run<int> (*this);
            Test::template run<void*> (*this);
            Test::template run<String> (*this);
        */
    }
    
    pub fn fill_with_random_values<K, V>(
        hash_map:     &mut HashMap<K,i32>,
        ground_truth: &mut hash_map_test::AssociativeMap<K,V>)  {
    
        todo!();
        /*
            RandomKeys<K> keyOracle (300, 3827829);
            Random valueOracle (48735);

            for (int i = 0; i < 10000; ++i)
            {
                auto key = keyOracle.next();
                auto value = valueOracle.nextInt();

                groundTruth.add (key, value);
                hashMap.set (key, value);
            }
        */
    }
}

lazy_static!{
    /*
    static HashMapTest hashMapTest;
    */
}
