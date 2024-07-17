crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct AudioParameterChoiceTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for AudioParameterChoiceTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("AudioParameterChoice", UnitTestCategories::audioProcessorParameters)
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl AudioParameterChoiceTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Three options switches at the correct points");
            {
                AudioParameterChoice choice ({}, {}, { "a", "b", "c" }, {});

                choice.setValueNotifyingHost (0.0f);
                expectEquals (choice.getIndex(), 0);

                choice.setValueNotifyingHost (0.2f);
                expectEquals (choice.getIndex(), 0);

                choice.setValueNotifyingHost (0.3f);
                expectEquals (choice.getIndex(), 1);

                choice.setValueNotifyingHost (0.7f);
                expectEquals (choice.getIndex(), 1);

                choice.setValueNotifyingHost (0.8f);
                expectEquals (choice.getIndex(), 2);

                choice.setValueNotifyingHost (1.0f);
                expectEquals (choice.getIndex(), 2);
            }

            beginTest ("Out-of-bounds input");
            {
                AudioParameterChoice choiceParam ({}, {}, { "a", "b", "c" }, {});

                choiceParam.setValueNotifyingHost (-0.5f);
                expectEquals (choiceParam.getIndex(), 0);

                choiceParam.setValueNotifyingHost (1.5f);
                expectEquals (choiceParam.getIndex(), 2);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static AudioParameterChoiceTests audioParameterChoiceTests;
    */
}
