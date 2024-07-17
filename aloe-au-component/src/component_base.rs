crate::ix!();

/**
  | classic MacErrors
  |
  */
pub const COMPONENT_BASE_NO_ERR: usize = 0;

/**
  | This global variable is so that new
  | instances know how they were instantiated:
  | via the Component Manager, or as
  | AudioComponents. It's ugly, but preferable
  | to altering the constructor of every class
  | in the hierarchy.
  |
  | It's safe because construction is protected
  | by ComponentInitLocker.
  */
pub enum ComponentBaseEInstanceType { 
    kComponentMgrInstance, 
    kAudioComponentInstance 
}

lazy_static!{
    /*
    static ComponentBaseEInstanceType sNewInstanceType;
    ComponentBase::ComponentBaseEInstanceType ComponentBase::sNewInstanceType;
    */
}

pub struct ComponentBase {
    component_instance: AudioComponentInstance,
    instance_type:      ComponentBaseEInstanceType,
}

impl AUPostConstructor for ComponentBase {

    fn post_constructor(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl AUPreDestructor for ComponentBase {

    fn pre_destructor(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

pub trait AUVersion {

    fn version(&mut self) -> OSStatus;
}

impl AUVersion for ComponentBase {

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    fn version(&mut self) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

impl ComponentBase {
    
    pub fn get_component_instance(&self) -> AudioComponentInstance {
        
        todo!();
        /*
            return mComponentInstance;
        */
    }
    
    pub fn is_plugin_object(&self) -> bool {
        
        todo!();
        /*
            return mInstanceType == kAudioComponentInstance;
        */
    }
    
    pub fn is_cmgr_object(&self) -> bool {
        
        todo!();
        /*
            return mInstanceType == kComponentMgrInstance;
        */
    }
    
    pub fn new(in_instance: AudioComponentInstance) -> Self {
    
        todo!();
        /*
        : component_instance(inInstance),
        : instance_type(sNewInstanceType),

            GetComponentDescription();
        */
    }
    
    pub fn ap_open(&mut self, 
        self_:         *mut c_void,
        comp_instance: AudioUnit) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;
        try {
            ComponentInitLocker lock;

            ComponentBase::sNewInstanceType = ComponentBase::kAudioComponentInstance;
            ComponentBase *cb = (ComponentBase *)(*ACPI->mConstruct)(&ACPI->mInstanceStorage, compInstance);
            cb->PostConstructor();  // allows base class to do additional initialization
            // once the derived class is fully constructed
            result = noErr;
        }
        COMPONENT_CATCH
        if (result)
            delete ACPI;
        return result;
        */
    }
    
    pub fn ap_close(&mut self, self_: *mut c_void) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;
        try {
            if (ACImp) {
                ACImp->PreDestructor();
                (*ACPI->mDestruct)(&ACPI->mInstanceStorage);
                free(self);
            }
        }
        COMPONENT_CATCH
        return result;
        */
    }

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn version(&mut self) -> OSStatus {
        
        todo!();
        /*
            return 0x00000001;
        */
    }
    
    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn component_entry_dispatch(&mut self, 
        p:    *mut ComponentParameters,
        this: *mut ComponentBase) -> OSStatus {
        
        todo!();
        /*
            if (This == NULL) return kAudio_ParamError;

        OSStatus result = noErr;

        switch (p->what) {
        case kComponentCloseSelect:
            This->PreDestructor();
            delete This;
            break;

        case kComponentVersionSelect:
            result = This->Version();
            break;

        case kComponentCanDoSelect:
            switch (GetSelectorForCanDo(p)) {
            case kComponentOpenSelect:
            case kComponentCloseSelect:
            case kComponentVersionSelect:
            case kComponentCanDoSelect:
                return 1;
            default:
                return 0;
            }

        default:
            result = badComponentSelector;
            break;
        }
        return result;
        */
    }
    
    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn get_selector_for_can_do(&mut self, params: *mut ComponentParameters) -> i16 {
        
        todo!();
        /*
            if (params->what != kComponentCanDoSelect) return 0;

        #if TARGET_CPU_X86
            SInt16 sel = params->params[0];
        #elif TARGET_CPU_X86_64
            SInt16 sel = params->params[1];
        #elif TARGET_CPU_PPC
            SInt16 sel = (params->params[0] >> 16);
        #else
            SInt16 sel = params->params[0];
        #endif

        return sel;
    /*
            printf ("flags:%d, paramSize: %d, what: %d\n\t", params->flags, params->paramSize, params->what);
            for (int i = 0; i < params->paramSize; ++i) {
                printf ("[%d]:%d(0x%x), ", i, params->params[i], params->params[i]);
            }
            printf("\n\tsel:%d\n", sel);
    */
        */
    }
    
    pub fn get_component_description(&self) -> AudioComponentDescription {
        
        todo!();
        /*
            AudioComponentDescription desc;
        OSStatus result = 1;

        if (IsPluginObject()) {
            ca_require_noerr(result = CB_GetComponentDescription (mComponentInstance, &desc), home);
        }
    #if !CA_USE_AUDIO_PLUGIN_ONLY
        else {
            ca_require_noerr(result = CMgr_GetComponentDescription (mComponentInstance, &desc), home);
        }
    #endif

    home:
        if (result)
            memset (&desc, 0, sizeof(AudioComponentDescription));

        return desc;
        */
    }
}
