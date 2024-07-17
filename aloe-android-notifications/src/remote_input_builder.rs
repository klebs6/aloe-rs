crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (constructor,           "<init>",                "(Ljava/lang/String;)V") 
          METHOD (build,                 "build",                 "()Landroid/app/RemoteInput;") 
          METHOD (setAllowFreeFormInput, "setAllowFreeFormInput", "(Z)Landroid/app/RemoteInput$Builder;") 
          METHOD (setChoices,            "setChoices",            "([Ljava/lang/CharSequence;)Landroid/app/RemoteInput$Builder;") 
          METHOD (setLabel,              "setLabel",              "(Ljava/lang/CharSequence;)Landroid/app/RemoteInput$Builder;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    RemoteInputBuilder, 
    "android/app/RemoteInput$Builder", 20
}
