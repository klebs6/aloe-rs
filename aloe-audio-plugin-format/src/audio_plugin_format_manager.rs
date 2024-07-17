crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format/aloe_AudioPluginFormatManager.h]

/**
  | This maintains a list of known AudioPluginFormats.
  | 
  | @see AudioPluginFormat
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioPluginFormatManager {
    formats: Vec<Box<AudioPluginFormat>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format/aloe_AudioPluginFormatManager.cpp]
impl AudioPluginFormatManager {

    /**
      | Adds the set of available standard formats,
      | e.g. Vst.
      |
      */
    pub fn add_default_formats(&mut self)  {
        
        todo!();
        /*
            #if ALOE_DEBUG
        // you should only call this method once!
        for (auto* format : formats)
        {
            ignoreUnused (format);

           #if ALOE_PLUGINHOST_Vst && (ALOE_MAC || ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD || ALOE_IOS)
            jassert (dynamic_cast<VstPluginFormat*> (format) == nullptr);
           #endif

           #if ALOE_PLUGINHOST_Vst3 && (ALOE_MAC || ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD)
            jassert (dynamic_cast<Vst3PluginFormat*> (format) == nullptr);
           #endif

           #if ALOE_PLUGINHOST_AU && (ALOE_MAC || ALOE_IOS)
            jassert (dynamic_cast<AudioUnitPluginFormat*> (format) == nullptr);
           #endif

           #if ALOE_PLUGINHOST_LADSPA && (ALOE_LINUX || ALOE_BSD)
            jassert (dynamic_cast<LADSPAPluginFormat*> (format) == nullptr);
           #endif
        }
       #endif

       #if ALOE_PLUGINHOST_AU && (ALOE_MAC || ALOE_IOS)
        formats.add (new AudioUnitPluginFormat());
       #endif

       #if ALOE_PLUGINHOST_Vst && (ALOE_MAC || ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD || ALOE_IOS)
        formats.add (new VstPluginFormat());
       #endif

       #if ALOE_PLUGINHOST_Vst3 && (ALOE_MAC || ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD)
        formats.add (new Vst3PluginFormat());
       #endif

       #if ALOE_PLUGINHOST_LADSPA && (ALOE_LINUX || ALOE_BSD)
        formats.add (new LADSPAPluginFormat());
       #endif
        */
    }
    
    /**
      | Returns the number of types of format
      | that are available.
      | 
      | Use getFormat() to get one of them.
      |
      */
    pub fn get_num_formats(&self) -> i32 {
        
        todo!();
        /*
            return formats.size();
        */
    }
    
    /**
      | Returns one of the available formats.
      | @see getNumFormats
      |
      */
    pub fn get_format(&self, index: i32) -> *mut AudioPluginFormat {
        
        todo!();
        /*
            return formats[index];
        */
    }
    
    /**
      | Returns a list of all the registered
      | formats.
      |
      */
    pub fn get_formats(&self) -> Vec<*mut AudioPluginFormat> {
        
        todo!();
        /*
            Vec<AudioPluginFormat*> a;
        a.addArray (formats);
        return a;
        */
    }
    
    /**
      | Adds a format to the list.
      | 
      | The object passed in will be owned and
      | deleted by the manager.
      |
      */
    pub fn add_format(&mut self, format: *mut AudioPluginFormat)  {
        
        todo!();
        /*
            formats.add (format);
        */
    }
    
    /**
      | Tries to load the type for this description,
      | by trying all the formats that this manager
      | knows about.
      | 
      | If it can't load the plugin, it returns
      | nullptr and leaves a message in the errorMessage
      | string.
      | 
      | If you intend to instantiate a AudioUnit
      | v3 plug-in then you must either use the
      | non-blocking asynchronous version
      | below - or call this method from a thread
      | other than the message thread and without
      | blocking the message thread.
      |
      */
    pub fn create_plugin_instance(&self, 
        description:   &PluginDescription,
        rate:          f64,
        block_size:    i32,
        error_message: &mut String) -> Box<AudioPluginInstance> {
        
        todo!();
        /*
            if (auto* format = findFormatForDescription (description, errorMessage))
            return format->createInstanceFromDescription (description, rate, blockSize, errorMessage);

        return {};
        */
    }
    
    /**
      | Tries to asynchronously load the type
      | for this description, by trying all
      | the formats that this manager knows
      | about.
      | 
      | The caller must supply a callback object
      | which will be called when the instantiation
      | has completed.
      | 
      | If it can't load the plugin then the callback
      | function will be called passing a nullptr
      | as the instance argument along with
      | an error message.
      | 
      | The callback function will be called
      | on the message thread so the caller must
      | not block the message thread.
      | 
      | The callback object will be deleted
      | automatically after it has been invoked.
      | 
      | The caller is responsible for deleting
      | the instance that is passed to the callback
      | function.
      | 
      | If you intend to instantiate a AudioUnit
      | v3 plug-in then you must use this non-blocking
      | asynchronous version - or call the synchronous
      | method from an auxiliary thread.
      |
      */
    pub fn create_plugin_instance_async(
        &mut self, 
        description:         &PluginDescription,
        initial_sample_rate: f64,
        initial_buffer_size: i32,
        callback:            PluginCreationCallback

    ) {
        
        todo!();
        /*
            String error;

        if (auto* format = findFormatForDescription (description, error))
            return format->createPluginInstanceAsync (description, initialSampleRate, initialBufferSize, std::move (callback));

        struct DeliverError  : public CallbackMessage
        {
            DeliverError (AudioPluginFormat::PluginCreationCallback c, const String& e)
                : call (std::move (c)), error (e)
            {
                post();
            }

            void messageCallback() override          { call (nullptr, error); }

            AudioPluginFormat::PluginCreationCallback call;
            String error;
        };

        new DeliverError (std::move (callback), error);
        */
    }
    
    pub fn find_format_for_description(&self, 
        description:   &PluginDescription,
        error_message: &mut String) -> *mut AudioPluginFormat {
        
        todo!();
        /*
            errorMessage = {};

        for (auto* format : formats)
            if (format->getName() == description.pluginFormatName
                  && format->fileMightContainThisPluginType (description.fileOrIdentifier))
                return format;

        errorMessage = NEEDS_TRANS ("No compatible plug-in format exists for this plug-in");

        return {};
        */
    }
    
    /**
      | Checks that the file or component for
      | this plugin actually still exists.
      | (This won't try to load the plugin)
      |
      */
    pub fn does_plugin_still_exist(&self, description: &PluginDescription) -> bool {
        
        todo!();
        /*
            for (auto* format : formats)
            if (format->getName() == description.pluginFormatName)
                return format->doesPluginStillExist (description);

        return false;
        */
    }
}
