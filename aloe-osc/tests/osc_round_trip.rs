#[test] fn empty_osc_message() {

    /*
        OSCMessage outMessage ("/test/empty");

        OSCOutputStream output;
        output.writeMessage (outMessage);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCMessage inMessage = input.readMessage();

        expectEquals (inMessage.size(), 0);
    */
}

#[test] fn osc_message_with_single_argument() {

    /*
        OSCMessage outMessage ("/test/one_arg", 42);

        OSCOutputStream output;
        output.writeMessage (outMessage);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCMessage inMessage = input.readMessage();

        expectEquals (inMessage.size(), 1);
        expectEquals (inMessage[0].getInt32(), 42);
    */
}

#[test] fn osc_message_with_multiple_arguments() {

    /*
        OSCMessage outMessage ("/test/four_args", 42, 0.5f, String ("foo"), String ("bar"));

        OSCOutputStream output;
        output.writeMessage (outMessage);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCMessage inMessage = input.readMessage();

        expectEquals (inMessage.size(), 4);
        expectEquals (inMessage[0].getInt32(), 42);
        expectEquals (inMessage[1].getFloat32(), 0.5f);
        expectEquals (inMessage[2].getString(), String ("foo"));
        expectEquals (inMessage[3].getString(), String ("bar"));
    */
}

#[test] fn empty_osc_bundle() {

    /*
        OSCBundle outBundle;

        OSCOutputStream output;
        output.writeBundle (outBundle);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCBundle inBundle = input.readBundle();

        expectEquals (inBundle.size(), 0);
    */
}
#[test] fn osc_bundle_with_single_message() {

    /*
        OSCMessage outMessage ("/test/one_arg", 42);
        OSCBundle outBundle;
        outBundle.addElement (outMessage);

        OSCOutputStream output;
        output.writeBundle (outBundle);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCBundle inBundle = input.readBundle();

        expectEquals (inBundle.size(), 1);

        OSCMessage inMessage = inBundle[0].getMessage();

        expectEquals (inMessage.getAddressPattern().toString(), String ("/test/one_arg"));
        expectEquals (inMessage.size(), 1);
        expectEquals (inMessage[0].getInt32(), 42);
    */
}

#[test] fn osc_bundle_with_multiple_messages() {

    /*
        OSCMessage outMessage1 ("/test/empty");
        OSCMessage outMessage2 ("/test/one_arg", 42);
        OSCMessage outMessage3 ("/test/four_args", 42, 0.5f, String ("foo"), String ("bar"));

        OSCBundle outBundle;
        outBundle.addElement (outMessage1);
        outBundle.addElement (outMessage2);
        outBundle.addElement (outMessage3);

        OSCOutputStream output;
        output.writeBundle (outBundle);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCBundle inBundle = input.readBundle();

        expectEquals (inBundle.size(), 3);

        {
            OSCMessage inMessage = inBundle[0].getMessage();

            expectEquals (inMessage.getAddressPattern().toString(), String ("/test/empty"));
            expectEquals (inMessage.size(), 0);
        }
        {
            OSCMessage inMessage = inBundle[1].getMessage();

            expectEquals (inMessage.getAddressPattern().toString(), String ("/test/one_arg"));
            expectEquals (inMessage.size(), 1);
            expectEquals (inMessage[0].getInt32(), 42);
        }
        {
            OSCMessage inMessage = inBundle[2].getMessage();

            expectEquals (inMessage.getAddressPattern().toString(), String ("/test/four_args"));
            expectEquals (inMessage.size(), 4);
            expectEquals (inMessage[0].getInt32(), 42);
            expectEquals (inMessage[1].getFloat32(), 0.5f);
            expectEquals (inMessage[2].getString(), String ("foo"));
            expectEquals (inMessage[3].getString(), String ("bar"));
        }
    */
}

#[test] fn osc_bundle_containing_another_bundle() {

    /*
        OSCBundle outBundleNested;
        outBundleNested.addElement (OSCMessage ("/test/one_arg", 42));

        OSCBundle outBundle;
        outBundle.addElement (outBundleNested);

        OSCOutputStream output;
        output.writeBundle (outBundle);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCBundle inBundle = input.readBundle();

        expectEquals (inBundle.size(), 1);
        expect (inBundle[0].isBundle());
        OSCBundle inBundleNested = inBundle[0].getBundle();
        expectEquals (inBundleNested.size(), 1);
        expect (inBundleNested[0].isMessage());

        OSCMessage msg = inBundleNested[0].getMessage();

        expectEquals (msg.getAddressPattern().toString(), String ("/test/one_arg"));
        expectEquals (msg.size(), 1);
        expectEquals (msg[0].getInt32(), 42);
    */
}

#[test] fn osc_bundle_containing_multiple_other_bundles() {

    /*
        OSCBundle outBundleNested1;
        outBundleNested1.addElement (OSCMessage ("/test/empty"));
        OSCBundle outBundleNested2;
        outBundleNested2.addElement (OSCMessage ("/test/one_arg", 42));

        OSCBundle outBundle;
        outBundle.addElement (outBundleNested1);
        outBundle.addElement (outBundleNested2);

        OSCOutputStream output;
        output.writeBundle (outBundle);

        OSCInputStream input (output.getData(), output.getDataSize());
        OSCBundle inBundle = input.readBundle();

        expectEquals (inBundle.size(), 2);

        {
            expect (inBundle[0].isBundle());
            OSCBundle inBundleNested = inBundle[0].getBundle();
            expectEquals (inBundleNested.size(), 1);
            expect (inBundleNested[0].isMessage());

            OSCMessage msg = inBundleNested[0].getMessage();

            expectEquals (msg.getAddressPattern().toString(), String ("/test/empty"));
            expectEquals (msg.size(), 0);
        }
        {
            expect (inBundle[1].isBundle());
            OSCBundle inBundleNested = inBundle[1].getBundle();
            expectEquals (inBundleNested.size(), 1);
            expect (inBundleNested[0].isMessage());

            OSCMessage msg = inBundleNested[0].getMessage();

            expectEquals (msg.getAddressPattern().toString(), String ("/test/one_arg"));
            expectEquals (msg.size(), 1);
            expectEquals (msg[0].getInt32(), 42);
        }
    */
}
