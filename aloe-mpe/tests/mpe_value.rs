#[cfg(ALOE_UNIT_TESTS)]
#[test] fn mpe_value_tests() {

    todo!();

    /*
    class MPEValueTests  : public UnitTest
    {

        MPEValueTests()
            : UnitTest ("MPEValue class", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            beginTest ("comparison operator");
            {
                MPEValue value1 = MPEValue::from7BitInt (7);
                MPEValue value2 = MPEValue::from7BitInt (7);
                MPEValue value3 = MPEValue::from7BitInt (8);

                expect (value1 == value1);
                expect (value1 == value2);
                expect (value1 != value3);
            }

            beginTest ("special values");
            {
                expectEquals (MPEValue::minValue().as7BitInt(), 0);
                expectEquals (MPEValue::minValue().as14BitInt(), 0);

                expectEquals (MPEValue::centreValue().as7BitInt(), 64);
                expectEquals (MPEValue::centreValue().as14BitInt(), 8192);

                expectEquals (MPEValue::maxValue().as7BitInt(), 127);
                expectEquals (MPEValue::maxValue().as14BitInt(), 16383);
            }

            beginTest ("zero/minimum value");
            {
                expectValuesConsistent (MPEValue::from7BitInt (0),  0, 0, -1.0f, 0.0f);
                expectValuesConsistent (MPEValue::from14BitInt (0), 0, 0, -1.0f, 0.0f);
            }

            beginTest ("maximum value");
            {
                expectValuesConsistent (MPEValue::from7BitInt (127),    127, 16383, 1.0f, 1.0f);
                expectValuesConsistent (MPEValue::from14BitInt (16383), 127, 16383, 1.0f, 1.0f);
            }

            beginTest ("centre value");
            {
                expectValuesConsistent (MPEValue::from7BitInt (64),    64, 8192, 0.0f, 0.5f);
                expectValuesConsistent (MPEValue::from14BitInt (8192), 64, 8192, 0.0f, 0.5f);
            }

            beginTest ("value halfway between min and centre");
            {
                expectValuesConsistent (MPEValue::from7BitInt (32),    32, 4096, -0.5f, 0.25f);
                expectValuesConsistent (MPEValue::from14BitInt (4096), 32, 4096, -0.5f, 0.25f);
            }
        }


        
        void expectValuesConsistent (MPEValue value,
                                     int expectedValueAs7BitInt,
                                     int expectedValueAs14BitInt,
                                     float expectedValueAsSignedFloat,
                                     float expectedValueAsUnsignedFloat)
        {
            expectEquals (value.as7BitInt(), expectedValueAs7BitInt);
            expectEquals (value.as14BitInt(), expectedValueAs14BitInt);
            expectFloatWithinRelativeError (value.asSignedFloat(), expectedValueAsSignedFloat, 0.0001f);
            expectFloatWithinRelativeError (value.asUnsignedFloat(), expectedValueAsUnsignedFloat, 0.0001f);
        }

        
        void expectFloatWithinRelativeError (float actualValue, float expectedValue, float maxRelativeError)
        {
            const float maxAbsoluteError = jmax (1.0f, std::abs (expectedValue)) * maxRelativeError;
            expect (std::abs (expectedValue - actualValue) < maxAbsoluteError);
        }
    };

    static MPEValueTests MPEValueUnitTests;

    */
}
