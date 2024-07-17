crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
#[test] fn midi_message_test() {

    todo!();
    /*
    struct MidiMessageTest  : public UnitTest
    {
        MidiMessageTest()
            : UnitTest ("MidiMessage", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            using std::begin;
            using std::end;

            beginTest ("ReadVariableLengthValue should return valid, backward-compatible results");
            {
                const std::vector<uint8> inputs[]
                {
                    { 0x00 },
                    { 0x40 },
                    { 0x7f },
                    { 0x81, 0x00 },
                    { 0xc0, 0x00 },
                    { 0xff, 0x7f },
                    { 0x81, 0x80, 0x00 },
                    { 0xc0, 0x80, 0x00 },
                    { 0xff, 0xff, 0x7f },
                    { 0x81, 0x80, 0x80, 0x00 },
                    { 0xc0, 0x80, 0x80, 0x00 },
                    { 0xff, 0xff, 0xff, 0x7f }
                };

                const int outputs[]
                {
                    0x00,
                    0x40,
                    0x7f,
                    0x80,
                    0x2000,
                    0x3fff,
                    0x4000,
                    0x100000,
                    0x1fffff,
                    0x200000,
                    0x8000000,
                    0xfffffff,
                };

                expectEquals (std::distance (begin (inputs), end (inputs)),
                              std::distance (begin (outputs), end (outputs)));

                size_t index = 0;

                ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wdeprecated-declarations")
                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (4996)

                for (const auto& input : inputs)
                {
                    auto copy = input;

                    while (copy.size() < 16)
                        copy.push_back (0);

                    const auto result = MidiMessage::readVariableLengthValue (copy.data(),
                                                                              (int) copy.size());

                    expect (result.isValid());
                    expectEquals (result.value, outputs[index]);
                    expectEquals (result.bytesUsed, (int) inputs[index].size());

                    int legacyNumUsed = 0;
                    const auto legacyResult = MidiMessage::readVariableLengthVal (copy.data(),
                                                                                  legacyNumUsed);

                    expectEquals (result.value, legacyResult);
                    expectEquals (result.bytesUsed, legacyNumUsed);

                    ++index;
                }

                ALOE_END_IGNORE_WARNINGS_GCC_LIKE
                ALOE_END_IGNORE_WARNINGS_MSVC
            }

            beginTest ("ReadVariableLengthVal should return 0 if input is truncated");
            {
                for (size_t i = 0; i != 16; ++i)
                {
                    std::vector<uint8> input;
                    input.resize (i, 0xFF);

                    const auto result = MidiMessage::readVariableLengthValue (input.data(),
                                                                              (int) input.size());

                    expect (! result.isValid());
                    expectEquals (result.value, 0);
                    expectEquals (result.bytesUsed, 0);
                }
            }

            const std::vector<uint8> metaEvents[]
            {
                // Format is 0xff, followed by a 'kind' byte, followed by a variable-length
                // 'data-length' value, followed by that many data bytes
                { 0xff, 0x00, 0x02, 0x00, 0x00 },                   // Sequence number
                { 0xff, 0x01, 0x00 },                               // Text event
                { 0xff, 0x02, 0x00 },                               // Copyright notice
                { 0xff, 0x03, 0x00 },                               // Track name
                { 0xff, 0x04, 0x00 },                               // Instrument name
                { 0xff, 0x05, 0x00 },                               // Lyric
                { 0xff, 0x06, 0x00 },                               // Marker
                { 0xff, 0x07, 0x00 },                               // Cue point
                { 0xff, 0x20, 0x01, 0x00 },                         // Channel prefix
                { 0xff, 0x2f, 0x00 },                               // End of track
                { 0xff, 0x51, 0x03, 0x01, 0x02, 0x03 },             // Set tempo
                { 0xff, 0x54, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05 }, // SMPTE offset
                { 0xff, 0x58, 0x04, 0x01, 0x02, 0x03, 0x04 },       // Time signature
                { 0xff, 0x59, 0x02, 0x01, 0x02 },                   // Key signature
                { 0xff, 0x7f, 0x00 },                               // Sequencer-specific
            };

            beginTest ("MidiMessage data constructor works for well-formed meta-events");
            {
                const auto status = (uint8) 0x90;

                for (const auto& input : metaEvents)
                {
                    int bytesUsed = 0;
                    const MidiMessage msg (input.data(), (int) input.size(), bytesUsed, status);

                    expect (msg.isMetaEvent());
                    expectEquals (msg.getMetaEventLength(), (int) input.size() - 3);
                    expectEquals (msg.getMetaEventType(), (int) input[1]);
                }
            }

            beginTest ("MidiMessage data constructor works for malformed meta-events");
            {
                const auto status = (uint8) 0x90;

                const auto runTest = [&] (const std::vector<uint8>& input)
                {
                    int bytesUsed = 0;
                    const MidiMessage msg (input.data(), (int) input.size(), bytesUsed, status);

                    expect (msg.isMetaEvent());
                    expectEquals (msg.getMetaEventLength(), jmax (0, (int) input.size() - 3));
                    expectEquals (msg.getMetaEventType(), input.size() >= 2 ? input[1] : -1);
                };

                runTest ({ 0xff });

                for (const auto& input : metaEvents)
                {
                    auto copy = input;
                    copy[2] = 0x40; // Set the size of the message to more bytes than are present

                    runTest (copy);
                }
            }
        }
    };

    static MidiMessageTest midiMessageTests;

    */
}
