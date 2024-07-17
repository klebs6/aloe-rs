crate::ix!();

#[cfg(test)]
pub mod parameter_group_tests {

    use super::*;

    pub struct TestAudioProcessor {
        base: AudioProcessor,
    }
    impl TestAudioProcessor {

        pub fn get_name(&self) -> String {
            
            todo!();
            /*
                return "ap";
            */
        }
        
        pub fn prepare_to_play(&mut self, _0: f64, _1: i32)  {
            
            todo!();
            /*
            
            */
        }
        
        pub fn release_resources(&mut self)  {
            
            todo!();
            /*
            
            */
        }
        
        pub fn process_block(&mut self, 
            _0: &mut AudioBuffer<f32>,
            _1: &mut MidiBuffer)  {
            
            todo!();
            /*
            
            */
        }
        
        pub fn get_tail_length_seconds(&self) -> f64 {
            
            todo!();
            /*
                return 0.0;
            */
        }
        
        pub fn accepts_midi(&self) -> bool {
            
            todo!();
            /*
                return false;
            */
        }
        
        pub fn produces_midi(&self) -> bool {
            
            todo!();
            /*
                return false;
            */
        }
        
        pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
            
            todo!();
            /*
                return nullptr;
            */
        }
        
        pub fn has_editor(&self) -> bool {
            
            todo!();
            /*
                return false;
            */
        }
        
        pub fn get_num_programs(&mut self) -> i32 {
            
            todo!();
            /*
                return 0;
            */
        }
        
        pub fn get_current_program(&mut self) -> i32 {
            
            todo!();
            /*
                return 0;
            */
        }
        
        pub fn set_current_program(&mut self, _0: i32)  {
            
            todo!();
            /*
            
            */
        }
        
        pub fn get_program_name(&mut self, _0: i32) -> String {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn change_program_name(&mut self, 
            _0: i32,
            _1: &String)  {
            
            todo!();
            /*
            
            */
        }
        
        pub fn get_state_information(&mut self, _0: &mut MemoryBlock)  {
            
            todo!();
            /*
            
            */
        }
        
        pub fn set_state_information(&mut self, 
            _0: *const c_void,
            _1: i32)  {
            
            todo!();
            /*
            
            */
        }
    }

    pub fn run_test()  {
        
        todo!();
        /*
            beginTest ("ParameterGroups");

            auto g1 = std::make_unique<AudioProcessorParameterGroup> ("g1", "g1", " - ");

            auto* p1 = new AudioParameterFloat ("p1", "p1", { 0.0f, 2.0f }, 0.5f);
            auto* p2 = new AudioParameterFloat ("p2", "p2", { 0.0f, 2.0f }, 0.5f);
            auto* p3 = new AudioParameterFloat ("p3", "p3", { 0.0f, 2.0f }, 0.5f);

            g1->addChild (std::unique_ptr<AudioParameterFloat> (p1));
            g1->addChild (std::unique_ptr<AudioParameterFloat> (p2),
                          std::unique_ptr<AudioParameterFloat> (p3));

            auto p4 = std::make_unique<AudioParameterFloat> ("p4", "p4", NormalisableRange<float> (0.0f, 2.0f), 0.5f);
            auto p5 = std::make_unique<AudioParameterFloat> ("p5", "p5", NormalisableRange<float> (0.0f, 2.0f), 0.5f);
            auto p6 = std::make_unique<AudioParameterFloat> ("p6", "p6", NormalisableRange<float> (0.0f, 2.0f), 0.5f);

            g1->addChild (std::move (p4));
            g1->addChild (std::move (p5),
                          std::move (p6));

            {
                auto topLevelParams = g1->getParameters (false);
                auto params = g1->getParameters (true);
                expect (topLevelParams == params);
                expectEquals (params.size(), 6);

                expect (params[0] == (AudioProcessorParameter*) p1);
                expect (params[1] == (AudioProcessorParameter*) p2);
                expect (params[2] == (AudioProcessorParameter*) p3);

                expect (dynamic_cast<AudioParameterFloat*> (params[3])->name == "p4");
                expect (dynamic_cast<AudioParameterFloat*> (params[4])->name == "p5");
                expect (dynamic_cast<AudioParameterFloat*> (params[5])->name == "p6");
            }

            auto* p7 = new AudioParameterFloat ("p7", "p7", { 0.0f, 2.0f }, 0.5f);
            auto* p8 = new AudioParameterFloat ("p8", "p8", { 0.0f, 2.0f }, 0.5f);
            auto* p9 = new AudioParameterFloat ("p9", "p9", { 0.0f, 2.0f }, 0.5f);

            auto p10 = std::make_unique<AudioParameterFloat> ("p10", "p10", NormalisableRange<float> (0.0f, 2.0f), 0.5f);
            auto p11 = std::make_unique<AudioParameterFloat> ("p11", "p11", NormalisableRange<float> (0.0f, 2.0f), 0.5f);
            auto p12 = std::make_unique<AudioParameterFloat> ("p12", "p12", NormalisableRange<float> (0.0f, 2.0f), 0.5f);

            auto g2 = std::make_unique<AudioProcessorParameterGroup> ("g2", "g2", " | ", std::unique_ptr<AudioParameterFloat> (p7));
            auto g3 = std::make_unique<AudioProcessorParameterGroup> ("g3", "g3", " | ", std::unique_ptr<AudioParameterFloat> (p8), std::unique_ptr<AudioParameterFloat> (p9));
            auto g4 = std::make_unique<AudioProcessorParameterGroup> ("g4", "g4", " | ", std::move (p10));
            auto g5 = std::make_unique<AudioProcessorParameterGroup> ("g5", "g5", " | ", std::move (p11), std::move (p12));

            g1->addChild (std::move (g2));
            g4->addChild (std::move (g5));
            g1->addChild (std::move (g3), std::move (g4));

            {
                auto topLevelParams = g1->getParameters (false);
                auto params = g1->getParameters (true);
                expectEquals (topLevelParams.size(), 6);
                expectEquals (params.size(), 12);

                expect (params[0] == (AudioProcessorParameter*) p1);
                expect (params[1] == (AudioProcessorParameter*) p2);
                expect (params[2] == (AudioProcessorParameter*) p3);

                expect (dynamic_cast<AudioParameterFloat*> (params[3])->name == "p4");
                expect (dynamic_cast<AudioParameterFloat*> (params[4])->name == "p5");
                expect (dynamic_cast<AudioParameterFloat*> (params[5])->name == "p6");

                expect (params[6] == (AudioProcessorParameter*) p7);
                expect (params[7] == (AudioProcessorParameter*) p8);
                expect (params[8] == (AudioProcessorParameter*) p9);

                expect (dynamic_cast<AudioParameterFloat*> (params[9]) ->name == "p10");
                expect (dynamic_cast<AudioParameterFloat*> (params[10])->name == "p11");
                expect (dynamic_cast<AudioParameterFloat*> (params[11])->name == "p12");
            }

            g1->addChild (std::make_unique<AudioProcessorParameterGroup> ("g6", "g6", " | ",
                                                                          std::make_unique<AudioParameterFloat> ("p13", "p13", NormalisableRange<float> (0.0f, 2.0f), 0.5f),
                                                                          std::make_unique<AudioProcessorParameterGroup> ("g7", "g7", " | ",
                                                                                                                          std::make_unique<AudioParameterFloat> ("p14", "p14", NormalisableRange<float> (0.0f, 2.0f), 0.5f)),
                                                                          std::make_unique<AudioParameterFloat> ("p15", "p15", NormalisableRange<float> (0.0f, 2.0f), 0.5f)));

            TestAudioProcessor processor;

            processor.addParameter (new AudioParameterFloat ("pstart", "pstart", NormalisableRange<float> (0.0f, 2.0f), 0.5f));
            auto groupParams = g1->getParameters (true);
            processor.addParameterGroup (std::move (g1));
            processor.addParameter (new AudioParameterFloat ("pend", "pend", NormalisableRange<float> (0.0f, 2.0f), 0.5f));

            auto& processorParams = processor.getParameters();
            expect (dynamic_cast<AudioParameterFloat*> (processorParams.getFirst())->name == "pstart");
            expect (dynamic_cast<AudioParameterFloat*> (processorParams.getLast()) ->name == "pend");

            auto numParams = processorParams.size();

            for (int i = 1; i < numParams - 1; ++i)
                expect (processorParams[i] == groupParams[i - 1]);
        */
    }
}
