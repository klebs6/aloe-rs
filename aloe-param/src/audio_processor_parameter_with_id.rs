crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioProcessorParameterWithID.h]

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
#[no_copy]
#[leak_detector]
pub struct AudioProcessorParameterWithID {

    base:     AudioProcessorParameter,

    /**
      | Provides access to the parameter's
      | ID string.
      |
      */
    paramid:  String,

    /**
      | Provides access to the parameter's
      | name.
      |
      */
    name:     String,

    /**
      | Provides access to the parameter's
      | label.
      |
      */
    label:    String,

    /**
      | Provides access to the parameter's
      | category.
      |
      */
    category: AudioProcessorParameterCategory,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_AudioProcessorParameterWithID.cpp]
impl AudioProcessorParameterWithID {

    /**
      | The creation of this object requires
      | providing a name and ID which will be
      | constant for its lifetime.
      |
      */
    pub fn new(
        id_to_use:       &str,
        name_to_use:     &str,
        label_to_use:    &str,
        category_to_use: Option<AudioProcessorParameterCategory>

    ) -> Self {

        let category_to_use =
                 category_to_use.unwrap_or(AudioProcessorParameterCategory::genericParameter);
    
        todo!();
        /*


            : paramID (idToUse), name (nameToUse), label (labelToUse), category (categoryToUse)
        */
    }
    
    pub fn get_name(&self, maximum_string_length: i32) -> String {
        
        todo!();
        /*
            return name.substring (0, maximumStringLength);
        */
    }
    
    pub fn get_label(&self) -> String {
        
        todo!();
        /*
            return label;
        */
    }
    
    pub fn get_category(&self) -> AudioProcessorParameterCategory {
        
        todo!();
        /*
            return category;
        */
    }
}
