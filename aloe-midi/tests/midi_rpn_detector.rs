crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiRPN.h]

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiRPN.cpp]

#[test] fn midi_rpn_detector_tests() {

    todo!();

    /*
    class MidiRPNDetectorTests   : public UnitTest
    {

        MidiRPNDetectorTests()
            : UnitTest ("MidiRPNDetector class", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            beginTest ("7-bit RPN");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (2, 101, 0,  rpn));
                expect (! detector.parseControllerMessage (2, 100, 7,  rpn));
                expect (detector.parseControllerMessage   (2, 6,   42, rpn));

                expectEquals (rpn.channel, 2);
                expectEquals (rpn.parameterNumber, 7);
                expectEquals (rpn.value, 42);
                expect (! rpn.isNRPN);
                expect (! rpn.is14BitValue);
            }

            beginTest ("14-bit RPN");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (1, 100, 44, rpn));
                expect (! detector.parseControllerMessage (1, 101, 2,  rpn));
                expect (! detector.parseControllerMessage (1, 38,  94, rpn));
                expect (detector.parseControllerMessage   (1, 6,   1,  rpn));

                expectEquals (rpn.channel, 1);
                expectEquals (rpn.parameterNumber, 300);
                expectEquals (rpn.value, 222);
                expect (! rpn.isNRPN);
                expect (rpn.is14BitValue);
            }

            beginTest ("RPNs on multiple channels simultaneously");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (1, 100, 44, rpn));
                expect (! detector.parseControllerMessage (2, 101, 0,  rpn));
                expect (! detector.parseControllerMessage (1, 101, 2,  rpn));
                expect (! detector.parseControllerMessage (2, 100, 7,  rpn));
                expect (! detector.parseControllerMessage (1, 38,  94, rpn));
                expect (detector.parseControllerMessage   (2, 6,   42, rpn));

                expectEquals (rpn.channel, 2);
                expectEquals (rpn.parameterNumber, 7);
                expectEquals (rpn.value, 42);
                expect (! rpn.isNRPN);
                expect (! rpn.is14BitValue);

                expect (detector.parseControllerMessage   (1, 6,   1,  rpn));

                expectEquals (rpn.channel, 1);
                expectEquals (rpn.parameterNumber, 300);
                expectEquals (rpn.value, 222);
                expect (! rpn.isNRPN);
                expect (rpn.is14BitValue);
            }

            beginTest ("14-bit RPN with value within 7-bit range");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (16, 100, 0 , rpn));
                expect (! detector.parseControllerMessage (16, 101, 0,  rpn));
                expect (! detector.parseControllerMessage (16, 38,  3,  rpn));
                expect (detector.parseControllerMessage   (16, 6,   0,  rpn));

                expectEquals (rpn.channel, 16);
                expectEquals (rpn.parameterNumber, 0);
                expectEquals (rpn.value, 3);
                expect (! rpn.isNRPN);
                expect (rpn.is14BitValue);
            }

            beginTest ("invalid RPN (wrong order)");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (2, 6,   42, rpn));
                expect (! detector.parseControllerMessage (2, 101, 0,  rpn));
                expect (! detector.parseControllerMessage (2, 100, 7,  rpn));
            }

            beginTest ("14-bit RPN interspersed with unrelated CC messages");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (16, 3,   80, rpn));
                expect (! detector.parseControllerMessage (16, 100, 0 , rpn));
                expect (! detector.parseControllerMessage (16, 4,   81, rpn));
                expect (! detector.parseControllerMessage (16, 101, 0,  rpn));
                expect (! detector.parseControllerMessage (16, 5,   82, rpn));
                expect (! detector.parseControllerMessage (16, 5,   83, rpn));
                expect (! detector.parseControllerMessage (16, 38,  3,  rpn));
                expect (! detector.parseControllerMessage (16, 4,   84, rpn));
                expect (! detector.parseControllerMessage (16, 3,   85, rpn));
                expect (detector.parseControllerMessage   (16, 6,   0,  rpn));

                expectEquals (rpn.channel, 16);
                expectEquals (rpn.parameterNumber, 0);
                expectEquals (rpn.value, 3);
                expect (! rpn.isNRPN);
                expect (rpn.is14BitValue);
            }

            beginTest ("14-bit NRPN");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (1, 98,  44, rpn));
                expect (! detector.parseControllerMessage (1, 99 , 2,  rpn));
                expect (! detector.parseControllerMessage (1, 38,  94, rpn));
                expect (detector.parseControllerMessage   (1, 6,   1,  rpn));

                expectEquals (rpn.channel, 1);
                expectEquals (rpn.parameterNumber, 300);
                expectEquals (rpn.value, 222);
                expect (rpn.isNRPN);
                expect (rpn.is14BitValue);
            }

            beginTest ("reset");
            {
                MidiRPNDetector detector;
                MidiRPNMessage rpn;
                expect (! detector.parseControllerMessage (2, 101, 0,  rpn));
                detector.reset();
                expect (! detector.parseControllerMessage (2, 100, 7,  rpn));
                expect (! detector.parseControllerMessage (2, 6,   42, rpn));
            }
        }
    };

    static MidiRPNDetectorTests MidiRPNDetectorUnitTests;


    class MidiRPNGeneratorTests   : public UnitTest
    {

        MidiRPNGeneratorTests()
            : UnitTest ("MidiRPNGenerator class", UnitTestCategories::midi)
        {}

        void runTest() override
        {
            beginTest ("generating RPN/NRPN");
            {
                {
                    MidiBuffer buffer = MidiRPNGenerator::generate (1, 23, 1337, true, true);
                    expectContainsRPN (buffer, 1, 23, 1337, true, true);
                }
                {
                    MidiBuffer buffer = MidiRPNGenerator::generate (16, 101, 34, false, false);
                    expectContainsRPN (buffer, 16, 101, 34, false, false);
                }
                {
                    MidiRPNMessage message = { 16, 101, 34, false, false };
                    MidiBuffer buffer = MidiRPNGenerator::generate (message);
                    expectContainsRPN (buffer, message);
                }
            }
        }


        
        void expectContainsRPN (const MidiBuffer& midiBuffer,
                                int channel,
                                int parameterNumber,
                                int value,
                                bool isNRPN,
                                bool is14BitValue)
        {
            MidiRPNMessage expected = { channel, parameterNumber, value, isNRPN, is14BitValue };
            expectContainsRPN (midiBuffer, expected);
        }

        
        void expectContainsRPN (const MidiBuffer& midiBuffer, MidiRPNMessage expected)
        {
            MidiRPNMessage result = MidiRPNMessage();
            MidiRPNDetector detector;

            for (const auto metadata : midiBuffer)
            {
                const auto midiMessage = metadata.getMessage();

                if (detector.parseControllerMessage (midiMessage.getChannel(),
                                                     midiMessage.getControllerNumber(),
                                                     midiMessage.getControllerValue(),
                                                     result))
                    break;
            }

            expectEquals (result.channel, expected.channel);
            expectEquals (result.parameterNumber, expected.parameterNumber);
            expectEquals (result.value, expected.value);
            expect (result.isNRPN == expected.isNRPN);
            expect (result.is14BitValue == expected.is14BitValue);
        }
    };

    static MidiRPNGeneratorTests MidiRPNGeneratorUnitTests;

    */
}

