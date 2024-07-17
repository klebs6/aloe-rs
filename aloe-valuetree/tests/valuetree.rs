crate::ix!();

#[test] fn value_tree_tests() {

    pub struct ValueTreeTests {
        base: UnitTest,
    }

    impl Default for ValueTreeTests {
        
        fn default() -> Self {
            todo!();
            /*


                : UnitTest ("ValueTrees", UnitTestCategories::values
            */
        }
    }

    impl ValueTreeTests {

        pub fn create_random_identifier(r: &mut Random) -> String {
            
            todo!();
            /*
                char buffer[50] = { 0 };
                const char chars[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-:";

                for (int i = 1 + r.nextInt (numElementsInArray (buffer) - 2); --i >= 0;)
                    buffer[i] = chars[r.nextInt (sizeof (chars) - 1)];

                String result (buffer);

                if (! XmlElement::isValidXmlName (result))
                    result = createRandomIdentifier (r);

                return result;
            */
        }
        
        pub fn create_random_wide_char_string(r: &mut Random) -> String {
            
            todo!();
            /*
                aloe_wchar buffer[50] = { 0 };

                for (int i = r.nextInt (numElementsInArray (buffer) - 1); --i >= 0;)
                {
                    if (r.nextBool())
                    {
                        do
                        {
                            buffer[i] = (aloe_wchar) (1 + r.nextInt (0x10ffff - 1));
                        }
                        while (! CharPointer_UTF16::canRepresent (buffer[i]));
                    }
                    else
                        buffer[i] = (aloe_wchar) (1 + r.nextInt (0x7e));
                }

                return CharPointer_UTF32 (buffer);
            */
        }
        
        pub fn create_random_tree(
            undo_manager: *mut UndoManager,
            depth:        i32,
            r:            &mut Random) -> ValueTree {
            
            todo!();
            /*
                ValueTree v (createRandomIdentifier (r));

                for (int i = r.nextInt (10); --i >= 0;)
                {
                    switch (r.nextInt (5))
                    {
                        case 0: v.setProperty (createRandomIdentifier (r), createRandomWideCharString (r), undoManager); break;
                        case 1: v.setProperty (createRandomIdentifier (r), r.nextInt(), undoManager); break;
                        case 2: if (depth < 5) v.addChild (createRandomTree (undoManager, depth + 1, r), r.nextInt (v.getNumChildren() + 1), undoManager); break;
                        case 3: v.setProperty (createRandomIdentifier (r), r.nextBool(), undoManager); break;
                        case 4: v.setProperty (createRandomIdentifier (r), r.nextDouble(), undoManager); break;
                        default: break;
                    }
                }

                return v;
            */
        }
        
        pub fn run_test(&mut self)  {
            
            todo!();
            /*
                {
                    beginTest ("ValueTree");

                    auto r = getRandom();

                    for (int i = 10; --i >= 0;)
                    {
                        MemoryOutputStream mo;
                        auto v1 = createRandomTree (nullptr, 0, r);
                        v1.writeToStream (mo);

                        MemoryInputStream mi (mo.getData(), mo.getDataSize(), false);
                        auto v2 = ValueTree::readFromStream (mi);
                        expect (v1.isEquivalentTo (v2));

                        MemoryOutputStream zipped;
                        {
                            GZIPCompressorOutputStream zippedOut (zipped);
                            v1.writeToStream (zippedOut);
                        }
                        expect (v1.isEquivalentTo (ValueTree::readFromGZIPData (zipped.getData(), zipped.getDataSize())));

                        auto xml1 = v1.createXml();
                        auto xml2 = v2.createCopy().createXml();
                        expect (xml1->isEquivalentTo (xml2.get(), false));

                        auto v4 = v2.createCopy();
                        expect (v1.isEquivalentTo (v4));
                    }
                }

                {
                    beginTest ("Float formatting");

                    ValueTree testVT ("Test");
                    Identifier number ("number");

                    std::map<double, String> tests;
                    tests[1] = "1.0";
                    tests[1.1] = "1.1";
                    tests[1.01] = "1.01";
                    tests[0.76378] = "0.76378";
                    tests[-10] = "-10.0";
                    tests[10.01] = "10.01";
                    tests[0.0123] = "0.0123";
                    tests[-3.7e-27] = "-3.7e-27";
                    tests[1e+40] = "1.0e40";
                    tests[-12345678901234567.0] = "-1.234567890123457e16";
                    tests[192000] = "192000.0";
                    tests[1234567] = "1.234567e6";
                    tests[0.00006] = "0.00006";
                    tests[0.000006] = "6.0e-6";

                    for (auto& test : tests)
                    {
                        testVT.setProperty (number, test.first, nullptr);
                        auto lines = StringArray::fromLines (testVT.toXmlString());
                        lines.removeEmptyStrings();
                        auto numLines = lines.size();
                        expect (numLines > 1);
                        expectEquals (lines[numLines - 1], "<Test number=\"" + test.second + "\"/>");
                    }
                }
            */
        }
    }

    lazy_static!{
        /*
        static ValueTreeTests valueTreeTests;
        */
    }
}
