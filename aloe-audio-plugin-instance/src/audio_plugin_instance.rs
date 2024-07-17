crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioPluginInstance.h]
lazy_static!{
    /*
    static bool audio_plugin_instance_deprecation_assertion_triggered = false;
    bool AudioPluginInstance::deprecationAssertiontriggered = false;
    */
}

/**
  | Base class for an active instance of
  | a plugin.
  | 
  | This derives from the AudioProcessor
  | class, and adds some extra functionality
  | that helps when wrapping dynamically
  | loaded plugins.
  | 
  | This class is not needed when writing
  | plugins, and you should never need to
  | derive your own sub-classes from it.
  | The plugin hosting classes use it internally
  | and will return AudioPluginInstance
  | objects which wrap external plugins.
  | 
  | @see AudioProcessor, AudioPluginFormat
  | 
  | @tags{Audio}
  |
  */
#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct AudioPluginInstance<'a> {

    base: AudioProcessor<'a>,
}

impl<'a> Drop for AudioPluginInstance<'a> {
    
    /**
      | Destructor.
      | 
      | Make sure that you delete any UI components
      | that belong to this plugin before deleting
      | the plugin.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
        
        */
    }
}

pub trait GetExtensions {

    fn get_extensions(&self, _0: &mut dyn ExtensionsVisitorInterface);
}

impl<'a> GetExtensions for AudioPluginInstance<'a> {

    /**
      | Allows retrieval of information related
      | to the inner workings of a particular
      | plugin format, such as the AEffect*
      | of a Vst, or the handle of an AudioUnit.
      | 
      | To use this, create a new class derived
      | from ExtensionsVisitor, and override
      | each of the visit member functions.
      | If this AudioPluginInstance wraps
      | a Vst3 plugin the visitVst3() member
      | will be called, while if the AudioPluginInstance
      | wraps an unknown format the visitUnknown()
      | member will be called. The argument
      | of the visit function can be queried
      | to extract information related to the
      | AudioPluginInstance's implementation.
      |
      */
    fn get_extensions(&self, _0: &mut dyn ExtensionsVisitorInterface)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioPluginInstance.cpp]
impl<'a> AudioPluginInstance<'a> {

    pub fn new_with_io_layouts(io_layouts: &AudioProcessorBusesProperties) -> Self {
    
        todo!();
        /*
        : audio_processor(ioLayouts),

        
        */
    }
    
    pub fn new<const NUM_LAYOUTS: usize>(channel_layout_list: [[i16; NUM_LAYOUTS]; 2]) -> Self {
    
        todo!();
        /*
        : audio_processor(channelLayoutList),
        */
    }
    
    /**
      | Returns a PluginDescription for this
      | plugin.
      | 
      | This is just a convenience method to
      | avoid calling fillInPluginDescription.
      |
      */
    pub fn get_plugin_description(&self) -> PluginDescription {
        
        todo!();
        /*
            PluginDescription desc;
        fillInPluginDescription (desc);
        return desc;
        */
    }
    
    pub fn get_platform_specific_data(&mut self)  {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    pub fn get_extensions(&self, visitor: &mut dyn ExtensionsVisitorInterface)  {
        
        todo!();
        /*
            visitor.visitUnknown ({});
        */
    }
    
    pub fn get_parameterid(&mut self, parameter_index: i32) -> String {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        // Currently there is no corresponding method available in the
        // AudioProcessorParameter class, and the previous behaviour of Aloe's
        // plug-in hosting code simply returns a string version of the index; to
        // maintain backwards compatibility you should perform the operation below
        // this comment. However the caveat is that for plug-ins which change their
        // number of parameters dynamically at runtime you cannot rely upon the
        // returned parameter ID mapping to the correct parameter. A comprehensive
        // solution to this problem requires some additional work in Aloe's hosting
        // code.
        return String (parameterIndex);
        */
    }
    
    pub fn get_parameter(&mut self, parameter_index: i32) -> f32 {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getValue();

        return 0.0f;
        */
    }
    
    pub fn set_parameter(&mut self, 
        parameter_index: i32,
        new_value:       f32)  {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            param->setValue (newValue);
        */
    }
    
    pub fn get_parameter_name(&mut self, parameter_index: i32) -> String {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getName (1024);

        return {};
        */
    }
    
    pub fn get_parameter_name_with_maxlen(
        &mut self, 
        parameter_index:       i32,
        maximum_string_length: i32

    ) -> String {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getName (maximumStringLength);

        return {};
        */
    }
    
    pub fn get_parameter_text(&mut self, parameter_index: i32) -> String {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getCurrentValueAsText();

        return {};
        */
    }
    
    pub fn get_parameter_text_with_maxlen(
        &mut self, 
        parameter_index:       i32,
        maximum_string_length: i32

    ) -> String {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getCurrentValueAsText().substring (0, maximumStringLength);

        return {};
        */
    }
    
    pub fn get_parameter_default_value(&mut self, parameter_index: i32) -> f32 {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getDefaultValue();

        return 0.0f;
        */
    }
    
    pub fn get_parameter_num_steps(&mut self, parameter_index: i32) -> i32 {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getNumSteps();

        return AudioProcessor::getDefaultNumParameterSteps();
        */
    }
    
    pub fn is_parameter_discrete(&self, parameter_index: i32) -> bool {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->isDiscrete();

        return false;
        */
    }
    
    pub fn is_parameter_automatable(&self, parameter_index: i32) -> bool {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->isAutomatable();

        return true;
        */
    }
    
    pub fn get_parameter_label(&self, parameter_index: i32) -> String {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getLabel();

        return {};
        */
    }
    
    pub fn is_parameter_orientation_inverted(&self, parameter_index: i32) -> bool {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->isOrientationInverted();

        return false;
        */
    }
    
    pub fn is_meta_parameter(&self, parameter_index: i32) -> bool {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->isMetaParameter();

        return false;
        */
    }
    
    pub fn get_parameter_category(&self, parameter_index: i32) -> AudioProcessorParameterCategory {
        
        todo!();
        /*
            assertOnceOnDeprecatedMethodUse();

        if (auto* param = getParameters()[parameterIndex])
            return param->getCategory();

        return AudioProcessorParameter::genericParameter;
        */
    }
    
    pub fn assert_once_on_deprecated_method_use(&self)  {
        
        todo!();
        /*
            if (! deprecationAssertiontriggered)
        {
            // If you hit this assertion then you are using at least one of the
            // methods marked as deprecated in this class. For now you can simply
            // continue past this point and subsequent uses of deprecated methods
            // will not trigger additional assertions. However, we will shortly be
            // removing these methods so you are strongly advised to look at the
            // implementation of the corresponding method in this class and use
            // that approach instead.
            jassertfalse;
        }

        deprecationAssertiontriggered = true;
        */
    }
}
