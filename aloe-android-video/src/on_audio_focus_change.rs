crate::ix!();

pub trait AudioManagerOnAudioFocusChangeListenerOwner
{
    fn on_audio_focus_change(&mut self, change_type: i32);
}

pub struct AudioManagerOnAudioFocusChangeListener<'a> {
    base:  AndroidInterfaceImplementer,
    owner: &'a mut dyn MediaPlayerListenerOwner,
}

impl<'a> AudioManagerOnAudioFocusChangeListener<'a> {
    
    pub fn new(owner_to_use: &mut dyn MediaPlayerListenerOwner) -> Self {
    
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

            if (methodName == "onAudioFocusChange" && numArgs == 1)
            {
                auto changeType = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));

                auto changeTypeInt = (int) env->CallIntMethod (changeType, JavaInteger.intValue);

                owner.onAudioFocusChange (changeTypeInt);
                return nullptr;
            }

            return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
