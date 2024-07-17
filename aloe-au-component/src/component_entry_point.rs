crate::ix!();

pub struct MissingType {}
pub type ComponentParameters = MissingType;

/**
  | @discussion This is only used for a component
  | manager version
  |
  */
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub struct ComponentEntryPoint<Class> {
    _p0: PhantomData<Class>,
}

#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
impl<Class> ComponentEntryPoint<Class> {
    
    pub fn dispatch(
        params: *mut ComponentParameters,
        obj:    *mut Class) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;

            try {
                if (params->what == kComponentOpenSelect) {
                    // solve a host of initialization thread safety issues.
                    ComponentInitLocker lock;

                    ComponentBase::sNewInstanceType = ComponentBase::kComponentMgrInstance;
                    ComponentInstance ci = (ComponentInstance)(params->params[0]);
                    Class *This = new Class((AudioComponentInstance)ci);
                    This->PostConstructor();    // allows base class to do additional initialization
                                                // once the derived class is fully constructed

                    CMgr_SetComponentInstanceStorage(ci, (Handle)This);
                } else
                    result = Class::ComponentEntryDispatch(params, obj);
            }
            COMPONENT_CATCH

            return result;
        */
    }
    
    pub fn register<'a>(
        comp_type:    OSType,
        sub_type:     OSType,
        manufacturer: OSType) -> Component<'a> {
        
        todo!();
        /*
            ComponentDescription    description = {compType, subType, manufacturer, 0, 0};
            Component   component = RegisterComponent(&description, (ComponentRoutineUPP) Dispatch, registerComponentGlobal, NULL, NULL, NULL);
            if (component != NULL) {
                SetDefaultComponent(component, defaultComponentAnyFlagsAnyManufacturerAnySubType);
            }
            return component;
        */
    }
}

/**
  | @note
  | 
  | Component Mgr is deprecated in ML. this
  | macro should not be used with new audio
  | components it is only for backwards
  | compatibility with Lion and SL. this
  | macro registers both a plugin and a component
  | mgr version.
  |
  */
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
macro_rules! audiocomponent_entry {
    ($FactoryType:ident, 
     $Class:ident) => {
        /*
        
            extern "C" OSStatus Class##Entry(ComponentParameters *params, Class *obj); 
            extern "C" OSStatus Class##Entry(ComponentParameters *params, Class *obj) { 
                return ComponentEntryPoint<Class>::Dispatch(params, obj); 
            } 
            extern "C" void * Class##Factory(const AudioComponentDescription *inDesc); 
            extern "C" void * Class##Factory(const AudioComponentDescription *inDesc) { 
                return FactoryType<Class>::Factory(inDesc); 
            }
        */
    }
}

/**
  | the only component we still support are the
  | carbon based view components
  |
  | you should be using this macro now to
  | exclusively register those types
  */
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
macro_rules! view_component_entry {
    ($Class:ident) => {
        /*
        
            extern "C" OSStatus Class##Entry(ComponentParameters *params, Class *obj); 
            extern "C" OSStatus Class##Entry(ComponentParameters *params, Class *obj) { 
                return ComponentEntryPoint<Class>::Dispatch(params, obj); 
            }
        */
    }
}

/**
  | this macro is used to generate the Entry Point
  | for a given Audio Plugin
  |
  | you should be using this macro now with audio
  | components
  */
#[cfg(CA_USE_AUDIO_PLUGIN_ONLY)]
macro_rules! audiocomponent_entry {
    ($FactoryType:ident, $Class:ident) => {
        /*
        
            extern "C" void * Class##Factory(const AudioComponentDescription *inDesc); 
            extern "C" void * Class##Factory(const AudioComponentDescription *inDesc) { 
                return FactoryType<Class>::Factory(inDesc); 
            }
        */
    }
}
