crate::ix!();

///---------------------
#[cfg(target_os="android")]
pub struct AndroidContentSharerPrepareFilesThread<'a> {
    base:                             Thread,
    owner:                            &'a mut AsyncUpdater<'a>,
    file_urls:                        Vec<Url>,
    result_file_uris:                 GlobalRef,
    package_name:                     String,
    uri_base:                         String,
    file_paths:                       Vec<String>,
    temporary_files_from_asset_files: Vec<File>,
    mime_types:                       Vec<String>,
}

#[cfg(target_os="android")]
impl<'a> Drop for AndroidContentSharerPrepareFilesThread<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            signalThreadShouldExit();
            waitForThreadToExit (10000);

            for (auto& f : temporaryFilesFromAssetFiles)
                f.deleteFile();
         */
    }
}

#[cfg(target_os="android")]
impl<'a> AndroidContentSharerPrepareFilesThread<'a> {

    pub fn new(
        owner_to_use:        &mut AsyncUpdater,
        file_urls_to_use:    &[Url],
        package_name_to_use: &String,
        uri_base_to_use:     &String) -> Self {
    
        todo!();
        /*


            : Thread ("AndroidContentSharerPrepareFilesThread"),
              owner (ownerToUse),
              fileUrls (fileUrlsToUse),
              resultFileUris (GlobalRef (LocalRef<jobject> (getEnv()->NewObject (JavaArrayList,
                                                                                 JavaArrayList.constructor,
                                                                                 fileUrls.size())))),
              packageName (packageNameToUse),
              uriBase (uriBaseToUse)
            startThread();
        */
    }
    
    pub fn get_result_file_uris(&mut self) -> jobject {
        
        todo!();
        /*
            return resultFileUris.get();
        */
    }
    
    pub fn get_mime_types(&self) -> &Vec<String> {
        
        todo!();
        /*
            return mimeTypes;
        */
    }
    
    pub fn get_file_paths(&self) -> &Vec<String> {
        
        todo!();
        /*
            return filePaths;
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

            bool canSpecifyMimeTypes = true;

            for (auto f : fileUrls)
            {
                auto scheme = f.getScheme();

                // Only "file://" scheme or no scheme (for files in app bundle) are allowed!
                jassert (scheme.isEmpty() || scheme == "file");

                if (scheme.isEmpty())
                {
                    // Raw resource names need to be all lower case
                    jassert (f.toString (true).toLowerCase() == f.toString (true));

                    // This will get us a file with file:// URI
                    f = copyAssetFileToTemporaryFile (env, f.toString (true));

                    if (f.isEmpty())
                        continue;
                }

                if (threadShouldExit())
                    return;

                auto filepath = Url::removeEscapeChars (f.toString (true).fromFirstOccurrenceOf ("file://", false, false));

                filePaths.add (filepath);

                auto filename = filepath.fromLastOccurrenceOf ("/", false, true);
                auto fileExtension = filename.fromLastOccurrenceOf (".", false, true);
                auto contentString = uriBase + String (filePaths.size() - 1) + "/" + filename;

                auto uri = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidUri, AndroidUri.parse,
                                                                           javaString (contentString).get()));

                if (canSpecifyMimeTypes)
                    canSpecifyMimeTypes = fileExtension.isNotEmpty();

                if (canSpecifyMimeTypes)
                    mimeTypes.addArray (getMimeTypesForFileExtension (fileExtension));
                else
                    mimeTypes.clear();

                env->CallBooleanMethod (resultFileUris, JavaArrayList.add, uri.get());
            }

            owner.triggerAsyncUpdate();
        */
    }
    
    pub fn copy_asset_file_to_temporary_file(&mut self, 
        env:      *mut JNIEnv,
        filename: &String) -> Url {
        
        todo!();
        /*
            auto resources = LocalRef<jobject> (env->CallObjectMethod (getAppContext().get(), AndroidContext.getResources));
            int fileId = env->CallIntMethod (resources, AndroidResources.getIdentifier, javaString (filename).get(),
                                             javaString ("raw").get(), javaString (packageName).get());

            // Raw resource not found. Please make sure that you include your file as a raw resource
            // and that you specify just the file name, without an extension.
            jassert (fileId != 0);

            if (fileId == 0)
                return {};

            auto assetFd = LocalRef<jobject> (env->CallObjectMethod (resources,
                                                                     AndroidResources.openRawResourceFd,
                                                                     fileId));

            auto inputStream = StreamCloser (LocalRef<jobject> (env->CallObjectMethod (assetFd,
                                                                                       AssetFileDescriptor.createInputStream)));

            if (jniCheckHasExceptionOccurredAndClear())
            {
                // Failed to open file stream for resource
                jassertfalse;
                return {};
            }

            auto tempFile = File::createTempFile ({});
            tempFile.createDirectory();
            tempFile = tempFile.getChildFile (filename);

            auto outputStream = StreamCloser (LocalRef<jobject> (env->NewObject (JavaFileOutputStream,
                                                                                 JavaFileOutputStream.constructor,
                                                                                 javaString (tempFile.getFullPathName()).get())));

            if (jniCheckHasExceptionOccurredAndClear())
            {
                // Failed to open file stream for temporary file
                jassertfalse;
                return {};
            }

            auto buffer = LocalRef<jbyteArray> (env->NewByteArray (1024));
            int bytesRead = 0;

            for (;;)
            {
                if (threadShouldExit())
                    return {};

                bytesRead = env->CallIntMethod (inputStream.stream, JavaFileInputStream.read, buffer.get());

                if (jniCheckHasExceptionOccurredAndClear())
                {
                    // Failed to read from resource file.
                    jassertfalse;
                    return {};
                }

                if (bytesRead < 0)
                    break;

                env->CallVoidMethod (outputStream.stream, JavaFileOutputStream.write, buffer.get(), 0, bytesRead);

                if (jniCheckHasExceptionOccurredAndClear())
                {
                    // Failed to write to temporary file.
                    jassertfalse;
                    return {};
                }
            }

            temporaryFilesFromAssetFiles.add (tempFile);

            return Url (tempFile);
        */
    }
}
