crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
#[test] fn cached_value_tests() {

    todo!();

    /*
    pub struct CachedValueTests {
        base: UnitTest,
    }

    impl Default for CachedValueTests {
        
        fn default() -> Self {
            todo!();
            /*


                : UnitTest ("CachedValues", UnitTestCategories::values
            */
        }
    }

    impl CachedValueTests {

        pub fn run_test(&mut self)  {
            
            todo!();
            /*
                beginTest ("default constructor");
                    {
                        CachedValue<String> cv;
                        expect (cv.isUsingDefault());
                        expect (cv.get() == String());
                    }

                    beginTest ("without default value");
                    {
                        ValueTree t ("root");
                        t.setProperty ("testkey", "testvalue", nullptr);

                        CachedValue<String> cv (t, "testkey", nullptr);

                        expect (! cv.isUsingDefault());
                        expect (cv.get() == "testvalue");

                        cv.resetToDefault();

                        expect (cv.isUsingDefault());
                        expect (cv.get() == String());
                    }

                    beginTest ("with default value");
                    {
                        ValueTree t ("root");
                        t.setProperty ("testkey", "testvalue", nullptr);

                        CachedValue<String> cv (t, "testkey", nullptr, "defaultvalue");

                        expect (! cv.isUsingDefault());
                        expect (cv.get() == "testvalue");

                        cv.resetToDefault();

                        expect (cv.isUsingDefault());
                        expect (cv.get() == "defaultvalue");
                    }

                    beginTest ("with default value (int)");
                    {
                        ValueTree t ("root");
                        t.setProperty ("testkey", 23, nullptr);

                        CachedValue<int> cv (t, "testkey", nullptr, 34);

                        expect (! cv.isUsingDefault());
                        expect (cv == 23);
                        expectEquals (cv.get(), 23);

                        cv.resetToDefault();

                        expect (cv.isUsingDefault());
                        expect (cv == 34);
                    }

                    beginTest ("with void value");
                    {
                        ValueTree t ("root");
                        t.setProperty ("testkey", var(), nullptr);

                        CachedValue<String> cv (t, "testkey", nullptr, "defaultvalue");

                        expect (! cv.isUsingDefault());
                        expect (cv == "");
                        expectEquals (cv.get(), String());
                    }

                    beginTest ("with non-existent value");
                    {
                        ValueTree t ("root");

                        CachedValue<String> cv (t, "testkey", nullptr, "defaultvalue");

                        expect (cv.isUsingDefault());
                        expect (cv == "defaultvalue");
                        expect (cv.get() == "defaultvalue");
                    }

                    beginTest ("with value changing");
                    {
                        ValueTree t ("root");
                        t.setProperty ("testkey", "oldvalue", nullptr);

                        CachedValue<String> cv (t, "testkey", nullptr, "defaultvalue");
                        expect (cv == "oldvalue");

                        t.setProperty ("testkey", "newvalue", nullptr);
                        expect (cv != "oldvalue");
                        expect (cv == "newvalue");
                    }

                    beginTest ("set value");
                    {
                        ValueTree t ("root");
                        t.setProperty ("testkey", 23, nullptr);

                        CachedValue<int> cv (t, "testkey", nullptr, 45);
                        cv = 34;

                        expectEquals ((int) t["testkey"], 34);

                        cv.resetToDefault();
                        expect (cv == 45);
                        expectEquals (cv.get(), 45);

                        expect (t["testkey"] == var());
                    }
            */
        }
    }

    lazy_static!{
        /*
        static CachedValueTests cachedValueTests;
        */
    }
    */
}
