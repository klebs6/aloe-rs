crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorEditorHostContext.h]

/**
  | This wraps a context menu for a specific
  | parameter, as provided by the host.
  | 
  | You can choose to create a standard PopupMenu
  | to display the host-provided options.
  | Alternatively, you can ask the host
  | to display a native menu at a specific
  | location.
  | 
  | @tags{Audio}
  |
  */
pub trait HostProvidedContextMenu
{

    /**
      | Get a PopupMenu holding entries specified
      | by the host.
      | 
      | Most hosts will populate this menu with
      | options that relate to the parameter,
      | such as displaying its automation lane.
      | You are free to modify this menu before
      | displaying it, if you wish to add additional
      | options.
      |
      */
    fn get_equivalent_popup_menu(&self) -> PopupMenu;

    /**
      | Asks the host to display its native menu
      | at a particular location.
      |
      */
    fn show_native_menu(&self, pos: Point<i32>);
}

/**
  | Calling AudioProcessorEditor::getHostContext()
  | may return a pointer to an instance of
  | this class.
  | 
  | At the moment, this can be used to retrieve
  | context menus for parameters in compatible
  | Vst3 hosts. Additional extensions
  | may be added here in the future.
  | 
  | @tags{Audio}
  |
  */
pub trait AudioProcessorEditorHostContext
{

    /**
      | Returns an object which can be used to
      | display a context menu for the parameter
      | with the given index.
      |
      */
    fn get_context_menu_for_parameter_index(
        &self, 
        _0: *const dyn AudioProcessorParameterInterface
    ) -> Box<dyn HostProvidedContextMenu>;
}
