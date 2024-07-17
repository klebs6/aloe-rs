crate::ix!();

pub trait MediaRecorderOnInfoListenerOwner
{
    fn on_info(&mut self, 
        media_recorder: &mut LocalRef<jobject>,
        what:           i32,
        extra:          i32);
}

pub struct MediaRecorderOnInfoListener<'a> {
    base: AndroidInterfaceImplementer,
    owner: &'a mut dyn MediaRecorderOnInfoListenerOwner,
}

impl<'a> MediaRecorderOnInfoListener<'a> {

    pub fn new(owner_to_use: &mut dyn MediaRecorderOnInfoListenerOwner) -> Self {
    
        todo!();
        /*
        : owner(ownerToUse),

        
        */
    }
    
    pub fn invoke(&mut self, 
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject {
        
        todo!();
        /*
            auto* env = getEnv();

            auto methodName = aloeString ((jstring) env->CallObjectMethod (method, JavaMethod.getName));

            int numArgs = args != nullptr ? env->GetArrayLength (args) : 0;

            if (methodName == "onInfo" && numArgs == 3)
            {
                auto mediaRecorder = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));
                auto what   = LocalRef<jobject> (env->GetObjectArrayElement (args, 1));
                auto extra  = LocalRef<jobject> (env->GetObjectArrayElement (args, 2));

                auto whatInt  = (int) env->CallIntMethod (what, JavaInteger.intValue);
                auto extraInt = (int) env->CallIntMethod (extra, JavaInteger.intValue);

                owner.onInfo (mediaRecorder, whatInt, extraInt);
                return nullptr;
            }

            return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
