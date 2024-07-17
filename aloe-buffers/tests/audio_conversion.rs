crate::ix!();

#[test] fn audio_conversion_tests() {

    todo!();

    /*
    #if ALOE_UNIT_TESTS

    class AudioConversionTests  : public UnitTest
    {

        AudioConversionTests()
            : UnitTest ("Audio data conversion", UnitTestCategories::audio)
        {}

        template <class F1, class E1, class F2, class E2>
        struct Test5
        {
            static void test (UnitTest& unitTest, Random& r)
            {
                test (unitTest, false, r);
                test (unitTest, true, r);
            }

            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6262)
            static void test (UnitTest& unitTest, bool inPlace, Random& r)
            {
                const int numSamples = 2048;
                int32 original [(size_t) numSamples],
                      converted[(size_t) numSamples],
                      reversed [(size_t) numSamples];

                {
                    AudioData::AudioDataPointer<F1, E1, AudioData::AudioDataNonInterleaved, AudioData::AudioDataNonConst> d (original);
                    bool clippingFailed = false;

                    for (int i = 0; i < numSamples / 2; ++i)
                    {
                        d.setAsFloat (r.nextFloat() * 2.2f - 1.1f);

                        if (! d.isFloatingPoint())
                            clippingFailed = d.getAsFloat() > 1.0f || d.getAsFloat() < -1.0f || clippingFailed;

                        ++d;
                        d.setAsInt32 (r.nextInt());
                        ++d;
                    }

                    unitTest.expect (! clippingFailed);
                }

                // convert data from the source to dest format..
                std::unique_ptr<AudioData::Converter> conv (new AudioData::ConverterInstance<AudioData::AudioDataPointer<F1, E1, AudioData::AudioDataNonInterleaved, AudioData::AudioDataConst>,
                                                                                             AudioData::AudioDataPointer<F2, E2, AudioData::AudioDataNonInterleaved, AudioData::AudioDataNonConst>>());
                conv->convertSamples (inPlace ? reversed : converted, original, numSamples);

                // ..and back again..
                conv.reset (new AudioData::ConverterInstance<AudioData::AudioDataPointer<F2, E2, AudioData::AudioDataNonInterleaved, AudioData::AudioDataConst>,
                                                             AudioData::AudioDataPointer<F1, E1, AudioData::AudioDataNonInterleaved, AudioData::AudioDataNonConst>>());
                if (! inPlace)
                    zeromem (reversed, sizeof (reversed));

                conv->convertSamples (reversed, inPlace ? reversed : converted, numSamples);

                {
                    int biggestDiff = 0;
                    AudioData::AudioDataPointer<F1, E1, AudioData::AudioDataNonInterleaved, AudioData::AudioDataConst> d1 (original);
                    AudioData::AudioDataPointer<F1, E1, AudioData::AudioDataNonInterleaved, AudioData::AudioDataConst> d2 (reversed);

                    const int errorMargin = 2 * AudioData::AudioDataPointer<F1, E1, AudioData::AudioDataNonInterleaved, AudioData::AudioDataConst>::get32BitResolution()
                                              + AudioData::AudioDataPointer<F2, E2, AudioData::AudioDataNonInterleaved, AudioData::AudioDataConst>::get32BitResolution();

                    for (int i = 0; i < numSamples; ++i)
                    {
                        biggestDiff = jmax (biggestDiff, std::abs (d1.getAsInt32() - d2.getAsInt32()));
                        ++d1;
                        ++d2;
                    }

                    unitTest.expect (biggestDiff <= errorMargin);
                }
            }
            ALOE_END_IGNORE_WARNINGS_MSVC
        };

        template <class F1, class E1, class FormatType>
        struct Test3
        {
            static void test (UnitTest& unitTest, Random& r)
            {
                Test5 <F1, E1, FormatType, AudioData::AudioDataBigEndian>::test (unitTest, r);
                Test5 <F1, E1, FormatType, AudioData::AudioDataLittleEndian>::test (unitTest, r);
            }
        };

        template <class FormatType, class Endianness>
        struct Test2
        {
            static void test (UnitTest& unitTest, Random& r)
            {
                Test3 <FormatType, Endianness, AudioData::AudioDataInt8>::test (unitTest, r);
                Test3 <FormatType, Endianness, AudioData::AudioDataUInt8>::test (unitTest, r);
                Test3 <FormatType, Endianness, AudioData::AudioDataInt16>::test (unitTest, r);
                Test3 <FormatType, Endianness, AudioData::AudioDataInt24>::test (unitTest, r);
                Test3 <FormatType, Endianness, AudioData::AudioDataInt32>::test (unitTest, r);
                Test3 <FormatType, Endianness, AudioData::AudioDataFloat32>::test (unitTest, r);
            }
        };

        template <class FormatType>
        struct Test1
        {
            static void test (UnitTest& unitTest, Random& r)
            {
                Test2 <FormatType, AudioData::AudioDataBigEndian>::test (unitTest, r);
                Test2 <FormatType, AudioData::AudioDataLittleEndian>::test (unitTest, r);
            }
        };

        void runTest() override
        {
            auto r = getRandom();
            beginTest ("Round-trip conversion: AudioDataInt8");
            Test1 <AudioData::AudioDataInt8>::test (*this, r);
            beginTest ("Round-trip conversion: AudioDataInt16");
            Test1 <AudioData::AudioDataInt16>::test (*this, r);
            beginTest ("Round-trip conversion: AudioDataInt24");
            Test1 <AudioData::AudioDataInt24>::test (*this, r);
            beginTest ("Round-trip conversion: AudioDataInt32");
            Test1 <AudioData::AudioDataInt32>::test (*this, r);
            beginTest ("Round-trip conversion: AudioDataFloat32");
            Test1 <AudioData::AudioDataFloat32>::test (*this, r);
        }
    };

    static AudioConversionTests audioConversionUnitTests;

    #endif
    */
}
