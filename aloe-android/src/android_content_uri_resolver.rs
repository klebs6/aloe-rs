crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidContentUriResolver {

}

#[cfg(target_os="android")]
impl AndroidContentUriResolver {

    pub fn get_stream_for_content_uri(
        url:          &Url,
        input_stream: bool) -> LocalRef<jobject> {
        
        todo!();
        /*
            // only use this method for content URIs
            jassert (url.getScheme() == "content");
            auto* env = getEnv();

            LocalRef<jobject> contentResolver (env->CallObjectMethod (getAppContext().get(), AndroidContext.getContentResolver));

            if (contentResolver)
                return LocalRef<jobject> ((env->CallObjectMethod (contentResolver.get(),
                                                                  inputStream ? ContentResolver.openInputStream
                                                                              : ContentResolver.openOutputStream,
                                                                  urlToUri (url).get())));

            return LocalRef<jobject>();
        */
    }
    
    pub fn get_local_file_from_content_uri(url: &Url) -> File {
        
        todo!();
        /*
            // only use this method for content URIs
            jassert (url.getScheme() == "content");

            auto authority  = url.getDomain();
            auto documentId = Url::removeEscapeChars (url.getSubPath().fromFirstOccurrenceOf ("/", false, false));
            auto tokens = Vec<String>::fromTokens (documentId, ":", "");

            if (authority == "com.android.externalstorage.documents")
            {
                auto storageId  = tokens[0];
                auto subpath    = tokens[1];

                auto storagePath = getStorageDevicePath (storageId);

                if (storagePath != File())
                    return storagePath.getChildFile (subpath);
            }
            else if (authority == "com.android.providers.downloads.documents")
            {
                auto type       = tokens[0];
                auto downloadId = tokens[1];

                if (type.equalsIgnoreCase ("raw"))
                {
                    return File (downloadId);
                }
                else if (type.equalsIgnoreCase ("downloads"))
                {
                    auto subDownloadPath = url.getSubPath().fromFirstOccurrenceOf ("tree/downloads", false, false);
                    return File (getWellKnownFolder ("DIRECTORY_DOWNLOADS").getFullPathName() + "/" + subDownloadPath);
                }
                else
                {
                    return getLocalFileFromContentUri (Url ("content://downloads/public_downloads/" + documentId));
                }
            }
            else if (authority == "com.android.providers.media.documents" && documentId.isNotEmpty())
            {
                auto type    = tokens[0];
                auto mediaId = tokens[1];

                if (type == "image")
                    type = "images";

                return getCursorDataColumn (Url ("content://media/external/" + type + "/media"),
                                            "_id=?", Vec<String> {mediaId});
            }

            return getCursorDataColumn (url);
        */
    }
    
    pub fn get_file_name_from_content_uri(url: &Url) -> String {
        
        todo!();
        /*
            auto uri = urlToUri (url);
            auto* env = getEnv();
            LocalRef<jobject> contentResolver (env->CallObjectMethod (getAppContext().get(), AndroidContext.getContentResolver));

            if (contentResolver == nullptr)
                return {};

            auto filename = getStringUsingDataColumn ("_display_name", env, uri, contentResolver);

            // Fallback to "_data" column
            if (filename.isEmpty())
            {
                auto path = getStringUsingDataColumn ("_data", env, uri, contentResolver);
                filename = path.fromLastOccurrenceOf ("/", false, true);
            }

            return filename;
        */
    }
    
    pub fn get_cursor_data_column(
        url:            &Url,
        selection:      &String,
        selection_args: &Vec<String>) -> String {
        
        todo!();
        /*
            auto uri = urlToUri (url);
            auto* env = getEnv();
            LocalRef<jobject> contentResolver (env->CallObjectMethod (getAppContext().get(), AndroidContext.getContentResolver));

            if (contentResolver)
            {
                LocalRef<jstring> columnName (javaString ("_data"));
                LocalRef<jobjectArray> projection (env->NewObjectArray (1, JavaString, columnName.get()));

                LocalRef<jobjectArray> args;

                if (selection.isNotEmpty())
                {
                    args = LocalRef<jobjectArray> (env->NewObjectArray (selectionArgs.size(), JavaString, javaString ("").get()));

                    for (int i = 0; i < selectionArgs.size(); ++i)
                        env->SetObjectArrayElement (args.get(), i, javaString (selectionArgs[i]).get());
                }

                LocalRef<jstring> jSelection (selection.isNotEmpty() ? javaString (selection) : LocalRef<jstring>());
                LocalRef<jobject> cursor (env->CallObjectMethod (contentResolver.get(), ContentResolver.query,
                                                                 uri.get(), projection.get(), jSelection.get(),
                                                                 args.get(), nullptr));

                if (jniCheckHasExceptionOccurredAndClear())
                {
                    // An exception has occurred, have you acquired RuntimePermission::readExternalStorage permission?
                    jassertfalse;
                    return {};
                }

                if (cursor)
                {
                    if (env->CallBooleanMethod (cursor.get(), AndroidCursor.moveToFirst) != 0)
                    {
                        auto columnIndex = env->CallIntMethod (cursor.get(), AndroidCursor.getColumnIndex, columnName.get());

                        if (columnIndex >= 0)
                        {
                            LocalRef<jstring> value ((jstring) env->CallObjectMethod (cursor.get(), AndroidCursor.getString, columnIndex));

                            if (value)
                                return aloeString (value.get());
                        }
                    }

                    env->CallVoidMethod (cursor.get(), AndroidCursor.close);
                }
            }

            return {};
        */
    }
    
    pub fn get_storage_device_path(storage_id: &String) -> File {
        
        todo!();
        /*
            // check for the primary alias
            if (storageId == "primary")
                return getPrimaryStorageDirectory();

            auto storageDevices = getSecondaryStorageDirectories();

            for (auto storageDevice : storageDevices)
                if (getStorageIdForMountPoint (storageDevice) == storageId)
                    return storageDevice;

            return {};
        */
    }
    
    pub fn get_primary_storage_directory() -> File {
        
        todo!();
        /*
            auto* env = getEnv();
            return aloeFile (LocalRef<jobject> (env->CallStaticObjectMethod (AndroidEnvironment, AndroidEnvironment.getExternalStorageDirectory)));
        */
    }
    
    pub fn get_secondary_storage_directories() -> Vec<File> {
        
        todo!();
        /*
            Vec<File> results;

            if (getAndroidSDKVersion() >= 19)
            {
                auto* env = getEnv();
                static jmethodID m = (env->GetMethodID (AndroidContext, "getExternalFilesDirs",
                                                        "(Ljava/lang/String;)[Ljava/io/File;"));
                if (m == nullptr)
                    return {};

                auto paths = convertFileArray (LocalRef<jobject> (env->CallObjectMethod (getAppContext().get(), m, nullptr)));

                for (auto path : paths)
                    results.add (getMountPointForFile (path));
            }
            else
            {
                // on older SDKs other external storages are located "next" to the primary
                // storage mount point
                auto mountFolder = getMountPointForFile (getPrimaryStorageDirectory())
                                        .getParentDirectory();

                // don't include every folder. Only folders which are actually mountpoints
                aloe_statStruct info;
                if (! aloe_stat (mountFolder.getFullPathName(), info))
                    return {};

                auto rootFsDevice = info.st_dev;

                for (const auto& iter : RangedDirectoryIterator (mountFolder, false, "*", File::findDirectories))
                {
                    auto candidate = iter.getFile();

                    if (aloe_stat (candidate.getFullPathName(), info)
                          && info.st_dev != rootFsDevice)
                        results.add (candidate);
                }

            }

            return results;
        */
    }
    
    pub fn get_storage_id_for_mount_point(mountpoint: &File) -> String {
        
        todo!();
        /*
            // currently this seems to work fine, but something
            // more intelligent may be needed in the future
            return mountpoint.getFileName();
        */
    }
    
    pub fn get_mount_point_for_file(file: &File) -> File {
        
        todo!();
        /*
            aloe_statStruct info;

            if (aloe_stat (file.getFullPathName(), info))
            {
                auto dev  = info.st_dev;
                File mountPoint = file;

                for (;;)
                {
                    auto parent = mountPoint.getParentDirectory();

                    if (parent == mountPoint)
                        break;

                    aloe_stat (parent.getFullPathName(), info);

                    if (info.st_dev != dev)
                        break;

                    mountPoint = parent;
                }

                return mountPoint;
            }

            return {};
        */
    }
    
    pub fn convert_file_array(obj: LocalRef<jobject>) -> Vec<File> {
        
        todo!();
        /*
            auto* env = getEnv();
            int n = (int) env->GetArrayLength ((jobjectArray) obj.get());
            Vec<File> files;

            for (int i = 0; i < n; ++i)
                files.add (aloeFile (LocalRef<jobject> (env->GetObjectArrayElement ((jobjectArray) obj.get(),
                                                                                     (jsize) i))));

            return files;
        */
    }
    
    pub fn get_string_using_data_column(
        column_name_to_use: &String,
        env:                *mut JNIEnv,
        uri:                &LocalRef<jobject>,
        content_resolver:   &LocalRef<jobject>) -> String {
        
        todo!();
        /*
            LocalRef<jstring> columnName (javaString (columnNameToUse));
            LocalRef<jobjectArray> projection (env->NewObjectArray (1, JavaString, columnName.get()));

            LocalRef<jobject> cursor (env->CallObjectMethod (contentResolver.get(), ContentResolver.query,
                                                             uri.get(), projection.get(), nullptr,
                                                             nullptr, nullptr));

            if (jniCheckHasExceptionOccurredAndClear())
            {
                // An exception has occurred, have you acquired RuntimePermission::readExternalStorage permission?
                jassertfalse;
                return {};
            }

            if (cursor == nullptr)
                return {};

            String fileName;

            if (env->CallBooleanMethod (cursor.get(), AndroidCursor.moveToFirst) != 0)
            {
                auto columnIndex = env->CallIntMethod (cursor.get(), AndroidCursor.getColumnIndex, columnName.get());

                if (columnIndex >= 0)
                {
                    LocalRef<jstring> value ((jstring) env->CallObjectMethod (cursor.get(), AndroidCursor.getString, columnIndex));

                    if (value)
                        fileName = aloeString (value.get());

                }
            }

            env->CallVoidMethod (cursor.get(), AndroidCursor.close);

            return fileName;
        */
    }
}
