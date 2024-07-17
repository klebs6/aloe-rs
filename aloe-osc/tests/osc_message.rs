#[test] fn basic_usage() {
    /*

        OSCMessage msg ("/test/param0");
        expectEquals (msg.size(), 0);
        expect (msg.getAddressPattern().toString() == "/test/param0");

        const int numTestArgs = 5;

        const int testInt = 42;
        const float testFloat = 3.14159f;
        const String testString = "Hello, World!";
        const OSCColour testColour = { 10, 20, 150, 200 };

        const uint8 testBlobData[5] = { 0xBB, 0xCC, 0xDD, 0xEE, 0xFF };
        const MemoryBlock testBlob (testBlobData,  sizeof (testBlobData));

        msg.addInt32 (testInt);
        msg.addFloat32 (testFloat);
        msg.addString (testString);
        msg.addBlob (testBlob);
        msg.addColour (testColour);

        expectEquals (msg.size(), numTestArgs);

        expectEquals (msg[0].getType(), OSCTypes::int32);
        expectEquals (msg[1].getType(), OSCTypes::float32);
        expectEquals (msg[2].getType(), OSCTypes::string);
        expectEquals (msg[3].getType(), OSCTypes::blob);
        expectEquals (msg[4].getType(), OSCTypes::colour);

        expect (msg[0].isInt32());
        expect (msg[1].isFloat32());
        expect (msg[2].isString());
        expect (msg[3].isBlob());
        expect (msg[4].isColour());

        expectEquals (msg[0].getInt32(), testInt);
        expectEquals (msg[1].getFloat32(), testFloat);
        expectEquals (msg[2].getString(), testString);
        expect (msg[3].getBlob() == testBlob);
        expect (msg[4].getColour().toInt32() == testColour.toInt32());

        expect (msg.begin() + numTestArgs == msg.end());

        auto arg = msg.begin();
        expect (arg->isInt32());
        expectEquals (arg->getInt32(), testInt);
        ++arg;
        expect (arg->isFloat32());
        expectEquals (arg->getFloat32(), testFloat);
        ++arg;
        expect (arg->isString());
        expectEquals (arg->getString(), testString);
        ++arg;
        expect (arg->isBlob());
        expect (arg->getBlob() == testBlob);
        ++arg;
        expect (arg->isColour());
        expect (arg->getColour().toInt32() == testColour.toInt32());
        ++arg;
        expect (arg == msg.end());

    */
}

#[test] fn initialization_with_argument_list() {
    /*
        int testInt = 42;
        float testFloat = 5.5;
        String testString = "Hello, World!";

        {
            OSCMessage msg ("/test", testInt);
            expect (msg.getAddressPattern().toString() == String ("/test"));
            expectEquals (msg.size(), 1);
            expect (msg[0].isInt32());
            expectEquals (msg[0].getInt32(), testInt);
        }
        {
            OSCMessage msg ("/test", testFloat);
            expect (msg.getAddressPattern().toString() == String ("/test"));
            expectEquals (msg.size(), 1);
            expect (msg[0].isFloat32());
            expectEquals (msg[0].getFloat32(), testFloat);
        }
        {
            OSCMessage msg ("/test", testString);
            expect (msg.getAddressPattern().toString() == String ("/test"));
            expectEquals (msg.size(), 1);
            expect (msg[0].isString());
            expectEquals (msg[0].getString(), testString);
        }
        {
            OSCMessage msg ("/test", testInt, testFloat, testString, testFloat, testInt);
            expect (msg.getAddressPattern().toString() == String ("/test"));
            expectEquals (msg.size(), 5);
            expect (msg[0].isInt32());
            expect (msg[1].isFloat32());
            expect (msg[2].isString());
            expect (msg[3].isFloat32());
            expect (msg[4].isInt32());

            expectEquals (msg[0].getInt32(), testInt);
            expectEquals (msg[1].getFloat32(), testFloat);
            expectEquals (msg[2].getString(), testString);
            expectEquals (msg[3].getFloat32(), testFloat);
            expectEquals (msg[4].getInt32(), testInt);
        }
    */
}
