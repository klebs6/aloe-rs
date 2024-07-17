crate::ix!();

//#[cfg(target_os="android")]
pub trait SurfaceHolderCallbackInterface {

    fn surface_changed(&mut self, 
        holder: LocalRef<jobject>,
        format: i32,
        width:  i32,
        height: i32);


    fn surface_created(&mut self, holder: LocalRef<jobject>);


    fn surface_destroyed(&mut self, holder: LocalRef<jobject>);
}

//#[cfg(target_os="android")]
pub struct SurfaceHolderCallback {
    base: AndroidInterfaceImplementer,
}

//#[cfg(target_os="android")]
impl AndroidInterfaceImplementerInterface for SurfaceHolderCallback {

    fn invoke(&mut self, 
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject {
        
        todo!();
        /*
            auto* env = getEnv();
            auto methodName = aloeString ((jstring) env->CallObjectMethod (method, JavaMethod.getName));
            LocalRef<jobject> holder (env->GetArrayLength (args) > 0 ? env->GetObjectArrayElement (args, 0) : (jobject) nullptr);

            if (methodName == "surfaceChanged")
            {
                int intArgs[3];

                for (int i = 0; i < 3; ++i)
                {
                    LocalRef<jobject> boxedType (env->GetObjectArrayElement (args, 1 + i));
                    intArgs[i] = env->CallIntMethod (boxedType.get(), JavaInteger.intValue);
                }

                surfaceChanged (std::move (holder), intArgs[0], intArgs[1], intArgs[2]);
            }
            else if (methodName == "surfaceCreated")
            {
                surfaceCreated (std::move (holder));
            }
            else if (methodName == "surfaceDestroyed")
            {
                surfaceDestroyed (std::move (holder));
            }
            else
            {
                return AndroidInterfaceImplementer::invoke (proxy, method, args);
            }

            return nullptr;
        */
    }
}
