crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DLLHandleCache {
    base:         DeletedAtShutdown,
    open_handles: Vec<Box<DLLHandle>>,
}

aloe_declare_singleton!{
    DLLHandleCache, 
    false
}

aloe_implement_singleton!{
    DLLHandleCache
}

impl Drop for DLLHandleCache {

    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

impl DLLHandleCache {

    pub fn find_or_create_handle(&mut self, module_path: &String) -> &mut DLLHandle {
        
        todo!();
        /*
            #if ALOE_LINUX || ALOE_BSD
            File file (getDLLFileFromBundle (modulePath));
           #else
            File file (modulePath);
           #endif

            auto it = std::find_if (openHandles.begin(), openHandles.end(),
                                    [&] (const std::unique_ptr<DLLHandle>& handle)
                                    {
                                         return file == handle->getFile();
                                    });

            if (it != openHandles.end())
                return *it->get();

            openHandles.push_back (std::make_unique<DLLHandle> (file));
            return *openHandles.back().get();
        */
    }

    #[cfg(any(target_os="linux",target_os="bsd"))]
    pub fn get_dll_file_from_bundle(&self, bundle_path: &String) -> File {
        
        todo!();
        /*
            auto machineName = []() -> String
            {
                struct utsname unameData;
                auto res = uname (&unameData);

                if (res != 0)
                    return {};

                return unameData.machine;
            }();

            File file (bundlePath);

            return file.getChildFile ("Contents")
                       .getChildFile (machineName + "-linux")
                       .getChildFile (file.getFileNameWithoutExtension() + ".so");
        */
    }
}
