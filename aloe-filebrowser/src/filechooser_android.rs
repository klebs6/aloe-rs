crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/aloe_android_FileChooser.cpp]

#[cfg(target_os="android")]
#[weak_referenceable]
pub struct FileChooserNative<'a> {
    owner:  &'a mut FileChooser<'a>,
    intent: GlobalRef,
}

#[cfg(target_os="android")]
impl<'a> FileChooserPimpl for FileChooserNative<'a> {

}

#[cfg(target_os="android")]
impl<'a> Drop for FileChooserNative<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                masterReference.clear();
                currentFileChooser = nullptr;
             */
    }
}

#[cfg(target_os="android")]
lazy_static!{
    /*
    static FileChooserNative* currentFileChooser;
    */
}

#[cfg(target_os="android")]
impl<'a> FileChooserNative<'a> {

    pub fn new(
        file_chooser: &mut FileChooser,
        flags:        i32) -> Self {
    
        todo!();
        /*
        : owner(fileChooser),

            if (currentFileChooser == nullptr)
                {
                    currentFileChooser = this;
                    auto* env = getEnv();

                    auto sdkVersion         = getAndroidSDKVersion();
                    auto saveMode           = ((flags & FileBrowserComponent::saveMode) != 0);
                    auto selectsDirectories = ((flags & FileBrowserComponent::canSelectDirectories) != 0);

                    // You cannot save a directory
                    jassert (! (saveMode && selectsDirectories));

                    if (sdkVersion < 19)
                    {
                        // native save dialogs are only supported in Android versions >= 19
                        jassert (! saveMode);
                        saveMode = false;
                    }

                    if (sdkVersion < 21)
                    {
                        // native directory chooser dialogs are only supported in Android versions >= 21
                        jassert (! selectsDirectories);
                        selectsDirectories = false;
                    }

                    const char* action = (selectsDirectories ? "android.intent.action.OPEN_DOCUMENT_TREE"
                                                             : (saveMode ? "android.intent.action.CREATE_DOCUMENT"
                                                             : (sdkVersion >= 19 ? "android.intent.action.OPEN_DOCUMENT"
                                                             : "android.intent.action.GET_CONTENT")));

                    intent = GlobalRef (LocalRef<jobject> (env->NewObject (AndroidIntent, AndroidIntent.constructWithString,
                                                                           javaString (action).get())));

                    if (owner.startingFile != File())
                    {
                        if (saveMode && (! owner.startingFile.isDirectory()))
                            env->CallObjectMethod (intent.get(), AndroidIntent.putExtraString,
                                                   javaString ("android.intent.extra.TITLE").get(),
                                                   javaString (owner.startingFile.getFileName()).get());

                        Url url (owner.startingFile);
                        LocalRef<jobject> uri (env->CallStaticObjectMethod (AndroidUri, AndroidUri.parse,
                                                                            javaString (url.toString (true)).get()));

                        if (uri)
                            env->CallObjectMethod (intent.get(), AndroidIntent.putExtraParcelable,
                                                   javaString ("android.provider.extra.INITIAL_URI").get(),
                                                   uri.get());
                    }

                    if (! selectsDirectories)
                    {
                        env->CallObjectMethod (intent.get(), AndroidIntent.addCategory,
                                               javaString ("android.intent.category.OPENABLE").get());

                        auto mimeTypes = convertFiltersToMimeTypes (owner.filters);

                        if (mimeTypes.size() == 1)
                        {
                            env->CallObjectMethod (intent.get(), AndroidIntent.setType, javaString (mimeTypes[0]).get());
                        }
                        else
                        {
                            String mimeGroup = "*";

                            if (mimeTypes.size() > 0)
                            {
                                mimeGroup = mimeTypes[0].upToFirstOccurrenceOf ("/", false, false);
                                auto allMimeTypesHaveSameGroup = true;

                                LocalRef<jobjectArray> jMimeTypes (env->NewObjectArray (mimeTypes.size(), JavaString,
                                                                                        javaString("").get()));

                                for (int i = 0; i < mimeTypes.size(); ++i)
                                {
                                    env->SetObjectArrayElement (jMimeTypes.get(), i, javaString (mimeTypes[i]).get());

                                    if (mimeGroup != mimeTypes[i].upToFirstOccurrenceOf ("/", false, false))
                                        allMimeTypesHaveSameGroup = false;
                                }

                                env->CallObjectMethod (intent.get(), AndroidIntent.putExtraStrings,
                                                       javaString ("android.intent.extra.MIME_TYPES").get(),
                                                       jMimeTypes.get());

                                if (! allMimeTypesHaveSameGroup)
                                    mimeGroup = "*";
                            }

                            env->CallObjectMethod (intent.get(), AndroidIntent.setType, javaString (mimeGroup + "/\*").get());
                        }
                    }
                }
                else
                    jassertfalse; // there can only be a single file chooser
        */
    }
    
    pub fn run_modally(&mut self)  {
        
        todo!();
        /*
            // Android does not support modal file choosers
                jassertfalse;
        */
    }
    
    pub fn launch(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

                if (currentFileChooser != nullptr)
                {
                    startAndroidActivityForResult (LocalRef<jobject> (env->NewLocalRef (intent.get())), /*READ_REQUEST_CODE*/ 42,
                                                   [myself = WeakReference<FileChooserNative> { this }] (int requestCode, int resultCode, LocalRef<jobject> intentData) mutable
                                                   {
                                                       if (myself != nullptr)
                                                           myself->onActivityResult (requestCode, resultCode, intentData);
                                                   });
                }
                else
                {
                    jassertfalse; // There is already a file chooser running
                }
        */
    }
    
    pub fn on_activity_result(&mut self, 
        request_code: i32,
        result_code:  i32,
        intent_data:  &LocalRef<jobject>)  {
        
        todo!();
        /*
            currentFileChooser = nullptr;
                auto* env = getEnv();

                Vec<Url> chosenURLs;

                if (resultCode == /*Activity.RESULT_OK*/ -1 && intentData != nullptr)
                {
                    LocalRef<jobject> uri (env->CallObjectMethod (intentData.get(), AndroidIntent.getData));

                    if (uri != nullptr)
                    {
                        auto jStr = (jstring) env->CallObjectMethod (uri, JavaObject.toString);

                        if (jStr != nullptr)
                            chosenURLs.add (Url (aloeString (env, jStr)));
                    }
                }

                owner.finished (chosenURLs);
        */
    }
    
    pub fn convert_filters_to_mime_types(file_filters: &String) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> result;
                auto wildcards = Vec<String>::fromTokens (fileFilters, ";", "");

                for (auto wildcard : wildcards)
                {
                    if (wildcard.upToLastOccurrenceOf (".", false, false) == "*")
                    {
                        auto extension = wildcard.fromLastOccurrenceOf (".", false, false);

                        result.addArray (getMimeTypesForFileExtension (extension));
                    }
                }

                result.removeDuplicates (false);
                return result;
        */
    }
}

#[cfg(target_os="android")]
lazy_static!{
    /*
    FileChooser::FileChooserNative* FileChooser::FileChooserNative::currentFileChooser = nullptr;
    */
}


#[cfg(target_os="android")]
impl<'a> FileChooser<'a> {
    
    pub fn show_platform_dialog(
        &mut self, 
        owner: &mut FileChooser<'a>,
        flags: i32,
        _2:    *mut FilePreviewComponent

    ) -> Arc<dyn FileChooserPimpl> {
        
        todo!();
        /*
            if (FileChooser::FileChooserNative::currentFileChooser == nullptr)
            return std::make_shared<FileChooser::FileChooserNative> (owner, flags);

        // there can only be one file chooser on Android at a once
        jassertfalse;
        return nullptr;
        */
    }
    
    /**
      | Returns if a native filechooser is currently
      | available on this platform.
      | 
      | -----------
      | @note
      | 
      | On iOS this will only return true if you
      | have iCloud permissions and code-signing
      | enabled in the Proaloer and have added
      | iCloud containers to your app in Apple's
      | online developer portal. Additionally,
      | the user must have installed the iCloud
      | app on their device and used the app at
      | least once.
      |
      */
    pub fn is_platform_dialog_available(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_DISABLE_NATIVE_FILECHOOSERS
        return false;
       #else
        return true;
       #endif
        */
    }
}
