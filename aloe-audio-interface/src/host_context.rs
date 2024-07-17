crate::ix!();

pub trait GetHostContext {

    /**
      | Gets a context object, if one is available.
      | 
      | Returns nullptr if the host does not
      | provide any information that the editor
      | can query.
      | 
      | The returned pointer is non-owning,
      | so do not attempt to free it.
      |
      */
    fn get_host_context(&self) -> *mut dyn AudioProcessorEditorHostContext {

        todo!();
        /*
            return hostContext;
        */
    }
}

pub trait SetHostContext {

    /**
      | Sets a context object that can be queried
      | to find information that the host makes
      | available to the plugin.
      | 
      | You will only need to call this function
      | if you are implementing a plugin host.
      |
      */
    fn set_host_context(&mut self, context: *mut dyn AudioProcessorEditorHostContext) {

        todo!();
        /*
            hostContext = context;
        */
    }
}
