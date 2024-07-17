#[cfg(ALOE_UNIT_TESTS)]
pub struct MPEMessagesTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for MPEMessagesTests {
    
    fn default() -> Self {
        todo!();
        /*
        : unit_test("MPEMessages class", UnitTestCategories::midi),

        
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl MPEMessagesTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("add zone");
            {
                {
                    MidiBuffer buffer = MPEMessages::setLowerZone (7);

                    const uint8 expectedBytes[] =
                    {
                        0xb0, 0x64, 0x06, 0xb0, 0x65, 0x00, 0xb0, 0x06, 0x07, // set up zone
                        0xb1, 0x64, 0x00, 0xb1, 0x65, 0x00, 0xb1, 0x06, 0x30, // per-note pbrange (default = 48)
                        0xb0, 0x64, 0x00, 0xb0, 0x65, 0x00, 0xb0, 0x06, 0x02  // master pbrange (default = 2)
                    };

                    testMidiBuffer (buffer, expectedBytes, sizeof (expectedBytes));
                }
                {
                    MidiBuffer buffer = MPEMessages::setUpperZone (5, 96, 0);

                    const uint8 expectedBytes[] =
                    {
                        0xbf, 0x64, 0x06, 0xbf, 0x65, 0x00, 0xbf, 0x06, 0x05, // set up zone
                        0xbe, 0x64, 0x00, 0xbe, 0x65, 0x00, 0xbe, 0x06, 0x60, // per-note pbrange (custom)
                        0xbf, 0x64, 0x00, 0xbf, 0x65, 0x00, 0xbf, 0x06, 0x00  // master pbrange (custom)
                    };

                    testMidiBuffer (buffer, expectedBytes, sizeof (expectedBytes));
                }
            }

            beginTest ("set per-note pitchbend range");
            {
                MidiBuffer buffer = MPEMessages::setLowerZonePerNotePitchbendRange (96);

                const uint8 expectedBytes[] = { 0xb1, 0x64, 0x00, 0xb1, 0x65, 0x00, 0xb1, 0x06, 0x60 };

                testMidiBuffer (buffer, expectedBytes, sizeof (expectedBytes));
            }


            beginTest ("set master pitchbend range");
            {
                MidiBuffer buffer = MPEMessages::setUpperZoneMasterPitchbendRange (60);

                const uint8 expectedBytes[] = { 0xbf, 0x64, 0x00, 0xbf, 0x65, 0x00, 0xbf, 0x06, 0x3c };

                testMidiBuffer (buffer, expectedBytes, sizeof (expectedBytes));
            }

            beginTest ("clear all zones");
            {
                MidiBuffer buffer = MPEMessages::clearAllZones();

                const uint8 expectedBytes[] = { 0xb0, 0x64, 0x06, 0xb0, 0x65, 0x00, 0xb0, 0x06, 0x00, // clear lower zone
                                                0xbf, 0x64, 0x06, 0xbf, 0x65, 0x00, 0xbf, 0x06, 0x00  // clear upper zone
                                              };

                testMidiBuffer (buffer, expectedBytes, sizeof (expectedBytes));
            }

            beginTest ("set complete state");
            {
                MPEZoneLayout layout;

                layout.setLowerZone (7, 96, 0);
                layout.setUpperZone (7);

                MidiBuffer buffer = MPEMessages::setZoneLayout (layout);

                const uint8 expectedBytes[] = {
                    0xb0, 0x64, 0x06, 0xb0, 0x65, 0x00, 0xb0, 0x06, 0x00,  // clear lower zone
                    0xbf, 0x64, 0x06, 0xbf, 0x65, 0x00, 0xbf, 0x06, 0x00,  // clear upper zone
                    0xb0, 0x64, 0x06, 0xb0, 0x65, 0x00, 0xb0, 0x06, 0x07,  // set lower zone
                    0xb1, 0x64, 0x00, 0xb1, 0x65, 0x00, 0xb1, 0x06, 0x60,  // per-note pbrange (custom)
                    0xb0, 0x64, 0x00, 0xb0, 0x65, 0x00, 0xb0, 0x06, 0x00,  // master pbrange (custom)
                    0xbf, 0x64, 0x06, 0xbf, 0x65, 0x00, 0xbf, 0x06, 0x07,  // set upper zone
                    0xbe, 0x64, 0x00, 0xbe, 0x65, 0x00, 0xbe, 0x06, 0x30,  // per-note pbrange (default = 48)
                    0xbf, 0x64, 0x00, 0xbf, 0x65, 0x00, 0xbf, 0x06, 0x02   // master pbrange (default = 2)
                };

                testMidiBuffer (buffer, expectedBytes, sizeof (expectedBytes));
            }
        */
    }
    
    pub fn test_midi_buffer(&mut self, 
        buffer:              &mut MidiBuffer,
        expected_bytes:      *const u8,
        expected_bytes_size: i32)  {
        
        todo!();
        /*
            uint8 actualBytes[128] = { 0 };
            extractRawBinaryData (buffer, actualBytes, sizeof (actualBytes));

            expectEquals (std::memcmp (actualBytes, expectedBytes, (std::size_t) expectedBytesSize), 0);
        */
    }
    
    pub fn extract_raw_binary_data(&mut self, 
        midi_buffer:       &MidiBuffer,
        buffer_to_copy_to: *const u8,
        max_bytes:         usize)  {
        
        todo!();
        /*
            std::size_t pos = 0;

            for (const auto metadata : midiBuffer)
            {
                const uint8* data = metadata.data;
                std::size_t dataSize = (std::size_t) metadata.numBytes;

                if (pos + dataSize > maxBytes)
                    return;

                std::memcpy ((void*) (bufferToCopyTo + pos), data, dataSize);
                pos += dataSize;
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static MPEMessagesTests MPEMessagesUnitTests;
    */
}
