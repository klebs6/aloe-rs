crate::ix!();

#[derive(Default)]
#[no_copy]
pub struct AloePluginFactoryClassEntry {
    info2:           PClassInfo2,
    infow:           PClassInfoW,
    create_function: Option<CreateFunction>, // default = {}
    is_unicode:      bool, // default = false
}

impl AloePluginFactoryClassEntry {
    
    pub fn new(
        info: &PClassInfo2,
        fn_:  CreateFunction) -> Self {
    
        todo!();
        /*
        : info2(info),
        : create_function(fn),

        
        */
    }
}
