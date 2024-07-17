crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterChoice.h]

/**
  | Provides a class of AudioProcessorParameter
  | that can be used to select an indexed,
  | named choice from a list.
  | 
  | @see AudioParameterFloat, AudioParameterInt,
  | AudioParameterBool
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioParameterChoice {

    base:                       RangedAudioParameter,

    /**
      | Provides access to the list of choices
      | that this parameter is working with.
      |
      */
    choices:                    Vec<String>,

    range:                      NormalisableRange<f32>,
    value:                      Atomic<f32>,
    default_value:              f32,
    string_from_index_function: fn(_0: i32, _1: i32) -> String,
    index_from_string_function: fn(_0: &String) -> i32,
}

impl Into<i32> for AudioParameterChoice {
    
    /**
      | Returns the current index of the selected
      | item.
      |
      */
    #[inline] fn into(self) -> i32 {
        todo!();
        /*
            return getIndex();
        */
    }
}

impl Into<String> for AudioParameterChoice {
    
    /**
      | Returns the name of the currently selected
      | item.
      |
      */
    #[inline] fn into(self) -> String {
        todo!();
        /*
            return getCurrentChoiceName();
        */
    }
}

pub trait ValueChanged {

    fn value_changed(&mut self, new_value: i32);
}

impl ValueChanged for AudioParameterChoice {

    /**
      | Override this method if you are interested
      | in receiving callbacks when the parameter
      | value changes.
      |
      */
    fn value_changed(&mut self, new_value: i32)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioParameterChoice.cpp]
impl Drop for AudioParameterChoice {
    fn drop(&mut self) {
        todo!();
        /*
            #if __cpp_lib_atomic_is_always_lock_free
         static_assert (std::atomic<float>::is_always_lock_free,
                        "AudioParameterChoice requires a lock-free std::atomic<float>");
        #endif
        */
    }
}

impl AudioParameterChoice {

    /**
      | Returns the current index of the selected
      | item.
      |
      */
    pub fn get_index(&self) -> i32 {
        
        todo!();
        /*
            return roundToInt (value.load());
        */
    }

    /**
      | Returns the name of the currently selected
      | item.
      |
      */
    pub fn get_current_choice_name(&self) -> String {
        
        todo!();
        /*
            return choices[getIndex()];
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
      | Creates a AudioParameterChoice with
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
      | @param choices
      | 
      | The set of choices to use
      | ----------
      | @param defaultItemIndex
      | 
      | The index of the default choice
      | ----------
      | @param parameterLabel
      | 
      | An optional label for the parameter's
      | value
      | ----------
      | @param stringFromIndex
      | 
      | An optional lambda function that converts
      | a choice index to a string with a maximum
      | length. This may be used by hosts to display
      | the parameter's value.
      | ----------
      | @param indexFromString
      | 
      | An optional lambda function that parses
      | a string and converts it into a choice
      | index. Some hosts use this to allow users
      | to type in parameter values.
      |
      */
    pub fn new(
        id_to_use:         &str,
        name_to_use:       &str,
        c:                 &Vec<String>,
        def:               i32,
        label_to_use:      Option<&str>,
        string_from_index: fn(_0: i32, _1: i32) -> String,
        index_from_string: fn(_0: &str) -> i32) -> Self {

        let label_to_use = label_to_use.unwrap_or("");
    
        todo!();
        /*


            : RangedAudioParameter (idToUse, nameToUse, labelToUse), choices (c),
         range ([this]
                {
                    NormalisableRange<float> rangeWithInterval { 0.0f, (float) choices.size() - 1.0f,
                                                                 [] (float, float end, float v) { return jlimit (0.0f, end, v * end); },
                                                                 [] (float, float end, float v) { return jlimit (0.0f, 1.0f, v / end); },
                                                                 [] (float start, float end, float v) { return (float) roundToInt (jlimit (start, end, v)); } };
                    rangeWithInterval.interval = 1.0f;
                    return rangeWithInterval;
                }()),
         value ((float) def),
         defaultValue (convertTo0to1 ((float) def)),
         stringFromIndexFunction (stringFromIndex),
         indexFromStringFunction (indexFromString)


        jassert (choices.size() > 1); // you must supply an actual set of items to choose from!

        if (stringFromIndexFunction == nullptr)
            stringFromIndexFunction = [this] (int index, int) { return choices [index]; };

        if (indexFromStringFunction == nullptr)
            indexFromStringFunction = [this] (const String& text) { return choices.indexOf (text); };
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
            value = convertFrom0to1 (newValue); valueChanged (getIndex());
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
            return choices.size();
        */
    }
    
    pub fn is_discrete(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_value_for_text(&self, text: &String) -> f32 {
        
        todo!();
        /*
            return convertTo0to1 ((float) indexFromStringFunction (text));
        */
    }
    
    pub fn get_text(&self, 
        v:      f32,
        length: i32) -> String {
        
        todo!();
        /*
            return stringFromIndexFunction ((int) convertFrom0to1 (v), length);
        */
    }
    
    /**
      | Changes the selected item to a new index.
      |
      */
    pub fn assign_from(&mut self, new_value: i32) -> &mut AudioParameterChoice {
        
        todo!();
        /*
            if (getIndex() != newValue)
            setValueNotifyingHost (convertTo0to1 ((float) newValue));

        return *this;
        */
    }
}
