crate::ix!();

pub trait AudioProcessorParameterInterface:
GetValue
+ SetValue
+ GetDefaultValue
+ GetNameWithMaxLen
+ GetLabel
+ GetNumSteps
+ IsDiscrete
+ IsBoolean
+ GetText
+ GetValueForText
+ CheckIsOrientationInverted
+ CheckIsAutomatable
+ CheckIsMetaParameter
+ GetCategory
+ GetCurrentValueAsText
+ GetAllValueStrings {}

pub trait GetValue {

    /**
      | Called by the host to find out the value
      | of this parameter.
      | 
      | Hosts will expect the value returned
      | to be between 0 and 1.0.
      | 
      | This could be called quite frequently,
      | so try to make your code efficient.
      | 
      | It's also likely to be called by non-UI
      | threads, so the code in here should be
      | thread-aware.
      |
      */
    fn get_value(&self) -> f32;
}

pub trait SetValue {

    /**
      | The host will call this method to change
      | the value of a parameter.
      | 
      | The host may call this at any time, including
      | during the audio processing callback,
      | so your implementation has to process
      | this very efficiently and avoid any
      | kind of locking.
      | 
      | If you want to set the value of a parameter
      | internally, e.g. from your editor component,
      | then don't call this directly - instead,
      | use the setValueNotifyingHost() method,
      | which will also send a message to the
      | host telling it about the change. If
      | the message isn't sent, the host won't
      | be able to automate your parameters
      | properly.
      | 
      | The value passed will be between 0 and
      | 1.0.
      |
      */
    fn set_value(&mut self, new_value: f32);
}

pub trait GetDefaultValue {

    /**
      | This should return the default value
      | for this parameter.
      |
      */
    fn get_default_value(&self) -> f32;
}

pub trait GetNameWithMaxLen {

    /**
      | Returns the name to display for this
      | parameter, which should be made to fit
      | within the given string length.
      |
      */
    fn get_name(&self, maximum_string_length: i32) -> String;
}

pub trait GetLabel {

    /**
      | Some parameters may be able to return
      | a label string for their units. For example
      | "Hz" or "%".
      |
      */
    fn get_label(&self) -> String;
}

pub trait GetNumSteps {

    /**
      | Returns the number of steps that this
      | parameter's range should be quantised
      | into.
      | 
      | If you want a continuous range of values,
      | don't override this method, and allow
      | the default implementation to return
      | AudioProcessor::getDefaultNumParameterSteps().
      | 
      | If your parameter is boolean, then you
      | may want to make this return 2.
      | 
      | The value that is returned may or may
      | not be used, depending on the host. If
      | you want the host to display stepped
      | automation values, rather than a continuous
      | interpolation between successive
      | values, you should override isDiscrete
      | to return true.
      | 
      | @see isDiscrete
      |
      */
    fn get_num_steps(&self) -> i32;
}

pub trait IsDiscrete {

    /**
      | Returns whether the parameter uses
      | discrete values, based on the result
      | of getNumSteps, or allows the host to
      | select values continuously.
      | 
      | This information may or may not be used,
      | depending on the host. If you want the
      | host to display stepped automation
      | values, rather than a continuous interpolation
      | between successive values, override
      | this method to return true.
      | 
      | @see getNumSteps
      |
      */
    fn is_discrete(&self) -> bool;

}

pub trait IsBoolean {

    /**
      | Returns whether the parameter represents
      | a boolean switch, typically with "On"
      | and "Off" states.
      | 
      | This information may or may not be used,
      | depending on the host. If you want the
      | host to display a switch, rather than
      | a two item dropdown menu, override this
      | method to return true. You also need
      | to override isDiscrete() to return
      | `true` and getNumSteps() to return
      | `2`.
      | 
      | @see isDiscrete getNumSteps
      |
      */
    fn is_boolean(&self) -> bool;

}

pub trait GetText {

    /**
      | Returns a textual version of the supplied
      | normalised parameter value.
      | 
      | The default implementation just returns
      | the floating point value as a string,
      | but this could do anything you need for
      | a custom type of value.
      |
      */
    fn get_text(&self, 
            normalised_value:      f32,
            maximum_string_length: i32) -> String;

}

pub trait GetValueForText {

    /**
      | Should parse a string and return the
      | appropriate value for it.
      |
      */
    fn get_value_for_text(&self, text: &String) -> f32;
}

pub trait CheckIsOrientationInverted {

    /**
      | This can be overridden to tell the host
      | that this parameter operates in the
      | reverse direction. (Not all plugin
      | formats or hosts will actually use this
      | information).
      |
      */
    fn is_orientation_inverted(&self) -> bool;
}

pub trait CheckIsAutomatable {

    /**
      | Returns true if the host can automate
      | this parameter.
      | 
      | By default, this returns true.
      |
      */
    fn is_automatable(&self) -> bool;
}

pub trait CheckIsMetaParameter {

    /**
      | Should return true if this parameter
      | is a "meta" parameter.
      | 
      | A meta-parameter is a parameter that
      | changes other params. It is used by some
      | hosts (e.g. AudioUnit hosts).
      | 
      | By default this returns false.
      |
      */
    fn is_meta_parameter(&self) -> bool;
}

pub trait GetCategory {

    /**
      | Returns the parameter's category.
      |
      */
    fn get_category(&self) -> AudioProcessorParameterCategory;
}

pub trait GetCurrentValueAsText {

    /**
      | Returns the current value of the parameter
      | as a String.
      | 
      | This function can be called when you
      | are hosting plug-ins to get a more specialised
      | textual representation of the current
      | value from the plug-in, for example
      | "On" rather than "1.0".
      | 
      | If you are implementing a plug-in then
      | you should ignore this function and
      | instead override getText.
      |
      */
    fn get_current_value_as_text(&self) -> String;
}

pub trait GetAllValueStrings {

    /**
      | Returns the set of strings which represent
      | the possible states a parameter can
      | be in.
      | 
      | If you are hosting a plug-in you can use
      | the result of this function to populate
      | a ComboBox listing the allowed values.
      | 
      | If you are implementing a plug-in then
      | you do not need to override this.
      |
      */
    fn get_all_value_strings(&self) -> Vec<String>;
}
