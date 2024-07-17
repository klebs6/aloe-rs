crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CachedImage<'a> {
    owner:                   &'a mut CachedImageList<'a>,
    pixel_data:              *mut ImagePixelData,
    texture:                 OpenGLTexture<'a>,
    last_used:               Time,
    image_size:              usize,
    texture_needs_reloading: bool, // default = true
}

pub type CachedImageListPtr<'a> = ReferenceCountedObjectPtr<CachedImageList<'a>>;

impl<'a> Drop for CachedImage<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (pixelData != nullptr)
                    pixelData->listeners.remove (&owner);
        */
    }
}

impl<'a> CachedImage<'a> {

    pub fn new(
        list: &mut CachedImageList,
        im:   *mut ImagePixelData) -> Self {
    
        todo!();
        /*


            : owner (list), pixelData (im),
                  lastUsed (Time::getCurrentTime()),
                  imageSize ((size_t) (im->width * im->height))

                pixelData->listeners.add (&owner);
        */
    }
    
    pub fn get_texture_info(&mut self) -> TextureInfo {
        
        todo!();
        /*
            if (pixelData == nullptr)
                    return {};

                TextureInfo t;

                if (textureNeedsReloading)
                {
                    textureNeedsReloading = false;
                    texture.loadImage (Image (*pixelData));
                }

                t.textureID = texture.getTextureID();
                t.imageWidth = pixelData->width;
                t.imageHeight = pixelData->height;
                t.fullWidthProportion  = (float) t.imageWidth  / (float) texture.getWidth();
                t.fullHeightProportion = (float) t.imageHeight / (float) texture.getHeight();

                lastUsed = Time::getCurrentTime();
                return t;
        */
    }
}
