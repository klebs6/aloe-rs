crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidContentSharerCursor<'a> {
    owner:  &'a mut dyn AndroidContentSharerCursorOwner,
    cursor: GlobalRef,
}

#[cfg(target_os="android")]
pub trait AndroidContentSharerCursorOwner {

    fn cursor_closed<'a>(&mut self, _0: &AndroidContentSharerCursor<'a>);
}

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
             METHOD (addRow,      "addRow", "([Ljava/lang/Object;)V") 
             METHOD (constructor, "<init>", "(J[Ljava/lang/String;)V") 
             CALLBACK (contentSharerCursorClosed, "contentSharerCursorClosed", "(J)V") 
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_bytecode!{
    AloeContentProviderCursor, 
    "com/rmsl/aloe/AloeContentProviderCursor", 
    16, 
    javaAloeContentProviderCursor, 
    sizeof (javaAloeContentProviderCursor)
}

// #undef JNI_CLASS_MEMBERS

#[cfg(target_os="android")]
impl<'a> AndroidContentSharerCursor<'a> {

    pub fn new(
        owner_to_use:     &mut dyn AndroidContentSharerCursorOwner,
        env:              *mut JNIEnv,
        content_provider: &LocalRef<jobject>,
        result_columns:   &LocalRef<jobjectArray>) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
              cursor (GlobalRef (LocalRef<jobject> (env->NewObject (AloeContentProviderCursor,
                                                                    AloeContentProviderCursor.constructor,
                                                                    reinterpret_cast<jlong> (this),
                                                                    resultColumns.get()))))
            // the content provider must be created first
            jassert (contentProvider.get() != nullptr);
        */
    }
    
    pub fn get_native_cursor(&mut self) -> jobject {
        
        todo!();
        /*
            return cursor.get();
        */
    }
    
    pub fn cursor_closed(&mut self)  {
        
        todo!();
        /*
            MessageManager::callAsync ([this] { owner.cursorClosed (*this); });
        */
    }
    
    pub fn add_row(&mut self, values: &mut LocalRef<jobjectArray>)  {
        
        todo!();
        /*
            auto* env = getEnv();

            env->CallVoidMethod (cursor.get(), AloeContentProviderCursor.addRow, values.get());
        */
    }

    #[JNICALL]
    pub fn content_sharer_cursor_closed(
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<AndroidContentSharerCursor*> (host))
                myself->cursorClosed();
        */
    }
}

lazy_static!{
    /*
    AndroidContentSharerCursor::AloeContentProviderCursor_Class AndroidContentSharerCursor::AloeContentProviderCursor;
    */
}
