crate::ix!();

/**
  | Plug-in entry point. \ingroup pluginBase
  | 
  | Any plug-in must define and export this
  | function. \n
  | 
  | A typical implementation of GetPluginFactory
  | looks like this
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | SMTG_EXPORT_SYMBOL IPluginFactory* PLUGIN_API GetPluginFactory ()
  | {
  |     if (!gPluginFactory)
  |     {
  |         static PFactoryInfo factoryInfo =
  |         {
  |             "My Company Name",
  |             "http://www.mywebpage.com",
  |             "mailto:myemail@address.com",
  |             PFactoryInfo::kNoFlags
  |         };
  | 
  |         gPluginFactory = new CPluginFactory (factoryInfo);
  | 
  |         static PClassInfo componentClass =
  |         {
  |             INLINE_UID (0x00000000, 0x00000000, 0x00000000, 0x00000000), // replace by a valid uid
  |             1,
  |             "Service",    // category
  |             "Name"
  |         };
  | 
  |         gPluginFactory->registerClass (&componentClass, MyComponentClass::newInstance);
  |     }
  |     else
  |         gPluginFactory->addRef ();
  | 
  |     return gPluginFactory;
  | }
  | \see \ref loadPlugin
  |
  */
#[SMTG_EXPORT_SYMBOL] 
#[PLUGIN_API] 
pub fn get_plugin_factory() -> *mut dyn IPluginFactory {
    
    todo!();
        /*
        
        */
}

#[PLUGIN_API] 
pub type GetFactoryProc = fn();
