crate::ix!();

pub trait TextureViewSurfaceTextureListenerOwner
{
    fn on_surface_texture_available(&mut self, 
        surface: &mut LocalRef<jobject>,
        width:   i32,
        height:  i32);

    fn on_surface_texture_destroyed(&mut self, surface: &mut LocalRef<jobject>) -> bool;

    fn on_surface_texture_size_changed(&mut self, 
        surface: &mut LocalRef<jobject>,
        width:   i32,
        height:  i32);

    fn on_surface_texture_updated(&mut self, surface: &mut LocalRef<jobject>);
}

//-----------------------------------------------------------
pub struct TextureViewSurfaceTextureListener<'a> {
    base:  AndroidInterfaceImplementer,
    owner: &'a mut dyn TextureViewSurfaceTextureListenerOwner,
}

impl<'a> TextureViewSurfaceTextureListener<'a> {

    pub fn new(owner_to_use: &mut dyn TextureViewSurfaceTextureListenerOwner) -> Self {
    
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

            if (methodName == "onSurfaceTextureAvailable" && numArgs == 3)
            {
                auto surface = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));
                auto width   = LocalRef<jobject> (env->GetObjectArrayElement (args, 1));
                auto height  = LocalRef<jobject> (env->GetObjectArrayElement (args, 2));

                auto widthInt  = env->CallIntMethod (width, JavaInteger.intValue);
                auto heightInt = env->CallIntMethod (height, JavaInteger.intValue);

                owner.onSurfaceTextureAvailable (surface, widthInt, heightInt);
                return nullptr;
            }
            else if (methodName == "onSurfaceTextureDestroyed" && numArgs == 1)
            {
                auto surface = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));
                auto result = owner.onSurfaceTextureDestroyed (surface);

                return env->CallStaticObjectMethod (JavaBoolean, JavaBoolean.valueOf, result);
            }
            else if (methodName == "onSurfaceTextureSizeChanged" && numArgs == 3)
            {
                auto surface = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));
                auto width   = LocalRef<jobject> (env->GetObjectArrayElement (args, 1));
                auto height  = LocalRef<jobject> (env->GetObjectArrayElement (args, 2));

                auto widthInt  = env->CallIntMethod (width, JavaInteger.intValue);
                auto heightInt = env->CallIntMethod (height, JavaInteger.intValue);

                owner.onSurfaceTextureSizeChanged (surface, widthInt, heightInt);
                return nullptr;
            }
            else if (methodName == "onSurfaceTextureUpdated" && numArgs == 1)
            {
                auto surface = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));

                owner.onSurfaceTextureUpdated (surface);
                return nullptr;
            }

            return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
