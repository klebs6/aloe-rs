
#[test] fn writing_osc_addresses() {
    /*
       OSCOutputStream outStream;
       const char check[16] = { '/', 't', 'e', 's', 't', '/', 'f', 'a', 'd', 'e', 'r', '7', '\0', '\0', '\0', '\0' };

       OSCAddress address ("/test/fader7");
       expect (outStream.writeAddress (address));

       expect (outStream.getDataSize() == sizeof (check));
       expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);
    */
}

#[test] fn writing_osc_address_patterns() {

    /*
       OSCOutputStream outStream;
       const char check[20] = { '/', '*', '/', '*', 'p', 'u', 't', '/', 'f', 'a', 'd', 'e', 'r', '[', '0', '-', '9', ']', '\0', '\0' };

       OSCAddressPattern ap ("\/\*\/\*put/fader[0-9]");
        expect (outStream.writeAddressPattern (ap));

        expect (outStream.getDataSize() == sizeof (check));
        expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);
    */
}

#[test] fn writing_osc_time_tags() {

    /*
            OSCOutputStream outStream;
            const char check[8] = { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01 };
            OSCTimeTag tag;

            expect (outStream.writeTimeTag (tag));
            expect (outStream.getDataSize() == 8);
            expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);

    */
}

#[test] fn writing_osc_type_tag_strings() {
    /*

            {
                OSCOutputStream outStream;

                OSCTypeList list;

                const char check[4] = { ',', '\0', '\0', '\0' };
                expect (outStream.writeTypeTagString (list));
                expect (outStream.getDataSize() == 4);
                expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);
            }

            {
                OSCOutputStream outStream;

                OSCTypeList list;
                list.add (OSCTypes::int32);
                list.add (OSCTypes::float32);

                const char check[4] = { ',', 'i', 'f', '\0' };
                expect (outStream.writeTypeTagString (list));
                expect (outStream.getDataSize() == sizeof (check));
                expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);
            }

            {
                OSCOutputStream outStream;

                OSCTypeList list;
                list.add (OSCTypes::blob);
                list.add (OSCTypes::blob);
                list.add (OSCTypes::string);

                const char check[8] = { ',', 'b', 'b', 's', '\0', '\0', '\0', '\0' };
                expect (outStream.writeTypeTagString (list));
                expect (outStream.getDataSize() == sizeof (check));
                expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);
            }

    */
}

#[test] fn writing_osc_arguments() {

    /*

        // test data:
        int testInt = -2015;
        const uint8 testIntRepresentation[] =  { 0xFF, 0xFF, 0xF8, 0x21 }; // big endian two's complement

        float testFloat = 345.6125f;
        const uint8 testFloatRepresentation[] = { 0x43, 0xAC, 0xCE, 0x66 }; // big endian IEEE 754

        String testString = "Hello, World!";
        const char testStringRepresentation[] = { 'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0' }; // padded to size % 4 == 0

        const uint8 testBlobData[] = { 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };
        const MemoryBlock testBlob (testBlobData, sizeof (testBlobData));
        const uint8 testBlobRepresentation[] = { 0x00, 0x00, 0x00, 0x05, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x00, 0x00 }; // padded to size % 4 == 0

        // write:

        {
            // int32:
            OSCArgument arg (testInt);
            OSCOutputStream outStream;

            expect (outStream.writeArgument (arg));
            expect (outStream.getDataSize() == 4);
            expect (std::memcmp (outStream.getData(), testIntRepresentation, sizeof (testIntRepresentation)) == 0);
        }
        {
            // float32:
            OSCArgument arg (testFloat);
            OSCOutputStream outStream;

            expect (outStream.writeArgument (arg));
            expect (outStream.getDataSize() == 4);
            expect (std::memcmp (outStream.getData(), testFloatRepresentation, sizeof (testFloatRepresentation)) == 0);

        }
        {
            // string:
            expect (testString.length() % 4 != 0); // check whether we actually cover padding
            static_assert (sizeof (testStringRepresentation) % 4 == 0, "Size must be a multiple of 4");

            OSCArgument arg (testString);
            OSCOutputStream outStream;

            expect (outStream.writeArgument (arg));
            expect (outStream.getDataSize() == sizeof (testStringRepresentation));
            expect (std::memcmp (outStream.getData(), testStringRepresentation, sizeof (testStringRepresentation)) == 0);

        }
        {
            // blob:
            expect (testBlob.getSize() % 4 != 0);  // check whether we actually cover padding
            static_assert (sizeof (testBlobRepresentation) % 4 == 0, "Size must be a multiple of 4");

            OSCArgument arg (testBlob);
            OSCOutputStream outStream;

            expect (outStream.writeArgument (arg));
            expect (outStream.getDataSize() == sizeof (testBlobRepresentation));
            expect (std::memcmp (outStream.getData(), testBlobRepresentation, sizeof (testBlobRepresentation)) == 0);

        }

    */
}

#[test] fn writing_strings_with_correct_padding() {
    /*

        // the only OSC-specific thing to check is the correct number of padding zeros

        {
            OSCArgument with15Chars ("123456789012345");
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with15Chars));
            expect (outStream.getDataSize() == 16);
        }
        {
            OSCArgument with16Chars ("1234567890123456");
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with16Chars));
            expect (outStream.getDataSize() == 20);
        }
        {
            OSCArgument with17Chars ("12345678901234567");
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with17Chars));
            expect (outStream.getDataSize() == 20);
        }
        {

            OSCArgument with18Chars ("123456789012345678");
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with18Chars));
            expect (outStream.getDataSize() == 20);
        }
        {

            OSCArgument with19Chars ("1234567890123456789");
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with19Chars));
            expect (outStream.getDataSize() == 20);
        }
        {

            OSCArgument with20Chars ("12345678901234567890");
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with20Chars));
            expect (outStream.getDataSize() == 24);
        }

    */
}

#[test] fn writing_blobs_with_correct_padding() {

    /*

        const char buffer[20] = {};
        {
            OSCArgument with15Bytes (MemoryBlock (buffer, 15));
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with15Bytes));
            expect (outStream.getDataSize() == 20);
        }
        {
            OSCArgument with16Bytes (MemoryBlock (buffer, 16));
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with16Bytes));
            expect (outStream.getDataSize() == 20);
        }
        {
            OSCArgument with17Bytes (MemoryBlock (buffer, 17));
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with17Bytes));
            expect (outStream.getDataSize() == 24);
        }
        {
            OSCArgument with18Bytes (MemoryBlock (buffer, 18));
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with18Bytes));
            expect (outStream.getDataSize() == 24);
        }
        {
            OSCArgument with19Bytes (MemoryBlock (buffer, 19));
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with19Bytes));
            expect (outStream.getDataSize() == 24);
        }
        {
            OSCArgument with20Bytes (MemoryBlock (buffer, 20));
            OSCOutputStream outStream;
            expect (outStream.writeArgument (with20Bytes));
            expect (outStream.getDataSize() == 24);
        }

    */
}

#[test] fn writing_osc_messages() {
    /*

        {
            int32 testInt = -2015;
            float testFloat = 345.6125f;
            String testString = "Hello, World!";

            const uint8 testBlobData[] = { 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };
            const MemoryBlock testBlob (testBlobData, sizeof (testBlobData));

            uint8 check[52] = { '/', 't', 'e', 's', 't', '\0', '\0', '\0',
                                ',', 'i', 'f', 's', 'b', '\0', '\0', '\0',
                                0xFF, 0xFF, 0xF8, 0x21,
                                0x43, 0xAC, 0xCE, 0x66,
                                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0',
                                0x00, 0x00, 0x00, 0x05, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x00, 0x00
            };

            OSCOutputStream outStream;

            OSCMessage msg ("/test");

            msg.addInt32 (testInt);
            msg.addFloat32 (testFloat);
            msg.addString (testString);
            msg.addBlob (testBlob);

            expect (outStream.writeMessage (msg));
            expect (outStream.getDataSize() == sizeof (check));
            expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);
        }

    */
}

#[test] fn writing_osc_bundle() {
    /*

            {
                int32 testInt = -2015;
                float testFloat = 345.6125f;
                String testString = "Hello, World!";
                const uint8 testBlobData[] = { 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };
                const MemoryBlock testBlob (testBlobData, sizeof (testBlobData));

                uint8 check[] = {
                    '#', 'b', 'u', 'n', 'd', 'l', 'e', '\0',
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,

                    0x00, 0x00, 0x00, 0x34,

                    '/', 't', 'e', 's', 't', '/', '1', '\0',
                    ',', 'i', 'f', 's', 'b', '\0', '\0', '\0',
                    0xFF, 0xFF, 0xF8, 0x21,
                    0x43, 0xAC, 0xCE, 0x66,
                    'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0',
                    0x00, 0x00, 0x00, 0x05, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x00, 0x00,

                    0x00, 0x00, 0x00, 0x0C,

                    '/', 't', 'e', 's', 't', '/', '2', '\0',
                    ',', '\0', '\0', '\0',

                    0x00, 0x00, 0x00, 0x10,

                    '#', 'b', 'u', 'n', 'd', 'l', 'e', '\0',
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
                };

                OSCOutputStream outStream;

                OSCBundle bundle;

                OSCMessage msg1 ("/test/1");
                msg1.addInt32 (testInt);
                msg1.addFloat32 (testFloat);
                msg1.addString (testString);
                msg1.addBlob (testBlob);
                bundle.addElement (msg1);

                OSCMessage msg2 ("/test/2");
                bundle.addElement (msg2);

                OSCBundle subBundle;
                bundle.addElement (subBundle);

                expect (outStream.writeBundle (bundle));
                expect (outStream.getDataSize() == sizeof (check));
                expect (std::memcmp (outStream.getData(), check, sizeof (check)) == 0);
            }

    */
}
