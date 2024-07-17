crate::ix!();

#[test] fn audio_channel_set() {

    todo!();

    /*
    #if ALOE_UNIT_TESTS

    class AudioChannelSetUnitTest  : public UnitTest
    {

        AudioChannelSetUnitTest()
            : UnitTest ("AudioChannelSetUnitTest", UnitTestCategories::audio)
        {}

        void runTest() override
        {
            auto max = AudioChannelSet::maxChannelsOfNamedLayout;

            beginTest ("maxChannelsOfNamedLayout is non-discrete");
            expect (AudioChannelSet::channelSetsWithNumberOfChannels (max).size() >= 2);

            beginTest ("channelSetsWithNumberOfChannels returns correct speaker count");
            {
                for (auto ch = 1; ch <= max; ++ch)
                {
                    auto channelSets = AudioChannelSet::channelSetsWithNumberOfChannels (ch);

                    for (auto set : channelSets)
                        expect (set.size() == ch);
                }
            }

            beginTest ("Ambisonics");
            {
                uint64 mask = 0;

                mask |= (1ull << AudioChannelSet::ambisonicACN0);
                checkAmbisonic (mask, 0, "0th Order Ambisonics");

                mask |= (1ull << AudioChannelSet::ambisonicACN1) | (1ull << AudioChannelSet::ambisonicACN2) | (1ull << AudioChannelSet::ambisonicACN3);
                checkAmbisonic (mask, 1, "1st Order Ambisonics");

                mask |= (1ull << AudioChannelSet::ambisonicACN4) | (1ull << AudioChannelSet::ambisonicACN5) | (1ull << AudioChannelSet::ambisonicACN6)
                      | (1ull << AudioChannelSet::ambisonicACN7) | (1ull << AudioChannelSet::ambisonicACN8);
                checkAmbisonic (mask, 2, "2nd Order Ambisonics");

                mask |= (1ull << AudioChannelSet::ambisonicACN9)  | (1ull << AudioChannelSet::ambisonicACN10) | (1ull << AudioChannelSet::ambisonicACN11)
                      | (1ull << AudioChannelSet::ambisonicACN12) | (1ull << AudioChannelSet::ambisonicACN13) | (1ull << AudioChannelSet::ambisonicACN14)
                      | (1ull << AudioChannelSet::ambisonicACN15);
                checkAmbisonic (mask, 3, "3rd Order Ambisonics");

                mask |= (1ull << AudioChannelSet::ambisonicACN16) | (1ull << AudioChannelSet::ambisonicACN17) | (1ull << AudioChannelSet::ambisonicACN18)
                      | (1ull << AudioChannelSet::ambisonicACN19) | (1ull << AudioChannelSet::ambisonicACN20) | (1ull << AudioChannelSet::ambisonicACN21)
                      | (1ull << AudioChannelSet::ambisonicACN22) | (1ull << AudioChannelSet::ambisonicACN23) | (1ull << AudioChannelSet::ambisonicACN24);
                checkAmbisonic (mask, 4, "4th Order Ambisonics");

                mask |= (1ull << AudioChannelSet::ambisonicACN25) | (1ull << AudioChannelSet::ambisonicACN26) | (1ull << AudioChannelSet::ambisonicACN27)
                      | (1ull << AudioChannelSet::ambisonicACN28) | (1ull << AudioChannelSet::ambisonicACN29) | (1ull << AudioChannelSet::ambisonicACN30)
                      | (1ull << AudioChannelSet::ambisonicACN31) | (1ull << AudioChannelSet::ambisonicACN32) | (1ull << AudioChannelSet::ambisonicACN33)
                      | (1ull << AudioChannelSet::ambisonicACN34) | (1ull << AudioChannelSet::ambisonicACN35);
                checkAmbisonic (mask, 5, "5th Order Ambisonics");
            }
        }


        void checkAmbisonic (uint64 mask, int order, const char* layoutName)
        {
            auto expected = AudioChannelSet::ambisonic (order);
            auto numChannels = expected.size();

            expect (numChannels == BigInteger ((int64) mask).countNumberOfSetBits());
            expect (channelSetFromMask (mask) == expected);

            expect (order == expected.getAmbisonicOrder());
            expect (expected.getDescription() == layoutName);

            auto layouts = AudioChannelSet::channelSetsWithNumberOfChannels (numChannels);
            expect (layouts.contains (expected));

            for (auto layout : layouts)
                expect (layout.getAmbisonicOrder() == (layout == expected ? order : -1));
        }

        static AudioChannelSet channelSetFromMask (uint64 mask)
        {
            Vec<AudioChannelSet::AudioChannelType> channels;
            for (int bit = 0; bit <= 62; ++bit)
                if ((mask & (1ull << bit)) != 0)
                    channels.add (static_cast<AudioChannelSet::AudioChannelType> (bit));

            return AudioChannelSet::channelSetWithChannels (channels);
        }
    };

    static AudioChannelSetUnitTest audioChannelSetUnitTest;

    #endif

    */
}
