crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct JSONTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for JSONTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("JSON", UnitTestCategories::json
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl JSONTests {

    pub fn create_random_wide_char_string(r: &mut Random) -> String {
        
        todo!();
        /*
            aloe_wchar buffer[40] = { 0 };

            for (int i = 0; i < numElementsInArray (buffer) - 1; ++i)
            {
                if (r.nextBool())
                {
                    do
                    {
                        buffer[i] = (aloe_wchar) (1 + r.nextInt (0x10ffff - 1));
                    }
                    while (! CharPointer_UTF16::canRepresent (buffer[i]));
                }
                else
                    buffer[i] = (aloe_wchar) (1 + r.nextInt (0xff));
            }

            return CharPointer_UTF32 (buffer);
        */
    }
    
    pub fn create_random_identifier(r: &mut Random) -> String {
        
        todo!();
        /*
            char buffer[30] = { 0 };

            for (int i = 0; i < numElementsInArray (buffer) - 1; ++i)
            {
                static const char chars[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-:";
                buffer[i] = chars [r.nextInt (sizeof (chars) - 1)];
            }

            return CharPointer_ASCII (buffer);
        */
    }

    /**
      | Creates a random double that can be easily
      |  stringified, to avoid false failures when
      |  decimal places are rounded or truncated
      |  slightly
      */
    pub fn create_random_double(r: &mut Random) -> var {
        
        todo!();
        /*
            return var ((r.nextDouble() * 1000.0) + 0.1);
        */
    }
    
    pub fn create_random_var(
        r:     &mut Random,
        depth: i32) -> var {
        
        todo!();
        /*
            switch (r.nextInt (depth > 3 ? 6 : 8))
            {
                case 0:     return {};
                case 1:     return r.nextInt();
                case 2:     return r.nextInt64();
                case 3:     return r.nextBool();
                case 4:     return createRandomDouble (r);
                case 5:     return createRandomWideCharString (r);

                case 6:
                {
                    var v (createRandomVar (r, depth + 1));

                    for (int i = 1 + r.nextInt (30); --i >= 0;)
                        v.append (createRandomVar (r, depth + 1));

                    return v;
                }

                case 7:
                {
                    auto o = new DynamicObject();

                    for (int i = r.nextInt (30); --i >= 0;)
                        o->setProperty (createRandomIdentifier (r), createRandomVar (r, depth + 1));

                    return o;
                }

                default:
                    return {};
            }
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            {
                beginTest ("JSON");

                auto r = getRandom();

                expect (JSON::parse (String()) == var());
                expect (JSON::parse ("{}").isObject());
                expect (JSON::parse ("[]").isArray());
                expect (JSON::parse ("[ 1234 ]")[0].isInt());
                expect (JSON::parse ("[ 12345678901234 ]")[0].isInt64());
                expect (JSON::parse ("[ 1.123e3 ]")[0].isDouble());
                expect (JSON::parse ("[ -1234]")[0].isInt());
                expect (JSON::parse ("[-12345678901234]")[0].isInt64());
                expect (JSON::parse ("[-1.123e3]")[0].isDouble());

                for (int i = 100; --i >= 0;)
                {
                    var v;

                    if (i > 0)
                        v = createRandomVar (r, 0);

                    const bool oneLine = r.nextBool();
                    String asString (JSON::toString (v, oneLine));
                    var parsed = JSON::parse ("[" + asString + "]")[0];
                    String parsedString (JSON::toString (parsed, oneLine));
                    expect (asString.isNotEmpty() && parsedString == asString);
                }
            }

            {
                beginTest ("Float formatting");

                std::map<double, String> tests;
                tests[1] = "1.0";
                tests[1.1] = "1.1";
                tests[1.01] = "1.01";
                tests[0.76378] = "0.76378";
                tests[-10] = "-10.0";
                tests[10.01] = "10.01";
                tests[0.0123] = "0.0123";
                tests[-3.7e-27] = "-3.7e-27";
                tests[1e+40] = "1.0e40";
                tests[-12345678901234567.0] = "-1.234567890123457e16";
                tests[192000] = "192000.0";
                tests[1234567] = "1.234567e6";
                tests[0.00006] = "0.00006";
                tests[0.000006] = "6.0e-6";

                for (auto& test : tests)
                    expectEquals (JSON::toString (test.first), test.second);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static JSONTests JSONUnitTests;
    */
}
