crate::ix!();

pub struct AloeVst3EditControllerEditorHostContext<'a> {
    processor:         &'a mut AloeAudioProcessor,
    component_handler: *mut VstIComponentHandler, // default = nullptr
    view:              *mut dyn IPlugView, // default = nullptr
}

impl<'a> AudioProcessorEditorHostContext for AloeVst3EditControllerEditorHostContext<'a> {

    fn get_context_menu_for_parameter_index(&self, _: *const (dyn AudioProcessorParameterInterface + 'static)) -> Box<(dyn aloe_audio_interface::HostProvidedContextMenu + 'static)> { todo!() }
}

impl<'a> AloeVst3EditControllerEditorHostContext<'a> {

    pub fn new(
        processor_in: &mut AloeAudioProcessor,
        handler:      *mut VstIComponentHandler,
        view_in:      *mut dyn IPlugView) -> Self {
    
        todo!();
        /*
        : processor(processorIn),
        : component_handler(handler),
        : view(viewIn),

        
        */
    }
    
    pub fn get_context_menu_for_parameter_index(&self, parameter: *const AudioProcessorParameter) 
        -> Box<dyn HostProvidedContextMenu> 
    {
        
        todo!();
        /*
            if (componentHandler == nullptr || view == nullptr)
                    return {};

                FUnknownPtr<VstIComponentHandler3> handler (componentHandler);

                if (handler == nullptr)
                    return {};

                const auto idToUse = parameter != nullptr ? processor.getVSTParamIDForIndex (parameter->getParameterIndex()) : 0;
                const auto menu = VSTComSmartPtr<VstIContextMenu> (handler->createContextMenu (view, &idToUse));
                return std::make_unique<AloeVst3EditControllerEditorContextMenu> (menu);
        */
    }
}

