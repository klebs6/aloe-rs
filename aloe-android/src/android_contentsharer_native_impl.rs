crate::ix!();

pub struct ContentUriElements
{
    index:    String,
    filename: String,
    filepath: String,
}

#[cfg(target_os="android")]
pub struct ContentSharerNativeImpl<'a> {

    base4:                              AsyncUpdater<'a>,
    base5:                              Timer,
    owner:                              &'a mut ContentSharer,
    package_name:                       String,
    uri_base:                           String,
    prepare_files_thread:               Box<AndroidContentSharerPrepareFilesThread<'a>>,
    succeeded:                          bool, // default = false
    error_description:                  String,
    sharing_activity_did_finish:        bool, // default = false
    cursors:                            Vec<Box<AndroidContentSharerCursor<'a>>>,
    asset_file_descriptors:             Vec<GlobalRef>,
    non_asset_file_open_lock:           CriticalSection,
    non_asset_file_paths_pending_share: Vec<String>,
    non_asset_files_pending_share:      Atomic<i32>, // default = { 0  }
    non_asset_file_observers:           Vec<Box<AndroidContentSharerFileObserver<'a>>>,
    master_reference:                   WeakReferenceMaster<ContentSharerNativeImpl<'a>>,
}

#[cfg(target_os="android")]
impl<'a> AndroidContentSharerCursorOwner for ContentSharerNativeImpl<'a> {

}

#[cfg(target_os="android")]
impl<'a> AndroidContentSharerFileObserverOwner for ContentSharerNativeImpl<'a> {

}

#[cfg(target_os="android")]
impl<'a> ContentSharerPimpl for ContentSharerNativeImpl<'a> {

}

#[cfg(target_os="android")]
impl<'a> Drop for ContentSharerNativeImpl<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            masterReference.clear();
         */
    }
}

#[cfg(target_os="android")]
#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
             CALLBACK (contentSharerQuery,          "contentSharerQuery",          "(Landroid/net/Uri;[Ljava/lang/String;)Landroid/database/Cursor;") 
             CALLBACK (contentSharerOpenFile,       "contentSharerOpenFile",       "(Landroid/net/Uri;Ljava/lang/String;)Landroid/content/res/AssetFileDescriptor;") 
             CALLBACK (contentSharerGetStreamTypes, "contentSharerGetStreamTypes", "(Landroid/net/Uri;Ljava/lang/String;)[Ljava/lang/String;") 
        */
    }
}

#[cfg(target_os="android")]
#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    AloeSharingContentProvider, "com/rmsl/aloe/AloeSharingContentProvider", 16
}

// #undef JNI_CLASS_MEMBERS

#[cfg(target_os="android")]
#[cfg(target_os="android")]
impl<'a> ContentSharerNativeImpl<'a> {

    pub fn new(cs: &mut ContentSharer) -> Self {
    
        todo!();
        /*


            : owner (cs),
              packageName (aloeString (LocalRef<jstring> ((jstring) getEnv()->CallObjectMethod (getAppContext().get(), AndroidContext.getPackageName)))),
              uriBase ("content://" + packageName + ".sharingcontentprovider/")
        */
    }
    
    pub fn share_files(&mut self, files: &[Url])  {
        
        todo!();
        /*
            if (! isContentSharingEnabled())
            {
                // You need to enable "Content Sharing" in Proaloer's Android exporter.
                jassertfalse;
                owner.sharingFinished (false, {});
            }

            prepareFilesThread.reset (new AndroidContentSharerPrepareFilesThread (*this, files, packageName, uriBase));
        */
    }
    
    pub fn share_text(&mut self, text: &String)  {
        
        todo!();
        /*
            if (! isContentSharingEnabled())
            {
                // You need to enable "Content Sharing" in Proaloer's Android exporter.
                jassertfalse;
                owner.sharingFinished (false, {});
            }

            auto* env = getEnv();

            auto intent = LocalRef<jobject> (env->NewObject (AndroidIntent, AndroidIntent.constructor));
            env->CallObjectMethod (intent, AndroidIntent.setAction,
                                   javaString ("android.intent.action.SEND").get());
            env->CallObjectMethod (intent, AndroidIntent.putExtra,
                                   javaString ("android.intent.extra.TEXT").get(),
                                   javaString (text).get());
            env->CallObjectMethod (intent, AndroidIntent.setType, javaString ("text/plain").get());

            auto chooserIntent = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidIntent, AndroidIntent.createChooser,
                                                                                 intent.get(), javaString ("Choose share target").get()));

            startAndroidActivityForResult (chooserIntent, 1003,
                                           [weakRef = WeakReference<ContentSharerNativeImpl> { this }] (int /*requestCode*/,
                                                                                                        int resultCode,
                                                                                                        LocalRef<jobject> /*intentData*/) mutable
                                           {
                                               if (weakRef != nullptr)
                                                   weakRef->sharingFinished (resultCode);
                                           });
        */
    }
    
    pub fn cursor_closed(&mut self, cursor: &AndroidContentSharerCursor)  {
        
        todo!();
        /*
            cursors.removeObject (&cursor);
        */
    }
    
    pub fn file_handle_closed(&mut self, _0: &AndroidContentSharerFileObserver)  {
        
        todo!();
        /*
            decrementPendingFileCountAndNotifyOwnerIfReady();
        */
    }
    
    pub fn open_file(&mut self, 
        content_provider: &LocalRef<jobject>,
        uri:              &LocalRef<jobject>,
        mode:             &LocalRef<jstring>) -> jobject {
        
        todo!();
        /*
            ignoreUnused (mode);

            WeakReference<ContentSharerNativeImpl> weakRef (this);

            if (weakRef == nullptr)
                return nullptr;

            auto* env = getEnv();

            auto uriElements = getContentUriElements (env, uri);

            if (uriElements.filepath.isEmpty())
                return nullptr;

            return getAssetFileDescriptor (env, contentProvider, uriElements.filepath);
        */
    }
    
    pub fn query(&mut self, 
        content_provider: &LocalRef<jobject>,
        uri:              &LocalRef<jobject>,
        projection:       &LocalRef<jobjectArray>) -> jobject {
        
        todo!();
        /*
            Vec<String> requestedColumns = javaStringArrayToAloe (projection);
            Vec<String> supportedColumns = getSupportedColumns();

            Vec<String> resultColumns;

            for (const auto& col : supportedColumns)
            {
                if (requestedColumns.contains (col))
                    resultColumns.add (col);
            }

            // Unsupported columns were queried, file sharing may fail.
            if (resultColumns.isEmpty())
                return nullptr;

            auto resultJavaColumns = aloeStringArrayToJava (resultColumns);

            auto* env = getEnv();

            auto cursor = cursors.add (new AndroidContentSharerCursor (*this, env, contentProvider,
                                                                       resultJavaColumns));

            auto uriElements = getContentUriElements (env, uri);

            if (uriElements.filepath.isEmpty())
                return cursor->getNativeCursor();

            auto values = LocalRef<jobjectArray> (env->NewObjectArray ((jsize) resultColumns.size(),
                                                                       JavaObject, nullptr));

            for (int i = 0; i < resultColumns.size(); ++i)
            {
                if (resultColumns.getReference (i) == "_display_name")
                {
                    env->SetObjectArrayElement (values, i, javaString (uriElements.filename).get());
                }
                else if (resultColumns.getReference (i) == "_size")
                {
                    auto javaFile = LocalRef<jobject> (env->NewObject (JavaFile, JavaFile.constructor,
                                                                       javaString (uriElements.filepath).get()));

                    jlong fileLength = env->CallLongMethod (javaFile, JavaFile.length);

                    env->SetObjectArrayElement (values, i, env->NewObject (JavaLong,
                                                                           JavaLong.constructor,
                                                                           fileLength));
                }
            }

            cursor->addRow (values);
            return cursor->getNativeCursor();
        */
    }
    
    pub fn get_stream_types(&mut self, 
        uri:              &LocalRef<jobject>,
        mime_type_filter: &LocalRef<jstring>) -> jobjectArray {
        
        todo!();
        /*
            auto* env = getEnv();

            auto extension = getContentUriElements (env, uri).filename.fromLastOccurrenceOf (".", false, true);

            if (extension.isEmpty())
                return nullptr;

            return aloeStringArrayToJava (filterMimeTypes (getMimeTypesForFileExtension (extension),
                                                                      aloeString (mimeTypeFilter.get())));
        */
    }
    
    pub fn sharing_finished(&mut self, result_code: i32)  {
        
        todo!();
        /*
            sharingActivityDidFinish = true;

            succeeded = resultCode == -1;

            // Give content sharer a chance to request file access.
            if (nonAssetFilesPendingShare.get() == 0)
                startTimer (2000);
            else
                notifyOwnerIfReady();
        */
    }
    
    pub fn is_content_sharing_enabled(&self) -> bool {
        
        todo!();
        /*
            auto* env = getEnv();

            LocalRef<jobject> packageManager (env->CallObjectMethod (getAppContext().get(), AndroidContext.getPackageManager));

            constexpr int getProviders = 8;
            auto packageInfo = LocalRef<jobject> (env->CallObjectMethod (packageManager,
                                                                         AndroidPackageManager.getPackageInfo,
                                                                         javaString (packageName).get(),
                                                                         getProviders));
            auto providers = LocalRef<jobjectArray> ((jobjectArray) env->GetObjectField (packageInfo,
                                                                                         AndroidPackageInfo.providers));

            if (providers == nullptr)
                return false;

            auto sharingContentProviderAuthority = packageName + ".sharingcontentprovider";
            const int numProviders = env->GetArrayLength (providers.get());

            for (int i = 0; i < numProviders; ++i)
            {
                auto providerInfo = LocalRef<jobject> (env->GetObjectArrayElement (providers, i));
                auto authority = LocalRef<jstring> ((jstring) env->GetObjectField (providerInfo,
                                                                                   AndroidProviderInfo.authority));

                if (aloeString (authority) == sharingContentProviderAuthority)
                    return true;
            }

            return false;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            jassert (prepareFilesThread != nullptr);

            if (prepareFilesThread == nullptr)
                return;

            filesPrepared (prepareFilesThread->getResultFileUris(), prepareFilesThread->getMimeTypes());
        */
    }
    
    pub fn files_prepared(&mut self, 
        file_uris:  jobject,
        mime_types: &Vec<String>)  {
        
        todo!();
        /*
            auto* env = getEnv();

            auto intent = LocalRef<jobject> (env->NewObject (AndroidIntent, AndroidIntent.constructor));
            env->CallObjectMethod (intent, AndroidIntent.setAction,
                                   javaString ("android.intent.action.SEND_MULTIPLE").get());

            env->CallObjectMethod (intent, AndroidIntent.setType,
                                   javaString (getCommonMimeType (mimeTypes)).get());

            constexpr int grantReadPermission = 1;
            env->CallObjectMethod (intent, AndroidIntent.setFlags, grantReadPermission);

            env->CallObjectMethod (intent, AndroidIntent.putParcelableArrayListExtra,
                                   javaString ("android.intent.extra.STREAM").get(),
                                   fileUris);

            auto chooserIntent = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidIntent,
                                                                                 AndroidIntent.createChooser,
                                                                                 intent.get(),
                                                                                 javaString ("Choose share target").get()));

            startAndroidActivityForResult (chooserIntent, 1003,
                                           [weakRef = WeakReference<ContentSharerNativeImpl> { this }] (int /*requestCode*/,
                                                                                                        int resultCode,
                                                                                                        LocalRef<jobject> /*intentData*/) mutable
                                           {
                                               if (weakRef != nullptr)
                                                   weakRef->sharingFinished (resultCode);
                                           });
        */
    }
    
    pub fn decrement_pending_file_count_and_notify_owner_if_ready(&mut self)  {
        
        todo!();
        /*
            --nonAssetFilesPendingShare;

            notifyOwnerIfReady();
        */
    }
    
    pub fn notify_owner_if_ready(&mut self)  {
        
        todo!();
        /*
            if (sharingActivityDidFinish && nonAssetFilesPendingShare.get() == 0)
                owner.sharingFinished (succeeded, {});
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

            notifyOwnerIfReady();
        */
    }
    
    pub fn get_content_uri_elements(&self, 
        env: *mut JNIEnv,
        uri: &LocalRef<jobject>) -> ContentUriElements {
        
        todo!();
        /*
            jassert (prepareFilesThread != nullptr);

            if (prepareFilesThread == nullptr)
                return {};

            auto fullUri = aloeString ((jstring) env->CallObjectMethod (uri.get(), AndroidUri.toString));

            auto index = fullUri.fromFirstOccurrenceOf (uriBase, false, false)
                                 .upToFirstOccurrenceOf ("/", false, true);

            auto filename = fullUri.fromLastOccurrenceOf ("/", false, true);

            return { index, filename, prepareFilesThread->getFilePaths()[index.getIntValue()] };
        */
    }
    
    pub fn get_supported_columns() -> Vec<String> {
        
        todo!();
        /*
            return Vec<String> ("_display_name", "_size");
        */
    }
    
    pub fn get_asset_file_descriptor(&mut self, 
        env:              *mut JNIEnv,
        content_provider: &LocalRef<jobject>,
        filepath:         &String) -> jobject {
        
        todo!();
        /*
            // This function can be called from multiple threads.
            {
                const ScopedLock sl (nonAssetFileOpenLock);

                if (! nonAssetFilePathsPendingShare.contains (filepath))
                {
                    nonAssetFilePathsPendingShare.add (filepath);
                    ++nonAssetFilesPendingShare;

                    nonAssetFileObservers.add (new AndroidContentSharerFileObserver (*this, env,
                                                                                     contentProvider,
                                                                                     filepath));
                }
            }

            auto javaFile = LocalRef<jobject> (env->NewObject (JavaFile, JavaFile.constructor,
                                                               javaString (filepath).get()));

            constexpr int modeReadOnly = 268435456;
            auto parcelFileDescriptor = LocalRef<jobject> (env->CallStaticObjectMethod (ParcelFileDescriptor,
                                                                                        ParcelFileDescriptor.open,
                                                                                        javaFile.get(), modeReadOnly));

            if (jniCheckHasExceptionOccurredAndClear())
            {
                // Failed to create file descriptor. Have you provided a valid file path/resource name?
                jassertfalse;
                return nullptr;
            }

            jlong startOffset = 0;
            jlong unknownLength = -1;

            assetFileDescriptors.add (GlobalRef (LocalRef<jobject> (env->NewObject (AssetFileDescriptor,
                                                                                    AssetFileDescriptor.constructor,
                                                                                    parcelFileDescriptor.get(),
                                                                                    startOffset, unknownLength))));

            return assetFileDescriptors.getReference (assetFileDescriptors.size() - 1).get();
        */
    }
    
    pub fn filter_mime_types(&mut self, 
        mime_types: &Vec<String>,
        filter:     &String) -> Vec<String> {
        
        todo!();
        /*
            String filterToUse (filter.removeCharacters ("*"));

            if (filterToUse.isEmpty() || filterToUse == "/")
                return mimeTypes;

            Vec<String> result;

            for (const auto& type : mimeTypes)
                if (String (type).contains (filterToUse))
                    result.add (type);

            return result;
        */
    }
    
    pub fn get_common_mime_type(&mut self, mime_types: &Vec<String>) -> String {
        
        todo!();
        /*
            if (mimeTypes.isEmpty())
                return "\*\/\*";

            auto commonMime = mimeTypes[0];
            bool lookForCommonGroup = false;

            for (int i = 1; i < mimeTypes.size(); ++i)
            {
                if (mimeTypes[i] == commonMime)
                    continue;

                if (! lookForCommonGroup)
                {
                    lookForCommonGroup = true;
                    commonMime = commonMime.upToFirstOccurrenceOf ("/", true, false);
                }

                if (! mimeTypes[i].startsWith (commonMime))
                    return "\*\/\*";
            }

            return lookForCommonGroup ? commonMime + "\*" : commonMime;
        */
    }

    #[JNICALL]
    pub fn content_sharer_query(
        _0:               *mut JNIEnv,
        content_provider: jobject,
        uri:              jobject,
        projection:       jobjectArray) -> jobject {
        
        todo!();
        /*
            if (auto *pimpl = (ContentSharer::ContentSharerNativeImpl *) ContentSharer::getInstance ()->pimpl.get ())
                return pimpl->query (LocalRef<jobject> (static_cast<jobject> (contentProvider)),
                                     LocalRef<jobject> (static_cast<jobject> (uri)),
                                     LocalRef<jobjectArray> (static_cast<jobjectArray> (projection)));

            return nullptr;
        */
    }

    #[JNICALL]
    pub fn content_sharer_open_file(
        _0:               *mut JNIEnv,
        content_provider: jobject,
        uri:              jobject,
        mode:             jstring) -> jobject {
        
        todo!();
        /*
            if (auto* pimpl = (ContentSharer::ContentSharerNativeImpl*) ContentSharer::getInstance()->pimpl.get())
                return pimpl->openFile (LocalRef<jobject> (static_cast<jobject> (contentProvider)),
                                        LocalRef<jobject> (static_cast<jobject> (uri)),
                                        LocalRef<jstring> (static_cast<jstring> (mode)));

            return nullptr;
        */
    }
    
    #[JNICALL]
    pub fn content_sharer_get_stream_types(
        _0:               *mut JNIEnv,
        content_provider: jobject,
        uri:              jobject,
        mime_type_filter: jstring) -> jobjectArray {
        
        todo!();
        /*
            if (auto* pimpl = (ContentSharer::ContentSharerNativeImpl*) ContentSharer::getInstance()->pimpl.get())
                return pimpl->getStreamTypes (LocalRef<jobject> (static_cast<jobject> (uri)),
                                              LocalRef<jstring> (static_cast<jstring> (mimeTypeFilter)));

            return nullptr;
        */
    }
}
