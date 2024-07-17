crate::ix!();

pub trait AUPostConstructor {

    fn post_constructor(&mut self);
}

pub trait AUPreConstructor {

    fn pre_constructor(&mut self);
}

//---------------------
pub trait AUPostDestructor {

    fn post_destructor(&mut self);
}

pub trait AUPreDestructor {

    fn pre_destructor(&mut self);
}

//---------------------
pub trait AUCreateExtendedElements {

    fn create_extended_elements(&mut self);
}

//------------------------
pub struct APFactory<APMethodLookup,Implementor> {
    _p0: PhantomData<APMethodLookup>,
    _p1: PhantomData<Implementor>,
}

impl<APMethodLookup,Implementor> APFactory<APMethodLookup,Implementor> {

    pub fn construct(
        memory:        *mut c_void,
        comp_instance: AudioComponentInstance)  {
        
        todo!();
        /*
            return new(memory) Implementor(compInstance);
        */
    }
    
    pub fn destruct(memory: *mut c_void)  {
        
        todo!();
        /*
            ((Implementor *)memory)->~Implementor();
        */
    }

    /**
      | This is the
      | AudioComponentFactoryFunction. It returns
      | an AudioComponentPlugInInstance.
      |
      | The actual implementation object is not
      | created until Open().
      */
    pub fn factory(in_desc: *const AudioComponentDescription) -> *mut AudioComponentPlugInInterface {
        
        todo!();
        /*
            AudioComponentPlugInInstance *acpi =
                    (AudioComponentPlugInInstance *)malloc( offsetof(AudioComponentPlugInInstance, mInstanceStorage) + sizeof(Implementor) );
            acpi->mPlugInInterface.Open = ComponentBase::AP_Open;
            acpi->mPlugInInterface.Close = ComponentBase::AP_Close;
            acpi->mPlugInInterface.Lookup = APMethodLookup::Lookup;
            acpi->mPlugInInterface.reserved = NULL;
            acpi->mConstruct = Construct;
            acpi->mDestruct = Destruct;
            acpi->mPad[0] = NULL;
            acpi->mPad[1] = NULL;
            return (AudioComponentPlugInInterface*)acpi;
        */
    }

    /**
       This is for runtime registration (not for
       plug-ins loaded from bundles).
      */
    pub fn register(
        ty:      u32,
        subtype: u32,
        manuf:   u32,
        name:    CFStringRef,
        vers:    u32,
        flags:   Option<u32>

    ) -> AudioComponent {

        let flags: u32 = flags.unwrap_or(0);

        todo!();
        /*
            AudioComponentDescription desc = { type, subtype, manuf, flags, 0 };
            return AudioComponentRegister(&desc, name, vers, Factory);
        */
    }
}
