crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_ImageCache.h]

/**
  | A global cache of images that have been
  | loaded from files or memory.
  | 
  | If you're loading an image and may need
  | to use the image in more than one place,
  | this is used to allow the same image to
  | be shared rather than loading multiple
  | copies into memory.
  | 
  | Another advantage is that after images
  | are released, they will be kept in memory
  | for a few seconds before it is actually
  | deleted, so if you're repeatedly loading/deleting
  | the same image, it'll reduce the chances
  | of having to reload it each time.
  | 
  | @see Image, ImageFileFormat
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
pub struct ImageCache {

}

#[derive(Default)]
#[no_copy]
pub struct ImageCachePimpl {
    base:          Timer,
    base2:         DeletedAtShutdown,
    images:        Vec<ImageCachePimplItem>,
    lock:          CriticalSection,
    cache_timeout: u32, // default = 5000
}

impl Drop for ImageCachePimpl {
    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

aloe_declare_singleton_singlethreaded_minimal!{
    ImageCachePimpl
}

pub struct ImageCachePimplItem
{
    image:         Image,
    hash_code:     i64,
    last_use_time: u32,
}

impl ImageCachePimpl {

    pub fn get_from_hash_code(&mut self, hash_code: i64) -> Image {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            for (auto& item : images)
            {
                if (item.hashCode == hashCode)
                {
                    item.lastUseTime = Time::getApproximateMillisecondCounter();
                    return item.image;
                }
            }

            return {};
        */
    }
    
    pub fn add_image_to_cache(&mut self, 
        image:     &Image,
        hash_code: i64)  {
        
        todo!();
        /*
            if (image.isValid())
            {
                if (! isTimerRunning())
                    startTimer (2000);

                const ScopedLock sl (lock);
                images.add ({ image, hashCode, Time::getApproximateMillisecondCounter() });
            }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto now = Time::getApproximateMillisecondCounter();

            const ScopedLock sl (lock);

            for (int i = images.size(); --i >= 0;)
            {
                auto& item = images.getReference(i);

                if (item.image.getReferenceCount() <= 1)
                {
                    if (now > item.lastUseTime + cacheTimeout || now < item.lastUseTime - 1000)
                        images.remove (i);
                }
                else
                {
                    item.lastUseTime = now; // multiply-referenced, so this image is still in use.
                }
            }

            if (images.isEmpty())
                stopTimer();
        */
    }
    
    pub fn release_unused_images(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            for (int i = images.size(); --i >= 0;)
                if (images.getReference(i).image.getReferenceCount() <= 1)
                    images.remove (i);
        */
    }
}

aloe_implement_singleton!{
    ImageCachePimpl
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_ImageCache.cpp]
impl ImageCache {

    /**
      | Checks the cache for an image with a particular
      | hashcode.
      | 
      | If there's an image in the cache with
      | this hashcode, it will be returned,
      | otherwise it will return an invalid
      | image.
      | 
      | -----------
      | @param hashCode
      | 
      | the hash code that was associated with
      | the image by addImageToCache() @see
      | addImageToCache
      |
      */
    pub fn get_from_hash_code(&mut self, hash_code: i64) -> Image {
        
        todo!();
        /*
            if (Pimpl::getInstanceWithoutCreating() != nullptr)
            return Pimpl::getInstanceWithoutCreating()->getFromHashCode (hashCode);

        return {};
        */
    }
    
    /**
      | Adds an image to the cache with a user-defined
      | hash-code.
      | 
      | The image passed-in will be referenced
      | (not copied) by the cache, so it's probably
      | a good idea not to draw into it after adding
      | it, otherwise this will affect all instances
      | of it that may be in use.
      | 
      | -----------
      | @param image
      | 
      | the image to add
      | ----------
      | @param hashCode
      | 
      | the hash-code to associate with it @see
      | getFromHashCode
      |
      */
    pub fn add_image_to_cache(&mut self, 
        image:     &Image,
        hash_code: i64)  {
        
        todo!();
        /*
            Pimpl::getInstance()->addImageToCache (image, hashCode);
        */
    }
    
    /**
      | Loads an image from a file, (or just returns
      | the image if it's already cached).
      | 
      | If the cache already contains an image
      | that was loaded from this file, that
      | image will be returned. Otherwise,
      | this method will try to load the file,
      | add it to the cache, and return it.
      | 
      | Remember that the image returned is
      | shared, so drawing into it might affect
      | other things that are using it! If you
      | want to draw on it, first call typename
      | Image::duplicateIfShared()
      | 
      | -----------
      | @param file
      | 
      | the file to try to load
      | 
      | -----------
      | @return
      | 
      | the image, or null if it there was an error
      | loading it @see getFromMemory, getFromCache,
      | ImageFileFormat::loadFrom
      |
      */
    pub fn get_from_file(&mut self, file: &File) -> Image {
        
        todo!();
        /*
            auto hashCode = file.hashCode64();
        auto image = getFromHashCode (hashCode);

        if (image.isNull())
        {
            image = ImageFileFormat::loadFrom (file);
            addImageToCache (image, hashCode);
        }

        return image;
        */
    }
    
    /**
      | Loads an image from an in-memory image
      | file, (or just returns the image if it's
      | already cached).
      | 
      | If the cache already contains an image
      | that was loaded from this block of memory,
      | that image will be returned. Otherwise,
      | this method will try to load the file,
      | add it to the cache, and return it.
      | 
      | Remember that the image returned is
      | shared, so drawing into it might affect
      | other things that are using it! If you
      | want to draw on it, first call typename
      | Image::duplicateIfShared()
      | 
      | -----------
      | @param imageData
      | 
      | the block of memory containing the image
      | data
      | ----------
      | @param dataSize
      | 
      | the data size in bytes
      | 
      | -----------
      | @return
      | 
      | the image, or an invalid image if it there
      | was an error loading it @see getFromMemory,
      | getFromCache, ImageFileFormat::loadFrom
      |
      */
    pub fn get_from_memory(&mut self, 
        image_data: *const c_void,
        data_size:  i32) -> Image {
        
        todo!();
        /*
            auto hashCode = (int64) (pointer_sized_int) imageData;
        auto image = getFromHashCode (hashCode);

        if (image.isNull())
        {
            image = ImageFileFormat::loadFrom (imageData, (size_t) dataSize);
            addImageToCache (image, hashCode);
        }

        return image;
        */
    }
    
    /**
      | Changes the amount of time before an
      | unused image will be removed from the
      | cache.
      | 
      | By default this is about 5 seconds.
      |
      */
    pub fn set_cache_timeout(&mut self, millisecs: i32)  {
        
        todo!();
        /*
            jassert (millisecs >= 0);
        Pimpl::getInstance()->cacheTimeout = (unsigned int) millisecs;
        */
    }
    
    /**
      | Releases any images in the cache that
      | aren't being referenced by active
      | 
      | Image objects.
      |
      */
    pub fn release_unused_images(&mut self)  {
        
        todo!();
        /*
            Pimpl::getInstance()->releaseUnusedImages();
        */
    }
}
