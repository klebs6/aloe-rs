crate::ix!();

#[cfg(target_os="android")]
impl Process {
    
    pub fn open_document(&mut self, 
        file_name: &String,
        _1:        &String) -> bool {
        
        todo!();
        /*
            Url targetURL (fileName);
        auto* env = getEnv();

        const LocalRef<jstring> action (javaString ("android.intent.action.VIEW"));
        LocalRef<jobject> intent (env->NewObject (AndroidIntent, AndroidIntent.constructWithUri, action.get(), urlToUri (targetURL).get()));

        env->CallVoidMethod (getCurrentActivity(), AndroidContext.startActivity, intent.get());
        return true;
        */
    }
}
