crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_android_Fonts.cpp]

#[cfg(not(ALOE_USE_FREETYPE))]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         STATICMETHOD (create,          "create",           "(Ljava/lang/String;I)Landroid/graphics/Typeface;") 
         STATICMETHOD (createFromFile,  "createFromFile",   "(Ljava/lang/String;)Landroid/graphics/Typeface;") 
         STATICMETHOD (createFromAsset, "createFromAsset",  "(Landroid/content/res/AssetManager;Ljava/lang/String;)Landroid/graphics/Typeface;")
        */
    }
}

#[cfg(not(ALOE_USE_FREETYPE))]
declare_jni_class!{
    TypefaceClass, "android/graphics/Typeface"
}

#[cfg(not(ALOE_USE_FREETYPE))]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor,          "<init>",           "()V") 
         METHOD (computeBounds,        "computeBounds",     "(Landroid/graphics/RectF;Z)V")
        */
    }
}

#[cfg(not(ALOE_USE_FREETYPE))]
declare_jni_class!{
    AndroidPath, "android/graphics/Path"
}

#[cfg(not(ALOE_USE_FREETYPE))]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor,   "<init>",   "()V") 
         FIELD  (left,           "left",     "F") 
         FIELD  (right,          "right",    "F") 
         FIELD  (top,            "top",      "F") 
         FIELD  (bottom,         "bottom",   "F") 
         METHOD (roundOut,       "roundOut", "(Landroid/graphics/Rect;)V")
        */
    }
}

#[cfg(not(ALOE_USE_FREETYPE))]
declare_jni_class!{
    AndroidRectF, 
    "android/graphics/RectF"
}

#[cfg(not(ALOE_USE_FREETYPE))]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         STATICMETHOD (getInstance, "getInstance", "(Ljava/lang/String;)Ljava/security/MessageDigest;") 
         METHOD       (update,      "update",      "([B)V") 
         METHOD       (digest,      "digest",      "()[B")
        */
    }
}

#[cfg(not(ALOE_USE_FREETYPE))]
declare_jni_class!{
    JavaMessageDigest, 
    "java/security/MessageDigest"
}

#[cfg(not(ALOE_USE_FREETYPE))]
pub const REFERENCE_FONT_SIZE:     f32 = 256.0;

#[cfg(not(ALOE_USE_FREETYPE))]
pub const REFERENCE_FONT_TO_UNITS: f32 = 1.0 / REFERENCE_FONT_SIZE;

