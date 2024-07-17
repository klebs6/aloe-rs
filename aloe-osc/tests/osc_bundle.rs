lazy_static!{
    /*
    int testInt = 127;
        float testFloat = 1.5;
    */
}

pub fn generate_test_bundle() -> OSCBundle {
    
    todo!();
        /*
            OSCBundle bundle;

            OSCMessage msg1 ("/test/fader");
            msg1.addInt32 (testInt);

            OSCMessage msg2 ("/test/foo");
            msg2.addString ("bar");
            msg2.addFloat32 (testFloat);

            bundle.addElement (msg1);
            bundle.addElement (msg2);

            return bundle;
        */
}

pub fn expect_bundle_equals_test_bundle(bundle: &OSCBundle)  {
    
    todo!();
        /*
            expect (bundle.size() == 2);
            expect (bundle[0].isMessage());
            expect (! bundle[0].isBundle());
            expect (bundle[1].isMessage());
            expect (! bundle[1].isBundle());

            int numElementsCounted = 0;
            for (auto& element : bundle)
            {
                expect (element.isMessage());
                expect (! element.isBundle());
                ++numElementsCounted;
            }
            expectEquals (numElementsCounted, 2);

            auto* e = bundle.begin();
            expect (e[0].getMessage().size() == 1);
            expect (e[0].getMessage().begin()->getInt32() == testInt);
            expect (e[1].getMessage().size() == 2);
            expect (e[1].getMessage()[1].getFloat32() == testFloat);
        */
}

#[test] fn construction() {
    /*
        OSCBundle bundle;
        expect (bundle.getTimeTag().isImmediately());
    */
}

#[test] fn construction_with_time_tag() {
    /*
        Time in100Seconds = (Time (Time::currentTimeMillis()) + RelativeTime (100.0));
        OSCBundle bundle (in100Seconds);
        expect (! bundle.getTimeTag().isImmediately());
        expect (bundle.getTimeTag().toTime() == in100Seconds);
    */
}

#[test] fn usage_when_containing_messages() {
    /*
        OSCBundle testBundle = generateTestBundle();
        expectBundleEqualsTestBundle (testBundle);
    */
}

#[test] fn usage_when_containing_other_bundles_recursively() {

    /*
        OSCBundle complexTestBundle;
        complexTestBundle.addElement (generateTestBundle());
        complexTestBundle.addElement (OSCMessage ("/test/"));
        complexTestBundle.addElement (generateTestBundle());

        expect (complexTestBundle.size() == 3);

        OSCBundle::OSCBundleElement* elements = complexTestBundle.begin();

        expect (! elements[0].isMessage());
        expect (elements[0].isBundle());
        expect (elements[1].isMessage());
        expect (! elements[1].isBundle());
        expect (! elements[2].isMessage());
        expect (elements[2].isBundle());

        expectBundleEqualsTestBundle (elements[0].getBundle());
        expect (elements[1].getMessage().size() == 0);
        expect (elements[1].getMessage().getAddressPattern().toString() == "/test");
        expectBundleEqualsTestBundle (elements[2].getBundle());
    */
}
