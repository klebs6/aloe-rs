crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
#[test] fn midi_file_test() {

    todo!();

    /*
    struct MidiFileTest  : public UnitTest
    {
        MidiFileTest()
            : UnitTest ("MidiFile", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            beginTest ("ReadTrack respects running status");
            {
                const auto sequence = parseSequence ([] (OutputStream& os)
                {
                    MidiFileHelpers::writeVariableLengthInt (os, 100);
                    writeBytes (os, { 0x90, 0x40, 0x40 });
                    MidiFileHelpers::writeVariableLengthInt (os, 200);
                    writeBytes (os, { 0x40, 0x40 });
                    MidiFileHelpers::writeVariableLengthInt (os, 300);
                    writeBytes (os, { 0xff, 0x2f, 0x00 });
                });

                expectEquals (sequence.getNumEvents(), 3);
                expect (sequence.getEventPointer (0)->message.isNoteOn());
                expect (sequence.getEventPointer (1)->message.isNoteOn());
                expect (sequence.getEventPointer (2)->message.isEndOfTrackMetaEvent());
            }

            beginTest ("ReadTrack returns available messages if input is truncated");
            {
                {
                    const auto sequence = parseSequence ([] (OutputStream& os)
                    {
                        // Incomplete delta time
                        writeBytes (os, { 0xff });
                    });

                    expectEquals (sequence.getNumEvents(), 0);
                }

                {
                    const auto sequence = parseSequence ([] (OutputStream& os)
                    {
                        // Complete delta with no following event
                        MidiFileHelpers::writeVariableLengthInt (os, 0xffff);
                    });

                    expectEquals (sequence.getNumEvents(), 0);
                }

                {
                    const auto sequence = parseSequence ([] (OutputStream& os)
                    {
                        // Complete delta with malformed following event
                        MidiFileHelpers::writeVariableLengthInt (os, 0xffff);
                        writeBytes (os, { 0x90, 0x40 });
                    });

                    expectEquals (sequence.getNumEvents(), 1);
                    expect (sequence.getEventPointer (0)->message.isNoteOff());
                    expectEquals (sequence.getEventPointer (0)->message.getNoteNumber(), 0x40);
                    expectEquals (sequence.getEventPointer (0)->message.getVelocity(), (uint8) 0x00);
                }
            }

            beginTest ("Header parsing works");
            {
                {
                    // No data
                    const auto header = parseHeader ([] (OutputStream&) {});
                    expect (! header.valid);
                }

                {
                    // Invalid initial byte
                    const auto header = parseHeader ([] (OutputStream& os)
                    {
                        writeBytes (os, { 0xff });
                    });

                    expect (! header.valid);
                }

                {
                    // Type block, but no header data
                    const auto header = parseHeader ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd' });
                    });

                    expect (! header.valid);
                }

                {
                    // We (ll-formed header, but track type is 0 and channels != 1
                    const auto header = parseHeader ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 0, 0, 16, 0, 1 });
                    });

                    expect (! header.valid);
                }

                {
                    // Well-formed header, but track type is 5
                    const auto header = parseHeader ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 5, 0, 16, 0, 1 });
                    });

                    expect (! header.valid);
                }

                {
                    // Well-formed header
                    const auto header = parseHeader ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 1, 0, 16, 0, 1 });
                    });

                    expect (header.valid);

                    expectEquals (header.value.fileType, (short) 1);
                    expectEquals (header.value.numberOfTracks, (short) 16);
                    expectEquals (header.value.timeFormat, (short) 1);
                    expectEquals ((int) header.value.bytesRead, 14);
                }
            }

            beginTest ("Read from stream");
            {
                {
                    // Empty input
                    const auto file = parseFile ([] (OutputStream&) {});
                    expect (! file.valid);
                }

                {
                    // Malformed header
                    const auto file = parseFile ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd' });
                    });

                    expect (! file.valid);
                }

                {
                    // Header, no channels
                    const auto file = parseFile ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 1, 0, 0, 0, 1 });
                    });

                    expect (file.valid);
                    expectEquals (file.value.getNumTracks(), 0);
                }

                {
                    // Header, one malformed channel
                    const auto file = parseFile ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 1, 0, 1, 0, 1 });
                        writeBytes (os, { 'M', 'T', 'r', '?' });
                    });

                    expect (! file.valid);
                }

                {
                    // Header, one channel with malformed message
                    const auto file = parseFile ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 1, 0, 1, 0, 1 });
                        writeBytes (os, { 'M', 'T', 'r', 'k', 0, 0, 0, 1, 0xff });
                    });

                    expect (file.valid);
                    expectEquals (file.value.getNumTracks(), 1);
                    expectEquals (file.value.getTrack (0)->getNumEvents(), 0);
                }

                {
                    // Header, one channel with incorrect length message
                    const auto file = parseFile ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 1, 0, 1, 0, 1 });
                        writeBytes (os, { 'M', 'T', 'r', 'k', 0x0f, 0, 0, 0, 0xff });
                    });

                    expect (! file.valid);
                }

                {
                    // Header, one channel, all well-formed
                    const auto file = parseFile ([] (OutputStream& os)
                    {
                        writeBytes (os, { 'M', 'T', 'h', 'd', 0, 0, 0, 6, 0, 1, 0, 1, 0, 1 });
                        writeBytes (os, { 'M', 'T', 'r', 'k', 0, 0, 0, 4 });

                        MidiFileHelpers::writeVariableLengthInt (os, 0x0f);
                        writeBytes (os, { 0x80, 0x00, 0x00 });
                    });

                    expect (file.valid);
                    expectEquals (file.value.getNumTracks(), 1);

                    auto& track = *file.value.getTrack (0);
                    expectEquals (track.getNumEvents(), 1);
                    expect (track.getEventPointer (0)->message.isNoteOff());
                    expectEquals (track.getEventPointer (0)->message.getTimeStamp(), (double) 0x0f);
                }
            }
        }

        template <typename Fn>
        static MidiMessageSequence parseSequence (Fn&& fn)
        {
            MemoryOutputStream os;
            fn (os);

            return MidiFileHelpers::readTrack (reinterpret_cast<const uint8*> (os.getData()),
                                               (int) os.getDataSize());
        }

        template <typename Fn>
        static MidiFileHelpers::Optional<MidiFileHelpers::MidiFileHeaderDetails> parseHeader (Fn&& fn)
        {
            MemoryOutputStream os;
            fn (os);

            return MidiFileHelpers::parseMidiHeader (reinterpret_cast<const uint8*> (os.getData()),
                                                     os.getDataSize());
        }

        template <typename Fn>
        static MidiFileHelpers::Optional<MidiFile> parseFile (Fn&& fn)
        {
            MemoryOutputStream os;
            fn (os);

            MemoryInputStream is (os.getData(), os.getDataSize(), false);
            MidiFile mf;

            int fileType = 0;

            if (mf.readFrom (is, true, &fileType))
                return mf;

            return {};
        }

        static void writeBytes (OutputStream& os, const std::vector<uint8>& bytes)
        {
            for (const auto& byte : bytes)
                os.writeByte ((char) byte);
        }
    };

    static MidiFileTest midiFileTests;

    */
}
