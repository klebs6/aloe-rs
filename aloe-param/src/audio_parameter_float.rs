crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterFloat.h]

/**
  | A subclass of AudioProcessorParameter
  | that provides an easy way to create a
  | parameter which maps onto a given NormalisableRange.
  | 
  | @see AudioParameterInt, AudioParameterBool,
  | AudioParameterChoice
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioParameterFloat {

    base:                       RangedAudioParameter,

    /**
      | Provides access to the parameter's
      | range.
      |
      */
    range:                      NormalisableRange<f32>,

    value:                      Atomic<f32>,
    default_value:              f32,
    string_from_value_function: fn(_0: f32, _1: i32) -> String,
    value_from_string_function: fn(_0: &String) -> f32,
}

impl Into<f32> for AudioParameterFloat {

    /**
      | Returns the parameter's current value.
      |
      */
    #[inline] fn into(self) -> f32 {
        todo!();
        /*
            return value;
        */
    }
}

pub trait ValueChanged {

    fn value_changed(&mut self, new_value: f32);
}

impl ValueChanged for AudioParameterFloat {

    /**
      | Override this method if you are interested
      | in receiving callbacks when the parameter
      | value changes.
      |
      */
    fn value_changed(&mut self, new_value: f32)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterFloat.cpp]
impl Drop for AudioParameterFloat {
    fn drop(&mut self) {
        todo!();
        /*
            #if __cpp_lib_atomic_is_always_lock_free
         static_assert (std::atomic<float>::is_always_lock_free,
                        "AudioParameterFloat requires a lock-free std::atomic<float>");
        #endif
        */
    }
}

impl AudioParameterFloat {

    /**
      | Returns the parameter's current value.
      |
      */
    pub fn get(&self) -> f32 {
        
        todo!();
        /*
            return value;
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
      | Creates a AudioParameterFloat with
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
      | @param normalisableRange
      | 
      | The NormalisableRange to use
      | ----------
      | @param defaultValue
      | 
      | The non-normalised default value
      | ----------
      | @param parameterLabel
      | 
      | An optional label for the parameter's
      | value
      | ----------
      | @param parameterCategory
      | 
      | An optional parameter category
      | ----------
      | @param stringFromValue
      | 
      | An optional lambda function that converts
      | a non-normalised value to a string with
      | a maximum length. This may be used by
      | hosts to display the parameter's value.
      | ----------
      | @param valueFromString
      | 
      | An optional lambda function that parses
      | a string and converts it into a non-normalised
      | value. Some hosts use this to allow users
      | to type in parameter values.
      |
      */
    pub fn new(
        id_to_use:         &String,
        name_to_use:       &String,
        r:                 NormalisableRange<f32>,
        def:               f32,
        label_to_use:      Option<&str>,
        category_to_use:   Option<AudioProcessorParameterCategory>,
        string_from_value: fn(_0: f32, _1: i32) -> String,
        value_from_string: fn(_0: &String) -> f32

    ) -> Self {
    
        let label_to_use = label_to_use.unwrap_or("");

        let category_to_use =
            category_to_use.unwrap_or(AudioProcessorParameterCategory::genericParameter);

        todo!();
        /*


            : RangedAudioParameter (idToUse, nameToUse, labelToUse, categoryToUse),
         range (r), value (def), defaultValue (def),
         stringFromValueFunction (stringFromValue),
         valueFromStringFunction (valueFromString)

        if (stringFromValueFunction == nullptr)
        {
            auto numDecimalPlacesToDisplay = [this]
            {
                int numDecimalPlaces = 7;

                if (range.interval != 0.0f)
                {
                    if (approximatelyEqual (std::abs (range.interval - std::floor (range.interval)), 0.0f))
                        return 0;

                    auto v = std::abs (roundToInt (range.interval * pow (10, numDecimalPlaces)));

                    while ((v % 10) == 0 && numDecimalPlaces > 0)
                    {
                        --numDecimalPlaces;
                        v /= 10;
                    }
                }

                return numDecimalPlaces;
            }();

            stringFromValueFunction = [numDecimalPlacesToDisplay] (float v, int length)
            {
                String asText (v, numDecimalPlacesToDisplay);
                return length > 0 ? asText.substring (0, length) : asText;
            };
        }

        if (valueFromStringFunction == nullptr)
            valueFromStringFunction = [] (const String& text) { return text.getFloatValue(); };
        */
    }
    
    /**
      | Creates a AudioParameterFloat with
      | an ID, name, and range.
      | 
      | On creation, its value is set to the default
      | value.
      | 
      | For control over skew factors, you can
      | use the other constructor and provide
      | a NormalisableRange.
      |
      */
    pub fn new_with_id_name_and_range(
        pid:       String,
        nm:        String,
        min_value: f32,
        max_value: f32,
        def:       f32) -> Self {
    
        todo!();
        /*
            : AudioParameterFloat (pid, nm, { minValue, maxValue, 0.01f }, def)
        */
    }
    
    pub fn get_value(&self) -> f32 {
        
        todo!();
        /*
            return convertTo0to1 (value);
        */
    }
    
    pub fn set_value(&mut self, new_value: f32)  {
        
        todo!();
        /*
            value = convertFrom0to1 (newValue); valueChanged (get());
        */
    }
    
    pub fn get_default_value(&self) -> f32 {
        
        todo!();
        /*
            return convertTo0to1 (defaultValue);
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return AudioProcessorParameterWithID::getNumSteps();
        */
    }
    
    pub fn get_text(&self, 
        v:      f32,
        length: i32) -> String {
        
        todo!();
        /*
            return stringFromValueFunction (convertFrom0to1 (v), length);
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            return convertTo0to1 (valueFromStringFunction (text));
        */
    }
    
    /**
      | Changes the parameter's current value.
      |
      */
    pub fn assign_from(&mut self, new_value: f32) -> &mut AudioParameterFloat {
        
        todo!();
        /*
            if (value != newValue)
            setValueNotifyingHost (convertTo0to1 (newValue));

        return *this;
        */
    }
}
