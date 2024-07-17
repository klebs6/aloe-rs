crate::ix!();

/**
  | Get code cache directory without yet
  | having a context object
  |
  */
pub fn get_code_cache_directory() -> File {
    
    todo!();
    /*
        int pid = getpid();
        File cmdline("/proc/" + String(pid) + "/cmdline");

        auto bundleId = cmdline.loadFileAsString().trimStart().trimEnd();

        if (bundleId.isEmpty())
            return {};

        return File("/data/data/" + bundleId + "/code_cache");
    */
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_android_JNIHelpers.h]

lazy_static!{
    /*
    extern JNIEnv* getEnv() ;
    */
}

lazy_static!{
    /*
    extern LocalRef<jobject> getAppContext() ;
    extern LocalRef<jobject> getCurrentActivity() ;
    extern LocalRef<jobject> getMainActivity() ;
    */
}


///TODO: find another way
lazy_static!{
    /*
    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (getAssets,                            "getAssets",                       "()Landroid/content/res/AssetManager;") \
     METHOD (getSystemService,                     "getSystemService",                "(Ljava/lang/String;)Ljava/lang/Object;") \
     METHOD (getPackageManager,                    "getPackageManager",               "()Landroid/content/pm/PackageManager;") \
     METHOD (getPackageName,                       "getPackageName",                  "()Ljava/lang/String;") \
     METHOD (getResources,                         "getResources",                    "()Landroid/content/res/Resources;") \
     METHOD (bindService,                          "bindService",                     "(Landroid/content/Intent;Landroid/content/ServiceConnection;I)Z") \
     METHOD (unbindService,                        "unbindService",                   "(Landroid/content/ServiceConnection;)V") \
     METHOD (startActivity,                        "startActivity",                   "(Landroid/content/Intent;)V") \
     METHOD (getContentResolver,                   "getContentResolver",              "()Landroid/content/ContentResolver;") \
     METHOD (getApplicationContext,                "getApplicationContext",           "()Landroid/content/Context;") \
     METHOD (getApplicationInfo,                   "getApplicationInfo",              "()Landroid/content/pm/ApplicationInfo;") \
     METHOD (checkCallingOrSelfPermission,         "checkCallingOrSelfPermission",    "(Ljava/lang/String;)I") \
     METHOD (getCacheDir,                          "getCacheDir",                     "()Ljava/io/File;")

    DECLARE_JNI_CLASS (AndroidContext, "android/content/Context")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (finish,                               "finish",                          "()V") \
     METHOD (getWindowManager,                     "getWindowManager",                "()Landroid/view/WindowManager;") \
     METHOD (setRequestedOrientation,              "setRequestedOrientation",         "(I)V") \
     METHOD (startIntentSenderForResult,           "startIntentSenderForResult",      "(Landroid/content/IntentSender;ILandroid/content/Intent;III)V") \
     METHOD (moveTaskToBack,                       "moveTaskToBack",                  "(Z)Z") \
     METHOD (startActivityForResult,               "startActivityForResult",          "(Landroid/content/Intent;I)V") \
     METHOD (getFragmentManager,                   "getFragmentManager",              "()Landroid/app/FragmentManager;") \
     METHOD (setContentView,                       "setContentView",                  "(Landroid/view/View;)V") \
     METHOD (getWindow,                            "getWindow",                       "()Landroid/view/Window;")

    DECLARE_JNI_CLASS (AndroidActivity, "android/app/Activity")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (startActivityForResult,               "startActivityForResult",          "(Landroid/content/Intent;I)V") \
     METHOD (setArguments,                         "setArguments",                    "(Landroid/os/Bundle;)V")

    DECLARE_JNI_CLASS_WITH_MIN_SDK (AndroidFragment, "android/app/Fragment", 11)
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (build,          "build",          "()Landroid/media/AudioAttributes;") \
      METHOD (constructor,    "<init>",         "()V") \
      METHOD (setContentType, "setContentType", "(I)Landroid/media/AudioAttributes$Builder;") \
      METHOD (setUsage,       "setUsage",       "(I)Landroid/media/AudioAttributes$Builder;")

    DECLARE_JNI_CLASS_WITH_MIN_SDK (AndroidAudioAttributesBuilder, "android/media/AudioAttributes$Builder", 21)
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (abandonAudioFocus, "abandonAudioFocus", "(Landroid/media/AudioManager$OnAudioFocusChangeListener;)I") \
      METHOD (requestAudioFocus, "requestAudioFocus", "(Landroid/media/AudioManager$OnAudioFocusChangeListener;II)I")

    DECLARE_JNI_CLASS (AndroidAudioManager, "android/media/AudioManager")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (createBitmap,     "createBitmap", "(IILandroid/graphics/Bitmap$Config;)Landroid/graphics/Bitmap;") \
      STATICMETHOD (createBitmapFrom, "createBitmap", "(Landroid/graphics/Bitmap;IIIILandroid/graphics/Matrix;Z)Landroid/graphics/Bitmap;") \
      METHOD (compress,  "compress",  "(Landroid/graphics/Bitmap$CompressFormat;ILjava/io/OutputStream;)Z") \
      METHOD (getHeight, "getHeight", "()I") \
      METHOD (getWidth,  "getWidth",  "()I") \
      METHOD (recycle,   "recycle",   "()V") \
      METHOD (setPixel,  "setPixel",  "(III)V") \
      METHOD (getPixels, "getPixels",  "([IIIIIII)V")

    DECLARE_JNI_CLASS (AndroidBitmap, "android/graphics/Bitmap")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (valueOf, "valueOf", "(Ljava/lang/String;)Landroid/graphics/Bitmap$Config;")

    DECLARE_JNI_CLASS (AndroidBitmapConfig, "android/graphics/Bitmap$Config")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (decodeByteArray, "decodeByteArray", "([BII)Landroid/graphics/Bitmap;")

    DECLARE_JNI_CLASS (AndroidBitmapFactory, "android/graphics/BitmapFactory")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK)    \
      METHOD (constructor,        "<init>",             "()V") \
      METHOD (containsKey,        "containsKey",        "(Ljava/lang/String;)Z") \
      METHOD (get,                "get",                "(Ljava/lang/String;)Ljava/lang/Object;") \
      METHOD (getBoolean,         "getBoolean",         "(Ljava/lang/String;)Z") \
      METHOD (getBundle,          "getBundle",          "(Ljava/lang/String;)Landroid/os/Bundle;") \
      METHOD (getCharSequence,    "getCharSequence",    "(Ljava/lang/String;)Ljava/lang/CharSequence;") \
      METHOD (getInt,             "getInt",             "(Ljava/lang/String;)I") \
      METHOD (getLong,            "getLong",            "(Ljava/lang/String;)J") \
      METHOD (getLongArray,       "getLongArray",       "(Ljava/lang/String;)[J") \
      METHOD (getParcelable,      "getParcelable",      "(Ljava/lang/String;)Landroid/os/Parcelable;") \
      METHOD (getString,          "getString",          "(Ljava/lang/String;)Ljava/lang/String;") \
      METHOD (getStringArrayList, "getStringArrayList", "(Ljava/lang/String;)Ljava/util/ArrayList;") \
      METHOD (keySet,             "keySet",             "()Ljava/util/Set;") \
      METHOD (putBoolean,         "putBoolean",         "(Ljava/lang/String;Z)V") \
      METHOD (putBundle,          "putBundle",          "(Ljava/lang/String;Landroid/os/Bundle;)V") \
      METHOD (putFloat,           "putFloat",           "(Ljava/lang/String;F)V") \
      METHOD (putInt,             "putInt",             "(Ljava/lang/String;I)V") \
      METHOD (putLong,            "putLong",            "(Ljava/lang/String;J)V") \
      METHOD (putLongArray,       "putLongArray",       "(Ljava/lang/String;[J)V") \
      METHOD (putString,          "putString",          "(Ljava/lang/String;Ljava/lang/String;)V") \
      METHOD (putStringArrayList, "putStringArrayList", "(Ljava/lang/String;Ljava/util/ArrayList;)V")

    DECLARE_JNI_CLASS (AndroidBundle, "android/os/Bundle")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (dumpReferenceTables, "dumpReferenceTables", "()V")

      DECLARE_JNI_CLASS (AndroidDebug, "android/os/Debug")
    #undef JNI_CLASS_MEMBERS

    #define ALOE_LOG_JNI_REFERENCES_TABLE getEnv()->CallStaticVoidMethod (AndroidDebug, AndroidDebug.dumpReferenceTables);

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (getRotation, "getRotation", "()I") \
     METHOD (getMetrics,  "getMetrics",  "(Landroid/util/DisplayMetrics;)V" ) \
     METHOD (getSize,     "getSize",     "(Landroid/graphics/Point;)V" )

    DECLARE_JNI_CLASS (AndroidDisplay, "android/view/Display")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (constructor,           "<init>",      "()V") \
      METHOD (constructorWithLooper, "<init>",      "(Landroid/os/Looper;)V") \
      METHOD (post,                  "post",        "(Ljava/lang/Runnable;)Z") \
      METHOD (postDelayed,           "postDelayed", "(Ljava/lang/Runnable;J)Z") \

    DECLARE_JNI_CLASS (AndroidHandler, "android/os/Handler")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (constructor, "<init>",     "(Ljava/lang/String;)V") \
      METHOD (getLooper,   "getLooper",  "()Landroid/os/Looper;") \
      METHOD (join,        "join",       "()V") \
      METHOD (start,       "start",      "()V")

    DECLARE_JNI_CLASS (AndroidHandlerThread, "android/os/HandlerThread")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (createChooser, "createChooser", "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;") \
      METHOD (addCategory,                    "addCategory",    "(Ljava/lang/String;)Landroid/content/Intent;") \
      METHOD (constructor,                    "<init>",         "()V") \
      METHOD (constructorWithContextAndClass, "<init>",         "(Landroid/content/Context;Ljava/lang/Class;)V") \
      METHOD (constructWithString,            "<init>",         "(Ljava/lang/String;)V") \
      METHOD (constructWithUri,               "<init>",         "(Ljava/lang/String;Landroid/net/Uri;)V") \
      METHOD (getAction,                      "getAction",      "()Ljava/lang/String;") \
      METHOD (getCategories,                  "getCategories",  "()Ljava/util/Set;") \
      METHOD (getData,                        "getData",        "()Landroid/net/Uri;") \
      METHOD (getExtras,                      "getExtras",      "()Landroid/os/Bundle;") \
      METHOD (getIntExtra,                    "getIntExtra",    "(Ljava/lang/String;I)I") \
      METHOD (getStringExtra,                 "getStringExtra", "(Ljava/lang/String;)Ljava/lang/String;") \
      METHOD (putExtra,                       "putExtra",       "(Ljava/lang/String;Ljava/lang/CharSequence;)Landroid/content/Intent;") \
      METHOD (putExtras,                      "putExtras",      "(Landroid/os/Bundle;)Landroid/content/Intent;") \
      METHOD (putExtraString,                 "putExtra",       "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;") \
      METHOD (putExtraStrings,                "putExtra",       "(Ljava/lang/String;[Ljava/lang/String;)Landroid/content/Intent;") \
      METHOD (putExtraParcelable,             "putExtra",       "(Ljava/lang/String;Landroid/os/Parcelable;)Landroid/content/Intent;") \
      METHOD (putParcelableArrayListExtra,    "putParcelableArrayListExtra", "(Ljava/lang/String;Ljava/util/ArrayList;)Landroid/content/Intent;") \
      METHOD (setAction,                      "setAction",      "(Ljava/lang/String;)Landroid/content/Intent;") \
      METHOD (setFlags,                       "setFlags",       "(I)Landroid/content/Intent;") \
      METHOD (setPackage,                     "setPackage",     "(Ljava/lang/String;)Landroid/content/Intent;") \
      METHOD (setType,                        "setType",        "(Ljava/lang/String;)Landroid/content/Intent;") \

    DECLARE_JNI_CLASS (AndroidIntent, "android/content/Intent")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor,    "<init>",        "()V") \
     METHOD (postRotate,     "postRotate",    "(FFF)Z") \
     METHOD (postScale,      "postScale",     "(FFFF)Z") \
     METHOD (postTranslate,  "postTranslate", "(FF)Z") \
     METHOD (setValues,      "setValues",     "([F)V") \
     METHOD (mapRect,        "mapRect",       "(Landroid/graphics/RectF;)Z")

    DECLARE_JNI_CLASS (AndroidMatrix, "android/graphics/Matrix")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (getPackageInfo, "getPackageInfo", "(Ljava/lang/String;I)Landroid/content/pm/PackageInfo;") \
     METHOD (resolveActivity, "resolveActivity", "(Landroid/content/Intent;I)Landroid/content/pm/ResolveInfo;") \
     METHOD (hasSystemFeature, "hasSystemFeature", "(Ljava/lang/String;)Z")

    DECLARE_JNI_CLASS (AndroidPackageManager, "android/content/pm/PackageManager")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     FIELD (requestedPermissions,   "requestedPermissions",   "[Ljava/lang/String;") \
     FIELD (activities,             "activities",             "[Landroid/content/pm/ActivityInfo;") \
     FIELD (providers,              "providers",              "[Landroid/content/pm/ProviderInfo;")

     DECLARE_JNI_CLASS (AndroidPackageInfo, "android/content/pm/PackageInfo")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     FIELD (name,        "name",        "Ljava/lang/String;") \
     FIELD (packageName, "packageName", "Ljava/lang/String;")

     DECLARE_JNI_CLASS (AndroidPackageItemInfo, "android/content/pm/PackageItemInfo")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor,   "<init>",           "(I)V") \
     METHOD (setColor,      "setColor",         "(I)V") \
     METHOD (setAlpha,      "setAlpha",         "(I)V") \
     METHOD (setTypeface,   "setTypeface",      "(Landroid/graphics/Typeface;)Landroid/graphics/Typeface;") \
     METHOD (ascent,        "ascent",           "()F") \
     METHOD (descent,       "descent",          "()F") \
     METHOD (setTextSize,   "setTextSize",      "(F)V") \
     METHOD (getTextWidths, "getTextWidths",    "(Ljava/lang/String;[F)I") \
     METHOD (setTextScaleX, "setTextScaleX",    "(F)V") \
     METHOD (getTextPath,   "getTextPath",      "(Ljava/lang/String;IIFFLandroid/graphics/Path;)V") \
     METHOD (getCharsPath,  "getTextPath",      "([CIIFFLandroid/graphics/Path;)V") \
     METHOD (setShader,     "setShader",        "(Landroid/graphics/Shader;)Landroid/graphics/Shader;") \

    DECLARE_JNI_CLASS (AndroidPaint, "android/graphics/Paint")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (create,          "<init>",           "(Landroid/graphics/Bitmap;)V") \
     METHOD (setMatrix,       "setMatrix",        "(Landroid/graphics/Matrix;)V") \
     METHOD (drawPath,        "drawPath",         "(Landroid/graphics/Path;Landroid/graphics/Paint;)V") \
     METHOD (drawBitmap,      "drawBitmap",       "([IIIFFIIZLandroid/graphics/Paint;)V") \
     METHOD (getClipBounds,   "getClipBounds",    "()Landroid/graphics/Rect;")

     DECLARE_JNI_CLASS (AndroidCanvas, "android/graphics/Canvas")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (getActivity, "getActivity", "(Landroid/content/Context;ILandroid/content/Intent;I)Landroid/app/PendingIntent;") \
      METHOD (getIntentSender, "getIntentSender", "()Landroid/content/IntentSender;")

    DECLARE_JNI_CLASS (AndroidPendingIntent, "android/app/PendingIntent")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (toString, "toString", "()Ljava/lang/String;")

    DECLARE_JNI_CLASS_WITH_MIN_SDK (AndroidRange, "android/util/Range", 21)
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (create,   "<init>",   "(II)V") \
     FIELD  (x,        "x",        "I") \
     FIELD  (y,        "y",        "I")

    DECLARE_JNI_CLASS (AndroidPoint, "android/graphics/Point")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor,   "<init>",   "(IIII)V") \
     FIELD (left,           "left",     "I") \
     FIELD (right,          "right",    "I") \
     FIELD (top,            "top",      "I") \
     FIELD (bottom,         "bottom",   "I")

    DECLARE_JNI_CLASS (AndroidRect, "android/graphics/Rect")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (getIdentifier,     "getIdentifier",     "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I") \
      METHOD (openRawResourceFd, "openRawResourceFd", "(I)Landroid/content/res/AssetFileDescriptor;")

    DECLARE_JNI_CLASS (AndroidResources, "android/content/res/Resources")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (getHeight, "getHeight", "()I") \
      METHOD (getWidth,  "getWidth",  "()I")

    DECLARE_JNI_CLASS_WITH_MIN_SDK (AndroidSize, "android/util/Size", 21)
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (parse, "parse", "(Ljava/lang/String;)Landroid/net/Uri;") \
      METHOD (toString, "toString", "()Ljava/lang/String;")

    DECLARE_JNI_CLASS (AndroidUri, "android/net/Uri")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (construct,                 "<init>",                    "(Landroid/content/Context;)V") \
     METHOD (layout,                    "layout",                    "(IIII)V") \
     METHOD (getLeft,                   "getLeft",                   "()I") \
     METHOD (getTop,                    "getTop",                    "()I") \
     METHOD (getWidth,                  "getWidth",                  "()I") \
     METHOD (getHeight,                 "getHeight",                 "()I") \
     METHOD (getLocationOnScreen,       "getLocationOnScreen",       "([I)V") \
     METHOD (getParent,                 "getParent",                 "()Landroid/view/ViewParent;") \
     METHOD (bringToFront,              "bringToFront",              "()V") \
     METHOD (requestFocus,              "requestFocus",              "()Z") \
     METHOD (hasFocus,                  "hasFocus",                  "()Z") \
     METHOD (invalidate,                "invalidate",                "(IIII)V") \
     METHOD (setVisibility,             "setVisibility",             "(I)V") \
     METHOD (setLayoutParams,           "setLayoutParams",           "(Landroid/view/ViewGroup$LayoutParams;)V") \
     METHOD (setSystemUiVisibility,     "setSystemUiVisibility",     "(I)V") \
     METHOD (findViewById,              "findViewById",              "(I)Landroid/view/View;") \
     METHOD (getRootView,               "getRootView",               "()Landroid/view/View;") \
     METHOD (addOnLayoutChangeListener, "addOnLayoutChangeListener", "(Landroid/view/View$OnLayoutChangeListener;)V") \
     METHOD (announceForAccessibility,  "announceForAccessibility",  "(Ljava/lang/CharSequence;)V") \

    DECLARE_JNI_CLASS (AndroidView, "android/view/View")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (addView,                       "addView",                       "(Landroid/view/View;)V") \
     METHOD (removeView,                    "removeView",                    "(Landroid/view/View;)V") \
     METHOD (requestSendAccessibilityEvent, "requestSendAccessibilityEvent", "(Landroid/view/View;Landroid/view/accessibility/AccessibilityEvent;)Z") \

    DECLARE_JNI_CLASS (AndroidViewGroup, "android/view/ViewGroup")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (getDecorView, "getDecorView",       "()Landroid/view/View;") \
     METHOD (setFlags,     "setFlags",           "(II)V") \
     METHOD (clearFlags,   "clearFlags",         "(I)V")

    DECLARE_JNI_CLASS (AndroidWindow, "android/view/Window")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (getDefaultDisplay, "getDefaultDisplay", "()Landroid/view/Display;")

    DECLARE_JNI_CLASS (AndroidWindowManager, "android/view/WindowManager")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (constructor, "<init>",   "(I)V") \
      METHOD (add,         "add",      "(Ljava/lang/Object;)Z") \
      METHOD (iterator,    "iterator", "()Ljava/util/Iterator;") \
      METHOD (get,         "get",      "(I)Ljava/lang/Object;") \
      METHOD (size,        "size",     "()I")

    DECLARE_JNI_CLASS (JavaArrayList, "java/util/ArrayList")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (valueOf, "valueOf", "(Z)Ljava/lang/Boolean;") \
      METHOD (booleanValue, "booleanValue", "()Z")

    DECLARE_JNI_CLASS (JavaBoolean, "java/lang/Boolean")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (get,        "get",       "([B)Ljava/nio/ByteBuffer;") \
      METHOD (remaining,  "remaining", "()I") \
      STATICMETHOD (wrap, "wrap",      "([B)Ljava/nio/ByteBuffer;")

    DECLARE_JNI_CLASS (JavaByteBuffer, "java/nio/ByteBuffer")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (toString, "toString", "()Ljava/lang/String;")

    DECLARE_JNI_CLASS (JavaCharSequence, "java/lang/CharSequence")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (forName, "forName", "(Ljava/lang/String;)Ljava/lang/Class;") \
      METHOD (getName,           "getName",           "()Ljava/lang/String;") \
      METHOD (getModifiers,      "getModifiers",      "()I")            \
      METHOD (isAnnotation,      "isAnnotation",      "()Z") \
      METHOD (isAnonymousClass,  "isAnonymousClass",  "()Z") \
      METHOD (isArray,           "isArray",           "()Z") \
      METHOD (isEnum,            "isEnum",            "()Z") \
      METHOD (isInterface,       "isInterface",       "()Z") \
      METHOD (isLocalClass,      "isLocalClass",      "()Z") \
      METHOD (isMemberClass,     "isMemberClass",     "()Z") \
      METHOD (isPrimitive,       "isPrimitive",       "()Z") \
      METHOD (isSynthetic,       "isSynthetic",       "()Z") \
      METHOD (getComponentType,  "getComponentType",  "()Ljava/lang/Class;") \
      METHOD (getSuperclass,     "getSuperclass",     "()Ljava/lang/Class;") \
      METHOD (getClassLoader,    "getClassLoader",    "()Ljava/lang/ClassLoader;") \

    DECLARE_JNI_CLASS (JavaClass, "java/lang/Class")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (toString, "toString", "()Ljava/lang/String;")

    DECLARE_JNI_CLASS (JavaEnum, "java/lang/Enum")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor,     "<init>",          "(Ljava/lang/String;)V") \
     METHOD (getAbsolutePath, "getAbsolutePath", "()Ljava/lang/String;") \
     METHOD (length,          "length",          "()J")

    DECLARE_JNI_CLASS (JavaFile, "java/io/File")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor, "<init>", "(Ljava/lang/String;)V") \
     METHOD (close,       "close",  "()V") \
     METHOD (read,        "read",   "([B)I")

    DECLARE_JNI_CLASS (JavaFileInputStream, "java/io/FileInputStream")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor, "<init>", "(Ljava/lang/String;)V") \
     METHOD (close,       "close",  "()V") \
     METHOD (write,       "write",  "([BII)V")

    DECLARE_JNI_CLASS (JavaFileOutputStream, "java/io/FileOutputStream")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (constructor,             "<init>", "()V") \
      METHOD (constructorWithCapacity, "<init>", "(I)V")

    DECLARE_JNI_CLASS (JavaHashMap, "java/util/HashMap")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      STATICMETHOD (parseInt, "parseInt", "(Ljava/lang/String;I)I") \
      STATICMETHOD (valueOf,  "valueOf",  "(I)Ljava/lang/Integer;") \
      METHOD (constructor, "<init>",   "(I)V") \
      METHOD (intValue,    "intValue", "()I")

    DECLARE_JNI_CLASS (JavaInteger, "java/lang/Integer")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (hasNext, "hasNext", "()Z") \
      METHOD (next,    "next",    "()Ljava/lang/Object;")

    DECLARE_JNI_CLASS (JavaIterator, "java/util/Iterator")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (get,  "get",  "(I)Ljava/lang/Object;") \
      METHOD (size, "size", "()I")

    DECLARE_JNI_CLASS (JavaList, "java/util/List")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (constructor, "<init>", "(J)V")

    DECLARE_JNI_CLASS (JavaLong, "java/lang/Long")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (get,    "get",    "(Ljava/lang/Object;)Ljava/lang/Object;") \
      METHOD (keySet, "keySet", "()Ljava/util/Set;") \
      METHOD (put,    "put",    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")

    DECLARE_JNI_CLASS (JavaMap, "java/util/Map")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (getName,           "getName",           "()Ljava/lang/String;") \
      METHOD (getModifiers,      "getModifiers",      "()I")            \
      METHOD (getParameterTypes, "getParameterTypes", "()[Ljava/lang/Class;") \
      METHOD (getReturnType,     "getReturnType",     "()Ljava/lang/Class;") \
      METHOD (invoke,            "invoke",            "(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;") \
      METHOD (hashCode,          "hashCode",          "()I") \
      METHOD (equals,            "equals",            "(Ljava/lang/Object;)Z") \

    DECLARE_JNI_CLASS (JavaMethod, "java/lang/reflect/Method")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (constructor, "<init>", "()V") \
      METHOD (getClass, "getClass", "()Ljava/lang/Class;") \
      METHOD (toString, "toString", "()Ljava/lang/String;")

    DECLARE_JNI_CLASS (JavaObject, "java/lang/Object")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (contains, "contains", "(Ljava/lang/Object;)Z") \
      METHOD (iterator, "iterator", "()Ljava/util/Iterator;") \
      METHOD (size,     "size",     "()I")

    DECLARE_JNI_CLASS (JavaSet, "java/util/Set")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (concat,   "concat",   "(Ljava/lang/String;)Ljava/lang/String;") \
      METHOD (getBytes, "getBytes", "()[B")

    DECLARE_JNI_CLASS (JavaString, "java/lang/String")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK)
    DECLARE_JNI_CLASS (AndroidBuild, "android/os/Build")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK)
    DECLARE_JNI_CLASS (AndroidBuildVersion, "android/os/Build$VERSION")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (registerActivityLifecycleCallbacks,   "registerActivityLifecycleCallbacks",   "(Landroid/app/Application$ActivityLifecycleCallbacks;)V") \
     METHOD (unregisterActivityLifecycleCallbacks, "unregisterActivityLifecycleCallbacks", "(Landroid/app/Application$ActivityLifecycleCallbacks;)V")

     DECLARE_JNI_CLASS (AndroidApplication, "android/app/Application")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor,     "<init>",          "(Landroid/content/Context;)V") \
     METHOD (getHolder,       "getHolder",       "()Landroid/view/SurfaceHolder;") \
     METHOD (getParent,       "getParent",       "()Landroid/view/ViewParent;")

     DECLARE_JNI_CLASS (AndroidSurfaceView, "android/view/SurfaceView")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (getSurface,     "getSurface",     "()Landroid/view/Surface;") \
     METHOD (addCallback,    "addCallback",    "(Landroid/view/SurfaceHolder$Callback;)V") \
     METHOD (removeCallback, "removeCallback", "(Landroid/view/SurfaceHolder$Callback;)V")

     DECLARE_JNI_CLASS (AndroidSurfaceHolder, "android/view/SurfaceHolder")
    #undef JNI_CLASS_MEMBERS
    */
}

#[inline] pub fn aloe_string_with_env(
        env: *mut JNIEnv,
        s:   jstring) -> String {
    
    todo!();
    /*
        if (s == nullptr)
                return {};

            const char* const utf8 = env->GetStringUTFChars (s, nullptr);
            CharPointer_UTF8 utf8CP (utf8);
            const String result (utf8CP);
            env->ReleaseStringUTFChars (s, utf8);
            return result;
    */
}

#[inline] pub fn aloe_string(s: jstring) -> String {
    
    todo!();
    /*
        return aloeString (getEnv(), s);
    */
}

#[cfg(target_os="android")]
#[inline] pub fn java_string(s: &String) -> LocalRef<jstring> {
    
    todo!();
    /*
        return LocalRef<jstring> (getEnv()->NewStringUTF (s.toUTF8()));
    */
}

#[cfg(target_os="android")]
#[inline] pub fn java_string_from_char(c: wchar_t) -> LocalRef<jstring> {
    
    todo!();
    /*
        char utf8[8] = { 0 };
            CharPointer_UTF8 (utf8).write (c);
            return LocalRef<jstring> (getEnv()->NewStringUTF (utf8));
    */
}

#[cfg(target_os="android")]
#[inline] pub fn aloe_string_array_to_java(aloe_array: &Vec<String>) -> LocalRef<jobjectArray> {
    
    todo!();
    /*
        auto* env = getEnv();

            LocalRef<jobjectArray> result (env->NewObjectArray ((jsize) aloeArray.size(),
                                                                JavaString,
                                                                javaString ("").get()));

            for (int i = 0; i < aloeArray.size(); ++i)
                env->SetObjectArrayElement (result, i, javaString (aloeArray [i]).get());

            return result;
    */
}

#[cfg(target_os="android")]
#[inline] pub fn java_string_array_to_aloe(java_array: &LocalRef<jobjectArray>) -> Vec<String> {
    
    todo!();
    /*
        if (javaArray.get() == nullptr)
                return {};

            auto* env = getEnv();

            Vec<String> result;

            for (int i = 0; i < env->GetArrayLength (javaArray.get()); ++i)
            {
                LocalRef<jstring> javaString ((jstring) env->GetObjectArrayElement (javaArray.get(), i));
                result.add (aloeString (javaString.get()));
            }

            return result;
    */
}

#[cfg(target_os="android")]
#[inline] pub fn jni_check_has_exception_occurred_and_clear() -> bool {
    
    todo!();
    /*
        auto* env = getEnv();

            LocalRef<jobject> exception (env->ExceptionOccurred());

            if (exception != nullptr)
            {
                env->ExceptionClear();
                return true;
            }

            return false;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_android_JNIHelpers.cpp]

pub const invocationHandleByteCode: &[u8] = &[
    31,139,8,8,215,115,161,94,0,3,105,110,118,111,99,97,116,105,111,110,72,97,110,100,108,101,66,121,116,101,67,111,100,101,46,
    100,101,120,0,109,148,65,107,19,65,20,199,223,236,78,146,90,211,116,77,141,214,88,33,151,130,7,117,91,172,80,73,17,161,32,53,93,
    17,108,233,65,5,217,38,155,102,219,237,110,220,108,99,172,8,173,40,42,244,36,245,226,65,232,165,138,7,15,226,65,193,147,120,244,
    166,130,95,192,155,23,189,21,68,252,207,206,180,137,218,133,223,204,155,247,222,206,123,111,119,230,85,156,86,247,208,201,
    83,84,121,127,155,63,122,245,209,24,205,141,63,156,124,186,176,229,110,12,151,158,157,126,219,76,39,136,234,68,212,154,25,201,
    146,122,38,56,81,158,164,126,15,248,10,160,162,95,128,129,99,24,82,152,71,152,92,123,24,86,116,162,53,204,203,26,209,29,112,
    15,108,128,231,224,37,248,2,126,128,4,252,6,192,56,184,6,102,65,21,220,2,171,224,1,120,2,94,128,215,224,29,248,0,62,129,111,224,
    59,248,169,203,184,72,157,146,36,115,233,82,185,118,131,189,160,7,232,138,171,154,204,95,200,53,77,218,83,170,214,180,146,
    35,77,238,153,139,107,212,99,27,35,141,122,213,218,80,181,239,83,250,108,60,51,234,139,247,213,148,191,68,188,61,13,149,208,142,
    97,24,228,180,99,63,194,69,206,122,44,111,233,50,223,186,33,52,7,32,93,30,2,35,68,25,229,1,95,46,235,140,173,5,97,17,107,153,
    97,154,203,245,212,89,216,17,103,24,33,71,81,141,88,215,135,52,226,44,131,90,121,252,141,250,184,172,109,170,222,233,219,67,83,
    33,234,191,158,186,155,122,156,218,108,38,69,212,52,106,204,210,209,223,180,243,48,53,139,60,214,88,251,219,203,178,116,236,
    223,165,190,117,50,254,15,42,243,49,215,119,163,51,196,74,148,47,45,149,157,243,126,51,40,219,145,27,248,19,182,95,241,156,240,
    196,188,221,180,41,97,149,44,203,34,110,137,113,208,42,7,139,102,184,216,240,204,121,188,98,238,250,94,145,242,86,197,246,154,
    238,130,105,251,126,16,197,54,115,186,22,6,55,26,69,202,90,98,91,211,179,253,57,243,226,236,188,83,142,138,148,235,208,197,126,
    246,172,231,20,17,173,173,14,157,170,7,95,115,215,104,255,187,93,112,162,90,80,41,18,155,33,109,166,68,125,87,118,137,202,237,
    112,174,65,137,178,231,216,33,25,21,183,81,183,163,114,237,156,235,219,158,187,236,80,102,91,35,66,46,56,212,85,221,182,36,93,
    169,73,46,198,81,168,199,71,66,77,103,60,240,35,167,21,145,241,215,242,146,83,165,68,61,12,90,55,137,71,53,23,1,155,182,183,
    132,237,216,193,84,70,203,23,181,185,210,113,156,146,84,102,146,246,99,188,127,153,14,235,253,185,94,72,155,164,105,236,208,0,
    235,231,196,116,113,134,87,87,248,186,174,225,246,50,1,123,163,235,236,179,206,216,138,248,207,198,63,103,65,204,219,61,66,235,
    232,19,122,71,175,224,29,253,34,65,237,158,145,164,118,223,208,13,41,199,231,170,32,223,89,23,62,5,169,23,247,135,25,82,31,223,
    169,130,140,43,250,140,174,252,197,61,226,133,246,253,34,37,15,170,196,133,44,122,218,31,165,24,139,249,12,5,0,0,0,0
];

lazy_static!{
    /*
    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     STATICMETHOD (newProxyInstance, "newProxyInstance", "(Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;") \

     DECLARE_JNI_CLASS (JavaProxy, "java/lang/reflect/Proxy")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor, "<init>", "(J)V") \
     METHOD (clear, "clear", "()V") \
     CALLBACK (aloe_invokeImplementer, "dispatchInvoke", "(JLjava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;") \
     CALLBACK (aloe_dispatchDelete, "dispatchFinalize", "(J)V")

     DECLARE_JNI_CLASS_WITH_BYTECODE (AloeInvocationHandler, "com/rmsl/aloe/AloeInvocationHandler", 10, invocationHandleByteCode, sizeof (invocationHandleByteCode))
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD       (findClass,            "findClass",            "(Ljava/lang/String;)Ljava/lang/Class;") \
     STATICMETHOD (getSystemClassLoader, "getSystemClassLoader", "()Ljava/lang/ClassLoader;")

     DECLARE_JNI_CLASS (JavaClassLoader, "java/lang/ClassLoader")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor, "<init>", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/ClassLoader;)V")

     DECLARE_JNI_CLASS (AndroidDexClassLoader, "dalvik/system/DexClassLoader")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (constructor, "<init>", "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V")

     DECLARE_JNI_CLASS_WITH_MIN_SDK (AndroidInMemoryDexClassLoader, "dalvik/system/InMemoryDexClassLoader", 26)
    #undef JNI_CLASS_MEMBERS
    */
}

/**
  | This function takes ownership of the
  | implementer. When the returned GlobalRef goes
  | out of scope (and no other Java routine has
  | a reference on the return-value) then the
  | implementer will be deleted as well.
  */
#[cfg(target_os="android")]
pub fn create_java_interface(
        implementer:     *mut AndroidInterfaceImplementer,
        interface_names: &Vec<String>,
        subclass:        LocalRef<jobject>) -> LocalRef<jobject> {
    
    todo!();
    /*
        auto* env = getEnv();

        implementer->javaSubClass = GlobalRef (subclass);

        // you need to override at least one interface
        jassert (interfaceNames.size() > 0);

        auto classArray = LocalRef<jobject> (env->NewObjectArray (interfaceNames.size(), JavaClass, nullptr));
        LocalRef<jobject> classLoader;

        for (auto i = 0; i < interfaceNames.size(); ++i)
        {
            auto aClass = LocalRef<jobject> (env->FindClass (interfaceNames[i].toRawUTF8()));

            if (aClass != nullptr)
            {
                if (i == 0)
                    classLoader = LocalRef<jobject> (env->CallObjectMethod (aClass, JavaClass.getClassLoader));

                env->SetObjectArrayElement ((jobjectArray) classArray.get(), i, aClass);
            }
            else
            {
                // interface class not found
                jassertfalse;
            }
        }

        auto invocationHandler = LocalRef<jobject> (env->NewObject (AloeInvocationHandler, AloeInvocationHandler.constructor,
                                                                    reinterpret_cast<jlong> (implementer)));

        // CreateJavaInterface() is expected to be called just once for a given implementer
        jassert (implementer->invocationHandler == nullptr);

        implementer->invocationHandler = GlobalRef (invocationHandler);

        return LocalRef<jobject> (env->CallStaticObjectMethod (JavaProxy, JavaProxy.newProxyInstance,
                                                               classLoader.get(), classArray.get(),
                                                               invocationHandler.get()));
    */
}

#[cfg(target_os="android")]
pub fn create_java_interface_with_names(
        implementer:     *mut AndroidInterfaceImplementer,
        interface_names: &Vec<String>) -> LocalRef<jobject> {
    
    todo!();
    /*
        return CreateJavaInterface (implementer, interfaceNames,
                                    LocalRef<jobject> (getEnv()->NewObject (JavaObject,
                                                                            JavaObject.constructor)));
    */
}

#[cfg(target_os="android")]
pub fn create_java_interface_with_name(
        implementer:    *mut AndroidInterfaceImplementer,
        interface_name: &String) -> LocalRef<jobject> {
    
    todo!();
    /*
        return CreateJavaInterface (implementer, Vec<String> (interfaceName));
    */
}

pub fn aloe_invoke_implementer(
        _0:     *mut JNIEnv,
        object: jobject,
        host:   i64,
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject {
    
    todo!();
    /*
        if (auto* myself = reinterpret_cast<AndroidInterfaceImplementer*> (host))
            return myself->invoke (proxy, method, args);

        return nullptr;
    */
}

pub fn aloe_dispatch_delete(
        _0:     *mut JNIEnv,
        object: jobject,
        host:   i64)  {
    
    todo!();
    /*
        if (auto* myself = reinterpret_cast<AndroidInterfaceImplementer*> (host))
            delete myself;
    */
}

pub fn get_android_sdk_version() -> i32 {
    
    todo!();
    /*
        // this is used so often that we need to cache this
        static int sdkVersion = []
        {
            // don't use any jni helpers as they might not have been initialised yet
            // when this method is used
            auto* env = getEnv();

            auto buildVersion = env->FindClass ("android/os/Build$VERSION");
            jassert (buildVersion != nullptr);

            auto sdkVersionField = env->GetStaticFieldID (buildVersion, "SDK_INT", "I");
            jassert (sdkVersionField != nullptr);

            return env->GetStaticIntField (buildVersion, sdkVersionField);
        }();

        return sdkVersion;
    */
}

pub fn is_permission_declared_in_manifest(requested_permission: &String) -> bool {
    
    todo!();
    /*
        auto* env = getEnv();

        LocalRef<jobject> pkgManager (env->CallObjectMethod (getAppContext().get(), AndroidContext.getPackageManager));
        LocalRef<jobject> pkgName (env->CallObjectMethod (getAppContext().get(), AndroidContext.getPackageName));
        LocalRef<jobject> pkgInfo (env->CallObjectMethod (pkgManager.get(), AndroidPackageManager.getPackageInfo,
                                                          pkgName.get(), 0x00001000 /* PERMISSIONS */));

        LocalRef<jobjectArray> permissions ((jobjectArray) env->GetObjectField (pkgInfo.get(), AndroidPackageInfo.requestedPermissions));
        int n = env->GetArrayLength (permissions);

        for (int i = 0; i < n; ++i)
        {
            LocalRef<jstring> jstr ((jstring) env->GetObjectArrayElement (permissions, i));
            String permissionId (aloeString (jstr));

            if (permissionId == requestedPermission)
                return true;
        }

        return false;
    */
}

/**
  | This byte-code is generated from
  | native/java/com/rmsl/aloe/FragmentOverlay.java
  | with min sdk version 16
  |
  | See aloe_core/native/java/README.txt on how to
  | generate this byte-code.
  */
pub const javaFragmentOverlay: &[u8] = &[
    31,139,8,8,26,116,161,94,0,3,106,97,118,97,70,114,97,103,109,101,110,116,79,118,101,114,108,97,121,46,100,101,120,0,133,149,
    77,136,28,69,20,199,255,53,253,181,159,179,147,221,184,140,235,198,140,43,70,197,224,172,104,36,56,99,216,152,32,204,100,226,71,
    54,204,97,227,165,153,105,39,189,206,118,79,186,123,150,4,20,53,4,146,131,8,6,252,130,28,114,80,65,48,8,226,65,196,83,8,66,64,
    65,146,75,252,184,152,179,160,160,4,17,5,255,175,187,58,27,150,136,195,252,250,189,122,245,234,189,170,215,213,85,93,239,248,
    216,226,163,187,96,79,85,156,198,103,91,86,175,30,189,252,253,193,79,203,15,189,242,199,245,246,129,179,245,238,53,27,24,0,56,
    222,126,108,26,250,183,155,182,7,145,217,199,200,86,149,201,58,37,255,248,156,143,18,229,87,186,93,47,0,47,155,192,11,148,87,
    12,224,7,242,27,249,157,220,32,127,145,127,200,93,244,217,69,154,228,37,242,42,57,73,206,144,55,201,89,242,62,57,79,62,36,31,
    147,11,228,34,185,76,174,144,107,228,103,242,43,249,147,216,22,80,38,139,228,9,210,36,47,146,51,228,45,114,158,92,32,95,146,175,
    201,183,132,211,4,167,3,46,19,14,25,33,163,122,173,227,100,70,214,76,24,62,93,223,41,58,91,186,13,237,227,104,125,66,235,111,
    208,103,82,235,239,81,47,106,253,3,234,83,90,255,196,200,234,38,250,23,212,183,104,253,18,245,105,173,127,147,230,82,152,133,
    204,179,144,230,40,112,118,119,235,246,130,158,199,28,196,47,235,23,121,135,150,101,100,227,239,76,165,129,249,84,218,216,150,
    202,44,142,197,21,111,79,165,137,74,42,29,220,163,199,47,164,210,194,189,200,214,172,0,157,37,211,229,55,98,103,210,160,69,108,
    87,173,172,134,131,146,248,202,204,87,42,82,129,188,255,71,221,159,247,4,37,155,126,69,214,209,76,223,193,117,43,91,255,50,55,
    220,44,147,61,194,48,187,217,187,28,177,38,199,212,41,245,182,243,209,186,61,202,88,69,200,72,89,255,47,28,35,107,10,43,10,135,
    25,209,161,117,2,115,106,22,65,197,96,149,199,177,178,196,136,75,183,70,116,210,246,96,137,121,159,47,166,239,49,203,127,227,
    127,242,59,105,254,201,52,191,212,86,246,142,12,148,247,23,150,100,62,183,205,179,56,5,83,21,117,221,108,189,231,160,101,166,
    143,166,117,81,154,124,191,73,111,174,139,71,33,213,77,237,99,215,253,192,79,246,96,235,211,145,219,91,243,130,228,217,117,47,
    234,187,39,30,94,117,215,93,168,6,84,19,133,102,11,170,133,249,150,27,116,163,208,239,86,221,193,160,186,223,119,251,97,47,31,
    85,67,249,102,111,39,12,18,154,170,141,84,212,48,115,179,39,140,171,79,13,131,110,223,171,97,123,171,19,174,85,163,181,184,95,
    93,29,118,188,234,166,244,53,76,183,100,6,213,190,27,244,170,203,73,228,7,189,26,84,27,102,187,209,104,201,179,213,66,161,221,
    132,213,110,138,65,4,45,70,187,41,102,114,164,129,153,35,183,9,97,117,250,97,236,193,233,12,6,135,143,250,49,204,174,155,184,
    112,186,126,188,230,199,49,38,122,94,178,55,234,13,101,42,49,28,182,90,97,208,163,57,114,131,228,144,23,15,251,52,151,194,96,
    111,39,241,215,253,228,68,102,194,236,102,203,51,46,91,30,70,194,96,95,228,185,137,135,98,174,233,158,185,48,56,228,29,27,122,
    113,242,156,23,73,106,63,12,98,29,173,242,223,125,122,180,19,6,203,137,27,37,152,212,138,182,143,15,54,6,96,60,202,130,236,11,
    187,30,198,162,116,124,170,91,113,34,83,50,19,41,192,54,56,197,194,206,26,246,83,30,168,99,143,177,227,254,178,83,60,253,14,22,
    212,3,78,177,126,233,244,10,30,55,118,220,55,79,219,187,216,73,167,39,105,129,178,248,121,155,175,191,102,254,100,90,39,121,
    146,220,130,165,254,54,13,117,206,42,168,239,200,57,155,210,158,220,244,205,139,204,239,4,217,143,249,189,96,96,227,110,200,247,
    172,220,15,114,118,228,119,132,141,141,123,66,85,178,182,220,21,170,148,157,11,114,190,22,42,89,124,185,63,12,237,35,231,138,
    28,80,42,63,115,74,153,46,247,211,191,81,33,150,205,216,6,0,0,0,0
    ];

lazy_static!{
    /*
    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (construct,   "<init>",   "()V") \
     METHOD (close,       "close",    "()V") \
     CALLBACK (FragmentOverlay::onActivityResultNative, "onActivityResultNative", "(JIILandroid/content/Intent;)V") \
     CALLBACK (FragmentOverlay::onCreateNative,         "onCreateNative",         "(JLandroid/os/Bundle;)V") \
     CALLBACK (FragmentOverlay::onStartNative,          "onStartNative",          "(J)V") \
     CALLBACK (FragmentOverlay::onRequestPermissionsResultNative, "onRequestPermissionsResultNative", "(JI[Ljava/lang/String;[I)V")

     DECLARE_JNI_CLASS_WITH_BYTECODE (AloeFragmentOverlay, "com/rmsl/aloe/FragmentOverlay", 16, javaFragmentOverlay, sizeof(javaFragmentOverlay))
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (show,   "show",   "(Landroid/app/FragmentManager;Ljava/lang/String;)V")

     DECLARE_JNI_CLASS (AndroidDialogFragment, "android/app/DialogFragment")
    #undef JNI_CLASS_MEMBERS
    */
}

/**
  | Allows you to start an activity without
  | requiring to have an activity
  |
  */
#[cfg(target_os="android")]
pub fn start_android_activity_for_result(
        intent:       &LocalRef<jobject>,
        request_code: i32,
        callback:     fn(
                _0: i32,
                _1: i32,
                _2: LocalRef<jobject>
        ) -> ())  {
    
    todo!();
    /*
        auto* activityLauncher = new ActivityLauncher (intent, requestCode, std::move (callback));
        activityLauncher->open();
    */
}

pub fn android_has_system_feature(property: &String) -> bool {
    
    todo!();
    /*
        LocalRef<jobject> appContext (getAppContext());

        if (appContext != nullptr)
        {
            auto* env = getEnv();

            LocalRef<jobject> packageManager (env->CallObjectMethod (appContext.get(), AndroidContext.getPackageManager));

            if (packageManager != nullptr)
                return env->CallBooleanMethod (packageManager.get(),
                                               AndroidPackageManager.hasSystemFeature,
                                               javaString (property).get()) != 0;
        }

        // unable to get app's context
        jassertfalse;
        return false;
    */
}

pub fn audio_manager_get_property(property: &String) -> String {
    
    todo!();
    /*
        if (getAndroidSDKVersion() >= 17)
        {
            auto* env = getEnv();
            LocalRef<jobject> audioManager (env->CallObjectMethod (getAppContext().get(), AndroidContext.getSystemService,
                                                                   javaString ("audio").get()));

            if (audioManager != nullptr)
            {
                LocalRef<jstring> jProperty (javaString (property));

                auto methodID = env->GetMethodID (AndroidAudioManager, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;");

                if (methodID != nullptr)
                    return aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (audioManager.get(),
                                                                                           methodID,
                                                                                           javaString (property).get())));
            }
        }

        return {};
    */
}
