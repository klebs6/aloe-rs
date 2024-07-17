crate::ix!();

pub struct AudioProcessorValueTreeStateTests {
    base: UnitTest,
}

pub mod audio_processor_value_tree_state_tests {
    use super::*;

    pub type Parameter       = AudioProcessorValueTreeState::Parameter;
    pub type ParameterGroup  = AudioProcessorParameterGroup;
    pub type ParameterLayout = AudioProcessorValueTreeState::ParameterLayout;

    ///--------------------------
    #[derive(Default)]
    pub struct TestAudioProcessor {
        base:  AudioProcessor,
        state: AudioProcessorValueTreeState, //{ *this, nullptr };
    }

    impl TestAudioProcessor {

        pub fn new(layout: ParameterLayout) -> Self {
        
            todo!();
            /*


                : state (*this, nullptr, "state", std::move (layout))
            */
        }
        
        pub fn get_name(&self) -> String {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn prepare_to_play(&mut self, _0: f64, _1: i32)  {
            
        }
        
        pub fn release_resources(&mut self)  {
            
        }
        
        pub fn process_block(&mut self, 
            _0: &mut AudioBuffer<f32>,
            _1: &mut MidiBuffer)  {
            
        }
        
        pub fn get_tail_length_seconds(&self) -> f64 {
            
        }
        
        pub fn accepts_midi(&self) -> bool {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn produces_midi(&self) -> bool {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn has_editor(&self) -> bool {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn get_num_programs(&mut self) -> i32 {
            
            todo!();
            /*
                return 1;
            */
        }
        
        pub fn get_current_program(&mut self) -> i32 {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn set_current_program(&mut self, _0: i32)  {
            
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
            
        }
        
        pub fn get_state_information(&mut self, _0: &mut MemoryBlock)  {
            
        }
        
        pub fn set_state_information(&mut self, 
            _0: *const c_void,
            _1: i32)  {
            
        }
    }

    ///--------------------------
    pub struct Listener {
        base:  AudioProcessorValueTreeState::Listener,
        id:    String,
        value: f32,
    }

    impl Listener {

        pub fn parameter_changed(&mut self, 
            id_in:    &String,
            value_in: f32)  {
            
            todo!();
            /*
                id = idIn;
                    value = valueIn;
            */
        }
    }
}

impl AudioProcessorValueTreeStateTests {
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            ScopedAloeInitialiser_GUI scopedAloeInitialiser_gui;

            beginTest ("After calling createAndAddParameter, the number of parameters increases by one");
            {
                TestAudioProcessor proc;

                proc.state.createAndAddParameter (std::make_unique<Parameter> (String(), String(), String(), NormalisableRange<float>(),
                                                                               0.0f, nullptr, nullptr));

                expectEquals (proc.getParameters().size(), 1);
            }

            beginTest ("After creating a normal named parameter, we can later retrieve that parameter");
            {
                TestAudioProcessor proc;

                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr));

                expect (proc.state.getParameter (key) == param);
            }

            beginTest ("After construction, the value tree has the expected format");
            {
                TestAudioProcessor proc ({
                    std::make_unique<AudioProcessorParameterGroup> ("A", "", "",
                        std::make_unique<AudioParameterBool> ("a", "", false),
                        std::make_unique<AudioParameterFloat> ("b", "", NormalisableRange<float>{}, 0.0f)),
                    std::make_unique<AudioProcessorParameterGroup> ("B", "", "",
                        std::make_unique<AudioParameterInt> ("c", "", 0, 1, 0),
                        std::make_unique<AudioParameterChoice> ("d", "", StringArray { "foo", "bar" }, 0)) });

                const auto valueTree = proc.state.copyState();

                expectEquals (valueTree.getNumChildren(), 4);

                for (auto child : valueTree)
                {
                    expect (child.hasType ("PARAM"));
                    expect (child.hasProperty ("id"));
                    expect (child.hasProperty ("value"));
                }
            }

            beginTest ("Meta parameters can be created");
            {
                TestAudioProcessor proc;

                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr, true));

                expect (param->isMetaParameter());
            }

            beginTest ("Automatable parameters can be created");
            {
                TestAudioProcessor proc;

                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr, false, true));

                expect (param->isAutomatable());
            }

            beginTest ("Discrete parameters can be created");
            {
                TestAudioProcessor proc;

                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr, false, false, true));

                expect (param->isDiscrete());
            }

            beginTest ("Custom category parameters can be created");
            {
                TestAudioProcessor proc;

                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr, false, false, false,
                                                                                                  AudioProcessorParameter::Category::inputMeter));

                expect (param->category == AudioProcessorParameter::Category::inputMeter);
            }

            beginTest ("Boolean parameters can be created");
            {
                TestAudioProcessor proc;

                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr, false, false, false,
                                                                                                  AudioProcessorParameter::Category::genericParameter, true));

                expect (param->isBoolean());
            }

            beginTest ("After creating a custom named parameter, we can later retrieve that parameter");
            {
                const auto key = "id";
                auto param = std::make_unique<AudioParameterBool> (key, "", false);
                const auto paramPtr = param.get();

                TestAudioProcessor proc (std::move (param));

                expect (proc.state.getParameter (key) == paramPtr);
            }

            beginTest ("After adding a normal parameter that already exists, the AudioProcessor parameters are unchanged");
            {
                TestAudioProcessor proc;
                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr));

                proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                               0.0f, nullptr, nullptr));

                expectEquals (proc.getParameters().size(), 1);
                expect (proc.getParameters().getFirst() == param);
            }

            beginTest ("After setting a parameter value, that value is reflected in the state");
            {
                TestAudioProcessor proc;
                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr));

                const auto value = 0.5f;
                param->setValueNotifyingHost (value);

                expectEquals (proc.state.getRawParameterValue (key)->load(), value);
            }

            beginTest ("After adding an APVTS::Parameter, its value is the default value");
            {
                TestAudioProcessor proc;
                const auto key = "id";
                const auto value = 5.0f;

                proc.state.createAndAddParameter (std::make_unique<Parameter> (
                    key,
                    String(),
                    String(),
                    NormalisableRange<float> (0.0f, 100.0f, 10.0f),
                    value,
                    nullptr,
                    nullptr));

                expectEquals (proc.state.getRawParameterValue (key)->load(), value);
            }

            beginTest ("Listeners receive notifications when parameters change");
            {
                Listener listener;
                TestAudioProcessor proc;
                const auto key = "id";
                const auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                                  0.0f, nullptr, nullptr));
                proc.state.addParameterListener (key, &listener);

                const auto value = 0.5f;
                param->setValueNotifyingHost (value);

                expectEquals (listener.id, String { key });
                expectEquals (listener.value, value);
            }

            beginTest ("Bool parameters have a range of 0-1");
            {
                const auto key = "id";

                TestAudioProcessor proc (std::make_unique<AudioParameterBool> (key, "", false));

                expect (proc.state.getParameterRange (key) == NormalisableRange<float> (0.0f, 1.0f, 1.0f));
            }

            beginTest ("Float parameters retain their specified range");
            {
                const auto key = "id";
                const auto range = NormalisableRange<float> { -100, 100, 0.7f, 0.2f, true };

                TestAudioProcessor proc (std::make_unique<AudioParameterFloat> (key, "", range, 0.0f));

                expect (proc.state.getParameterRange (key) == range);
            }

            beginTest ("Int parameters retain their specified range");
            {
                const auto key = "id";
                const auto min = -27;
                const auto max = 53;

                TestAudioProcessor proc (std::make_unique<AudioParameterInt> (key, "", min, max, 0));

                expect (proc.state.getParameterRange (key) == NormalisableRange<float> (float (min), float (max), 1.0f));
            }

            beginTest ("Choice parameters retain their specified range");
            {
                const auto key = "id";
                const auto choices = StringArray { "", "", "" };

                TestAudioProcessor proc (std::make_unique<AudioParameterChoice> (key, "", choices, 0));

                expect (proc.state.getParameterRange (key) == NormalisableRange<float> (0.0f, (float) (choices.size() - 1), 1.0f));
                expect (proc.state.getParameter (key)->getNumSteps() == choices.size());
            }

            beginTest ("When the parameter value is changed, normal parameter values are updated");
            {
                TestAudioProcessor proc;
                const auto key = "id";
                const auto initialValue = 0.2f;
                auto param = proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                                            initialValue, nullptr, nullptr));
                proc.state.state = ValueTree { "state" };

                auto value = proc.state.getParameterAsValue (key);
                expectEquals (float (value.getValue()), initialValue);

                const auto newValue = 0.75f;
                value = newValue;

                expectEquals (param->getValue(), newValue);
                expectEquals (proc.state.getRawParameterValue (key)->load(), newValue);
            }

            beginTest ("When the parameter value is changed, custom parameter values are updated");
            {
                const auto key = "id";
                const auto choices = StringArray ("foo", "bar", "baz");
                auto param = std::make_unique<AudioParameterChoice> (key, "", choices, 0);
                const auto paramPtr = param.get();
                TestAudioProcessor proc (std::move (param));

                const auto newValue = 2.0f;
                auto value = proc.state.getParameterAsValue (key);
                value = newValue;

                expectEquals (paramPtr->getCurrentChoiceName(), choices[int (newValue)]);
                expectEquals (proc.state.getRawParameterValue (key)->load(), newValue);
            }

            beginTest ("When the parameter value is changed, listeners are notified");
            {
                Listener listener;
                TestAudioProcessor proc;
                const auto key = "id";
                proc.state.createAndAddParameter (std::make_unique<Parameter> (key, String(), String(), NormalisableRange<float>(),
                                                                               0.0f, nullptr, nullptr));
                proc.state.addParameterListener (key, &listener);
                proc.state.state = ValueTree { "state" };

                const auto newValue = 0.75f;
                proc.state.getParameterAsValue (key) = newValue;

                expectEquals (listener.value, newValue);
                expectEquals (listener.id, String { key });
            }

            beginTest ("When the parameter value is changed, listeners are notified");
            {
                const auto key = "id";
                const auto choices = StringArray { "foo", "bar", "baz" };
                Listener listener;
                TestAudioProcessor proc (std::make_unique<AudioParameterChoice> (key, "", choices, 0));
                proc.state.addParameterListener (key, &listener);

                const auto newValue = 2.0f;
                proc.state.getParameterAsValue (key) = newValue;

                expectEquals (listener.value, newValue);
                expectEquals (listener.id, String (key));
            }
        */
    }
}
