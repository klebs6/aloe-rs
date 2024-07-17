crate::ix!();

#[cfg(target_os="android")]
#[no_copy]
pub struct JNIClassBase {
    class_path:     *const u8,
    byte_code:      *const c_void,
    byte_code_size: usize,
    minsdk:         i32,
    class_ref:      jclass, // default = nullptr
}

#[cfg(target_os="android")]
impl Into<jclass> for JNIClassBase {
    
    #[inline] fn into(self) -> jclass {
        todo!();
        /*
            return classRef;
        */
    }
}

#[cfg(target_os="android")]
pub trait JNIClassBaseInterface {

    fn initialise_fields(&mut self, _0: *mut JNIEnv);
}

#[cfg(target_os="android")]
impl Drop for JNIClassBase {

    fn drop(&mut self) {
        todo!();
        /* 
        getClasses().removeFirstMatchingValue (this);
 */
    }
}

#[cfg(target_os="android")]
impl JNIClassBase {
    
    #[inline] pub fn get_class_path(&self) -> *const u8 {
        
        todo!();
        /*
            return classPath;
        */
    }
    
    pub fn new(
        cp:           *const u8,
        class_minsdk: i32,
        bc:           *const c_void,
        n:            usize) -> Self {
    
        todo!();
        /*
        : class_path(cp),
        : byte_code(bc),
        : byte_code_size(n),
        : minsdk(classMinSDK),
        : class_ref(nullptr),

            SystemJavaClassComparator comparator;

        getClasses().addSorted (comparator, this);
        */
    }
    
    pub fn get_classes(&mut self) -> &mut Vec<*mut JNIClassBase> {
        
        todo!();
        /*
            static Vec<JNIClassBase*> classes;
        return classes;
        */
    }
    
    pub fn initialise(&mut self, env: *mut JNIEnv)  {
        
        todo!();
        /*
            auto sdkVersion = getAndroidSDKVersion();

        if (sdkVersion >= minSDK)
        {
            LocalRef<jstring> classNameAndPackage (javaString (String (classPath).replaceCharacter (L'/', L'.')));
            static Vec<GlobalRef> byteCodeLoaders;

            if (! SystemJavaClassComparator::isSystemClass(this))
            {
                LocalRef<jobject> defaultClassLoader (env->CallStaticObjectMethod (JavaClassLoader, JavaClassLoader.getSystemClassLoader));
                tryLoadingClassWithClassLoader (env, defaultClassLoader.get());

                if (classRef == nullptr)
                {
                    for (auto& byteCodeLoader : byteCodeLoaders)
                    {
                        tryLoadingClassWithClassLoader (env, byteCodeLoader.get());

                        if (classRef != nullptr)
                            break;
                    }

                    // fallback by trying to load the class from bytecode
                    if (byteCode != nullptr)
                    {
                        LocalRef<jobject> byteCodeClassLoader;

                        MemoryOutputStream uncompressedByteCode;

                        {
                            MemoryInputStream rawGZipData (byteCode, byteCodeSize, false);
                            GZIPDecompressorInputStream gzipStream (&rawGZipData, false, GZIPDecompressorInputStream::gzipFormat);
                            uncompressedByteCode.writeFromInputStream (gzipStream, -1);
                        }

                        if (sdkVersion >= 26)
                        {
                            LocalRef<jbyteArray> byteArray (env->NewByteArray ((jsize) uncompressedByteCode.getDataSize()));
                            jboolean isCopy;
                            auto* dst = env->GetByteArrayElements (byteArray.get(), &isCopy);
                            memcpy (dst, uncompressedByteCode.getData(), uncompressedByteCode.getDataSize());
                            env->ReleaseByteArrayElements (byteArray.get(), dst, 0);

                            LocalRef<jobject> byteBuffer (env->CallStaticObjectMethod (JavaByteBuffer, JavaByteBuffer.wrap, byteArray.get()));

                            byteCodeClassLoader = LocalRef<jobject> (env->NewObject (AndroidInMemoryDexClassLoader,
                                                                                     AndroidInMemoryDexClassLoader.constructor,
                                                                                     byteBuffer.get(), defaultClassLoader.get()));
                        }
                        else if (uncompressedByteCode.getDataSize() >= 32)
                        {
                            auto codeCacheDir = getCodeCacheDirectory();

                            // The dex file has an embedded 20-byte long SHA-1 signature at offset 12
                            auto fileName = String::toHexString ((char*)uncompressedByteCode.getData() + 12, 20, 0) + ".dex";
                            auto dexFile = codeCacheDir.getChildFile (fileName);
                            auto optimizedDirectory = codeCacheDir.getChildFile ("optimized_cache");
                            optimizedDirectory.createDirectory();

                            if (dexFile.replaceWithData (uncompressedByteCode.getData(), uncompressedByteCode.getDataSize()))
                            {
                                byteCodeClassLoader = LocalRef<jobject> (env->NewObject (AndroidDexClassLoader,
                                                                                         AndroidDexClassLoader.constructor,
                                                                                         javaString (dexFile.getFullPathName()).get(),
                                                                                         javaString (optimizedDirectory.getFullPathName()).get(),
                                                                                         nullptr,
                                                                                         defaultClassLoader.get()));
                            }
                            else
                            {
                                // can't write to cache folder
                                jassertfalse;
                            }
                        }

                        if (byteCodeClassLoader != nullptr)
                        {
                            tryLoadingClassWithClassLoader (env, byteCodeClassLoader.get());
                            byteCodeLoaders.add (GlobalRef(byteCodeClassLoader));
                        }
                    }
                }
            }

            if (classRef == nullptr)
                classRef = (jclass) env->NewGlobalRef (LocalRef<jobject> (env->FindClass (classPath)));

            jassert (classRef != nullptr);
            initialiseFields (env);
        }
        */
    }
    
    pub fn try_loading_class_with_class_loader(&mut self, 
        env:          *mut JNIEnv,
        class_loader: jobject)  {
        
        todo!();
        /*
            LocalRef<jstring> classNameAndPackage (javaString (String (classPath).replaceCharacter (L'/', L'.')));

        // Android SDK <= 19 has a bug where the class loader might throw an exception but still return
        // a non-nullptr. So don't assign the result of this call to a jobject just yet...
        auto classObj = env->CallObjectMethod (classLoader, JavaClassLoader.findClass, classNameAndPackage.get());

        if (jthrowable exception = env->ExceptionOccurred ())
        {
            env->ExceptionClear();
            classObj = nullptr;
        }

        // later versions of Android don't throw at all, so re-check the object
        if (classObj != nullptr)
            classRef = (jclass) env->NewGlobalRef (LocalRef<jobject> (classObj));
        */
    }
    
    pub fn release(&mut self, env: *mut JNIEnv)  {
        
        todo!();
        /*
            if (classRef != nullptr)
            env->DeleteGlobalRef (classRef);
        */
    }
    
    pub fn initialise_all_classes(&mut self, env: *mut JNIEnv)  {
        
        todo!();
        /*
            const Vec<JNIClassBase*>& classes = getClasses();
        for (int i = classes.size(); --i >= 0;)
            classes.getUnchecked(i)->initialise (env);
        */
    }
    
    pub fn release_all_classes(&mut self, env: *mut JNIEnv)  {
        
        todo!();
        /*
            const Vec<JNIClassBase*>& classes = getClasses();
        for (int i = classes.size(); --i >= 0;)
            classes.getUnchecked(i)->release (env);
        */
    }
    
    pub fn resolve_method(&mut self, 
        env:         *mut JNIEnv,
        method_name: *const u8,
        params:      *const u8) -> jmethodID {
        
        todo!();
        /*
            jmethodID m = env->GetMethodID (classRef, methodName, params);
        jassert (m != nullptr);
        return m;
        */
    }
    
    pub fn resolve_static_method(&mut self, 
        env:         *mut JNIEnv,
        method_name: *const u8,
        params:      *const u8) -> jmethodID {
        
        todo!();
        /*
            jmethodID m = env->GetStaticMethodID (classRef, methodName, params);
        jassert (m != nullptr);
        return m;
        */
    }
    
    pub fn resolve_field(&mut self, 
        env:        *mut JNIEnv,
        field_name: *const u8,
        signature:  *const u8) -> jfieldID {
        
        todo!();
        /*
            jfieldID f = env->GetFieldID (classRef, fieldName, signature);
        jassert (f != nullptr);
        return f;
        */
    }
    
    pub fn resolve_static_field(&mut self, 
        env:        *mut JNIEnv,
        field_name: *const u8,
        signature:  *const u8) -> jfieldID {
        
        todo!();
        /*
            jfieldID f = env->GetStaticFieldID (classRef, fieldName, signature);
        jassert (f != nullptr);
        return f;
        */
    }
    
    pub fn resolve_callbacks(&mut self, 
        env:              *mut JNIEnv,
        native_callbacks: &[JNINativeMethod])  {
        
        todo!();
        /*
            if (nativeCallbacks.size() > 0)
            env->RegisterNatives (classRef, nativeCallbacks.begin(), (jint) nativeCallbacks.size());
        */
    }
}
