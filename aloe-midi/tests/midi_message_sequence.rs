crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
#[test] fn midi_message_sequence_test() {

    todo!();

    /*
    struct MidiMessageSequenceTest  : public UnitTest
    {
        MidiMessageSequenceTest()
            : UnitTest ("MidiMessageSequence", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            MidiMessageSequence s;

            s.addEvent (MidiMessage::noteOn  (1, 60, 0.5f).withTimeStamp (0.0));
            s.addEvent (MidiMessage::noteOff (1, 60, 0.5f).withTimeStamp (4.0));
            s.addEvent (MidiMessage::noteOn  (1, 30, 0.5f).withTimeStamp (2.0));
            s.addEvent (MidiMessage::noteOff (1, 30, 0.5f).withTimeStamp (8.0));

            beginTest ("Start & end time");
            expectEquals (s.getStartTime(), 0.0);
            expectEquals (s.getEndTime(), 8.0);
            expectEquals (s.getEventTime (1), 2.0);

            beginTest ("Matching note off & ons");
            s.updateMatchedPairs();
            expectEquals (s.getTimeOfMatchingKeyUp (0), 4.0);
            expectEquals (s.getTimeOfMatchingKeyUp (1), 8.0);
            expectEquals (s.getIndexOfMatchingKeyUp (0), 2);
            expectEquals (s.getIndexOfMatchingKeyUp (1), 3);

            beginTest ("Time & indices");
            expectEquals (s.getNextIndexAtTime (0.5), 1);
            expectEquals (s.getNextIndexAtTime (2.5), 2);
            expectEquals (s.getNextIndexAtTime (9.0), 4);

            beginTest ("Deleting events");
            s.deleteEvent (0, true);
            expectEquals (s.getNumEvents(), 2);

            beginTest ("Merging sequences");
            MidiMessageSequence s2;
            s2.addEvent (MidiMessage::noteOn  (2, 25, 0.5f).withTimeStamp (0.0));
            s2.addEvent (MidiMessage::noteOn  (2, 40, 0.5f).withTimeStamp (1.0));
            s2.addEvent (MidiMessage::noteOff (2, 40, 0.5f).withTimeStamp (5.0));
            s2.addEvent (MidiMessage::noteOn  (2, 80, 0.5f).withTimeStamp (3.0));
            s2.addEvent (MidiMessage::noteOff (2, 80, 0.5f).withTimeStamp (7.0));
            s2.addEvent (MidiMessage::noteOff (2, 25, 0.5f).withTimeStamp (9.0));

            s.addSequence (s2, 0.0, 0.0, 8.0); // Intentionally cut off the last note off
            s.updateMatchedPairs();

            expectEquals (s.getNumEvents(), 7);
            expectEquals (s.getIndexOfMatchingKeyUp (0), -1); // Truncated note, should be no note off
            expectEquals (s.getTimeOfMatchingKeyUp (1), 5.0);
        }
    };

    static MidiMessageSequenceTest midiMessageSequenceTests;

    */
}
