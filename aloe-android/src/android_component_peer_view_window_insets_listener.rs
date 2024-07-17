crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidComponentPeerViewWindowInsetsListener {
    base: AndroidInterfaceImplementer,
}

#[cfg(target_os="android")]
impl AndroidComponentPeerViewWindowInsetsListener {

    pub fn on_apply_window_insets(&mut self, 
        v:      LocalRef<jobject>,
        insets: LocalRef<jobject>) -> jobject {
        
        todo!();
        /*
            auto* env = getEnv();

                LocalRef<jobject> displayCutout (env->CallObjectMethod (insets.get(), AndroidWindowInsets.getDisplayCutout));

                if (displayCutout != nullptr)
                {
                    const auto& mainDisplay = *Desktop::getInstance().getDisplays().getPrimaryDisplay();
                    auto newSafeAreaInsets = androidDisplayCutoutToBorderSize (displayCutout, mainDisplay.scale);

                    if (newSafeAreaInsets != mainDisplay.safeAreaInsets)
                        forceDisplayUpdate();

                    auto* fieldId = env->GetStaticFieldID (AndroidWindowInsets, "CONSUMED", "Landroid/view/WindowInsets");
                    jassert (fieldId != nullptr);

                    return env->GetStaticObjectField (AndroidWindowInsets, fieldId);
                }

                jmethodID onApplyWindowInsetsMethodId = env->GetMethodID (AndroidView,
                                                                          "onApplyWindowInsets",
                                                                          "(Landroid/view/WindowInsets;)Landroid/view/WindowInsets;");

                jassert (onApplyWindowInsetsMethodId != nullptr);

                return env->CallObjectMethod (v.get(), onApplyWindowInsetsMethodId, insets.get());
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

                if (methodName == "onApplyWindowInsets")
                {
                    jassert (env->GetArrayLength (args) == 2);

                    LocalRef<jobject> windowView (env->GetObjectArrayElement (args, 0));
                    LocalRef<jobject> insets     (env->GetObjectArrayElement (args, 1));

                    return onApplyWindowInsets (std::move (windowView), std::move (insets));
                }

                // invoke base class
                return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
