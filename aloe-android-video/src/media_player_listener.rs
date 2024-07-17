crate::ix!();

pub trait MediaPlayerListenerOwner
{
    fn on_prepared(&mut self, media_player: &mut LocalRef<jobject>);

    fn on_buffering_update(&mut self, 
        media_player: &mut LocalRef<jobject>,
        progress:     i32);

    fn on_seek_complete(&mut self, media_player: &mut LocalRef<jobject>);

    fn on_completion(&mut self, media_player: &mut LocalRef<jobject>);

    fn on_info(&mut self, 
        media_player: &mut LocalRef<jobject>,
        what:         i32,
        extra:        i32) -> bool;

    fn on_error(&mut self, 
        media_player: &mut LocalRef<jobject>,
        what:         i32,
        extra:        i32) -> bool;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/native/aloe_android_Video.h]

pub struct MediaPlayerListener<'a> {
    base:  AndroidInterfaceImplementer,
    owner: &'a mut dyn MediaPlayerListenerOwner,
}

impl<'a> MediaPlayerListener<'a> {
    
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

            if (methodName == "onPrepared" && numArgs == 1)
            {
                auto mediaPlayer = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));

                owner.onPrepared (mediaPlayer);
                return nullptr;
            }

            if (methodName == "onCompletion" && numArgs == 1)
            {
                auto mediaPlayer = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));

                owner.onCompletion (mediaPlayer);
                return nullptr;
            }

            if (methodName == "onInfo" && numArgs == 3)
            {
                auto mediaPlayer = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));
                auto what        = LocalRef<jobject> (env->GetObjectArrayElement (args, 1));
                auto extra       = LocalRef<jobject> (env->GetObjectArrayElement (args, 2));

                auto whatInt  = (int) env->CallIntMethod (what, JavaInteger.intValue);
                auto extraInt = (int) env->CallIntMethod (extra, JavaInteger.intValue);

                auto res = owner.onInfo (mediaPlayer, whatInt, extraInt);
                return env->CallStaticObjectMethod (JavaBoolean, JavaBoolean.valueOf, (jboolean) res);
            }

            if (methodName == "onError" && numArgs == 3)
            {
                auto mediaPlayer = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));
                auto what        = LocalRef<jobject> (env->GetObjectArrayElement (args, 1));
                auto extra       = LocalRef<jobject> (env->GetObjectArrayElement (args, 2));

                auto whatInt  = (int) env->CallIntMethod (what, JavaInteger.intValue);
                auto extraInt = (int) env->CallIntMethod (extra, JavaInteger.intValue);

                auto res = owner.onError (mediaPlayer, whatInt, extraInt);
                return env->CallStaticObjectMethod (JavaBoolean, JavaBoolean.valueOf, (jboolean) res);
            }

            if (methodName == "onSeekComplete" && numArgs == 1)
            {
                auto mediaPlayer = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));

                owner.onSeekComplete (mediaPlayer);
                return nullptr;
            }

            if (methodName == "onBufferingUpdate" && numArgs == 2)
            {
                auto mediaPlayer = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));

                auto progress    = LocalRef<jobject> (env->GetObjectArrayElement (args, 1));
                auto progressInt = (int) env->CallIntMethod (progress, JavaInteger.intValue);

                owner.onBufferingUpdate (mediaPlayer, progressInt);

                return nullptr;
            }

            return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
