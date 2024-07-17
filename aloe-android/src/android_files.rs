crate::ix!();

#[cfg(target_os="android")]
pub fn aloe_file(obj: LocalRef<jobject>) -> File {
    
    todo!();
    /*
        auto* env = getEnv();

        if (env->IsInstanceOf (obj.get(), JavaFile) != 0)
            return File (aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (obj.get(),
                                                                                         JavaFile.getAbsolutePath))));

        return {};
    */
}

#[cfg(target_os="android")]
pub fn get_well_known_folder(folder_id: *const u8) -> File {
    
    todo!();
    /*
        auto* env = getEnv();
        auto fieldId = env->GetStaticFieldID (AndroidEnvironment, folderId, "Ljava/lang/String;");

        if (fieldId == nullptr)
        {
            // unknown field in environment
            jassertfalse;
            return {};
        }

        LocalRef<jobject> fieldValue (env->GetStaticObjectField (AndroidEnvironment, fieldId));

        if (fieldValue == nullptr)
            return {};

        LocalRef<jobject> downloadFolder (env->CallStaticObjectMethod (AndroidEnvironment,
                                                                       AndroidEnvironment.getExternalStoragePublicDirectory,
                                                                       fieldValue.get()));

        return (downloadFolder ? aloeFile (downloadFolder) : File());
    */
}

#[cfg(target_os="android")]
pub fn url_to_uri(url: &Url) -> LocalRef<jobject> {
    
    todo!();
    /*
        return LocalRef<jobject> (getEnv()->CallStaticObjectMethod (AndroidUri, AndroidUri.parse,
                                                                    javaString (url.toString (true)).get()));
    */
}

#[cfg(target_os="android")]
pub fn aloe_create_content_uri_output_stream<W: Write>(url: &Url) -> *mut W {
    
    todo!();
    /*
        auto stream = AndroidContentUriResolver::getStreamForContentUri (url, false);

        return (stream.get() != nullptr ? new AndroidContentUriOutputStream (std::move (stream)) : nullptr);
    */
}


#[cfg(target_os="android")]
pub fn get_documents_directory() -> File {
    
    todo!();
    /*
        auto* env = getEnv();

        if (getAndroidSDKVersion() >= 19)
            return getWellKnownFolder ("DIRECTORY_DOCUMENTS");

        return aloeFile (LocalRef<jobject> (env->CallStaticObjectMethod (AndroidEnvironment, AndroidEnvironment.getDataDirectory)));
    */
}

#[cfg(target_os="android")]
pub fn get_app_data_dir(data_dir: bool) -> File {
    
    todo!();
    /*
        auto* env = getEnv();

        LocalRef<jobject> applicationInfo (env->CallObjectMethod (getAppContext().get(), AndroidContext.getApplicationInfo));
        LocalRef<jobject> jString (env->GetObjectField (applicationInfo.get(), dataDir ? AndroidApplicationInfo.dataDir : AndroidApplicationInfo.publicSourceDir));

        return  {aloeString ((jstring) jString.get())};
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_android_Files.cpp]

lazy_static!{
    /*
    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
      METHOD (constructor, "<init>",     "(Landroid/content/Context;Landroid/media/MediaScannerConnection$MediaScannerConnectionClient;)V") \
      METHOD (connect,     "connect",    "()V") \
      METHOD (disconnect,  "disconnect", "()V") \
      METHOD (scanFile,    "scanFile",   "(Ljava/lang/String;Ljava/lang/String;)V") \

    DECLARE_JNI_CLASS (MediaScannerConnection, "android/media/MediaScannerConnection")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (query,            "query",            "(Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;)Landroid/database/Cursor;") \
     METHOD (openInputStream,  "openInputStream",  "(Landroid/net/Uri;)Ljava/io/InputStream;") \
     METHOD (openOutputStream, "openOutputStream", "(Landroid/net/Uri;)Ljava/io/OutputStream;")

    DECLARE_JNI_CLASS (ContentResolver, "android/content/ContentResolver")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (moveToFirst,     "moveToFirst",     "()Z") \
     METHOD (getColumnIndex,  "getColumnIndex",  "(Ljava/lang/String;)I") \
     METHOD (getString,       "getString",       "(I)Ljava/lang/String;") \
     METHOD (close,           "close",           "()V") \

    DECLARE_JNI_CLASS (AndroidCursor, "android/database/Cursor")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     STATICMETHOD (getExternalStorageDirectory, "getExternalStorageDirectory", "()Ljava/io/File;") \
     STATICMETHOD (getExternalStoragePublicDirectory, "getExternalStoragePublicDirectory", "(Ljava/lang/String;)Ljava/io/File;") \
     STATICMETHOD (getDataDirectory, "getDataDirectory", "()Ljava/io/File;")

    DECLARE_JNI_CLASS (AndroidEnvironment, "android/os/Environment")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (close, "close", "()V") \
     METHOD (flush, "flush", "()V") \
     METHOD (write, "write", "([BII)V")

    DECLARE_JNI_CLASS (AndroidOutputStream, "java/io/OutputStream")
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     FIELD (publicSourceDir, "publicSourceDir", "Ljava/lang/String;") \
     FIELD (dataDir, "dataDir", "Ljava/lang/String;")

    DECLARE_JNI_CLASS (AndroidApplicationInfo, "android/content/pm/ApplicationInfo")
    #undef JNI_CLASS_MEMBERS
    */
}
