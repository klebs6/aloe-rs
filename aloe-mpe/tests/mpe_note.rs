
#[cfg(ALOE_UNIT_TESTS)]
#[test] fn mpe_note_tests() {

    todo!();

    /*
    class MPENoteTests : public UnitTest
    {

        MPENoteTests()
            : UnitTest ("MPENote class", UnitTestCategories::midi)
        {}

        
        void runTest() override
        {
            beginTest ("getFrequencyInHertz");
            {
                MPENote note;
                note.initialNote = 60;
                note.totalPitchbendInSemitones = -0.5;
                expectEqualsWithinOneCent (note.getFrequencyInHertz(), 254.178);
            }
        }


        
        void expectEqualsWithinOneCent (double frequencyInHertzActual,
                                        double frequencyInHertzExpected)
        {
            double ratio = frequencyInHertzActual / frequencyInHertzExpected;
            double oneCent = 1.0005946;
            expect (ratio < oneCent);
            expect (ratio > 1.0 / oneCent);
        }
    };

    static MPENoteTests MPENoteUnitTests;

    */
}
