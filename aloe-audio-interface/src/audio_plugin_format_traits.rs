crate::ix!();

pub trait AudioPluginFormatInterface
: CanScanForPlugins 
+ CreatePluginInstance
+ DoesPluginStillExist
+ FileMightContainThisPluginType 
+ FindAllTypesForFile
+ GetDefaultLocationsToSearch 
+ GetNameOfPluginFromIdentifier 
+ IsTrivialToScan 
+ PluginNeedsRescanning
+ RequiresUnblockedMessageThreadDuringCreation 
+ SearchPathsForPlugins

/*
  | Returns the format name.
  | 
  | E.g. "VST", "AudioUnit", etc.
  |
  */
+ GetName 
{ }


/**
  | A callback lambda that is passed to createPluginInstanceAsync()
  |
  */
pub type PluginCreationCallback = fn(_0: Box<dyn AudioPluginInstanceInterface>, _1: &String) -> ();

pub trait AudioPluginInstanceInterface: FillInPluginDescription { }

pub trait FileMightContainThisPluginType {

    /**
      | Should do a quick check to see if this
      | file or directory might be a plugin of
      | this format.
      | 
      | This is for searching for potential
      | files, so it shouldn't actually try
      | to load the plugin or do anything time-consuming.
      |
      */
    fn file_might_contain_this_plugin_type(&mut self, file_or_identifier: &String) -> bool;
}

pub trait GetNameOfPluginFromIdentifier {

    /**
      | Returns a readable version of the name
      | of the plugin that this identifier refers
      | to.
      |
      */
    fn get_name_of_plugin_from_identifier(&mut self, file_or_identifier: &String) -> String;
}

pub trait CreatePluginInstance {

    /**
      | Implementors must override this function.
      | This is guaranteed to be called on the
      | message thread. You may call the callback
      | on any thread.
      |
      */
    fn create_plugin_instance(&mut self, 
        _0:                  &PluginDescription,
        initial_sample_rate: f64,
        initial_buffer_size: i32,
        _3:                  PluginCreationCallback);
}

pub trait RequiresUnblockedMessageThreadDuringCreation {

    /**
      | Returns true if instantiation of this
      | plugin type must be done from a non-message
      | thread.
      |
      */
    fn requires_unblocked_message_thread_during_creation(&self, _0: &PluginDescription) -> bool;
}

