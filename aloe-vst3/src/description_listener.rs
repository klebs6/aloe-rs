crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DescriptionLister<'a> {
    base: DescriptionFactory<'a>,
    list: Vec<Box<PluginDescription>>,
}

impl<'a> DescriptionLister<'a> {
    
    pub fn new(
        host:           *mut Vst3HostContext,
        plugin_factory: *mut dyn IPluginFactory) -> Self {
    
        todo!();
        /*
        : description_factory(host, pluginFactory),

        
        */
    }
    
    pub fn perform_on_description(&mut self, desc: &mut PluginDescription) -> Result<(),()> {
        
        todo!();
        /*
            list.add (new PluginDescription (desc));
            return Result::ok();
        */
    }
}
