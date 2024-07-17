crate::ix!();

pub trait ImageReaderOnImageAvailableListenerOwner
{
    fn on_image_available(&mut self, image_reader: &mut LocalRef<jobject>);
}

pub struct ImageReaderOnImageAvailableListener<'a> {
    base:  AndroidInterfaceImplementer,
    owner: &'a mut dyn ImageReaderOnImageAvailableListenerOwner,
}

impl<'a> ImageReaderOnImageAvailableListener<'a> {

    pub fn new(owner_to_use: &mut dyn ImageReaderOnImageAvailableListenerOwner) -> Self {
    
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

            if (methodName == "onImageAvailable" && numArgs == 1)
            {
                auto imageReader = LocalRef<jobject> (env->GetObjectArrayElement (args, 0));

                owner.onImageAvailable (imageReader);
                return nullptr;
            }

            return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
