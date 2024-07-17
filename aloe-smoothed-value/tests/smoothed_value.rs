crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct CommonSmoothedValueTests<SmoothedValueType> {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for CommonSmoothedValueTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("CommonSmoothedValueTests", UnitTestCategories::smoothedValues
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl CommonSmoothedValueTests<SmoothedValueType> {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Initial state");
            {
                SmoothedValueType sv;

                auto value = sv.getCurrentValue();
                expectEquals (sv.getTargetValue(), value);

                sv.getNextValue();
                expectEquals (sv.getCurrentValue(), value);
                expect (! sv.isSmoothing());
            }

            beginTest ("Resetting");
            {
                auto initialValue = 15.0f;

                SmoothedValueType sv (initialValue);
                sv.reset (3);
                expectEquals (sv.getCurrentValue(), initialValue);

                auto targetValue = initialValue + 1.0f;
                sv.setTargetValue (targetValue);
                expectEquals (sv.getTargetValue(), targetValue);
                expectEquals (sv.getCurrentValue(), initialValue);
                expect (sv.isSmoothing());

                auto currentValue = sv.getNextValue();
                expect (currentValue > initialValue);
                expectEquals (sv.getCurrentValue(), currentValue);
                expectEquals (sv.getTargetValue(), targetValue);
                expect (sv.isSmoothing());

                sv.reset (5);

                expectEquals (sv.getCurrentValue(), targetValue);
                expectEquals (sv.getTargetValue(),  targetValue);
                expect (! sv.isSmoothing());

                sv.getNextValue();
                expectEquals (sv.getCurrentValue(), targetValue);

                sv.setTargetValue (1.5f);
                sv.getNextValue();

                float newStart = 0.2f;
                sv.setCurrentAndTargetValue (newStart);
                expectEquals (sv.getNextValue(), newStart);
                expectEquals (sv.getTargetValue(), newStart);
                expectEquals (sv.getCurrentValue(), newStart);
                expect (! sv.isSmoothing());
            }

            beginTest ("Sample rate");
            {
                SmoothedValueType svSamples { 3.0f };
                auto svTime = svSamples;

                auto numSamples = 12;

                svSamples.reset (numSamples);
                svTime.reset (numSamples * 2, 1.0);

                for (int i = 0; i < numSamples; ++i)
                {
                    svTime.skip (1);
                    expectWithinAbsoluteError (svSamples.getNextValue(),
                                               svTime.getNextValue(),
                                               1.0e-7f);
                }
            }

            beginTest ("Block processing");
            {
                SmoothedValueType sv (1.0f);

                sv.reset (12);
                sv.setTargetValue (2.0f);

                const auto numSamples = 15;

                AudioBuffer<float> referenceData (1, numSamples);

                for (int i = 0; i < numSamples; ++i)
                    referenceData.setSample (0, i, sv.getNextValue());

                expect (referenceData.getSample (0, 0) > 0);
                expect (referenceData.getSample (0, 10) < sv.getTargetValue());
                expectWithinAbsoluteError (referenceData.getSample (0, 11),
                                           sv.getTargetValue(),
                                           2.0e-7f);

                auto getUnitData = [] (int numSamplesToGenerate)
                {
                    AudioBuffer<float> result (1, numSamplesToGenerate);

                    for (int i = 0; i < numSamplesToGenerate; ++i)
                        result.setSample (0, i, 1.0f);

                    return result;
                };

                auto compareData = [this] (const AudioBuffer<float>& test,
                                           const AudioBuffer<float>& reference)
                {
                    for (int i = 0; i < test.getNumSamples(); ++i)
                        expectWithinAbsoluteError (test.getSample (0, i),
                                                   reference.getSample (0, i),
                                                   2.0e-7f);
                };

                auto testData = getUnitData (numSamples);
                sv.setCurrentAndTargetValue (1.0f);
                sv.setTargetValue (2.0f);
                sv.applyGain (testData.getWritePointer (0), numSamples);
                compareData (testData, referenceData);

                testData = getUnitData (numSamples);
                AudioBuffer<float> destData (1, numSamples);
                sv.setCurrentAndTargetValue (1.0f);
                sv.setTargetValue (2.0f);
                sv.applyGain (destData.getWritePointer (0),
                               testData.getReadPointer (0),
                               numSamples);
                compareData (destData, referenceData);
                compareData (testData, getUnitData (numSamples));

                testData = getUnitData (numSamples);
                sv.setCurrentAndTargetValue (1.0f);
                sv.setTargetValue (2.0f);
                sv.applyGain (testData, numSamples);
                compareData (testData, referenceData);
            }

            beginTest ("Skip");
            {
                SmoothedValueType sv;

                sv.reset (12);
                sv.setCurrentAndTargetValue (1.0f);
                sv.setTargetValue (2.0f);

                Vec<float> reference;

                for (int i = 0; i < 15; ++i)
                    reference.add (sv.getNextValue());

                sv.setCurrentAndTargetValue (1.0f);
                sv.setTargetValue (2.0f);

                expectWithinAbsoluteError (sv.skip (1), reference[0], 1.0e-6f);
                expectWithinAbsoluteError (sv.skip (1), reference[1], 1.0e-6f);
                expectWithinAbsoluteError (sv.skip (2), reference[3], 1.0e-6f);
                sv.skip (3);
                expectWithinAbsoluteError (sv.getCurrentValue(), reference[6], 1.0e-6f);
                expectEquals (sv.skip (300), sv.getTargetValue());
                expectEquals (sv.getCurrentValue(), sv.getTargetValue());
            }

            beginTest ("Negative");
            {
                SmoothedValueType sv;

                auto numValues = 12;
                sv.reset (numValues);

                std::vector<std::pair<float, float>> ranges = { { -1.0f, -2.0f },
                                                                { -100.0f, -3.0f } };

                for (auto range : ranges)
                {
                    auto start = range.first, end = range.second;

                    sv.setCurrentAndTargetValue (start);
                    sv.setTargetValue (end);

                    auto val = sv.skip (numValues / 2);

                    if (end > start)
                        expect (val > start && val < end);
                    else
                        expect (val < start && val > end);

                    auto nextVal = sv.getNextValue();
                    expect (end > start ? (nextVal > val) : (nextVal < val));

                    auto endVal = sv.skip (500);
                    expectEquals (endVal, end);
                    expectEquals (sv.getNextValue(), end);
                    expectEquals (sv.getCurrentValue(), end);

                    sv.setCurrentAndTargetValue (start);
                    sv.setTargetValue (end);

                    SmoothedValueType positiveSv { -start };
                    positiveSv.reset (numValues);
                    positiveSv.setTargetValue (-end);

                    for (int i = 0; i < numValues + 2; ++i)
                        expectEquals (sv.getNextValue(), -positiveSv.getNextValue());
                }
            }
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_SmoothedValue.cpp]

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static CommonSmoothedValueTests <SmoothedValue<float, ValueSmoothingTypes::Linear>> commonLinearSmoothedValueTests;
    static CommonSmoothedValueTests <SmoothedValue<float, ValueSmoothingTypes::Multiplicative>> commonMultiplicativeSmoothedValueTests;
    */
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct SmoothedValueTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for SmoothedValueTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("SmoothedValueTests", UnitTestCategories::smoothedValues
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl SmoothedValueTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Linear moving target");
            {
                SmoothedValue<float, ValueSmoothingTypes::Linear> sv;

                sv.reset (12);
                float initialValue = 0.0f;
                sv.setCurrentAndTargetValue (initialValue);
                sv.setTargetValue (1.0f);

                auto delta = sv.getNextValue() - initialValue;

                sv.skip (6);

                auto newInitialValue = sv.getCurrentValue();
                sv.setTargetValue (newInitialValue + 2.0f);
                auto doubleDelta = sv.getNextValue() - newInitialValue;

                expectWithinAbsoluteError (doubleDelta, delta * 2.0f, 1.0e-7f);
            }

            beginTest ("Multiplicative curve");
            {
                SmoothedValue<double, ValueSmoothingTypes::Multiplicative> sv;

                auto numSamples = 12;
                AudioBuffer<double> values (2, numSamples + 1);

                sv.reset (numSamples);
                sv.setCurrentAndTargetValue (1.0);
                sv.setTargetValue (2.0f);

                values.setSample (0, 0, sv.getCurrentValue());

                for (int i = 1; i < values.getNumSamples(); ++i)
                    values.setSample (0, i, sv.getNextValue());

                sv.setTargetValue (1.0f);
                values.setSample (1, values.getNumSamples() - 1, sv.getCurrentValue());

                for (int i = values.getNumSamples() - 2; i >= 0 ; --i)
                    values.setSample (1, i, sv.getNextValue());

                for (int i = 0; i < values.getNumSamples(); ++i)
                    expectWithinAbsoluteError (values.getSample (0, i), values.getSample (1, i), 1.0e-9);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static SmoothedValueTests smoothedValueTests;
    */
}
