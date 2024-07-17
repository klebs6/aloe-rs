crate::ix!();

pub trait AndroidRunnableOwner
{
    fn run(&mut self);
}

pub struct AndroidRunnable<'a> {
    base:  AndroidInterfaceImplementer,
    owner: &'a mut dyn AndroidRunnableOwner,
}

impl<'a> AndroidRunnable<'a> {

    pub fn new(owner_to_use: &mut dyn AndroidRunnableOwner) -> Self {
    
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

            if (methodName == "run")
            {
                owner.run();
                return nullptr;
            }

            // invoke base class
            return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}
