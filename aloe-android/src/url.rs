crate::ix!();

#[cfg(target_os="android")]
impl Url {
    
    pub fn is_local_file(&self) -> bool {
        
        todo!();
        /*
            if (getScheme() == "file")
            return true;

        if (getScheme() == "content")
        {
            auto file = AndroidContentUriResolver::getLocalFileFromContentUri (*this);
            return (file != File());
        }

        return false;
        */
    }
    
    pub fn get_local_file(&self) -> File {
        
        todo!();
        /*
            if (getScheme() == "content")
        {
            auto path = AndroidContentUriResolver::getLocalFileFromContentUri (*this);

            // This Url does not refer to a local file
            // Call Url::isLocalFile to first check if the Url
            // refers to a local file.
            jassert (path != File());

            return path;
        }

        return fileFromFileSchemeURL (*this);
        */
    }
    
    pub fn get_file_name(&self) -> String {
        
        todo!();
        /*
            if (getScheme() == "content")
            return AndroidContentUriResolver::getFileNameFromContentUri (*this);

        return toString (false).fromLastOccurrenceOf ("/", false, true);
        */
    }
    
    pub fn download_to_file(
        &mut self, 
        target_location: &File,
        extra_headers:   String,
        listener:        *mut dyn UrlDownloadTaskListener,
        should_use_post: bool

    ) -> Box<UrlDownloadTask> {
        
        todo!();
        /*
            return Url::DownloadTask::createFallbackDownloader (*this, targetLocation, extraHeaders, listener, shouldUsePost);
        */
    }
}
