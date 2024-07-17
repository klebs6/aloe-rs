crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidContentSharerFileObserver<'a> {
    file_was_read:      bool, // default = false
    num_opened_handles: i32, // default = 0
    owner:              &'a mut dyn AndroidContentSharerFileObserverOwner,
    filepath:           String,
    file_observer:      GlobalRef,
}

pub const ANDROID_CONTENT_SHARER_FILE_OBSERVER_OPEN:           i32 = 32;
pub const ANDROID_CONTENT_SHARER_FILE_OBSERVER_ACCESS:         i32 = 1;
pub const ANDROID_CONTENT_SHARER_FILE_OBSERVER_CLOSE_WRITE:    i32 = 8;
pub const ANDROID_CONTENT_SHARER_FILE_OBSERVER_CLOSE_NO_WRITE: i32 = 16;

#[cfg(target_os="android")]
pub trait AndroidContentSharerFileObserverOwner {
    fn file_handle_closed(&mut self, _0: &AndroidContentSharerFileObserver);
}

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
             METHOD (constructor,   "<init>",        "(JLjava/lang/String;I)V") 
             METHOD (startWatching, "startWatching", "()V") 
             METHOD (stopWatching,  "stopWatching",  "()V") 
             CALLBACK (contentSharerFileObserverEvent, "contentSharerFileObserverEvent", "(JILjava/lang/String;)V") 
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_bytecode!{
    AloeContentProviderFileObserver, 
    "com/rmsl/aloe/AloeContentProviderFileObserver", 
    16, 
    javaAloeContentProviderFileObserver, 
    sizeof (javaAloeContentProviderFileObserver)
}

// #undef JNI_CLASS_MEMBERS

#[cfg(target_os="android")]
impl<'a> AndroidContentSharerFileObserver<'a> {

    pub fn new(
        owner_to_use:     &mut dyn AndroidContentSharerFileObserverOwner,
        env:              *mut JNIEnv,
        content_provider: &LocalRef<jobject>,
        filepath_to_use:  &String) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
              filepath (filepathToUse),
              fileObserver (GlobalRef (LocalRef<jobject> (env->NewObject (AloeContentProviderFileObserver,
                                                                          AloeContentProviderFileObserver.constructor,
                                                                          reinterpret_cast<jlong> (this),
                                                                          javaString (filepath).get(),
                                                                          open | access | closeWrite | closeNoWrite))))

            // the content provider must be created first
            jassert (contentProvider.get() != nullptr);

            env->CallVoidMethod (fileObserver, AloeContentProviderFileObserver.startWatching);
        */
    }
    
    pub fn on_file_event(&mut self, 
        event: i32,
        path:  &LocalRef<jstring>)  {
        
        todo!();
        /*
            ignoreUnused (path);

            if (event == open)
            {
                ++numOpenedHandles;
            }
            else if (event == access)
            {
                fileWasRead = true;
            }
            else if (event == closeNoWrite || event == closeWrite)
            {
                --numOpenedHandles;

                // numOpenedHandles may get negative if we don't receive open handle event.
                if (fileWasRead && numOpenedHandles <= 0)
                {
                    MessageManager::callAsync ([this]
                    {
                        getEnv()->CallVoidMethod (fileObserver, AloeContentProviderFileObserver.stopWatching);
                        owner.fileHandleClosed (*this);
                    });
                }
            }
        */
    }

    #[JNICALL]
    pub fn content_sharer_file_observer_event(
        _0:            *mut JNIEnv,
        file_observer: jobject,
        host:          i64,
        event:         i32,
        path:          jstring)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidContentSharerFileObserver*> (host))
                myself->onFileEvent (event, LocalRef<jstring> (path));
        */
    }
}

lazy_static!{
    /*
    AndroidContentSharerFileObserver::AloeContentProviderFileObserver_Class AndroidContentSharerFileObserver::AloeContentProviderFileObserver;
    */
}
