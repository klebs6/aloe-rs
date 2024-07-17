
#[cfg(ALOE_UNIT_TESTS)]
#[test] fn mpe_utils_unit_tests() {

    todo!();

    /*
    struct MPEUtilsUnitTests  : public UnitTest
    {
        MPEUtilsUnitTests()
            : UnitTest ("MPE Utilities", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            beginTest ("MPEChannelAssigner");
            {
                MPEZoneLayout layout;

                // lower
                {
                    layout.setLowerZone (15);

                    // lower zone
                    MPEChannelAssigner channelAssigner (layout.getLowerZone());

                    // check that channels are assigned in correct order
                    int noteNum = 60;
                    for (int ch = 2; ch <= 16; ++ch)
                        expectEquals (channelAssigner.findMidiChannelForNewNote (noteNum++), ch);

                    // check that note-offs are processed
                    channelAssigner.noteOff (60);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (60), 2);

                    channelAssigner.noteOff (61);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (61), 3);

                    // check that assigned channel was last to play note
                    channelAssigner.noteOff (65);
                    channelAssigner.noteOff (66);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (66), 8);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (65), 7);

                    // find closest channel playing nonequal note
                    expectEquals (channelAssigner.findMidiChannelForNewNote (80), 16);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (55), 2);

                    // all notes off
                    channelAssigner.allNotesOff();

                    // last note played
                    expectEquals (channelAssigner.findMidiChannelForNewNote (66), 8);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (65), 7);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (80), 16);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (55), 2);

                    // normal assignment
                    expectEquals (channelAssigner.findMidiChannelForNewNote (101), 3);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (20), 4);
                }

                // upper
                {
                    layout.setUpperZone (15);

                    // upper zone
                    MPEChannelAssigner channelAssigner (layout.getUpperZone());

                    // check that channels are assigned in correct order
                    int noteNum = 60;
                    for (int ch = 15; ch >= 1; --ch)
                        expectEquals (channelAssigner.findMidiChannelForNewNote (noteNum++), ch);

                    // check that note-offs are processed
                    channelAssigner.noteOff (60);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (60), 15);

                    channelAssigner.noteOff (61);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (61), 14);

                    // check that assigned channel was last to play note
                    channelAssigner.noteOff (65);
                    channelAssigner.noteOff (66);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (66), 9);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (65), 10);

                    // find closest channel playing nonequal note
                    expectEquals (channelAssigner.findMidiChannelForNewNote (80), 1);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (55), 15);

                    // all notes off
                    channelAssigner.allNotesOff();

                    // last note played
                    expectEquals (channelAssigner.findMidiChannelForNewNote (66), 9);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (65), 10);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (80), 1);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (55), 15);

                    // normal assignment
                    expectEquals (channelAssigner.findMidiChannelForNewNote (101), 14);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (20), 13);
                }

                // legacy
                {
                    MPEChannelAssigner channelAssigner;

                    // check that channels are assigned in correct order
                    int noteNum = 60;
                    for (int ch = 1; ch <= 16; ++ch)
                        expectEquals (channelAssigner.findMidiChannelForNewNote (noteNum++), ch);

                    // check that note-offs are processed
                    channelAssigner.noteOff (60);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (60), 1);

                    channelAssigner.noteOff (61);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (61), 2);

                    // check that assigned channel was last to play note
                    channelAssigner.noteOff (65);
                    channelAssigner.noteOff (66);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (66), 7);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (65), 6);

                    // find closest channel playing nonequal note
                    expectEquals (channelAssigner.findMidiChannelForNewNote (80), 16);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (55), 1);

                    // all notes off
                    channelAssigner.allNotesOff();

                    // last note played
                    expectEquals (channelAssigner.findMidiChannelForNewNote (66), 7);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (65), 6);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (80), 16);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (55), 1);

                    // normal assignment
                    expectEquals (channelAssigner.findMidiChannelForNewNote (101), 2);
                    expectEquals (channelAssigner.findMidiChannelForNewNote (20), 3);
                }
            }

            beginTest ("MPEChannelRemapper");
            {
                // 3 different MPE 'sources', constant IDs
                const int sourceID1 = 0;
                const int sourceID2 = 1;
                const int sourceID3 = 2;

                MPEZoneLayout layout;

                {
                    layout.setLowerZone (15);

                    // lower zone
                    MPEChannelRemapper channelRemapper (layout.getLowerZone());

                    // first source, shouldn't remap
                    for (int ch = 2; ch <= 16; ++ch)
                    {
                        auto noteOn = MidiMessage::noteOn (ch, 60, 1.0f);

                        channelRemapper.remapMidiChannelIfNeeded (noteOn, sourceID1);
                        expectEquals (noteOn.getChannel(), ch);
                    }

                    auto noteOn = MidiMessage::noteOn (2, 60, 1.0f);

                    // remap onto oldest last-used channel
                    channelRemapper.remapMidiChannelIfNeeded (noteOn, sourceID2);
                    expectEquals (noteOn.getChannel(), 2);

                    // remap onto oldest last-used channel
                    channelRemapper.remapMidiChannelIfNeeded (noteOn, sourceID3);
                    expectEquals (noteOn.getChannel(), 3);

                    // remap to correct channel for source ID
                    auto noteOff = MidiMessage::noteOff (2, 60, 1.0f);
                    channelRemapper.remapMidiChannelIfNeeded (noteOff, sourceID3);
                    expectEquals (noteOff.getChannel(), 3);
                }

                {
                    layout.setUpperZone (15);

                    // upper zone
                    MPEChannelRemapper channelRemapper (layout.getUpperZone());

                    // first source, shouldn't remap
                    for (int ch = 15; ch >= 1; --ch)
                    {
                        auto noteOn = MidiMessage::noteOn (ch, 60, 1.0f);

                        channelRemapper.remapMidiChannelIfNeeded (noteOn, sourceID1);
                        expectEquals (noteOn.getChannel(), ch);
                    }

                    auto noteOn = MidiMessage::noteOn (15, 60, 1.0f);

                    // remap onto oldest last-used channel
                    channelRemapper.remapMidiChannelIfNeeded (noteOn, sourceID2);
                    expectEquals (noteOn.getChannel(), 15);

                    // remap onto oldest last-used channel
                    channelRemapper.remapMidiChannelIfNeeded (noteOn, sourceID3);
                    expectEquals (noteOn.getChannel(), 14);

                    // remap to correct channel for source ID
                    auto noteOff = MidiMessage::noteOff (15, 60, 1.0f);
                    channelRemapper.remapMidiChannelIfNeeded (noteOff, sourceID3);
                    expectEquals (noteOff.getChannel(), 14);
                }
            }
        }
    };

    static MPEUtilsUnitTests MPEUtilsUnitTests;

    */
}
