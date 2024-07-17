crate::ix!();

pub trait ValueChanged {

    /**
      | Override this method if you are interested
      | in receiving callbacks when the parameter
      | value changes.
      |
      */
    fn value_changed(&mut self, new_value: i32);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterInt.h]

/**
  | Provides a class of AudioProcessorParameter
  | that can be used as an integer value with
  | a given range.
  | 
  | @see AudioParameterFloat, AudioParameterBool,
  | AudioParameterChoice
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioParameterInt {
    base:                     RangedAudioParameter,
    range:                    NormalisableRange<f32>,
    value:                    Atomic<f32>,
    default_value:            f32,
    string_from_int_function: fn(_0: i32, _1: i32) -> String,
    int_from_string_function: fn(_0: &String) -> i32,
}

pub trait AudioParameterIntInterface: ValueChanged {}

impl Into<i32> for AudioParameterInt {

    /**
      | Returns the parameter's current value
      | as an integer.
      |
      */
    #[inline] fn into(self) -> i32 {
        todo!();
        /*
            return get();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterInt.cpp]
impl Drop for AudioParameterInt {
    fn drop(&mut self) {
        todo!();
        /*
            #if __cpp_lib_atomic_is_always_lock_free
         static_assert (std::atomic<float>::is_always_lock_free,
                        "AudioParameterInt requires a lock-free std::atomic<float>");
        #endif
        */
    }
}

impl AudioParameterInt {

    /**
      | Returns the parameter's current value
      | as an integer.
      |
      */
    pub fn get(&self) -> i32 {
        
        todo!();
        /*
            return roundToInt (value.load());
        */
    }

    /**
      | Returns the parameter's range.
      |
      */
    pub fn get_range(&self) -> Range<i32> {
        
        todo!();
        /*
            return { (int) getNormalisableRange().start, (int) getNormalisableRange().end };
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
      | Creates a AudioParameterInt with the
      | specified parameters.
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
      | @param minValue
      | 
      | The minimum parameter value
      | ----------
      | @param maxValue
      | 
      | The maximum parameter value
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
      | @param stringFromInt
      | 
      | An optional lambda function that converts
      | a int value to a string with a maximum
      | length. This may be used by hosts to display
      | the parameter's value.
      | ----------
      | @param intFromString
      | 
      | An optional lambda function that parses
      | a string and converts it into an int.
      | Some hosts use this to allow users to
      | type in parameter values.
      |
      */
    pub fn new(
        id_to_use:       &str,
        name_to_use:     &str,
        min_value:       i32,
        max_value:       i32,
        def:             i32,
        label_to_use:    Option<&str>,
        string_from_int: fn(_0: i32, _1: i32) -> String,
        int_from_string: fn(_0: &str) -> i32

    ) -> Self {

        let label_to_use = label_to_use.unwrap_or("");

        todo!();
        /*


            : RangedAudioParameter (idToUse, nameToUse, labelToUse),
         range ([minValue, maxValue]
                {
                    NormalisableRange<float> rangeWithInterval { (float) minValue, (float) maxValue,
                                                                 [] (float start, float end, float v) { return jlimit (start, end, v * (end - start) + start); },
                                                                 [] (float start, float end, float v) { return jlimit (0.0f, 1.0f, (v - start) / (end - start)); },
                                                                 [] (float start, float end, float v) { return (float) roundToInt (jlimit (start, end, v)); } };
                     rangeWithInterval.interval = 1.0f;
                     return rangeWithInterval;
                }()),
         value ((float) def),
         defaultValue (convertTo0to1 ((float) def)),
         stringFromIntFunction (stringFromInt),
         intFromStringFunction (intFromString)

        jassert (minValue < maxValue); // must have a non-zero range of values!

        if (stringFromIntFunction == nullptr)
            stringFromIntFunction = [] (int v, int) { return String (v); };

        if (intFromStringFunction == nullptr)
            intFromStringFunction = [] (const String& text) { return text.getIntValue(); };
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
            return defaultValue;
        */
    }
    
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            return ((int) getNormalisableRange().getRange().getLength()) + 1;
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            return convertTo0to1 ((float) intFromStringFunction (text));
        */
    }
    
    pub fn get_text(&self, 
        v:      f32,
        length: i32) -> String {
        
        todo!();
        /*
            return stringFromIntFunction ((int) convertFrom0to1 (v), length);
        */
    }
    
    pub fn value_changed(&mut self, _0: i32)  { }
    
    /**
      | Changes the parameter's current value
      | to a new integer.
      | 
      | The value passed in will be snapped to
      | the permitted range before being used.
      |
      */
    pub fn assign_from(&mut self, new_value: i32) -> &mut AudioParameterInt {
        
        todo!();
        /*
            if (get() != newValue)
            setValueNotifyingHost (convertTo0to1 ((float) newValue));

        return *this;
        */
    }
}
