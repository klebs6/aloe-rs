#[test] fn reading_osc_addresses() {

    /*
        const char buffer[16] = {
            '/', 't', 'e', 's', 't', '/', 'f', 'a',
            'd', 'e', 'r', '7', '\0', '\0', '\0', '\0' };

        // reading a valid osc address:
        {
            OSCInputStream inStream (buffer, sizeof (buffer));
            OSCAddress address = inStream.readAddress();

            expect (inStream.getPosition() == sizeof (buffer));
            expectEquals (address.toString(), String ("/test/fader7"));
        }

        // check various possible failures:
        {
            // zero padding is present, but size is not modulo 4:
            OSCInputStream inStream (buffer, 15);
            expectThrowsType (inStream.readAddress(), OSCFormatError)
        }
        {
            // zero padding is missing:
            OSCInputStream inStream (buffer, 12);
            expectThrowsType (inStream.readAddress(), OSCFormatError)
        }
        {
            // pattern does not start with a forward slash:
            OSCInputStream inStream (buffer + 4, 12);
            expectThrowsType (inStream.readAddress(), OSCFormatError)
        }
    */
}

#[test] fn reading_osc_address_patterns() {
    /*
        const char buffer[20] = {
            '/', '*', '/', '*', 'p', 'u', 't', '/',
            'f', 'a', 'd', 'e', 'r', '[', '0', '-',
            '9', ']', '\0', '\0' };

        // reading a valid osc address pattern:
        {
            OSCInputStream inStream (buffer, sizeof (buffer));
            expectDoesNotThrow (inStream.readAddressPattern());
        }
        {
            OSCInputStream inStream (buffer, sizeof (buffer));
            OSCAddressPattern ap = inStream.readAddressPattern();

            expect (inStream.getPosition() == sizeof (buffer));
            expectEquals (ap.toString(), String ("\/\*\/\*put/fader[0-9]"));
            expect (ap.containsWildcards());
        }

        // check various possible failures:
        {
            // zero padding is present, but size is not modulo 4:
            OSCInputStream inStream (buffer, 19);
            expectThrowsType (inStream.readAddressPattern(), OSCFormatError)
        }
        {
            // zero padding is missing:
            OSCInputStream inStream (buffer, 16);
            expectThrowsType (inStream.readAddressPattern(), OSCFormatError)
        }
        {
            // pattern does not start with a forward slash:
            OSCInputStream inStream (buffer + 4, 16);
            expectThrowsType (inStream.readAddressPattern(), OSCFormatError)
        }
    */
}

#[test] fn reading_osc_time_tags() {

    /*
    {
        char buffer[8] = { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01 };
        OSCInputStream inStream (buffer, sizeof (buffer));

        OSCTimeTag tag = inStream.readTimeTag();
        expect (tag.isImmediately());
    }
    {
        char buffer[8] = { 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 };
        OSCInputStream inStream (buffer, sizeof (buffer));

        OSCTimeTag tag = inStream.readTimeTag();
        expect (! tag.isImmediately());
    }
    */
}

#[test] fn reading_osc_arguments() {

    /*
    {
        // test data:
        int testInt = -2015;
        const uint8 testIntRepresentation[] =  { 0xFF, 0xFF, 0xF8, 0x21 }; // big endian two's complement

        float testFloat = 345.6125f;
        const uint8 testFloatRepresentation[] = { 0x43, 0xAC, 0xCE, 0x66 }; // big endian IEEE 754

        String testString = "Hello, World!";
        const char testStringRepresentation[] = {
            'H', 'e', 'l', 'l', 'o', ',', ' ', 'W',
            'o', 'r', 'l', 'd', '!', '\0', '\0', '\0' }; // padded to size % 4 == 0

        const uint8 testBlobData[] = { 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };
        const MemoryBlock testBlob (testBlobData, sizeof (testBlobData));
        const uint8 testBlobRepresentation[] = {
            0x00, 0x00, 0x00, 0x05,
            0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x00, 0x00 }; // size prefixed + padded to size % 4 == 0

        // read:
        {
            {
                // int32:
                OSCInputStream inStream (testIntRepresentation, sizeof (testIntRepresentation));
                OSCArgument arg = inStream.readArgument (OSCTypes::int32);

                expect (inStream.getPosition() == 4);
                expect (arg.isInt32());
                expectEquals (arg.getInt32(), testInt);
            }
            {
                // float32:
                OSCInputStream inStream (testFloatRepresentation, sizeof (testFloatRepresentation));
                OSCArgument arg = inStream.readArgument (OSCTypes::float32);

                expect (inStream.getPosition() == 4);
                expect (arg.isFloat32());
                expectEquals (arg.getFloat32(), testFloat);
            }
            {
                // string:
                OSCInputStream inStream (testStringRepresentation, sizeof (testStringRepresentation));
                OSCArgument arg = inStream.readArgument (OSCTypes::string);

                expect (inStream.getPosition() == sizeof (testStringRepresentation));
                expect (arg.isString());
                expectEquals (arg.getString(), testString);
            }
            {
                // blob:
                OSCInputStream inStream (testBlobRepresentation, sizeof (testBlobRepresentation));
                OSCArgument arg = inStream.readArgument (OSCTypes::blob);

                expect (inStream.getPosition() == sizeof (testBlobRepresentation));
                expect (arg.isBlob());
                expect (arg.getBlob() == testBlob);
            }
        }

        // read invalid representations:

        {
            // not enough bytes
            {
                const uint8 rawData[] = { 0xF8, 0x21 };

                OSCInputStream inStream (rawData, sizeof (rawData));

                expectThrowsType (inStream.readArgument (OSCTypes::int32), OSCFormatError);
                expectThrowsType (inStream.readArgument (OSCTypes::float32), OSCFormatError);
            }

            // test string not being padded to multiple of 4 bytes:
            {
                const char rawData[] = {
                    'H', 'e', 'l', 'l', 'o', ',', ' ', 'W',
                    'o', 'r', 'l', 'd', '!', '\0' }; // padding missing

                OSCInputStream inStream (rawData, sizeof (rawData));

                expectThrowsType (inStream.readArgument (OSCTypes::string), OSCFormatError);
            }
            {
                const char rawData[] = {
                    'H', 'e', 'l', 'l', 'o', ',', ' ', 'W',
                    'o', 'r', 'l', 'd', '!', '\0', 'x', 'x' }; // padding with non-zero chars

                OSCInputStream inStream (rawData, sizeof (rawData));

                expectThrowsType (inStream.readArgument (OSCTypes::string), OSCFormatError);
            }

            // test blob not being padded to multiple of 4 bytes:
            {
                const uint8 rawData[] = { 0x00, 0x00, 0x00, 0x05, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF }; // padding missing

                OSCInputStream inStream (rawData, sizeof (rawData));

                expectThrowsType (inStream.readArgument (OSCTypes::blob), OSCFormatError);
            }

            // test blob having wrong size
            {
                const uint8 rawData[] = { 0x00, 0x00, 0x00, 0x12, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };

                OSCInputStream inStream (rawData, sizeof (rawData));

                expectThrowsType (inStream.readArgument (OSCTypes::blob), OSCFormatError);
            }
        }
    }

    */
}

#[test] fn reading_osc_messages() {

    /*

        {
            // valid empty message
            const char data[] = {
                '/', 't', 'e', 's', 't', '\0', '\0', '\0',
                ',', '\0', '\0', '\0' };

            OSCInputStream inStream (data, sizeof (data));

            auto msg = inStream.readMessage();
            expect (msg.getAddressPattern().toString() == "/test");
            expect (msg.size() == 0);
        }

        {
            // invalid message: no type tag string
            const char data[] = {
                '/', 't', 'e', 's', 't', '\0', '\0', '\0',
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W',
                'o', 'r', 'l', 'd', '!', '\0', '\0', '\0' };

            OSCInputStream inStream (data, sizeof (data));

            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        {
            // invalid message: no type tag string and also empty
            const char data[] = { '/', 't', 'e', 's', 't', '\0', '\0', '\0' };

            OSCInputStream inStream (data, sizeof (data));

            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        // invalid message: wrong padding
        {
            const char data[] = { '/', 't', 'e', 's', 't', '\0', '\0', '\0', ',', '\0', '\0', '\0' };
            OSCInputStream inStream (data, sizeof (data) - 1);

            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        // invalid message: says it contains an arg, but doesn't
        {
            const char data[] = { '/', 't', 'e', 's', 't', '\0', '\0', '\0', ',', 'i', '\0', '\0' };
            OSCInputStream inStream (data, sizeof (data));

            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        // invalid message: binary size does not match size deducted from type tag string
        {
            const char data[] = { '/', 't', 'e', 's', 't', '\0', '\0', '\0', ',', 'i', 'f', '\0' };
            OSCInputStream inStream (data, sizeof (data));

            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

    */
}

#[test] fn reading_osc_messages_contents() {

    /*

        // valid non-empty message.

        {
            int32 testInt = -2015;
            float testFloat = 345.6125f;
            String testString = "Hello, World!";

            const uint8 testBlobData[] = { 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };
            const MemoryBlock testBlob (testBlobData, sizeof (testBlobData));

            uint8 data[] = {
                '/', 't', 'e', 's', 't', '\0', '\0', '\0',
                ',', 'i', 'f', 's', 'b', '\0', '\0', '\0',
                0xFF, 0xFF, 0xF8, 0x21,
                0x43, 0xAC, 0xCE, 0x66,
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0',
                0x00, 0x00, 0x00, 0x05, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x00, 0x00
            };

            OSCInputStream inStream (data, sizeof (data));

            auto msg = inStream.readMessage();

            expectEquals (msg.getAddressPattern().toString(), String ("/test"));
            expectEquals (msg.size(), 4);

            expectEquals (msg[0].getType(), OSCTypes::int32);
            expectEquals (msg[1].getType(), OSCTypes::float32);
            expectEquals (msg[2].getType(), OSCTypes::string);
            expectEquals (msg[3].getType(), OSCTypes::blob);

            expectEquals (msg[0].getInt32(), testInt);
            expectEquals (msg[1].getFloat32(), testFloat);
            expectEquals (msg[2].getString(), testString);
            expect (msg[3].getBlob() == testBlob);
        }

    */
}

#[test] fn reading_osc_messages_handling_corrupt_messages() {

    /*

        // invalid messages

        {
            OSCInputStream inStream (nullptr, 0);
            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        {
            const uint8 data[] = { 0x00 };
            OSCInputStream inStream (data, 0);
            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        {
            uint8 data[] = {
                '/', 't', 'e', 's', 't', '\0', '\0', '\0',
                ',', 'i', 'f', 's', 'b',   // type tag string not padded
                0xFF, 0xFF, 0xF8, 0x21,
                0x43, 0xAC, 0xCE, 0x66,
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0',
                0x00, 0x00, 0x00, 0x05, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x00, 0x00
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        {
            uint8 data[] = {
                '/', 't', 'e', 's', 't', '\0', '\0', '\0',
                ',', 'i', 'f', 's', 'b', '\0', '\0', '\0',
                0xFF, 0xFF, 0xF8, 0x21,
                0x43, 0xAC, 0xCE, 0x66,
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0'  // rest of message cut off
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

    */
}

#[test] fn reading_osc_messages_handling_messages_without_type_tag_strings() {

    /*

        {
            uint8 data[] = { '/', 't', 'e', 's', 't', '\0', '\0', '\0' };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

        {
            uint8 data[] = {
                '/', 't', 'e', 's', 't', '\0', '\0', '\0',
                0xFF, 0xFF, 0xF8, 0x21,
                0x43, 0xAC, 0xCE, 0x66,
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0',
                0x00, 0x00, 0x00, 0x05, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x00, 0x00
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readMessage(), OSCFormatError);
        }

    */
}

#[test] fn reading_osc_bundles() {
    /*

        // valid bundle (empty)
        {
            uint8 data[] = {
                '#', 'b', 'u', 'n', 'd', 'l', 'e', '\0',
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            };

            OSCInputStream inStream (data, sizeof (data));
            OSCBundle bundle = inStream.readBundle();

            expect (bundle.getTimeTag().isImmediately());
            expect (bundle.size() == 0);
        }

        // valid bundle (containing both messages and other bundles)

        {
            int32 testInt = -2015;
            float testFloat = 345.6125f;
            String testString = "Hello, World!";
            const uint8 testBlobData[] = { 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };
            const MemoryBlock testBlob (testBlobData, sizeof (testBlobData));

            uint8 data[] = {
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
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
            };

            OSCInputStream inStream (data, sizeof (data));
            OSCBundle bundle = inStream.readBundle();

            expect (bundle.getTimeTag().isImmediately());
            expect (bundle.size() == 3);

            OSCBundle::Element* elements = bundle.begin();

            expect (elements[0].isMessage());
            expect (elements[0].getMessage().getAddressPattern().toString() == "/test/1");
            expect (elements[0].getMessage().size() == 4);
            expect (elements[0].getMessage()[0].isInt32());
            expect (elements[0].getMessage()[1].isFloat32());
            expect (elements[0].getMessage()[2].isString());
            expect (elements[0].getMessage()[3].isBlob());
            expectEquals (elements[0].getMessage()[0].getInt32(), testInt);
            expectEquals (elements[0].getMessage()[1].getFloat32(), testFloat);
            expectEquals (elements[0].getMessage()[2].getString(), testString);
            expect (elements[0].getMessage()[3].getBlob() == testBlob);

            expect (elements[1].isMessage());
            expect (elements[1].getMessage().getAddressPattern().toString() == "/test/2");
            expect (elements[1].getMessage().size() == 0);

            expect (elements[2].isBundle());
            expect (! elements[2].getBundle().getTimeTag().isImmediately());
        }

        // invalid bundles.

        {
            uint8 data[] = {
                '#', 'b', 'u', 'n', 'd', 'l', 'e', '\0',
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,

                0x00, 0x00, 0x00, 0x34,  // wrong bundle element size (too large)

                '/', 't', 'e', 's', 't', '/', '1', '\0',
                ',', 's', '\0', '\0',
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0'
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readBundle(), OSCFormatError);
        }

        {
            uint8 data[] = {
                '#', 'b', 'u', 'n', 'd', 'l', 'e', '\0',
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,

                0x00, 0x00, 0x00, 0x08,  // wrong bundle element size (too small)

                '/', 't', 'e', 's', 't', '/', '1', '\0',
                ',', 's', '\0', '\0',
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\0', '\0', '\0'
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readBundle(), OSCFormatError);
        }

        {
            uint8 data[] = {
                '#', 'b', 'u', 'n', 'd', 'l', 'e', '\0',
                0x00, 0x00, 0x00, 0x00  // incomplete time tag
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readBundle(), OSCFormatError);
        }

        {
            uint8 data[] = {
                '#', 'b', 'u', 'n', 'x', 'l', 'e', '\0', // wrong initial string
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readBundle(), OSCFormatError);
        }

        {
            uint8 data[] = {
                '#', 'b', 'u', 'n', 'd', 'l', 'e', // padding missing from string
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            };

            OSCInputStream inStream (data, sizeof (data));
            expectThrowsType (inStream.readBundle(), OSCFormatError);
        }


    */
}
