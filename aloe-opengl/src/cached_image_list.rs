crate::ix!();

/**
  | This list persists in the OpenGLContext,
  | and will re-use cached textures which
  | are created from Images.
  |
  */
#[no_copy]
#[leak_detector]
pub struct CachedImageList<'a> {
    base:           ReferenceCountedObject,
    context:        &'a mut OpenGLContext<'a>,
    images:         Vec<Box<CachedImage<'a>>>,
    total_size:     usize, // default = 0
    max_cache_size: usize,
}

impl<'a> ImagePixelDataListener for CachedImageList<'a> {

    fn image_data_changed(&mut self, im: *mut ImagePixelData)  {
        
        todo!();
        /*
            if (auto* c = findCachedImage (im))
                c->textureNeedsReloading = true;
        */
    }
    
    fn image_data_being_deleted(&mut self, im: *mut ImagePixelData)  {
        
        todo!();
        /*
            for (int i = images.size(); --i >= 0;)
            {
                auto& ci = *images.getUnchecked(i);

                if (ci.pixelData == im)
                {
                    if (canUseContext())
                    {
                        totalSize -= ci.imageSize;
                        images.remove (i);
                    }
                    else
                    {
                        ci.pixelData = nullptr;
                    }

                    break;
                }
            }
        */
    }
}

impl<'a> CachedImageList<'a> {

    pub fn new(c: &mut OpenGLContext) -> Self {
    
        todo!();
        /*
        : context(c),
        : max_cache_size(c.getImageCacheSize()),
        */
    }
    
    pub fn get(c: &mut OpenGLContext) -> *mut CachedImageList<'a> {
        
        todo!();
        /*
            const char cacheValueID[] = "CachedImages";
            auto list = static_cast<CachedImageList*> (c.getAssociatedObject (cacheValueID));

            if (list == nullptr)
            {
                list = new CachedImageList (c);
                c.setAssociatedObject (cacheValueID, list);
            }

            return list;
        */
    }
    
    pub fn get_texture_for(&mut self, image: &Image) -> TextureInfo {
        
        todo!();
        /*
            auto pixelData = image.getPixelData();
            auto* c = findCachedImage (pixelData);

            if (c == nullptr)
            {
                if (auto fb = OpenGLImageType::getFrameBufferFrom (image))
                {
                    TextureInfo t;
                    t.textureID = fb->getTextureID();
                    t.imageWidth = image.getWidth();
                    t.imageHeight = image.getHeight();
                    t.fullWidthProportion  = 1.0f;
                    t.fullHeightProportion = 1.0f;

                    return t;
                }

                c = images.add (new CachedImage (*this, pixelData));
                totalSize += c->imageSize;

                while (totalSize > maxCacheSize && images.size() > 1 && totalSize > 0)
                    removeOldestItem();
            }

            return c->getTextureInfo();
        */
    }
    
    pub fn can_use_context(&self) -> bool {
        
        todo!();
        /*
            return OpenGLContext::getCurrentContext() == &context;
        */
    }
    
    
    pub fn find_cached_image(&self, pixel_data: *mut ImagePixelData) -> *mut CachedImage {
        
        todo!();
        /*
            for (auto& i : images)
                if (i->pixelData == pixelData)
                    return i;

            return {};
        */
    }
    
    pub fn remove_oldest_item(&mut self)  {
        
        todo!();
        /*
            CachedImage* oldest = nullptr;

            for (auto& i : images)
                if (oldest == nullptr || i->lastUsed < oldest->lastUsed)
                    oldest = i;

            if (oldest != nullptr)
            {
                totalSize -= oldest->imageSize;
                images.removeObject (oldest);
            }
        */
    }
}
