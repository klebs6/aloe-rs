crate::ix!();

#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
pub struct ComponentRegistrar<Class,const Type: OSType,const Subtype: OSType,const Manufacturer: OSType> {
    _p0: PhantomData<Class>,
}

#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
impl<Class,const Type: OSType,const Subtype: OSType,const Manufacturer: OSType> Default for 
ComponentRegistrar<Class,Type,Subtype,Manufacturer> {
    
    fn default() -> Self {
        todo!();
        /*


            ComponentEntryPoint<Class>::Register(Type, Subtype, Manufacturer);
        */
    }
}

///-------------------------
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
macro_rules! component_register {
    ($Class:ident, 
     $Type:ident, 
     $Subtype:ident, 
     $Manufacturer:ident) => {
        /*
        
            static ComponentRegistrar<Class, Type, Subtype, Manufacturer>   gRegistrar##Class
        */
    }
}

#[cfg(CA_USE_AUDIO_PLUGIN_ONLY)]
macro_rules! component_entry    { ($Class:ident) => { } }

#[cfg(CA_USE_AUDIO_PLUGIN_ONLY)]
macro_rules! component_register { ($Class:ident) => { } }
