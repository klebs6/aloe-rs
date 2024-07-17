/*!
  | Description : Vst Edit Controller
  | Implementation
  |
  */
crate::ix!();


//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vsteditcontroller.h]

/**
  | Default implementation for a Vst 3 edit
  | controller. \ingroup vstClasses
  | 
  | Can be used as base class for a specific
  | controller implementation
  |
  */
pub struct EditController {
    base:               ComponentBase,
    component_handler:  *mut dyn IComponentHandler,
    component_handler2: *mut dyn IComponentHandler2,
    parameters:         ParameterContainer,
}

lazy_static!{
    /*
    static KnobMode edit_controller_host_knob_mode;
    KnobMode EditController::hostKnobMode = edit_controller_kcircular_mode;
    */
}

impl EditController {

    pub fn new() -> Self {
    
        todo!();
        /*
        : component_handler(nullptr),
        : component_handler2(nullptr),

        
        */
    }
    
    #[PLUGIN_API]
    pub fn set_component_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_parameter_count(&mut self) -> i32 {
        
        todo!();
        /*
            return parameters.getParameterCount ();
        */
    }
    
    #[PLUGIN_API]
    pub fn get_parameter_info(&mut self, 
        param_index: i32,
        info:        &mut ParameterInfo) -> tresult {
        
        todo!();
        /*
            if (Parameter* parameter = parameters.getParameterByIndex (paramIndex))
        {
            info = parameter->getInfo ();
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_param_string_by_value(&mut self, 
        tag:              ParamID,
        value_normalized: ParamValue,
        string:           String128) -> tresult {
        
        todo!();
        /*
            if (Parameter* parameter = getParameterObject (tag))
        {
            parameter->toString (valueNormalized, string);
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_param_value_by_string(&mut self, 
        tag:              ParamID,
        string:           *mut TChar,
        value_normalized: &mut ParamValue) -> tresult {
        
        todo!();
        /*
            if (Parameter* parameter = getParameterObject (tag))
        {
            if (parameter->fromString (string, valueNormalized))
            {
                return kResultTrue;
            }
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn normalized_param_to_plain(&mut self, 
        tag:              ParamID,
        value_normalized: ParamValue) -> ParamValue {
        
        todo!();
        /*
            if (Parameter* parameter = getParameterObject (tag))
        {
            return parameter->toPlain (valueNormalized);
        }
        return valueNormalized;
        */
    }
    
    #[PLUGIN_API]
    pub fn plain_param_to_normalized(&mut self, 
        tag:         ParamID,
        plain_value: ParamValue) -> ParamValue {
        
        todo!();
        /*
            if (Parameter* parameter = getParameterObject (tag))
        {
            return parameter->toNormalized (plainValue);
        }
        return plainValue;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_param_normalized(&mut self, tag: ParamID) -> ParamValue {
        
        todo!();
        /*
            if (Parameter* parameter = getParameterObject (tag))
        {
            return parameter->getNormalized ();
        }
        return 0.;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_param_normalized(&mut self, 
        tag:   ParamID,
        value: ParamValue) -> tresult {
        
        todo!();
        /*
            if (Parameter* parameter = getParameterObject (tag))
        {
            parameter->setNormalized (value);
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_component_handler(&mut self, new_handler: *mut dyn IComponentHandler) -> tresult {
        
        todo!();
        /*
            if (componentHandler == newHandler)
        {
            return kResultTrue;
        }

        if (componentHandler)
        {
            componentHandler->release ();
        }

        componentHandler = newHandler;
        if (componentHandler)
        {
            componentHandler->addRef ();
        }

        // try to get the extended version
        if (componentHandler2)
        {
            componentHandler2->release ();
            componentHandler2 = nullptr;
        }

        if (newHandler)
        {
            newHandler->queryInterface (IComponentHandler2::iid, (void**)&componentHandler2);
        }
        return kResultTrue;
        */
    }
    
    pub fn begin_edit(&mut self, tag: ParamID) -> tresult {
        
        todo!();
        /*
            if (componentHandler)
        {
            return componentHandler->beginEdit (tag);
        }
        return kResultFalse;
        */
    }
    
    pub fn perform_edit(&mut self, 
        tag:              ParamID,
        value_normalized: ParamValue) -> tresult {
        
        todo!();
        /*
            if (componentHandler)
        {
            return componentHandler->performEdit (tag, valueNormalized);
        }
        return kResultFalse;
        */
    }
    
    pub fn end_edit(&mut self, tag: ParamID) -> tresult {
        
        todo!();
        /*
            if (componentHandler)
        {
            return componentHandler->endEdit (tag);
        }
        return kResultFalse;
        */
    }
    
    pub fn start_group_edit(&mut self) -> tresult {
        
        todo!();
        /*
            if (componentHandler2)
        {
            return componentHandler2->startGroupEdit ();
        }
        return kNotImplemented;
        */
    }
    
    pub fn finish_group_edit(&mut self) -> tresult {
        
        todo!();
        /*
            if (componentHandler2)
        {
            return componentHandler2->finishGroupEdit ();
        }
        return kNotImplemented;
        */
    }
    
    pub fn get_parameter_info_by_tag(&mut self, 
        tag:  ParamID,
        info: &mut ParameterInfo) -> tresult {
        
        todo!();
        /*
            if (Parameter* parameter = getParameterObject (tag))
        {
            info = parameter->getInfo ();
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    pub fn set_dirty(&mut self, state: TBool) -> tresult {
        
        todo!();
        /*
            if (componentHandler2)
        {
            return componentHandler2->setDirty (state);
        }
        return kNotImplemented;
        */
    }
    
    pub fn request_open_editor(&mut self, name: FIDString) -> tresult {
        
        todo!();
        /*
            if (componentHandler2)
        {
            return componentHandler2->requestOpenEditor (name);
        }
        return kNotImplemented;
        */
    }
}

impl IPluginBase for EditController {

    #[PLUGIN_API]
    fn initialize(&mut self, context: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            return ComponentBase::initialize (context);
        */
    }
    
    #[PLUGIN_API]
    fn terminate(&mut self) -> tresult {
        
        todo!();
        /*
            parameters.removeAll ();

        if (componentHandler)
        {
            componentHandler->release ();
            componentHandler = nullptr;
        }

        if (componentHandler2)
        {
            componentHandler2->release ();
            componentHandler2 = nullptr;
        }

        return ComponentBase::terminate ();
        */
    }
}

impl IEditController for EditController {

    #[PLUGIN_API]
    fn set_component_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn set_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn get_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn get_parameter_count(&mut self) -> i32 {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn get_parameter_info(&mut self, 
        param_index: i32,
        info:        &mut ParameterInfo) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn get_param_string_by_value(&mut self, 
        tag:              ParamID,
        value_normalized: ParamValue,
        string:           String128) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn get_param_value_by_string(&mut self, 
        tag:              ParamID,
        string:           *mut TChar,
        value_normalized: &mut ParamValue) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn normalized_param_to_plain(&mut self, 
        tag:              ParamID,
        value_normalized: ParamValue) -> ParamValue {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn plain_param_to_normalized(&mut self, 
        tag:         ParamID,
        plain_value: ParamValue) -> ParamValue {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn get_param_normalized(&mut self, tag: ParamID) -> ParamValue {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn set_param_normalized(&mut self, 
        tag:   ParamID,
        value: ParamValue) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn set_component_handler(&mut self, handler: *mut dyn IComponentHandler) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn create_view(&mut self, name: FIDString) -> *mut dyn IPlugView {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

impl FUnknown for EditController {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut c_void) -> i32 { 
        todo!() 
    }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }

}

impl IEditController2 for EditController {

    #[PLUGIN_API]
    fn set_knob_mode(&mut self, mode: KnobMode) -> tresult {
        
        todo!();
        /*
            hostKnobMode = mode; return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn open_help(&mut self, only_check: TBool) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn open_about_box(&mut self, only_check: TBool) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
}

impl ComponentBaseInterface for EditController {}

impl EditController {

    fn get_host_knob_mode() -> KnobMode {
        
        todo!();
        /*
            return hostKnobMode;
        */
    }
    
    fn get_component_handler(&self) -> *mut dyn IComponentHandler {
        
        todo!();
        /*
            return componentHandler;
        */
    }

    lazy_static!{
        /*
        OBJ_METHODS (EditController, ComponentBase)
            DEFINE_INTERFACES
                DEF_INTERFACE (IEditController)
                DEF_INTERFACE (IEditController2)
            END_DEFINE_INTERFACES (ComponentBase)
            REFCOUNT_METHODS (ComponentBase)
        */
    }
}
