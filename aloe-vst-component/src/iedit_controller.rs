crate::ix!();

/**
  | Edit controller component interface:
  | Vst::IEditController \ingroup vstIPlug
  | vst300
  | 
  | - [plug imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | The controller part of an effect or instrument
  | with parameter handling (export, definition,
  | conversion...). \see \ref IComponent::getControllerClassId,
  | \ref IMidiMapping
  |
  */
pub trait IEditController: IPluginBase {

    /**
      | Receives the component state.
      |
      */
    #[PLUGIN_API]
    fn set_component_state(&mut self, state: *mut dyn IBStream) -> tresult;

    /**
      | Sets the controller state.
      |
      */
    #[PLUGIN_API]
    fn set_state(&mut self, state: *mut dyn IBStream) -> tresult;

    /**
      | Gets the controller state.
      |
      */
    #[PLUGIN_API]
    fn get_state(&mut self, state: *mut dyn IBStream) -> tresult;


    /* ----- parameters ------------------------- */

    /**
      | Returns the number of parameters exported.
      |
      */
    #[PLUGIN_API]
    fn get_parameter_count(&mut self) -> i32;

    /**
      | Gets for a given index the parameter
      | information.
      |
      */
    #[PLUGIN_API]
    fn get_parameter_info(&mut self, 
            param_index: i32,
            info:        &mut ParameterInfo) -> tresult;

    /**
      | Gets for a given paramID and normalized
      | value its associated string representation.
      |
      */
    #[PLUGIN_API]
    fn get_param_string_by_value(&mut self, 
            id:               ParamID,
            value_normalized: ParamValue,
            string:           String128) -> tresult;

    /**
      | Gets for a given paramID and string its
      | normalized value.
      |
      */
    #[PLUGIN_API]
    fn get_param_value_by_string(&mut self, 
            id:               ParamID,
            string:           *mut TChar,
            value_normalized: &mut ParamValue) -> tresult;

    /**
      | Returns for a given paramID and a normalized
      | value its plain representation (for
      | example -6 for -6dB - see \ref vst3AutomationIntro).
      |
      */
    #[PLUGIN_API]
    fn normalized_param_to_plain(&mut self, 
            id:               ParamID,
            value_normalized: ParamValue) -> ParamValue;

    /**
      | Returns for a given paramID and a plain
      | value its normalized value. (see \ref
      | vst3AutomationIntro)
      |
      */
    #[PLUGIN_API]
    fn plain_param_to_normalized(&mut self, 
            id:          ParamID,
            plain_value: ParamValue) -> ParamValue;

    /**
      | Returns the normalized value of the
      | parameter associated to the paramID.
      |
      */
    #[PLUGIN_API]
    fn get_param_normalized(&mut self, id: ParamID) -> ParamValue;

    /**
      | Sets the normalized value to the parameter
      | associated to the paramID. The controller
      | must never pass this value-change back
      | to the host via the IComponentHandler.
      | It should update the according
      | 
      | GUI element(s) only!
      |
      */
    #[PLUGIN_API]
    fn set_param_normalized(&mut self, 
            id:    ParamID,
            value: ParamValue) -> tresult;


    /* ----- handler ---------------------------- */

    /**
      | Gets from host a handler which allows
      | the Plugin-in to communicate with the
      | host.
      | 
      | -----------
      | @note
      | 
      | This is mandatory if the host is using
      | the IEditController!
      |
      */
    #[PLUGIN_API]
    fn set_component_handler(&mut self, handler: *mut dyn IComponentHandler) -> tresult;

    /* ----- view ------------------------------- */

    /**
      | Creates the editor view of the plug-in,
      | currently only "editor" is supported,
      | see \ref ViewType.
      | 
      | The life time of the editor view will
      | never exceed the life time of this controller
      | instance.
      |
      */
    #[PLUGIN_API]
    fn create_view(&mut self, name: FIDString) -> *mut dyn IPlugView;
}

lazy_static!{
    /*
    static const FUID iedit_controller_iid;
    */
}

declare_class_iid!{
    IEditController, 
    0xDCD7BBE3, 
    0x7742448D, 
    0xA874AACC, 
    0x979C759E
}
