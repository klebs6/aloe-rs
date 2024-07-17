crate::ix!();

pub trait ValueChanged {

    /**
      | Override this method if you are interested
      | in receiving callbacks when the parameter
      | value changes.
      |
      */
    fn value_changed(&mut self, new_value: bool);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterBool.h]

/**
  | Provides a class of AudioProcessorParameter
  | that can be used as a boolean value.
  | 
  | @see AudioParameterFloat, AudioParameterInt,
  | AudioParameterChoice
  | 
  | @tags{Audio}
  |
  */
pub struct AudioParameterBool {
    base: RangedAudioParameter,
}

pub trait AudioParameterBoolInterface: ValueChanged {}

impl Into<bool> for AudioParameterBool {

    /**
      | Returns the parameter's current boolean
      | value.
      |
      */
    #[inline] fn into(self) -> bool {
        todo!();
        /*
            return get();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterBool.cpp]
impl Drop for AudioParameterBool {

    fn drop(&mut self) {
        todo!();
        /*
            #if __cpp_lib_atomic_is_always_lock_free
         static_assert (std::atomic<float>::is_always_lock_free,
                        "AudioParameterBool requires a lock-free std::atomic<float>");
        #endif
        */
    }
}

impl AudioParameterBool {

    /**
      | Returns the parameter's current boolean
      | value.
      |
      */
    pub fn get(&self) -> bool {
        
        todo!();
        /*
            return value >= 0.5f;
        */
    }

    /**
      | Returns the range of values that the
      | parameter can take.
      |
      */
    pub fn get_normalisable_range(&self) -> &NormalisableRange<f32> {
        
        todo!();
        /*
            return range;
        */
    }
    
    /**
      | Creates a AudioParameterBool with
      | the specified parameters.
      | 
      | -----------
      | @param parameterID
      | 
      | The parameter ID to use
      | ----------
      | @param parameterName
      | 
      | The parameter name to use
      | ----------
      | @param defaultValue
      | 
      | The default value
      | ----------
      | @param parameterLabel
      | 
      | An optional label for the parameter's
      | value
      | ----------
      | @param stringFromBool
      | 
      | An optional lambda function that converts
      | a bool value to a string with a maximum
      | length. This may be used by hosts to display
      | the parameter's value.
      | ----------
      | @param boolFromString
      | 
      | An optional lambda function that parses
      | a string and converts it into a bool value.
      | Some hosts use this to allow users to
      | type in parameter values.
      |
      */
    pub fn new(
        id_to_use:        &String,
        name_to_use:      &String,
        def:              bool,
        label_to_use:     Option<&str>,
        string_from_bool: fn(_0: bool, _1: i32) -> String,
        bool_from_string: fn(_0: &String) -> bool

    ) -> Self {

        let label_to_use = label_to_use.unwrap_or("");

        todo!();
        /*


            : RangedAudioParameter (idToUse, nameToUse, labelToUse),
         value (def ? 1.0f : 0.0f),
         defaultValue (value),
         stringFromBoolFunction (stringFromBool),
         boolFromStringFunction (boolFromString)

        if (stringFromBoolFunction == nullptr)
            stringFromBoolFunction = [] (bool v, int) { return v ? TRANS("On") : TRANS("Off"); };

        if (boolFromStringFunction == nullptr)
        {
            StringArray onStrings;
            onStrings.add (TRANS("on"));
            onStrings.add (TRANS("yes"));
            onStrings.add (TRANS("true"));

            StringArray offStrings;
            offStrings.add (TRANS("off"));
            offStrings.add (TRANS("no"));
            offStrings.add (TRANS("false"));

            boolFromStringFunction = [onStrings, offStrings] (const String& text)
            {
                String lowercaseText (text.toLowerCase());

                for (auto& testText : onStrings)
                    if (lowercaseText == testText)
                        return true;

                for (auto& testText : offStrings)
                    if (lowercaseText == testText)
                        return false;

                return text.getIntValue() != 0;
            };
        }
        */
    }
    
    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            return value;
        */
    }
    
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            value = newValue; valueChanged (get());
        */
    }
    
    pub fn get_default_value(&self) -> f32 {
        
        todo!();
        /*
            return defaultValue;
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return 2;
        */
    }
    
    pub fn is_discrete(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_boolean(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn value_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            return boolFromStringFunction (text) ? 1.0f : 0.0f;
        */
    }
    
    pub fn get_text(&self, 
        v:              f32,
        maximum_length: i32) -> String {
        
        todo!();
        /*
            return stringFromBoolFunction (v >= 0.5f, maximumLength);
        */
    }
    
    /**
      | Changes the parameter's current value
      | to a new boolean.
      |
      */
    pub fn assign_from(&mut self, new_value: bool) -> &mut AudioParameterBool {
        
        todo!();
        /*
            if (get() != newValue)
            setValueNotifyingHost (newValue ? 1.0f : 0.0f);

        return *this;
        */
    }
}
