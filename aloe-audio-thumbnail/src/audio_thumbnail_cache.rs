crate::ix!();

pub fn get_thumbnail_cache_file_magic_header() -> i32 {
    
    todo!();
    /*
        return (int) ByteOrder::littleEndianInt ("ThmC");
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioThumbnailCache.h]

/**
  | An instance of this class is used to manage
  | multiple AudioThumbnail objects.
  | 
  | The cache runs a single background thread
  | that is shared by all the thumbnails
  | that need it, and it maintains a set of
  | low-res previews in memory, to avoid
  | having to re-scan audio files too often.
  | 
  | @see AudioThumbnail
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioThumbnailCache {
    thread:                  TimeSliceThread,
    thumbs:                  Vec<Box<ThumbnailCacheEntry>>,
    lock:                    CriticalSection,
    max_num_thumbs_to_store: i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioThumbnailCache.cpp]
impl AudioThumbnailCache {

    /**
      | Returns the thread that client thumbnails
      | can use.
      |
      */
    pub fn get_time_slice_thread(&mut self) -> &mut TimeSliceThread {
        
        todo!();
        /*
            return thread;
        */
    }

    /**
      | Creates a cache object.
      | 
      | The maxNumThumbsToStore parameter
      | lets you specify how many previews should
      | be kept in memory at once.
      |
      */
    pub fn new(max_num_thumbs: i32) -> Self {
    
        todo!();
        /*


            : thread ("thumb cache"),
          maxNumThumbsToStore (maxNumThumbs)

        jassert (maxNumThumbsToStore > 0);
        thread.startThread (2);
        */
    }
    
    pub fn find_thumb_for(&self, hash: i64) -> *mut ThumbnailCacheEntry {
        
        todo!();
        /*
            for (int i = thumbs.size(); --i >= 0;)
            if (thumbs.getUnchecked(i)->hash == hash)
                return thumbs.getUnchecked(i);

        return nullptr;
        */
    }
    
    pub fn find_oldest_thumb(&self) -> i32 {
        
        todo!();
        /*
            int oldest = 0;
        uint32 oldestTime = Time::getMillisecondCounter() + 1;

        for (int i = thumbs.size(); --i >= 0;)
        {
            const ThumbnailCacheEntry* const te = thumbs.getUnchecked(i);

            if (te->lastUsed < oldestTime)
            {
                oldest = i;
                oldestTime = te->lastUsed;
            }
        }

        return oldest;
        */
    }
    
    /**
      | Reloads the specified thumb if this
      | cache contains the appropriate stored
      | data.
      | 
      | This is called automatically by the
      | AudioThumbnail class, so you shouldn't
      | normally need to call it directly.
      |
      */
    pub fn load_thumb<RX: Read>(
        &mut self, 
        thumb:     &mut dyn AudioThumbnailBase<RX>,
        hash_code: i64

    ) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (ThumbnailCacheEntry* te = findThumbFor (hashCode))
        {
            te->lastUsed = Time::getMillisecondCounter();

            MemoryInputStream in (te->data, false);
            thumb.loadFrom (in);
            return true;
        }

        return loadNewThumb (thumb, hashCode);
        */
    }
    
    /**
      | Stores the cachable data from the specified
      | thumb in this cache.
      | 
      | This is called automatically by the
      | AudioThumbnail class, so you shouldn't
      | normally need to call it directly.
      |
      */
    pub fn store_thumb<RX: Read>(
        &mut self, 
        thumb:     &dyn AudioThumbnailBase<RX>,
        hash_code: i64

    )  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        ThumbnailCacheEntry* te = findThumbFor (hashCode);

        if (te == nullptr)
        {
            te = new ThumbnailCacheEntry (hashCode);

            if (thumbs.size() < maxNumThumbsToStore)
                thumbs.add (te);
            else
                thumbs.set (findOldestThumb(), te);
        }

        {
            MemoryOutputStream out (te->data, false);
            thumb.saveTo (out);
        }

        saveNewlyFinishedThumbnail (thumb, hashCode);
        */
    }
    
    /**
      | Clears out any stored thumbnails.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        thumbs.clear();
        */
    }
    
    /**
      | Tells the cache to forget about the thumb
      | with the given hashcode.
      |
      */
    pub fn remove_thumb(&mut self, hash_code: i64)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (int i = thumbs.size(); --i >= 0;)
            if (thumbs.getUnchecked(i)->hash == hashCode)
                thumbs.remove (i);
        */
    }
    
    /**
      | Attempts to re-load a saved cache of
      | thumbnails from a stream. The cache
      | data must have been written by the writeToStream()
      | method. This will replace all currently-loaded
      | thumbnails with the new data.
      |
      */
    pub fn read_from_stream<R: Read>(&mut self, source: &mut R) -> bool {
        
        todo!();
        /*
            if (source.readInt() != getThumbnailCacheFileMagicHeader())
            return false;

        const ScopedLock sl (lock);
        clear();
        int numThumbnails = jmin (maxNumThumbsToStore, source.readInt());

        while (--numThumbnails >= 0 && ! source.isExhausted())
            thumbs.add (new ThumbnailCacheEntry (source));

        return true;
        */
    }
    
    /**
      | Writes all currently-loaded cache
      | data to a stream. The resulting data
      | can be re-loaded with readFromStream().
      |
      */
    pub fn write_to_stream<W: Write>(&mut self, out: &mut W)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        out.writeInt (getThumbnailCacheFileMagicHeader());
        out.writeInt (thumbs.size());

        for (int i = 0; i < thumbs.size(); ++i)
            thumbs.getUnchecked(i)->write (out);
        */
    }
    
    /**
      | This can be overridden to provide a custom
      | callback for saving thumbnails once
      | they have finished being loaded.
      |
      */
    pub fn save_newly_finished_thumbnail<RX: Read>(
        &mut self, 
        _0: &dyn AudioThumbnailBase<RX>,
        _1: i64

    ) {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | This can be overridden to provide a custom
      | callback for loading thumbnails from
      | pre-saved files to save the cache the
      | trouble of having to create them.
      |
      */
    pub fn load_new_thumb<RX: Read>(
        &mut self, 
        _0: &mut dyn AudioThumbnailBase<RX>,
        _1: i64

    ) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
