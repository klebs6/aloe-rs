crate::ix!();

#[cfg(target_os="android")]
pub struct OpenSLEngineHolder {
    sl_library: DynamicLibrary, // default = "libOpenSLES.so" 
    engine:     SlRef<SLEngineItf_>,
}

#[cfg(target_os="android")]
impl Default for OpenSLEngineHolder {
    
    fn default() -> Self {
        todo!();
        /*


            if (auto createEngine = (CreateEngineFunc) slLibrary.getFunction ("slCreateEngine"))
            {
                SLObjectItf obj = nullptr;
                auto err = createEngine (&obj, 0, nullptr, 0, nullptr, nullptr);

                if (err != SL_RESULT_SUCCESS || obj == nullptr || *obj == nullptr
                    || (*obj)->Realize (obj, 0) != SL_RESULT_SUCCESS)
                {
                    destroyObject (obj);
                }

                engine = SlRef<SLEngineItf_>::cast (SlObjectRef (obj));
            
        */
    }
}

#[cfg(target_os="android")]
pub fn get_engine_holder<'a>() -> &'a mut OpenSLEngineHolder {
    
    todo!();
    /*
        static OpenSLEngineHolder holder;
        return holder;
    */
}
