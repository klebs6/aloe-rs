
#[cfg(ALOE_UNIT_TESTS)]
#[test] fn mpe_synthesiser_base_tests() {

    todo!();

    /*
    namespace
    {
        class MpeSynthesizerBaseTests : public UnitTest
        {
            enum class CallbackKind { process, midi };

            struct StartAndLength
            {
                StartAndLength (int s, int l) : start (s), length (l) {}

                int start  = 0;
                int length = 0;

                std::tuple<const int&, const int&> tie() const  { return std::tie (start, length); }

                bool operator== (const StartAndLength& other) const  { return tie() == other.tie(); }
                bool operator!= (const StartAndLength& other) const  { return tie() != other.tie(); }

                bool operator< (const StartAndLength& other) const  { return tie() < other.tie(); }
            };

            struct Events
            {
                std::vector<StartAndLength> blocks;
                std::vector<MidiMessage> messages;
                std::vector<CallbackKind> order;
            };

            class MockSynthesizer  : public MPESynthesizerBase
            {
            
                Events events;

                void handleMidiEvent (const MidiMessage& m) override
                {
                    events.messages.emplace_back (m);
                    events.order.emplace_back (CallbackKind::midi);
                }

            
                using MPESynthesizerBase::renderNextSubBlock;

                void renderNextSubBlock (AudioBuffer<float>&,
                                         int startSample,
                                         int numSamples) override
                {
                    events.blocks.push_back ({ startSample, numSamples });
                    events.order.emplace_back (CallbackKind::process);
                }
            };

            static MidiBuffer makeTestBuffer (const int bufferLength)
            {
                MidiBuffer result;

                for (int i = 0; i != bufferLength; ++i)
                    result.addEvent ({}, i);

                return result;
            }

        
            MpeSynthesizerBaseTests()
                : UnitTest ("MPE Synthesizer Base", UnitTestCategories::midi) {}

            void runTest() override
            {
                const auto sumBlockLengths = [] (const std::vector<StartAndLength>& b)
                {
                    const auto addBlock = [] (int acc, const StartAndLength& info) { return acc + info.length; };
                    return std::accumulate (b.begin(), b.end(), 0, addBlock);
                };

                beginTest ("Rendering sparse subblocks works");
                {
                    const int blockSize = 512;
                    const auto midi = [&] { MidiBuffer b; b.addEvent ({}, blockSize / 2); return b; }();
                    AudioBuffer<float> audio (1, blockSize);

                    const auto processEvents = [&] (int start, int length)
                    {
                        MockSynthesizer synth;
                        synth.setMinimumRenderingSubdivisionSize (1, false);
                        synth.setCurrentPlaybackSampleRate (44100);
                        synth.renderNextBlock (audio, midi, start, length);
                        return synth.events;
                    };

                    {
                        const auto e = processEvents (0, blockSize);
                        expect (e.blocks.size() == 2);
                        expect (e.messages.size() == 1);
                        expect (std::is_sorted (e.blocks.begin(), e.blocks.end()));
                        expect (sumBlockLengths (e.blocks) == blockSize);
                        expect (e.order == std::vector<CallbackKind> { CallbackKind::process,
                                                                       CallbackKind::midi,
                                                                       CallbackKind::process });
                    }
                }

                beginTest ("Rendering subblocks processes only contained midi events");
                {
                    const int blockSize = 512;
                    const auto midi = makeTestBuffer (blockSize);
                    AudioBuffer<float> audio (1, blockSize);

                    const auto processEvents = [&] (int start, int length)
                    {
                        MockSynthesizer synth;
                        synth.setMinimumRenderingSubdivisionSize (1, false);
                        synth.setCurrentPlaybackSampleRate (44100);
                        synth.renderNextBlock (audio, midi, start, length);
                        return synth.events;
                    };

                    {
                        const int subBlockLength = 0;
                        const auto e = processEvents (0, subBlockLength);
                        expect (e.blocks.size() == 0);
                        expect (e.messages.size() == 0);
                        expect (std::is_sorted (e.blocks.begin(), e.blocks.end()));
                        expect (sumBlockLengths (e.blocks) == subBlockLength);
                    }

                    {
                        const int subBlockLength = 0;
                        const auto e = processEvents (1, subBlockLength);
                        expect (e.blocks.size() == 0);
                        expect (e.messages.size() == 0);
                        expect (std::is_sorted (e.blocks.begin(), e.blocks.end()));
                        expect (sumBlockLengths (e.blocks) == subBlockLength);
                    }

                    {
                        const int subBlockLength = 1;
                        const auto e = processEvents (1, subBlockLength);
                        expect (e.blocks.size() == 1);
                        expect (e.messages.size() == 1);
                        expect (std::is_sorted (e.blocks.begin(), e.blocks.end()));
                        expect (sumBlockLengths (e.blocks) == subBlockLength);
                        expect (e.order == std::vector<CallbackKind> { CallbackKind::midi,
                                                                       CallbackKind::process });
                    }

                    {
                        const auto e = processEvents (0, blockSize);
                        expect (e.blocks.size() == blockSize);
                        expect (e.messages.size() == blockSize);
                        expect (std::is_sorted (e.blocks.begin(), e.blocks.end()));
                        expect (sumBlockLengths (e.blocks) == blockSize);
                        expect (e.order.front() == CallbackKind::midi);
                    }
                }

                beginTest ("Subblocks respect their minimum size");
                {
                    const int blockSize = 512;
                    const auto midi = makeTestBuffer (blockSize);
                    AudioBuffer<float> audio (1, blockSize);

                    const auto blockLengthsAreValid = [] (const std::vector<StartAndLength>& info, int minLength, bool strict)
                    {
                        if (info.size() <= 1)
                            return true;

                        const auto lengthIsValid = [&] (const StartAndLength& s) { return minLength <= s.length; };
                        const auto begin = strict ? info.begin() : std::next (info.begin());
                        // The final block is allowed to be shorter than the minLength
                        return std::all_of (begin, std::prev (info.end()), lengthIsValid);
                    };

                    for (auto strict : { false, true })
                    {
                        for (auto subblockSize : { 1, 16, 32, 64, 1024 })
                        {
                            MockSynthesizer synth;
                            synth.setMinimumRenderingSubdivisionSize (subblockSize, strict);
                            synth.setCurrentPlaybackSampleRate (44100);
                            synth.renderNextBlock (audio, midi, 0, blockSize);

                            const auto& e = synth.events;
                            expectWithinAbsoluteError (float (e.blocks.size()),
                                                       std::ceil ((float) blockSize / (float) subblockSize),
                                                       1.0f);
                            expect (e.messages.size() == blockSize);
                            expect (std::is_sorted (e.blocks.begin(), e.blocks.end()));
                            expect (sumBlockLengths (e.blocks) == blockSize);
                            expect (blockLengthsAreValid (e.blocks, subblockSize, strict));
                        }
                    }

                    {
                        MockSynthesizer synth;
                        synth.setMinimumRenderingSubdivisionSize (32, true);
                        synth.setCurrentPlaybackSampleRate (44100);
                        synth.renderNextBlock (audio, MidiBuffer{}, 0, 16);

                        expect (synth.events.blocks == std::vector<StartAndLength> { { 0, 16 } });
                        expect (synth.events.order == std::vector<CallbackKind> { CallbackKind::process });
                        expect (synth.events.messages.empty());
                    }
                }
            }
        };

        MpeSynthesizerBaseTests mpeSynthesizerBaseTests;
    }
    */
}
