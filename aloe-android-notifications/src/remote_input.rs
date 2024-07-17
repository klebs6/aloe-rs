crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          STATICMETHOD (getResultsFromIntent, "getResultsFromIntent", "(Landroid/content/Intent;)Landroid/os/Bundle;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    RemoteInput, 
    "android/app/RemoteInput", 20
}
