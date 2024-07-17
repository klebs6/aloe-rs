crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiBuffer.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiBuffer.cpp]

#[cfg(ALOE_UNIT_TESTS)]
#[test] fn midi_buffer_test() {
    todo!();
    /*
    struct MidiBufferTest  : public UnitTest
    {
        MidiBufferTest()
            : UnitTest ("MidiBuffer", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            beginTest ("Clear messages");
            {
                const auto message = MidiMessage::noteOn (1, 64, 0.5f);

                const auto testBuffer = [&]
                {
                    MidiBuffer buffer;
                    buffer.addEvent (message, 0);
                    buffer.addEvent (message, 10);
                    buffer.addEvent (message, 20);
                    buffer.addEvent (message, 30);
                    return buffer;
                }();

                {
                    auto buffer = testBuffer;
                    buffer.clear (10, 0);
                    expectEquals (buffer.getNumEvents(), 4);
                }

                {
                    auto buffer = testBuffer;
                    buffer.clear (10, 1);
                    expectEquals (buffer.getNumEvents(), 3);
                }

                {
                    auto buffer = testBuffer;
                    buffer.clear (10, 10);
                    expectEquals (buffer.getNumEvents(), 3);
                }

                {
                    auto buffer = testBuffer;
                    buffer.clear (10, 20);
                    expectEquals (buffer.getNumEvents(), 2);
                }

                {
                    auto buffer = testBuffer;
                    buffer.clear (10, 30);
                    expectEquals (buffer.getNumEvents(), 1);
                }

                {
                    auto buffer = testBuffer;
                    buffer.clear (10, 300);
                    expectEquals (buffer.getNumEvents(), 1);
                }
            }
        }
    };

    static MidiBufferTest midiBufferTest;
    */
}

