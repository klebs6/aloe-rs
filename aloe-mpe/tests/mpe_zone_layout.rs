#[cfg(ALOE_UNIT_TESTS)]
#[test] fn mpe_zone_layout_tests() {

    todo!();

    /*
    class MPEZoneLayoutTests  : public UnitTest
    {

        MPEZoneLayoutTests()
            : UnitTest ("MPEZoneLayout class", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            beginTest ("initialisation");
            {
                MPEZoneLayout layout;
                expect (! layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());
            }

            beginTest ("adding zones");
            {
                MPEZoneLayout layout;

                layout.setLowerZone (7);

                expect (layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 7);

                layout.setUpperZone (7);

                expect (layout.getLowerZone().isActive());
                expect (layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 7);
                expectEquals (layout.getUpperZone().getMasterChannel(), 16);
                expectEquals (layout.getUpperZone().numMemberChannels, 7);

                layout.setLowerZone (3);

                expect (layout.getLowerZone().isActive());
                expect (layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 3);
                expectEquals (layout.getUpperZone().getMasterChannel(), 16);
                expectEquals (layout.getUpperZone().numMemberChannels, 7);

                layout.setUpperZone (3);

                expect (layout.getLowerZone().isActive());
                expect (layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 3);
                expectEquals (layout.getUpperZone().getMasterChannel(), 16);
                expectEquals (layout.getUpperZone().numMemberChannels, 3);

                layout.setLowerZone (15);

                expect (layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 15);
            }

            beginTest ("clear all zones");
            {
                MPEZoneLayout layout;

                expect (! layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());

                layout.setLowerZone (7);
                layout.setUpperZone (2);

                expect (layout.getLowerZone().isActive());
                expect (layout.getUpperZone().isActive());

                layout.clearAllZones();

                expect (! layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());
            }

            beginTest ("process MIDI buffers");
            {
                MPEZoneLayout layout;
                MidiBuffer buffer;

                buffer = MPEMessages::setLowerZone (7);
                layout.processNextMidiBuffer (buffer);

                expect (layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 7);

                buffer = MPEMessages::setUpperZone (7);
                layout.processNextMidiBuffer (buffer);

                expect (layout.getLowerZone().isActive());
                expect (layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 7);
                expectEquals (layout.getUpperZone().getMasterChannel(), 16);
                expectEquals (layout.getUpperZone().numMemberChannels, 7);

                {
                    buffer = MPEMessages::setLowerZone (10);
                    layout.processNextMidiBuffer (buffer);

                    expect (layout.getLowerZone().isActive());
                    expect (layout.getUpperZone().isActive());
                    expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                    expectEquals (layout.getLowerZone().numMemberChannels, 10);
                    expectEquals (layout.getUpperZone().getMasterChannel(), 16);
                    expectEquals (layout.getUpperZone().numMemberChannels, 4);


                    buffer = MPEMessages::setLowerZone (10, 33, 44);
                    layout.processNextMidiBuffer (buffer);

                    expectEquals (layout.getLowerZone().numMemberChannels, 10);
                    expectEquals (layout.getLowerZone().perNotePitchbendRange, 33);
                    expectEquals (layout.getLowerZone().masterPitchbendRange, 44);
                }

                {
                    buffer = MPEMessages::setUpperZone (10);
                    layout.processNextMidiBuffer (buffer);

                    expect (layout.getLowerZone().isActive());
                    expect (layout.getUpperZone().isActive());
                    expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                    expectEquals (layout.getLowerZone().numMemberChannels, 4);
                    expectEquals (layout.getUpperZone().getMasterChannel(), 16);
                    expectEquals (layout.getUpperZone().numMemberChannels, 10);

                    buffer = MPEMessages::setUpperZone (10, 33, 44);

                    layout.processNextMidiBuffer (buffer);

                    expectEquals (layout.getUpperZone().numMemberChannels, 10);
                    expectEquals (layout.getUpperZone().perNotePitchbendRange, 33);
                    expectEquals (layout.getUpperZone().masterPitchbendRange, 44);
                }

                buffer = MPEMessages::clearAllZones();
                layout.processNextMidiBuffer (buffer);

                expect (! layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());
            }

            beginTest ("process individual MIDI messages");
            {
                MPEZoneLayout layout;

                layout.processNextMidiEvent ({ 0x80, 0x59, 0xd0 });  // unrelated note-off msg
                layout.processNextMidiEvent ({ 0xb0, 0x64, 0x06 });  // RPN part 1
                layout.processNextMidiEvent ({ 0xb0, 0x65, 0x00 });  // RPN part 2
                layout.processNextMidiEvent ({ 0xb8, 0x0b, 0x66 });  // unrelated CC msg
                layout.processNextMidiEvent ({ 0xb0, 0x06, 0x03 });  // RPN part 3
                layout.processNextMidiEvent ({ 0x90, 0x60, 0x00 });  // unrelated note-on msg

                expect (layout.getLowerZone().isActive());
                expect (! layout.getUpperZone().isActive());
                expectEquals (layout.getLowerZone().getMasterChannel(), 1);
                expectEquals (layout.getLowerZone().numMemberChannels, 3);
                expectEquals (layout.getLowerZone().perNotePitchbendRange, 48);
                expectEquals (layout.getLowerZone().masterPitchbendRange, 2);
            }
        }
    };

    static MPEZoneLayoutTests MPEZoneLayoutUnitTests;

    */
}
