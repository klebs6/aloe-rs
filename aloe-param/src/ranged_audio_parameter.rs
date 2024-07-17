crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_RangedAudioParameter.h]

/**
  | This abstract base class is used by some
  | AudioProcessorParameter helper classes.
  | 
  | @see AudioParameterFloat, AudioParameterInt,
  | AudioParameterBool, AudioParameterChoice
  | 
  | @tags{Audio}
  |
  */
pub struct RangedAudioParameter {
    base: AudioProcessorParameterWithID,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_RangedAudioParameter.cpp]
impl RangedAudioParameter {

    /**
      | The creation of this object requires
      | providing a name and ID which will be
      | constant for its lifetime.
      |
      */
    pub fn new(
        parameterid:        &str,
        parameter_name:     &str,
        parameter_label:    &str,
        parameter_category: Option<AudioProcessorParameterCategory>

    ) -> Self {

        let parameter_category =
                 parameter_category.unwrap_or(AudioProcessorParameterCategory::genericParameter);
    
        todo!();
        /*
        : audio_processor_parameter_withid(parameterID, parameterName, parameterLabel, parameterCategory),

        
        */
    }
    
    /**
      | Returns the number of steps for this
      | parameter based on the normalisable
      | range's interval.
      | 
      | If you are using lambda functions to
      | define the normalisable range's snapping
      | behaviour then you should override
      | this function so that it returns the
      | number of snapping points.
      |
      */
    pub fn get_num_steps(&self) -> i32 {
        
        todo!();
        /*
            const auto& range = getNormalisableRange();

        if (range.interval > 0)
            return (static_cast<int> ((range.end - range.start) / range.interval) + 1);

        return AudioProcessor::getDefaultNumParameterSteps();
        */
    }
    
    /**
      | Normalises and snaps a value based on
      | the normalisable range.
      |
      */
    pub fn convert_to0to1(&self, v: f32) -> f32 {
        
        todo!();
        /*
            const auto& range = getNormalisableRange();
        return range.convertTo0to1 (range.snapToLegalValue (v));
        */
    }
    
    /**
      | Denormalises and snaps a value based
      | on the normalisable range.
      |
      */
    pub fn convert_from0to1(&self, v: f32) -> f32 {
        
        todo!();
        /*
            const auto& range = getNormalisableRange();
        return range.snapToLegalValue (range.convertFrom0to1 (jlimit (0.0f, 1.0f, v)));
        */
    }
}
