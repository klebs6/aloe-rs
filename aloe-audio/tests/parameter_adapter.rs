crate::ix!();

#[cfg(test)]
pub mod parameter_adapter_tests {

    use super::*;

    #[test] fn default_value_returned_correctly() {
        /*
            const auto test = [&] (NormalisableRange<float> range, float value)
            {
                AudioParameterFloat param ({}, {}, range, value, {});

                AudioProcessorValueTreeState::ParameterAdapter adapter (param);

                expectEquals (adapter.getDenormalisedDefaultValue(), value);
            };

            test ({ -100, 100 }, 0);
            test ({ -2.5, 12.5 }, 10);
        */
    }

    #[test] fn denormalized_parameter_values_can_be_retrieved() {
        /*
            const auto test = [&] (NormalisableRange<float> range, float value)
            {
                AudioParameterFloat param ({}, {}, range, {}, {});
                AudioProcessorValueTreeState::ParameterAdapter adapter (param);

                adapter.setDenormalisedValue (value);

                expectEquals (adapter.getDenormalisedValue(), value);
                expectEquals (adapter.getRawDenormalisedValue().load(), value);
            };

            test ({ -20, -10 }, -15);
            test ({ 0, 7.5 }, 2.5);
        */
    }

    #[test] fn floats_can_be_converted_to_text() {
        /*
            const auto test = [&] (NormalisableRange<float> range, float value, String expected)
            {
                AudioParameterFloat param ({}, {}, range, {}, {});
                AudioProcessorValueTreeState::ParameterAdapter adapter (param);

                expectEquals (adapter.getTextForDenormalisedValue (value), expected);
            };

            test ({ -100, 100 }, 0, "0.0000000");
            test ({ -2.5, 12.5 }, 10, "10.0000000");
            test ({ -20, -10 }, -15, "-15.0000000");
            test ({ 0, 7.5 }, 2.5, "2.5000000");
        */
    }

    #[test] fn text_can_be_converted_to_floats() {

        /*
            const auto test = [&] (NormalisableRange<float> range, String text, float expected)
            {
                AudioParameterFloat param ({}, {}, range, {}, {});
                AudioProcessorValueTreeState::ParameterAdapter adapter (param);

                expectEquals (adapter.getDenormalisedValueForText (text), expected);
            };

            test ({ -100, 100 }, "0.0", 0);
            test ({ -2.5, 12.5 }, "10.0", 10);
            test ({ -20, -10 }, "-15.0", -15);
            test ({ 0, 7.5 }, "2.5", 2.5);
        */
    }
}

