crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format/aloe_AudioPluginFormat.h]

/**
  | The base class for a type of plugin format,
  | such as VST, AudioUnit, LADSPA, etc.
  | 
  | @see AudioPluginFormatManager
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioPluginFormat {
    base: MessageListener,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format/aloe_AudioPluginFormat.cpp]
impl AudioPluginFormat {

    /**
      | Tries to recreate a type from a previously
      | generated PluginDescription. @see
      | AudioPluginFormatManager::createInstance
      |
      */
    pub fn create_instance_from_description(
        &mut self, 
        desc:                &PluginDescription,
        initial_sample_rate: f64,
        initial_buffer_size: i32

    ) -> Box<AudioPluginInstance> {
        
        todo!();
        /*
            String errorMessage;
        return createInstanceFromDescription (desc, initialSampleRate, initialBufferSize, errorMessage);
        */
    }
    
    /**
      | Same as above but with the possibility
      | of returning an error message. @see
      | AudioPluginFormatManager::createInstance
      |
      */
    pub fn create_instance_from_description_with_error(
        &mut self, 
        desc:                &PluginDescription,
        initial_sample_rate: f64,
        initial_buffer_size: i32,
        error_message:       &mut String

    ) -> Box<AudioPluginInstance> {
        
        todo!();
        /*
            if (MessageManager::getInstance()->isThisTheMessageThread()
              && requiresUnblockedMessageThreadDuringCreation (desc))
        {
            errorMessage = NEEDS_TRANS ("This plug-in cannot be instantiated synchronously");
            return {};
        }

        WaitableEvent finishedSignal;
        std::unique_ptr<AudioPluginInstance> instance;

        auto callback = [&] (std::unique_ptr<AudioPluginInstance> p, const String& error)
        {
           errorMessage = error;
           instance = std::move (p);
           finishedSignal.signal();
        };

        if (! MessageManager::getInstance()->isThisTheMessageThread())
            createPluginInstanceAsync (desc, initialSampleRate, initialBufferSize, std::move (callback));
        else
            createPluginInstance (desc, initialSampleRate, initialBufferSize, std::move (callback));

        finishedSignal.wait();
        return instance;
        */
    }
    
    /**
      | Tries to recreate a type from a previously
      | generated PluginDescription.
      | 
      | When the plugin has been created, it
      | will be passed to the caller via an asynchronous
      | call to the PluginCreationCallback
      | lambda that was provided. @see AudioPluginFormatManager::createPluginInstanceAsync
      |
      */
    pub fn create_plugin_instance_async(&mut self, 
        description:         &PluginDescription,
        initial_sample_rate: f64,
        initial_buffer_size: i32,
        callback:            PluginCreationCallback)  {
        
        todo!();
        /*
            jassert (callback != nullptr);
        postMessage (new AudioPluginFormatAsyncCreateMessage (description, initialSampleRate, initialBufferSize, std::move (callback)));
        */
    }
    
    pub fn handle_message(&mut self, message: &Message)  {
        
        todo!();
        /*
            if (auto m = dynamic_cast<const AudioPluginFormatAsyncCreateMessage*> (&message))
            createPluginInstance (m->desc, m->sampleRate, m->bufferSize, std::move (m->callbackToUse));
        */
    }
}
