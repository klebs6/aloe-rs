crate::ix!();

//#[cfg(ALOE_CONTENT_SHARING)]
pub trait ContentSharerPimpl
{
    fn share_files(&mut self, files: &[Url]);

    fn share_text(&mut self, text: &String);
}

/**
  | A singleton class responsible for sharing
  | content between apps and devices.
  | 
  | You can share text, images, files or
  | an arbitrary data block.
  | 
  | @tags{GUI}
  |
  */
pub struct ContentSharer {
    base:                  DeletedAtShutdown,
    temporary_files:       Vec<File>,
    callback:              fn(_0: bool, _1: String) -> (),

    #[cfg(ALOE_CONTENT_SHARING)]
    pimpl:                 Box<ContentSharerPimpl>,

    #[cfg(ALOE_CONTENT_SHARING)]
    prepare_images_thread: Box<ContentSharerPrepareImagesThread>,

    #[cfg(ALOE_CONTENT_SHARING)]
    prepare_data_thread:   Box<ContentSharerPrepareDataThread>,
}

impl ContentSharer {
    
    #[cfg(target_os="android")]
    pub fn create_pimpl(&mut self) -> *mut ContentSharerPimpl {
        
        todo!();
        /*
            return new ContentSharerNativeImpl (*this);
        */
    }
}

impl Drop for ContentSharer {

    fn drop(&mut self) {
        todo!();
        /*      clearSingletonInstance();  */
    }
}

aloe_declare_singleton!{
    ContentSharer, false
}

aloe_implement_singleton!{
    ContentSharer
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_ContentSharer.cpp]
impl ContentSharer {

    /**
      | Shares the given files. Each Url should
      | be either a full file path or it should
      | point to a resource within the application
      | bundle. For resources on iOS it should
      | be something like "content/image.png"
      | if you want to specify a file from application
      | bundle located in "content" directory.
      | On Android you should specify only a
      | filename, without an extension.
      | 
      | Upon completion you will receive a callback
      | with a sharing result. Note:
      | 
      | Sadly on Android the returned success
      | flag may be wrong as there is no standard
      | way the sharing targets report if the
      | sharing operation succeeded. Also,
      | the optional error message is always
      | empty on Android.
      |
      */
    pub fn share_files(&mut self, 
        files:           &[Url],
        callback_to_use: fn(_0: bool, _1: &String) -> ())  {
        
        todo!();
        /*
            #if ALOE_CONTENT_SHARING
        startNewShare (callbackToUse);
        pimpl->shareFiles (files);
      #else
        ignoreUnused (files);

        // Content sharing is not available on this platform!
        jassertfalse;

        if (callbackToUse)
            callbackToUse (false, "Content sharing is not available on this platform!");
      #endif
        */
    }

    #[cfg(ALOE_CONTENT_SHARING)]
    pub fn start_new_share(&mut self, callback_to_use: fn(_0: bool, _1: &String) -> ())  {
        
        todo!();
        /*
            // You should not start another sharing operation before the previous one is finished.
        // Forcibly stopping a previous sharing operation is rarely a good idea!
        jassert (pimpl == nullptr);
        pimpl.reset();

        prepareDataThread = nullptr;
        prepareImagesThread = nullptr;

        deleteTemporaryFiles();

        // You need to pass a valid callback.
        jassert (callbackToUse);
        callback = std::move (callbackToUse);

        pimpl.reset (createPimpl());
        */
    }
    
    /**
      | Shares the given text.
      | 
      | Upon completion you will receive a callback
      | with a sharing result. Note:
      | 
      | Sadly on Android the returned success
      | flag may be wrong as there is no standard
      | way the sharing targets report if the
      | sharing operation succeeded. Also,
      | the optional error message is always
      | empty on Android.
      |
      */
    pub fn share_text(&mut self, 
        text:            &String,
        callback_to_use: fn(_0: bool, _1: &String) -> ())  {
        
        todo!();
        /*
            #if ALOE_CONTENT_SHARING
        startNewShare (callbackToUse);
        pimpl->shareText (text);
      #else
        ignoreUnused (text);

        // Content sharing is not available on this platform!
        jassertfalse;

        if (callbackToUse)
            callbackToUse (false, "Content sharing is not available on this platform!");
      #endif
        */
    }
    
    /**
      | A convenience function to share an image.
      | This is useful when you have images loaded
      | in memory. The images will be written
      | to temporary files first, so if you have
      | the images in question stored on disk
      | already call shareFiles() instead.
      | 
      | By default, images will be saved to PNG
      | files, but you can supply a custom
      | 
      | ImageFileFormat to override this.
      | The custom file format will be owned
      | and deleted by the sharer. e.g.
      | 
      | -----------
      | @code
      | 
      | Graphics g (myImage);
      | g.setColour (Colours::green);
      | g.fillEllipse (20, 20, 300, 200);
      | Vec<Image> images;
      | images.add (myImage);
      | ContentSharer::getInstance()->shareImages (images, myCallback);
      | 
      | Upon completion you will receive a callback
      | with a sharing result. Note:
      | 
      | Sadly on Android the returned success
      | flag may be wrong as there is no standard
      | way the sharing targets report if the
      | sharing operation succeeded. Also,
      | the optional error message is always
      | empty on Android.
      |
      */
    pub fn share_images(&mut self, 
        images:                   &[Image],
        callback_to_use:          fn(_0: bool, _1: &String) -> (),
        image_file_format_to_use: *mut ImageFileFormat)  {
        
        todo!();
        /*
            #if ALOE_CONTENT_SHARING
        startNewShare (callbackToUse);
        prepareImagesThread.reset (new ContentSharerPrepareImagesThread (*this, images, imageFileFormatToUse));
      #else
        ignoreUnused (images, imageFileFormatToUse);

        // Content sharing is not available on this platform!
        jassertfalse;

        if (callbackToUse)
            callbackToUse (false, "Content sharing is not available on this platform!");
      #endif
        */
    }

    #[cfg(ALOE_CONTENT_SHARING)]
    pub fn files_to_share_prepared(&mut self)  {
        
        todo!();
        /*
            Vec<Url> urls;

        for (const auto& tempFile : temporaryFiles)
            urls.add (Url (tempFile));

        prepareImagesThread = nullptr;
        prepareDataThread = nullptr;

        pimpl->shareFiles (urls);
        */
    }
    
    /**
      | A convenience function to share arbitrary
      | data. The data will be written to a temporary
      | file and then that file will be shared.
      | If you have your data stored on disk already,
      | call shareFiles() instead.
      | 
      | Upon completion you will receive a callback
      | with a sharing result. Note:
      | 
      | Sadly on Android the returned success
      | flag may be wrong as there is no standard
      | way the sharing targets report if the
      | sharing operation succeeded. Also,
      | the optional error message is always
      | empty on Android.
      |
      */
    pub fn share_data(&mut self, 
        mb:              &MemoryBlock,
        callback_to_use: fn(_0: bool, _1: &String) -> ())  {
        
        todo!();
        /*
            #if ALOE_CONTENT_SHARING
        startNewShare (callbackToUse);
        prepareDataThread.reset (new ContentSharerPrepareDataThread (*this, mb));
      #else
        ignoreUnused (mb);

        if (callbackToUse)
            callbackToUse (false, "Content sharing not available on this platform!");
      #endif
        */
    }
    
    pub fn sharing_finished(&mut self, 
        succeeded:         bool,
        error_description: &String)  {
        
        todo!();
        /*
            deleteTemporaryFiles();

        std::function<void (bool, String)> cb;
        std::swap (cb, callback);

        String error (errorDescription);

      #if ALOE_CONTENT_SHARING
        pimpl.reset();
      #endif

        if (cb)
            cb (succeeded, error);
        */
    }
    
    pub fn delete_temporary_files(&mut self)  {
        
        todo!();
        /*
            for (auto& f : temporaryFiles)
            f.deleteFile();

        temporaryFiles.clear();
        */
    }
    
    #[cfg(target_os="ios")]
    #[cfg(ALOE_CONTENT_SHARING)]
    pub fn create_pimpl(&mut self) -> *mut ContentSharer::ContentSharerPimpl {
        
        todo!();
        /*
            return new ContentSharerNativeImpl (*this);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_ContentSharer.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/aloe_ios_ContentSharer.cpp]

