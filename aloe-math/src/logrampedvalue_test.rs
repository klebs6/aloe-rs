crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_LogRampedValue_test.cpp]

lazy_static!{
    /*
    static CommonSmoothedValueTests <LogRampedValue <float>> commonLogRampedValueTests;
    */
}

pub struct LogRampedValueTests {
    base: UnitTest,
}

impl Default for LogRampedValueTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("LogRampedValueTests", UnitTestCategories::dsp
        */
    }
}

impl LogRampedValueTests {


    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Curve");
            {
                Vec<double> levels = { -0.12243, -1.21245, -12.2342, -22.4683, -30.0, -61.18753 };

                for (auto level : levels)
                {
                    Vec<Range<double>> ranges = { Range<double> (0.0,    1.0),
                                                    Range<double> (-2.345, 0.0),
                                                    Range<double> (-2.63,  3.56),
                                                    Range<double> (3.3,    -0.2) };

                    for (auto range : ranges)
                    {
                        LogRampedValue<double> slowStart { range.getStart() } , fastStart { range.getEnd() };

                        auto numSamples = 12;
                        slowStart.reset (numSamples);
                        fastStart.reset (numSamples);

                        slowStart.setLogParameters (level, true);
                        fastStart.setLogParameters (level, false);

                        slowStart.setTargetValue (range.getEnd());
                        fastStart.setTargetValue (range.getStart());

                        AudioBuffer<double> results (2, numSamples + 1);

                        results.setSample (0, 0, slowStart.getCurrentValue());
                        results.setSample (1, 0, fastStart.getCurrentValue());

                        for (int i = 1; i < results.getNumSamples(); ++i)
                        {
                            results.setSample (0, i, slowStart.getNextValue());
                            results.setSample (1, i, fastStart.getNextValue());
                        }

                        for (int i = 0; i < results.getNumSamples(); ++i)
                            expectWithinAbsoluteError (results.getSample (0, i),
                                                       results.getSample (1, results.getNumSamples() - (i + 1)),
                                                       1.0e-7);

                        auto expectedMidpoint = range.getStart() + (range.getLength() * Decibels::decibelsToGain (level));
                        expectWithinAbsoluteError (results.getSample (0, numSamples / 2),
                                                   expectedMidpoint,
                                                   1.0e-7);
                    }
                }
            }
        */
    }
}

lazy_static!{
    /*
    static LogRampedValueTests LogRampedValueTests;
    */
}
