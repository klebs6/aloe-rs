crate::ix!();

#[test] fn value_with_default_tests() {

    todo!();

    /*
    pub struct ValueWithDefaultTests {
        base: UnitTest,
    }

    impl Default for ValueWithDefaultTests {
        
        fn default() -> Self {
            todo!();
            /*


                : UnitTest ("ValueWithDefault", UnitTestCategories::values
            */
        }
    }

    impl ValueWithDefaultTests {

        pub fn run_test(&mut self)  {
            
            todo!();
            /*
                beginTest ("default constructor");
                {
                    ValueWithDefault vwd;
                    expect (vwd.isUsingDefault());
                    expect (vwd.get() == Var());
                }

                beginTest ("missing property");
                {
                    ValueTree t ("root");
                    ValueWithDefault vwd (t, "testKey", nullptr, "default");

                    expect (vwd.isUsingDefault());
                    expectEquals (vwd.get().toString(), String ("default"));
                }

                beginTest ("non-empty property");
                {
                    ValueTree t ("root");
                    t.setProperty ("testKey", "non-default", nullptr);

                    ValueWithDefault vwd (t, "testKey", nullptr, "default");

                    expect (! vwd.isUsingDefault());
                    expectEquals (vwd.get().toString(), String ("non-default"));
                }

                beginTest ("set default");
                {
                    ValueTree t ("root");

                    ValueWithDefault vwd (t, "testkey", nullptr);
                    vwd.setDefault ("default");

                    expect (vwd.isUsingDefault());
                    expectEquals (vwd.get().toString(), String ("default"));
                }

                beginTest ("set value");
                {
                    ValueTree t ("root");
                    t.setProperty ("testkey", "testvalue", nullptr);

                    ValueWithDefault vwd (t, "testkey", nullptr, "default");
                    vwd = "newvalue";

                    expect (! vwd.isUsingDefault());
                    expectEquals (t["testkey"].toString(), String ("newvalue"));

                    vwd.resetToDefault();

                    expect (vwd.isUsingDefault());
                    expect (t["testkey"] == Var());
                }
            */
        }
    }

    lazy_static!{
        /*
        static ValueWithDefaultTests valueWithDefaultTests;
        */
    }
    */
}

