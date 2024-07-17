crate::ix!();

#[cfg(target_os="android")]
impl MessageManager {
    
    pub fn do_platform_specific_initialisation(&mut self)  {
        
        todo!();
        /*
            AndroidMessageQueue::getInstance();
        */
    }
    
    pub fn do_platform_specific_shutdown(&mut self)  {
        
        todo!();
        /*
            AndroidMessageQueue::deleteInstance();
        */
    }
    
    pub fn post_message_to_system_queue(&mut self, message: *mut dyn MessageBaseInterface) -> bool {
        
        todo!();
        /*
            return AndroidMessageQueue::getInstance()->post (message);
        */
    }
    
    pub fn broadcast_message(&mut self, _0: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn run_dispatch_loop(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn stop_dispatch_loop(&mut self)  {
        
        todo!();
        /*
            struct QuitCallback  : public CallbackMessage
        {
            QuitCallback() {}

            void messageCallback() override
            {
                auto* env = getEnv();
                LocalRef<jobject> activity (getCurrentActivity());

                if (activity != nullptr)
                {
                    jmethodID quitMethod = env->GetMethodID (AndroidActivity, "finishAndRemoveTask", "()V");

                    if (quitMethod != nullptr)
                    {
                        env->CallVoidMethod (activity.get(), quitMethod);
                        return;
                    }

                    quitMethod = env->GetMethodID (AndroidActivity, "finish", "()V");
                    jassert (quitMethod != nullptr);
                    env->CallVoidMethod (activity.get(), quitMethod);
                }
                else
                {
                    jassertfalse;
                }
            }
        };

        (new QuitCallback())->post();
        quitMessagePosted = true;
        */
    }
}
